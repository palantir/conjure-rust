// Copyright 2019 Palantir Technologies, Inc.
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
use proc_macro2::TokenStream;
use quote::quote;

use crate::context::Context;
use crate::types::{
    ArgumentDefinition, AuthType, EndpointDefinition, HeaderParameterType, ParameterType,
    ServiceDefinition, Type,
};

pub fn generate(ctx: &Context, def: &ServiceDefinition) -> TokenStream {
    let docs = ctx.docs(def.docs());
    let name = ctx.type_name(&format!("{}Client", def.service_name().name()));

    let endpoints = def
        .endpoints()
        .iter()
        .map(|e| generate_endpoint(ctx, def, e));

    quote! {
        #docs
        #[derive(Clone, Debug)]
        pub struct #name<T>(T);

        impl<T> #name<T>
        where
            T: conjure_http::client::Client
        {
            /// Creates a new client.
            #[inline]
            pub fn new(client: T) -> #name<T> {
                #name(client)
            }

            #(#endpoints)*
        }
    }
}

fn generate_endpoint(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
) -> TokenStream {
    let docs = ctx.docs(endpoint.docs());
    let deprecated = match endpoint.deprecated() {
        Some(docs) => {
            let docs = &**docs;
            quote! {
                #[deprecated(note = #docs)]
            }
        }
        None => quote!(),
    };
    let name = ctx.field_name(endpoint.endpoint_name());

    let body_arg = body_arg(endpoint);
    let params = params(ctx, body_arg);

    let auth_arg = auth_arg(endpoint);
    let args = endpoint.args().iter().map(|a| {
        let name = ctx.field_name(a.arg_name());
        let ty = arg_type(ctx, def, a);
        quote!(#name: #ty)
    });

    let ret = return_type(ctx, endpoint);
    let ret_name = return_type_name(ctx, def, &ret);

    let where_ = where_(ctx, body_arg);
    let setup_body = setup_body(ctx, body_arg);

    let request = quote!(request_);

    let body = generate_body(ctx, body_arg);

    let method = endpoint
        .http_method()
        .as_str()
        .parse::<TokenStream>()
        .unwrap();

    let set_uri = set_uri(ctx, endpoint, &request);
    let set_auth = set_auth(&request, endpoint);
    let set_content_type = set_content_type(ctx, &request, body_arg);
    let set_accept = set_accept(&request, &ret);
    let set_headers = set_headers(ctx, endpoint, &request);
    let make_request = make_request(ctx, &ret, &request);

    quote! {
        #docs
        #deprecated
        pub fn #name #params(&self #auth_arg #(, #args)*) -> Result<#ret_name, conjure_http::private::Error>
        #where_
        {
            #setup_body
            let mut #request = conjure_http::private::http::Request::new(#body);
            *#request.method_mut() = conjure_http::private::http::Method::#method;
            #set_uri
            #set_auth
            #set_content_type
            #set_accept
            #set_headers
            #make_request
        }
    }
}

fn body_arg(endpoint: &EndpointDefinition) -> Option<&ArgumentDefinition> {
    endpoint.args().iter().find(|a| match a.param_type() {
        ParameterType::Body(_) => true,
        _ => false,
    })
}

fn params(ctx: &Context, body_arg: Option<&ArgumentDefinition>) -> TokenStream {
    match body_arg {
        Some(a) if ctx.is_binary(a.type_()) => quote!(<U>),
        _ => quote!(),
    }
}

fn where_(ctx: &Context, body_arg: Option<&ArgumentDefinition>) -> TokenStream {
    match body_arg {
        Some(a) if ctx.is_binary(a.type_()) => quote!(where U: conjure_http::client::IntoWriteBody),
        _ => quote!(),
    }
}

fn auth_arg(endpoint: &EndpointDefinition) -> TokenStream {
    match endpoint.auth() {
        Some(_) => quote!(, auth_: &conjure_object::BearerToken),
        None => quote!(),
    }
}

fn arg_type(ctx: &Context, def: &ServiceDefinition, arg: &ArgumentDefinition) -> TokenStream {
    if ctx.is_binary(arg.type_()) {
        quote!(U)
    } else {
        ctx.borrowed_rust_type(def.service_name(), arg.type_())
    }
}

fn return_type<'a>(ctx: &Context, endpoint: &'a EndpointDefinition) -> ReturnType<'a> {
    match endpoint.returns() {
        Some(ret) => match ctx.is_optional(ret) {
            Some(inner) if ctx.is_binary(inner) => ReturnType::OptionalBinary,
            _ if ctx.is_binary(ret) => ReturnType::Binary,
            _ => ReturnType::Json(ret),
        },
        None => ReturnType::None,
    }
}

fn return_type_name(ctx: &Context, def: &ServiceDefinition, ty: &ReturnType<'_>) -> TokenStream {
    match ty {
        ReturnType::None => quote!(()),
        ReturnType::Json(ty) => ctx.rust_type(def.service_name(), ty),
        ReturnType::Binary => quote!(T::ResponseBody),
        ReturnType::OptionalBinary => quote!(Option<T::ResponseBody>),
    }
}

fn setup_body(ctx: &Context, body: Option<&ArgumentDefinition>) -> TokenStream {
    match body {
        Some(body) if ctx.is_binary(body.type_()) => {
            let name = ctx.field_name(body.arg_name());
            quote! {
                let mut #name = #name.into_write_body();
            }
        }
        _ => quote!(),
    }
}

fn generate_body(ctx: &Context, body: Option<&ArgumentDefinition>) -> TokenStream {
    let body = match body {
        Some(body) => body,
        None => return quote!(conjure_http::client::Body::Empty),
    };

    let name = ctx.field_name(body.arg_name());
    if ctx.is_binary(body.type_()) {
        quote! {
            conjure_http::client::Body::Streaming(&mut #name)
        }
    } else {
        quote! {
            conjure_http::client::Body::Fixed(
                conjure_http::private::json::to_vec(&#name)
                    .map_err(conjure_http::private::Error::internal)?,
            )
        }
    }
}

fn set_uri(ctx: &Context, endpoint: &EndpointDefinition, request: &TokenStream) -> TokenStream {
    let path = quote!(path_);
    let path_expr = generate_path(ctx, endpoint);

    let build_path = match generate_query(ctx, endpoint, &path) {
        Some(query) => {
            quote! {
                let mut #path = #path_expr;
                #query
            }
        }
        None => {
            quote! {
                let #path = #path_expr;
            }
        }
    };

    quote! {
        #build_path
        *#request.uri_mut() = conjure_http::private::http::Uri::from_shared(#path.into())
            .expect("URI should be valid");
    }
}

fn generate_path(ctx: &Context, endpoint: &EndpointDefinition) -> TokenStream {
    let mut template = String::new();
    let mut parameters = vec![];

    // skip the empty component before the leading `/`
    for component in endpoint.http_path().split('/').skip(1) {
        let component = if component.starts_with('{') && component.ends_with('}') {
            let name = &component[1..component.len() - 1];
            let name = ctx.field_name(name);

            let parameter = quote! {
                conjure_http::private::percent_encode(
                    conjure_object::ToPlain::to_plain(&#name).as_bytes(),
                    conjure_http::private::PATH_SEGMENT_ENCODE_SET,
                )
            };
            parameters.push(parameter);

            "{}"
        } else {
            component
        };
        template.push('/');
        template.push_str(component);
    }

    if parameters.is_empty() {
        quote! {
            #template.to_string()
        }
    } else {
        quote! {
            format!(
                #template,
                #(#parameters,)*
            )
        }
    }
}

fn generate_query(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    path: &TokenStream,
) -> Option<TokenStream> {
    let mut singles = vec![];
    let mut iters = vec![];

    for argument in endpoint.args() {
        let query = match argument.param_type() {
            ParameterType::Query(query) => query,
            _ => continue,
        };

        if ctx.is_iterable(argument.type_()) {
            iters.push((query, argument));
        } else {
            singles.push((query, argument));
        }
    }

    if singles.is_empty() && iters.is_empty() {
        return None;
    }

    let need_first = singles.is_empty();

    let singles = singles.iter().enumerate().map(|(i, (query, argument))| {
        let prefix = if i == 0 { '?' } else { '&' };

        let name = ctx.field_name(argument.arg_name());
        let first_part = format!("{}{}=", prefix, query.param_id());

        quote! {
            #path.push_str(#first_part);
            #path.extend(
                conjure_http::private::percent_encode(
                    conjure_object::ToPlain::to_plain(&#name).as_bytes(),
                    conjure_http::private::QUERY_ENCODE_SET,
                ),
            );
        }
    });

    let first = quote!(first_);

    let decl_first = if need_first {
        quote! {
            let mut #first = true;
        }
    } else {
        quote!()
    };

    let iters = iters.iter().map(|(query, argument)| {
        let name = ctx.field_name(argument.arg_name());

        let first_part = if need_first {
            let first_part = format!("{}=", query.param_id());
            quote! {
                let ch = if #first {
                    #first = false;
                    '?'
                } else {
                    '&'
                };
                #path.push(ch);
                #path.push_str(#first_part);
            }
        } else {
            let first_part = format!("&{}=", query.param_id());
            quote! {
                #path.push_str(#first_part);
            }
        };

        quote! {
            for value in #name.iter() {
                #first_part
                #path.extend(
                    conjure_http::private::percent_encode(
                        conjure_object::ToPlain::to_plain(value).as_bytes(),
                        conjure_http::private::QUERY_ENCODE_SET,
                    ),
                );
            }
        }
    });

    Some(quote! {
        #(#singles)*
        #decl_first
        #(#iters)*
    })
}

fn set_auth(request: &TokenStream, endpoint: &EndpointDefinition) -> TokenStream {
    let (header, template) = match endpoint.auth() {
        Some(AuthType::Cookie(cookie)) => {
            (quote!(COOKIE), format!("{}={{}}", cookie.cookie_name()))
        }
        Some(AuthType::Header(_)) => (quote!(AUTHORIZATION), "Bearer {}".to_string()),
        None => return quote!(),
    };

    quote! {
        #request.headers_mut().insert(
            conjure_http::private::http::header::#header,
            conjure_http::private::http::header::HeaderValue::from_shared(
                format!(#template, auth_.as_str()).into(),
            ).expect("bearer tokens are valid headers"),
        );
    }
}

fn set_content_type(
    ctx: &Context,
    request: &TokenStream,
    body: Option<&ArgumentDefinition>,
) -> TokenStream {
    let body = match body {
        Some(body) => body,
        None => return quote!(),
    };

    let content_type = if ctx.is_binary(body.type_()) {
        "application/octet-stream"
    } else {
        "application/json"
    };

    quote! {
        #request.headers_mut().insert(
            conjure_http::private::http::header::CONTENT_TYPE,
            conjure_http::private::http::header::HeaderValue::from_static(#content_type),
        );
    }
}

fn set_accept(request: &TokenStream, ty: &ReturnType<'_>) -> TokenStream {
    let content_type = match ty {
        ReturnType::None => return quote!(),
        ReturnType::Json(_) => "application/json",
        ReturnType::Binary | ReturnType::OptionalBinary => "application/octet-stream",
    };

    quote! {
        #request.headers_mut().insert(
            conjure_http::private::http::header::ACCEPT,
            conjure_http::private::http::header::HeaderValue::from_static(#content_type),
        );
    }
}

fn set_headers(ctx: &Context, endpoint: &EndpointDefinition, request: &TokenStream) -> TokenStream {
    let mut set_headers = vec![];

    for argument in endpoint.args() {
        let header = match argument.param_type() {
            ParameterType::Header(header) => header,
            _ => continue,
        };

        let mut set_header = set_header(ctx, header, argument, request);

        if ctx.is_iterable(argument.type_()) {
            let name = ctx.field_name(argument.arg_name());
            set_header = quote! {
                if let Some(#name) = #name {
                    #set_header
                }
            }
        }

        set_headers.push(set_header);
    }

    quote! {
        #(#set_headers)*
    }
}

fn set_header(
    ctx: &Context,
    header: &HeaderParameterType,
    argument: &ArgumentDefinition,
    request: &TokenStream,
) -> TokenStream {
    // HeaderName::from_static expects http2-style lowercased headers
    let header = header.param_id().to_lowercase();
    let name = ctx.field_name(argument.arg_name());

    quote! {
        #request.headers_mut().insert(
            conjure_http::private::http::header::HeaderName::from_static(#header),
            conjure_http::private::http::header::HeaderValue::from_shared(
                conjure_object::ToPlain::to_plain(&#name).into(),
            ).map_err(conjure_http::private::Error::internal_safe)?,
        );
    }
}

fn make_request(ctx: &Context, ty: &ReturnType, request: &TokenStream) -> TokenStream {
    let response = quote! {
        self.0.request(#request)?
    };

    match ty {
        ReturnType::None => {
            quote! {
                #response;
                Ok(())
            }
        }
        ReturnType::Json(ty) => {
            let mut convert = quote! {
                conjure_http::private::json::client_from_reader(response.body_mut())
                    .map_err(conjure_http::private::Error::internal)
            };
            if ctx.is_iterable(ty) {
                convert = quote! {
                    if response.status() == conjure_http::private::http::StatusCode::NO_CONTENT {
                        Ok(Default::default())
                    } else {
                        #convert
                    }
                };
            }

            quote! {
                let mut response = #response;
                #convert
            }
        }
        ReturnType::Binary => {
            quote! {
                let response = #response;
                Ok(response.into_body())
            }
        }
        ReturnType::OptionalBinary => {
            quote! {
                let response = #response;
                if response.status() == conjure_http::private::http::StatusCode::NO_CONTENT {
                    Ok(None)
                } else {
                    Ok(Some(response.into_body()))
                }
            }
        }
    }
}

enum ReturnType<'a> {
    None,
    Json(&'a Type),
    Binary,
    OptionalBinary,
}
