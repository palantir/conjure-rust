// Copyright 2022 Palantir Technologies, Inc.
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
use crate::path::{self, PathComponent};
use crate::Asyncness;
use http::HeaderName;
use percent_encoding::AsciiSet;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashMap;
use structmeta::StructMeta;
use syn::{
    parse_macro_input, Error, FnArg, ItemTrait, LitStr, Meta, Pat, ReturnType, TraitItem,
    TraitItemFn, Type,
};

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

pub fn generate(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as ItemTrait);

    let client = generate_client(&mut item);

    quote! {
        #item

        #client
    }
    .into()
}

fn generate_client(trait_: &mut ItemTrait) -> TokenStream {
    let vis = &trait_.vis;
    let trait_name = &trait_.ident;
    let type_name = Ident::new(&format!("{}Client", trait_name), trait_name.span());

    let asyncness = match Asyncness::resolve(trait_) {
        Ok(asyncness) => asyncness,
        Err(e) => return e.into_compile_error(),
    };

    let service_trait = match asyncness {
        Asyncness::Sync => quote!(Service),
        Asyncness::Async => quote!(AsyncService),
    };

    let impl_attrs = match asyncness {
        Asyncness::Sync => quote!(),
        Asyncness::Async => quote!(#[conjure_http::private::async_trait]),
    };

    let where_ = match asyncness {
        Asyncness::Sync => quote!(C: conjure_http::client::Client),
        Asyncness::Async => quote! {
            C: conjure_http::client::AsyncClient + Sync + Send,
            C::ResponseBody: 'static + Send,
        },
    };

    let methods = trait_
        .items
        .iter_mut()
        .filter_map(|item| match item {
            TraitItem::Fn(meth) => Some(meth),
            _ => None,
        })
        .map(|m| generate_client_method(trait_name, asyncness, m));

    quote! {
        #vis struct #type_name<C> {
            client: C,
        }

        impl<C> conjure_http::client::#service_trait<C> for #type_name<C> {
            fn new(client: C) -> Self {
                #type_name { client }
            }
        }

        #impl_attrs
        impl<C> #trait_name for #type_name<C>
        where #where_
        {
            #(#methods)*
        }
    }
}

fn generate_client_method(
    trait_name: &Ident,
    asyncness: Asyncness,
    method: &mut TraitItemFn,
) -> TokenStream {
    let mut endpoint_attrs = method
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("endpoint"));
    let Some(endpoint_attr) = endpoint_attrs.next() else {
        return Error::new_spanned(method, "missing #[endpoint] attribute").into_compile_error();
    };
    let endpoint = match endpoint_attr.parse_args::<EndpointConfig>() {
        Ok(endpoint) => endpoint,
        Err(e) => return e.into_compile_error(),
    };
    let duplicates = endpoint_attrs
        .map(|a| Error::new_spanned(a, "duplicate #[endpoint] attribute").into_compile_error())
        .collect::<Vec<_>>();
    if !duplicates.is_empty() {
        return quote!(#(#duplicates)*);
    }

    let async_ = match asyncness {
        Asyncness::Sync => quote!(),
        Asyncness::Async => quote!(async),
    };

    let client_trait = match asyncness {
        Asyncness::Sync => quote!(Client),
        Asyncness::Async => quote!(AsyncClient),
    };

    let await_ = match asyncness {
        Asyncness::Sync => quote!(),
        Asyncness::Async => quote!(.await),
    };

    let request_args = match method
        .sig
        .inputs
        .iter_mut()
        .flat_map(|a| ArgType::new(a).transpose())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(request_args) => request_args,
        Err(e) => return e.into_compile_error(),
    };

    let name = &method.sig.ident;
    let args = &method.sig.inputs;
    let ret = &method.sig.output;

    let request = quote!(__request);
    let response = quote!(__response);
    let http_method = &endpoint.method;

    let create_request = create_request(asyncness, &request, &request_args);
    let add_path = add_path(&request, &request_args, &endpoint);
    let add_accept = add_accept(asyncness, &request, &endpoint, &method.sig.output);
    let add_auth = add_auth(&request, &request_args);
    let add_headers = add_headers(&request, &request_args);
    let add_endpoint = add_endpoint(trait_name, method, &endpoint, &request);
    let handle_response = handle_response(asyncness, &endpoint, &response);

    quote! {
        #async_ fn #name(#args) #ret {
            #create_request
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

fn create_request(asyncness: Asyncness, request: &TokenStream, args: &[ArgType]) -> TokenStream {
    let mut it = args.iter().filter_map(|a| match a {
        ArgType::Body(arg) => Some(arg),
        _ => None,
    });
    let Some(arg) = it.next() else {
        let body = match asyncness {
            Asyncness::Sync => quote!(RequestBody),
            Asyncness::Async => quote!(AsyncRequestBody),
        };
        return quote! {
            let mut #request = conjure_http::private::Request::new(
                conjure_http::client::#body::Empty,
            );
        };
    };

    if let Some(arg) = it.next() {
        return Error::new_spanned(&arg.ident, "only one #[body] argument allowed")
            .into_compile_error();
    }

    let trait_ = match asyncness {
        Asyncness::Sync => quote!(SerializeRequest),
        Asyncness::Async => quote!(AsyncSerializeRequest),
    };

    let serializer = arg.attr.serializer.as_ref().map_or_else(
        || quote!(conjure_http::client::ConjureRequestSerializer),
        |t| quote!(#t),
    );
    let ident = &arg.ident;

    quote! {
        let __content_type = <
            #serializer as conjure_http::client::#trait_<_, C::BodyWriter>
        >::content_type(&#ident);
        let __content_length = <
            #serializer as conjure_http::client::#trait_<_, C::BodyWriter>
        >::content_length(&#ident);
        let __body = <
            #serializer as conjure_http::client::#trait_<_, C::BodyWriter>
        >::serialize(#ident)?;

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

fn add_path(
    request: &TokenStream,
    request_args: &[ArgType],
    endpoint: &EndpointConfig,
) -> TokenStream {
    let builder = quote!(__path);

    let path_writes = add_path_components(&endpoint.path, &builder, request_args);

    let query_params = request_args
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

fn add_path_components(
    path_lit: &LitStr,
    builder: &TokenStream,
    request_args: &[ArgType],
) -> TokenStream {
    let path = match path::parse(path_lit) {
        Ok(path) => path,
        Err(e) => return e.into_compile_error(),
    };

    let path_params = request_args
        .iter()
        .filter_map(|a| match a {
            ArgType::Path(param) => Some((param.ident.to_string(), param)),
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    let mut path_writes = vec![];
    let mut literal_buf = String::new();
    for component in path {
        match component {
            PathComponent::Literal(lit) => {
                literal_buf.push('/');
                literal_buf.push_str(
                    &percent_encoding::percent_encode(lit.as_bytes(), COMPONENT).to_string(),
                );
            }
            PathComponent::Parameter(param) => {
                if !literal_buf.is_empty() {
                    path_writes.push(quote! {
                        #builder.push_literal(#literal_buf);
                    });
                    literal_buf = String::new();
                }

                let Some(param) = path_params.get(&param) else {
                    path_writes.push(
                        Error::new_spanned(
                            path_lit,
                            format_args!("invalid path parameter `{param}`"),
                        )
                        .into_compile_error(),
                    );
                    continue;
                };

                let ident = &param.ident;
                let encoder = param.attr.encoder.as_ref().map_or_else(
                    || quote!(conjure_http::client::DisplayParamEncoder),
                    |e| quote!(#e),
                );

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

fn add_query_arg(builder: &TokenStream, arg: &Arg<ParamAttr>) -> TokenStream {
    let ident = &arg.ident;
    let name =
        percent_encoding::percent_encode(arg.attr.name.value().as_bytes(), COMPONENT).to_string();
    let encoder = arg.attr.encoder.as_ref().map_or_else(
        || quote!(conjure_http::client::DisplayParamEncoder),
        |e| quote!(#e),
    );

    quote! {
        let __query_args = <#encoder as conjure_http::client::EncodeParam<_>>::encode(#ident)?;
        for __query_arg in __query_args {
            #builder.push_query_parameter_raw(#name, &__query_arg);
        }
    }
}

fn add_accept(
    asyncness: Asyncness,
    request: &TokenStream,
    endpoint: &EndpointConfig,
    ret_ty: &ReturnType,
) -> TokenStream {
    let Some(accept) = &endpoint.accept else {
        return quote!();
    };

    let trait_ = match asyncness {
        Asyncness::Sync => quote!(DeserializeResponse),
        Asyncness::Async => quote!(AsyncDeserializeResponse),
    };

    let ret = match ret_ty {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, ty) => quote!(#ty),
    };

    quote! {
        let __accept = <#accept as conjure_http::client::#trait_<
            <#ret as conjure_http::private::ExtractOk>::Ok,
            C::ResponseBody,
        >>::accept();
        if let Some(__accept) = __accept {
            #request.headers_mut().insert(conjure_http::private::header::ACCEPT, __accept);
        }
    }
}

fn add_auth(request: &TokenStream, args: &[ArgType]) -> TokenStream {
    let mut it = args.iter().filter_map(|a| match a {
        ArgType::Auth(auth) => Some(auth),
        _ => None,
    });
    let Some(auth_param) = it.next() else {
        return quote!();
    };

    if let Some(param) = it.next() {
        return Error::new_spanned(&param.ident, "only one #[auth] argument allowed")
            .into_compile_error();
    }

    let pat = &auth_param.ident;

    match &auth_param.attr.cookie_name {
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

fn add_headers(request: &TokenStream, args: &[ArgType]) -> TokenStream {
    let add_headers = args
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

fn add_header(request: &TokenStream, arg: &Arg<ParamAttr>) -> TokenStream {
    let header_name = match arg.attr.name.value().parse::<HeaderName>() {
        Ok(header_name) => header_name,
        Err(e) => return Error::new_spanned(&arg.attr.name, e).into_compile_error(),
    };

    let ident = &arg.ident;
    let name = header_name.as_str();
    let encoder = arg.attr.encoder.as_ref().map_or_else(
        || quote!(conjure_http::client::DisplayHeaderEncoder),
        |v| quote!(#v),
    );

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

fn add_endpoint(
    trait_name: &Ident,
    method: &TraitItemFn,
    endpoint: &EndpointConfig,
    request: &TokenStream,
) -> TokenStream {
    let service = trait_name.to_string();
    let name = method.sig.ident.to_string();
    let path = &endpoint.path;

    quote! {
        #request.extensions_mut().insert(conjure_http::client::Endpoint::new(
            #service,
            std::option::Option::Some(std::env!("CARGO_PKG_VERSION")),
            #name,
            #path,
        ));
    }
}

fn handle_response(
    asyncness: Asyncness,
    endpoint: &EndpointConfig,
    response: &TokenStream,
) -> TokenStream {
    match &endpoint.accept {
        Some(accept) => {
            let trait_ = match asyncness {
                Asyncness::Sync => quote!(DeserializeResponse),
                Asyncness::Async => quote!(AsyncDeserializeResponse),
            };
            let await_ = match asyncness {
                Asyncness::Sync => quote!(),
                Asyncness::Async => quote!(.await),
            };

            quote! {
                <#accept as conjure_http::client::#trait_<_, _>>::deserialize(#response) #await_
            }
        }
        None => quote!(conjure_http::private::Result::Ok(())),
    }
}

#[derive(StructMeta)]
struct EndpointConfig {
    method: Ident,
    path: LitStr,
    accept: Option<Type>,
}

enum ArgType {
    Path(Arg<PathAttr>),
    Query(Arg<ParamAttr>),
    Header(Arg<ParamAttr>),
    Auth(Arg<AuthAttr>),
    Body(Arg<BodyAttr>),
}

struct Arg<T> {
    ident: Ident,
    attr: T,
}

#[derive(StructMeta)]
struct PathAttr {
    encoder: Option<Type>,
}

#[derive(StructMeta)]
struct ParamAttr {
    name: LitStr,
    encoder: Option<Type>,
}

#[derive(StructMeta)]
struct AuthAttr {
    cookie_name: Option<LitStr>,
}

#[derive(StructMeta)]
struct BodyAttr {
    serializer: Option<Type>,
}

impl ArgType {
    fn new(arg: &mut FnArg) -> syn::Result<Option<Self>> {
        // Ignore the self arg.
        let FnArg::Typed(pat_type) = arg else {
            return Ok(None);
        };

        // FIXME we should probably just rename the arguments in our impl?
        let ident = match &*pat_type.pat {
            Pat::Ident(pat_ident) => &pat_ident.ident,
            _ => {
                return Err(Error::new_spanned(
                    &pat_type.pat,
                    "expected an ident pattern",
                ))
            }
        };

        let mut type_ = None;

        // FIXME detect multiple attrs
        for attr in &pat_type.attrs {
            if attr.path().is_ident("path") {
                let attr = match attr.meta {
                    Meta::Path(_) => PathAttr { encoder: None },
                    _ => attr.parse_args()?,
                };
                type_ = Some(ArgType::Path(Arg {
                    ident: ident.clone(),
                    attr,
                }));
            } else if attr.path().is_ident("query") {
                type_ = Some(ArgType::Query(Arg {
                    ident: ident.clone(),
                    attr: attr.parse_args()?,
                }));
            } else if attr.path().is_ident("header") {
                type_ = Some(ArgType::Header(Arg {
                    ident: ident.clone(),
                    attr: attr.parse_args()?,
                }));
            } else if attr.path().is_ident("auth") {
                let attr = match attr.meta {
                    Meta::Path(_) => AuthAttr { cookie_name: None },
                    _ => attr.parse_args()?,
                };
                type_ = Some(ArgType::Auth(Arg {
                    ident: ident.clone(),
                    attr,
                }));
            } else if attr.path().is_ident("body") {
                let attr = match attr.meta {
                    Meta::Path(_) => BodyAttr { serializer: None },
                    _ => attr.parse_args()?,
                };
                type_ = Some(ArgType::Body(Arg {
                    ident: ident.clone(),
                    attr,
                }));
            }
        }

        // Rust doesn't support "helper" attributes in attribute macros, so we need to strip out our
        // helper attributes on arguments.
        strip_arg_attrs(arg);

        type_
            .ok_or_else(|| Error::new_spanned(arg, "missing argument type annotation"))
            .map(Some)
    }
}

fn strip_arg_attrs(arg: &mut FnArg) {
    let FnArg::Typed(arg) = arg else { return };

    arg.attrs.retain(|attr| {
        !["path", "query", "header", "body", "auth"]
            .iter()
            .any(|v| attr.path().is_ident(v))
    });
}
