// Copyright 2023 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::path::PathSegment;
use derive_getters::Getters;
use percent_encoding::AsciiSet;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use staged_builder::staged_builder;
use std::collections::HashMap;
use syn::{Attribute, Expr, Generics, LitStr, Type, Visibility};

// https://url.spec.whatwg.org/#query-percent-encode-set
const QUERY: &AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>');

// https://url.spec.whatwg.org/#path-percent-encode-set
const PATH: &AsciiSet = &QUERY.add(b'?').add(b'`').add(b'{').add(b'}');

// https://url.spec.whatwg.org/#userinfo-percent-encode-set
const USERINFO: &AsciiSet = &PATH
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'|');

// https://url.spec.whatwg.org/#component-percent-encode-set
const COMPONENT: &AsciiSet = &USERINFO.add(b'$').add(b'%').add(b'&').add(b'+').add(b',');

#[staged_builder]
#[derive(Getters)]
pub struct Service {
    #[builder(list(item(type = Attribute)))]
    attrs: Vec<Attribute>,
    vis: Visibility,
    name: Ident,
    version: Expr,
    generics: Generics,
    #[builder(default, into)]
    request_writer_param: Option<Ident>,
    #[builder(default, into)]
    response_body_param: Option<Ident>,
    #[builder(default, into)]
    r#trait: Option<Ident>,
    r#async: bool,
    #[builder(list(item(type = Endpoint)))]
    endpoints: Vec<Endpoint>,
}

#[staged_builder]
#[derive(Getters)]
pub struct Endpoint {
    #[builder(list(item(type = Attribute)))]
    attrs: Vec<Attribute>,
    method: Ident,
    #[builder(into)]
    path: LitStr,
    #[builder(list(item(type = PathSegment)))]
    path_segments: Vec<PathSegment>,
    accept: Type,
    name: Ident,
    #[builder(list(item(type = ArgType)))]
    args: Vec<ArgType>,
    return_type: Type,
}

pub enum ArgType {
    Path(PathArg),
    Query(QueryArg),
    Header(HeaderArg),
    Auth(AuthArg),
    Body(BodyArg),
}

impl ArgType {
    fn name(&self) -> &Ident {
        match self {
            ArgType::Path(a) => &a.name,
            ArgType::Query(a) => &a.name,
            ArgType::Header(a) => &a.name,
            ArgType::Auth(a) => &a.name,
            ArgType::Body(a) => &a.name,
        }
    }

    fn r#type(&self) -> &Type {
        match self {
            ArgType::Path(a) => &a.r#type,
            ArgType::Query(a) => &a.r#type,
            ArgType::Header(a) => &a.r#type,
            ArgType::Auth(a) => &a.r#type,
            ArgType::Body(a) => &a.r#type,
        }
    }
}

#[staged_builder]
#[derive(Getters)]
pub struct PathArg {
    name: Ident,
    r#type: Type,
    encoder: Type,
}

#[staged_builder]
#[derive(Getters)]
pub struct QueryArg {
    name: Ident,
    r#type: Type,
    key: LitStr,
    encoder: Type,
}

#[staged_builder]
#[derive(Getters)]
pub struct HeaderArg {
    name: Ident,
    r#type: Type,
    header_name: LitStr,
    encoder: Type,
}

#[staged_builder]
#[derive(Getters)]
pub struct AuthArg {
    name: Ident,
    r#type: Type,
    #[builder(default, into)]
    cookie_name: Option<LitStr>,
}

#[staged_builder]
#[derive(Getters)]
pub struct BodyArg {
    name: Ident,
    r#type: Type,
    serializer: Type,
}

pub fn generate(service: &Service) -> TokenStream {
    let attrs = &service.attrs;
    let vis = &service.vis;
    let name = &service.name;

    let service_trait = if service.r#async {
        quote!(AsyncService)
    } else {
        quote!(Service)
    };

    let impl_attrs = if service.r#async {
        quote!(#[conjure_http::private::async_trait])
    } else {
        quote!()
    };

    let (_, type_generics, _) = service.generics.split_for_impl();
    let trait_impl = match &service.r#trait {
        Some(name) => quote!(#name #type_generics for),
        None => quote!(),
    };

    let mut impl_generics = service.generics.clone();

    let client_param = quote!(__C);
    impl_generics
        .params
        .push(syn::parse2(client_param.clone()).unwrap());

    let where_clause = impl_generics.make_where_clause();
    let client_trait = if service.r#async {
        quote!(AsyncClient)
    } else {
        quote!(Client)
    };
    let mut client_bindings = vec![];
    if let Some(param) = &service.request_writer_param {
        client_bindings.push(quote!(BodyWriter = #param));
    }
    if let Some(param) = &service.response_body_param {
        client_bindings.push(quote!(ResponseBody = #param));
    }
    let extra_client_predicates = if service.r#async {
        quote!(+ conjure_http::private::Sync + conjure_http::private::Send)
    } else {
        quote!()
    };
    where_clause.predicates.push(
        syn::parse2(quote! {
            #client_param: conjure_http::client::#client_trait<#(#client_bindings),*> #extra_client_predicates
        })
        .unwrap());
    if service.r#async {
        where_clause.predicates.push(
            syn::parse2(quote!(#client_param::ResponseBody: 'static + conjure_http::private::Send))
                .unwrap(),
        );
    }

    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();

    let methods = service
        .endpoints
        .iter()
        .map(|endpoint| generate_client_method(&client_param, service, endpoint));

    quote! {
        #(#attrs)*
        #[derive(Clone, Debug)]
        #vis struct #name<C> {
            client: C,
        }

        impl<C> conjure_http::client::#service_trait<C> for #name<C> {
            fn new(client: C) -> Self {
                #name { client }
            }
        }

        #impl_attrs
        impl #impl_generics #trait_impl #name<#client_param>
        #where_clause
        {
            #(#methods)*
        }
    }
}

fn generate_client_method(
    client_param: &TokenStream,
    service: &Service,
    endpoint: &Endpoint,
) -> TokenStream {
    let attrs = &endpoint.attrs;

    let vis = if service.r#trait.is_some() {
        quote!()
    } else {
        quote!(pub)
    };

    let async_ = if service.r#async {
        quote!(async)
    } else {
        quote!()
    };

    let client_trait = if service.r#async {
        quote!(AsyncClient)
    } else {
        quote!(Client)
    };

    let await_ = if service.r#async {
        quote!(.await)
    } else {
        quote!()
    };

    let name = &endpoint.name;
    let args = endpoint.args.iter().map(|a| {
        let name = a.name();
        let ty = a.r#type();
        quote!(#name: #ty)
    });
    let return_type = &endpoint.return_type;

    let request = quote!(__request);
    let response = quote!(__response);
    let http_method = &endpoint.method;

    let create_request = create_request(client_param, &request, service, endpoint);
    let add_path = add_path(&request, endpoint);
    let add_accept = add_accept(client_param, &request, service, endpoint);
    let add_auth = add_auth(&request, endpoint);
    let add_headers = add_headers(&request, endpoint);
    let add_endpoint = add_endpoint(&request, service, endpoint);
    let handle_response = handle_response(&response, service, endpoint);

    quote! {
        #(#attrs)*
        #vis #async_ fn #name(&self #(, #args)*) -> #return_type {
            #create_request;
            *#request.method_mut() = conjure_http::private::Method::#http_method;
            #add_path
            #add_accept
            #add_auth
            #add_headers
            #add_endpoint
            let #response = conjure_http::client::#client_trait::send(&self.client, #request) #await_?;
            #handle_response
        }
    }
}

fn create_request(
    client_param: &TokenStream,
    request: &TokenStream,
    service: &Service,
    endpoint: &Endpoint,
) -> TokenStream {
    let arg = endpoint.args.iter().find_map(|a| match a {
        ArgType::Body(arg) => Some(arg),
        _ => None,
    });
    let Some(arg) = arg else {
        let body = if service.r#async {
            quote!(AsyncRequestBody)
        } else {
            quote!(RequestBody)
        };
        return quote! {
            let mut #request = conjure_http::private::Request::new(
                conjure_http::client::#body::Empty,
            );
        }
    };

    let trait_ = if service.r#async {
        quote!(AsyncSerializeRequest)
    } else {
        quote!(SerializeRequest)
    };

    let serializer = &arg.serializer;
    let name = &arg.name;

    quote! {
        let __content_type = <
            #serializer as conjure_http::client::#trait_<_, #client_param::BodyWriter>
        >::content_type(&#name);
        let __content_length = <
            #serializer as conjure_http::client::#trait_<_, #client_param::BodyWriter>
        >::content_length(&#name);
        let __body = <
            #serializer as conjure_http::client::#trait_<_, #client_param::BodyWriter>
        >::serialize(#name)?;

        let mut #request = conjure_http::private::Request::new(__body);
        #request.headers_mut().insert(
            conjure_http::private::header::CONTENT_TYPE,
            __content_type,
        );
        if let conjure_http::private::Option::Some(__content_length) = __content_length {
            #request.headers_mut().insert(
                conjure_http::private::header::CONTENT_LENGTH,
                conjure_http::private::http::HeaderValue::from(__content_length),
            );
        }
    }
}

fn add_path(request: &TokenStream, endpoint: &Endpoint) -> TokenStream {
    let builder = quote!(__path);

    let path_writes = add_path_components(&builder, endpoint);

    let query_params = endpoint
        .args
        .iter()
        .filter_map(|arg| match arg {
            ArgType::Query(arg) => Some(arg),
            _ => None,
        })
        .map(|arg| add_query_arg(&builder, arg));

    quote! {
        let mut #builder = conjure_http::private::UriBuilder::new();
        #path_writes
        #(#query_params)*
        *#request.uri_mut() = #builder.build();
    }
}

fn add_path_components(builder: &TokenStream, endpoint: &Endpoint) -> TokenStream {
    let path_params = endpoint
        .args
        .iter()
        .filter_map(|a| match a {
            ArgType::Path(param) => Some((param.name.to_string(), param)),
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    let mut path_writes = vec![];
    let mut literal_buf = String::new();
    for component in &endpoint.path_segments {
        match component {
            PathSegment::Literal(lit) => {
                literal_buf.push('/');
                literal_buf.push_str(
                    &percent_encoding::percent_encode(lit.as_bytes(), COMPONENT).to_string(),
                );
            }
            PathSegment::Parameter { name, .. } => {
                if !literal_buf.is_empty() {
                    path_writes.push(quote! {
                        #builder.push_literal(#literal_buf);
                    });
                    literal_buf = String::new();
                }

                let param = path_params[name];

                let ident = &param.name;
                let encoder = &param.encoder;

                path_writes.push(quote! {
                    let __path_args = <#encoder as conjure_http::client::EncodeParam<_>>::encode(#ident)?;
                    for __path_arg in __path_args {
                        #builder.push_path_parameter_raw(&__path_arg);
                    }
                });
            }
        }
    }

    if !literal_buf.is_empty() {
        path_writes.push(quote! {
            #builder.push_literal(#literal_buf);
        });
    }

    quote! {
        #(#path_writes)*
    }
}

fn add_query_arg(builder: &TokenStream, arg: &QueryArg) -> TokenStream {
    let ident = &arg.name;
    let name = percent_encoding::percent_encode(arg.key.value().as_bytes(), COMPONENT).to_string();
    let encoder = &arg.encoder;

    quote! {
        let __query_args = <#encoder as conjure_http::client::EncodeParam<_>>::encode(#ident)?;
        for __query_arg in __query_args {
            #builder.push_query_parameter_raw(#name, &__query_arg);
        }
    }
}

fn add_accept(
    client_param: &TokenStream,
    request: &TokenStream,
    service: &Service,
    endpoint: &Endpoint,
) -> TokenStream {
    let accept = &endpoint.accept;

    let trait_ = if service.r#async {
        quote!(AsyncDeserializeResponse)
    } else {
        quote!(DeserializeResponse)
    };

    let return_type = &endpoint.return_type;

    quote! {
        let __accept = <#accept as conjure_http::client::#trait_<
            <#return_type as conjure_http::private::ExtractOk>::Ok,
            #client_param::ResponseBody,
        >>::accept();
        if let conjure_http::private::Option::Some(__accept) = __accept {
            #request.headers_mut().insert(conjure_http::private::header::ACCEPT, __accept);
        }
    }
}

fn add_auth(request: &TokenStream, endpoint: &Endpoint) -> TokenStream {
    let auth_param = endpoint.args.iter().find_map(|a| match a {
        ArgType::Auth(auth) => Some(auth),
        _ => None,
    });
    let Some(auth_param) = auth_param else {
        return quote!();
    };

    let pat = &auth_param.name;

    match &auth_param.cookie_name {
        Some(cookie_name) => {
            let prefix = format!("{}=", cookie_name.value());
            quote! {
                conjure_http::private::encode_cookie_auth(&mut #request, #prefix, #pat);
            }
        }
        None => quote! {
            conjure_http::private::encode_header_auth(&mut #request, #pat);
        },
    }
}

fn add_headers(request: &TokenStream, endpoint: &Endpoint) -> TokenStream {
    let add_headers = endpoint
        .args
        .iter()
        .filter_map(|arg| match arg {
            ArgType::Header(arg) => Some(arg),
            _ => None,
        })
        .map(|arg| add_header(request, arg));

    quote! {
        #(#add_headers)*
    }
}

fn add_header(request: &TokenStream, arg: &HeaderArg) -> TokenStream {
    let ident = &arg.name;
    let name = arg.header_name.value().to_ascii_lowercase();
    let encoder = &arg.encoder;

    quote! {
        let __header_values = <#encoder as conjure_http::client::EncodeHeader<_>>::encode(#ident)?;
        for __header_value in __header_values {
            #request.headers_mut().append(
                conjure_http::private::header::HeaderName::from_static(#name),
                __header_value,
            );
        }
    }
}

fn add_endpoint(request: &TokenStream, service: &Service, endpoint: &Endpoint) -> TokenStream {
    let service_name = service.name.to_string();
    let version = &service.version;
    let name = endpoint.name.to_string();
    let path = &endpoint.path;

    quote! {
        #request.extensions_mut().insert(conjure_http::client::Endpoint::new(
            #service_name,
            #version,
            #name,
            #path,
        ));
    }
}

fn handle_response(response: &TokenStream, service: &Service, endpoint: &Endpoint) -> TokenStream {
    let accept = &endpoint.accept;
    let trait_ = if service.r#async {
        quote!(AsyncDeserializeResponse)
    } else {
        quote!(DeserializeResponse)
    };
    let await_ = if service.r#async {
        quote!(.await)
    } else {
        quote!()
    };

    quote! {
        <#accept as conjure_http::client::#trait_<_, _>>::deserialize(#response) #await_
    }
}
