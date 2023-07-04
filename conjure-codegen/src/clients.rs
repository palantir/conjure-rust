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
use crate::context::Context;
use crate::http_paths::{self, PathSegment};
use crate::types::{
    ArgumentDefinition, AuthType, EndpointDefinition, ParameterType, ServiceDefinition, Type,
};
use conjure_codegen_shared::client::{self, Endpoint, Service};
use conjure_codegen_shared::path;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Attribute, Generics, LitStr};

#[derive(Copy, Clone)]
enum Style {
    Async,
    Sync,
}

pub fn generate(ctx: &Context, def: &ServiceDefinition) -> TokenStream {
    let async_ = generate_inner(ctx, def, Style::Async);
    let sync = generate_inner(ctx, def, Style::Sync);

    quote! {
        #async_

        #sync
    }
}

fn generate_inner(ctx: &Context, def: &ServiceDefinition, style: Style) -> TokenStream {
    let suffix = match style {
        Style::Async => "AsyncClient",
        Style::Sync => "Client",
    };
    let name = ctx.type_name(&format!("{}{}", def.service_name().name(), suffix));

    let version = match ctx.version() {
        Some(version) => quote!(conjure_http::private::Option::Some(#version)),
        None => quote!(conjure_http::private::Option::None),
    };

    let generics = Generics {
        lt_token: None,
        params: Punctuated::new(),
        gt_token: None,
        where_clause: None,
    };

    let attrs = Attribute::parse_outer.parse2(ctx.docs(def.docs())).unwrap();

    let endpoints = def.endpoints().iter().map(|e| endpoint(ctx, def, e));

    let service = Service::builder()
        .vis(syn::parse2(quote!(pub)).unwrap())
        .name(name)
        .version(syn::parse2(version).unwrap())
        .generics(generics)
        .r#async(matches!(style, Style::Async))
        .attrs(attrs)
        .endpoints(endpoints)
        .build();

    client::generate(&service)
}

fn endpoint(ctx: &Context, def: &ServiceDefinition, endpoint: &EndpointDefinition) -> Endpoint {
    let method = Ident::new(endpoint.http_method().as_str(), Span::call_site());
    let path = LitStr::new(endpoint.http_path(), Span::call_site());
    let name = ctx.field_name(endpoint.endpoint_name());

    let result = ctx.result_ident(def.service_name());
    let ret = return_type(ctx, endpoint);
    let ret_name = return_type_name(ctx, def, &ret);
    let return_type =
        syn::parse2(quote!(#result<#ret_name, conjure_http::private::Error>)).unwrap();

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
    let attrs = quote! {
        #docs
        #deprecated
    };
    let attrs = Attribute::parse_outer.parse2(attrs).unwrap();

    let path_segments = path::parse(endpoint.http_path());

    Endpoint::builder()
        .method(method)
        .path(path)
        .name(name)
        .return_type(return_type)
        .attrs(attrs)
        .path_segments(path_segments);

    let async_ = match style {
        Style::Async => quote!(async),
        Style::Sync => quote!(),
    };

    let name = ctx.field_name(endpoint.endpoint_name());

    let body_arg = body_arg(endpoint);
    let params = params(ctx, body_arg);

    let auth = quote!(auth_);
    let auth_arg = auth_arg(endpoint, &auth);
    let args = endpoint.args().iter().map(|a| {
        let name = ctx.field_name(a.arg_name());
        let ty = arg_type(ctx, def, a);
        quote!(#name: #ty)
    });

    let result = ctx.result_ident(def.service_name());
    let ret = return_type(ctx, endpoint);
    let ret_name = return_type_name(ctx, def, &ret);
    let where_ = where_(ctx, style, body_arg);

    let request = quote!(request_);
    let setup_request = setup_request(ctx, body_arg, style, &request);

    let method = endpoint
        .http_method()
        .as_str()
        .parse::<TokenStream>()
        .unwrap();

    let setup_path = setup_path(ctx, endpoint, &request);

    let setup_headers = setup_headers(ctx, endpoint, &request, &auth);

    let setup_response_headers = setup_response_headers(&ret, &request);

    let setup_endpoint_extension = setup_endpoint_extension(ctx, def, endpoint, &request);

    let await_ = match style {
        Style::Async => quote!(.await),
        Style::Sync => quote!(),
    };

    let response = quote!(response_);
    let decode_response = setup_decode_response(ctx, &ret, style, &response);

    quote! {
        #docs
        #deprecated
        pub #async_ fn #name #params(&self #auth_arg #(, #args)*) -> #result<#ret_name, conjure_http::private::Error>
        #where_
        {
            #setup_request
            *#request.method_mut() = conjure_http::private::http::Method::#method;
            #setup_path
            #setup_headers
            #setup_response_headers
            #setup_endpoint_extension

            let #response = self.0.send(#request)#await_?;
            #decode_response
        }
    }
}

fn body_arg(endpoint: &EndpointDefinition) -> Option<&ArgumentDefinition> {
    endpoint
        .args()
        .iter()
        .find(|a| matches!(a.param_type(), ParameterType::Body(_)))
}

fn params(ctx: &Context, body_arg: Option<&ArgumentDefinition>) -> TokenStream {
    match body_arg {
        Some(a) if ctx.is_binary(a.type_()) => quote!(<U>),
        _ => quote!(),
    }
}

fn where_(ctx: &Context, style: Style, body_arg: Option<&ArgumentDefinition>) -> TokenStream {
    match body_arg {
        Some(a) if ctx.is_binary(a.type_()) => {
            let bound = match style {
                Style::Async => {
                    quote!(conjure_http::client::AsyncWriteBody<T::BodyWriter> + Sync + Send)
                }
                Style::Sync => quote!(conjure_http::client::WriteBody<T::BodyWriter>),
            };
            quote!(where U: #bound,)
        }
        _ => quote!(),
    }
}

fn auth_arg(endpoint: &EndpointDefinition, auth: &TokenStream) -> TokenStream {
    match endpoint.auth() {
        Some(_) => quote!(, #auth: &conjure_object::BearerToken),
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

fn accept(ty: &ReturnType<'_>) -> syn::Type {
    let accept = match ty {
        ReturnType::None => quote!(conjure_http::client::UnitResponseDeserializer),
        ReturnType::Json(_) => quote!(conjure_http::client::ConjureResponseDeserializer),
        ReturnType::Binary => todo!(),
        ReturnType::OptionalBinary => todo!(),
    };
    syn::parse2(accept).unwrap()
}

fn return_type_name(ctx: &Context, def: &ServiceDefinition, ty: &ReturnType<'_>) -> TokenStream {
    match ty {
        ReturnType::None => quote!(()),
        ReturnType::Json(ty) => ctx.rust_type(def.service_name(), ty),
        ReturnType::Binary => quote!(T::ResponseBody),
        ReturnType::OptionalBinary => {
            let option = ctx.option_ident(def.service_name());
            quote!(#option<T::ResponseBody>)
        }
    }
}

fn setup_request(
    ctx: &Context,
    body_arg: Option<&ArgumentDefinition>,
    style: Style,
    request: &TokenStream,
) -> TokenStream {
    match body_arg {
        Some(body_arg) => {
            let name = ctx.field_name(body_arg.arg_name());
            if ctx.is_binary(body_arg.type_()) {
                match style {
                    Style::Sync => quote! {
                        let mut #name = #name;
                        let mut #request = conjure_http::private::encode_binary_request(&mut #name as _);
                    },
                    Style::Async => quote! {
                        conjure_http::private::pin_mut!(#name);
                        let mut #request = conjure_http::private::async_encode_binary_request(#name as _);
                    },
                }
            } else {
                let function = match style {
                    Style::Sync => quote!(encode_serializable_request),
                    Style::Async => quote!(async_encode_serializable_request),
                };
                quote! {
                    let mut #request = conjure_http::private::#function(&#name);
                }
            }
        }
        None => {
            let function = match style {
                Style::Sync => quote!(encode_empty_request),
                Style::Async => quote!(async_encode_empty_request),
            };
            quote! {
                let mut #request = conjure_http::private::#function();
            }
        }
    }
}

fn setup_path(ctx: &Context, endpoint: &EndpointDefinition, request: &TokenStream) -> TokenStream {
    let path = quote!(path_);
    let path_components = setup_path_components(ctx, endpoint, &path);
    let query_components = setup_query_components(ctx, endpoint, &path);

    quote! {
        let mut #path = conjure_http::private::UriBuilder::new();
        #path_components
        #query_components
        *#request.uri_mut() = #path.build();
    }
}

fn setup_path_components(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    path: &TokenStream,
) -> TokenStream {
    let path_params = endpoint
        .args()
        .iter()
        .filter(|arg| matches!(arg.param_type(), &ParameterType::Path(_)))
        .map(|arg| {
            let key = &***arg.arg_name();
            let value = ctx.field_name(key);
            (key, value)
        })
        .collect::<HashMap<_, _>>();

    let mut calls = vec![];
    let mut cur = String::new();
    for segment in http_paths::parse(endpoint.http_path()) {
        match segment {
            PathSegment::Literal(lit) => {
                cur.push('/');
                cur.push_str(lit);
            }
            PathSegment::Parameter { name, .. } => {
                if !cur.is_empty() {
                    calls.push(quote!(#path.push_literal(#cur);));
                    cur.clear();
                }
                let arg = &path_params[name];
                calls.push(quote!(#path.push_path_parameter(&#arg);));
            }
        }
    }

    if !cur.is_empty() {
        calls.push(quote!(#path.push_literal(#cur);));
    }

    quote!(#(#calls)*)
}

fn setup_query_components(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    path: &TokenStream,
) -> TokenStream {
    let mut calls = vec![];

    for argument in endpoint.args() {
        let query = match argument.param_type() {
            ParameterType::Query(query) => query,
            _ => continue,
        };

        let key = &**query.param_id();
        let name = ctx.field_name(argument.arg_name());

        let call = if ctx.is_optional(argument.type_()).is_some() {
            quote! {
                #path.push_optional_query_parameter(#key, &#name);
            }
        } else if ctx.is_list(argument.type_()) {
            quote! {
                #path.push_list_query_parameter(#key, &#name);
            }
        } else if ctx.is_set(argument.type_()) {
            quote! {
                #path.push_set_query_parameter(#key, &#name);
            }
        } else {
            quote! {
                #path.push_query_parameter(#key, &#name);
            }
        };
        calls.push(call);
    }

    quote!(#(#calls)*)
}

fn setup_headers(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    request: &TokenStream,
    auth: &TokenStream,
) -> TokenStream {
    let mut calls = vec![];

    if let Some(call) = auth_header(endpoint, request, auth) {
        calls.push(call);
    }

    for argument in endpoint.args() {
        let header = match argument.param_type() {
            ParameterType::Header(header) => header,
            _ => continue,
        };

        // HeaderName::from_static expects http2-style lowercased headers
        let header = header.param_id().to_lowercase();
        let name = ctx.field_name(argument.arg_name());

        let call = if ctx.is_optional(argument.type_()).is_some() {
            quote! {
                conjure_http::private::encode_optional_header(&mut #request, #header, &#name)?;
            }
        } else {
            quote! {
                conjure_http::private::encode_header(&mut #request, #header, &#name)?;
            }
        };

        calls.push(call);
    }

    quote!(#(#calls)*)
}

fn auth_header(
    endpoint: &EndpointDefinition,
    request: &TokenStream,
    auth: &TokenStream,
) -> Option<TokenStream> {
    match endpoint.auth() {
        Some(AuthType::Cookie(cookie)) => {
            let prefix = format!("{}=", cookie.cookie_name());
            Some(quote! {
                conjure_http::private::encode_cookie_auth(&mut #request, #prefix, #auth);
            })
        }
        Some(AuthType::Header(_)) => Some(quote! {
            conjure_http::private::encode_header_auth(&mut #request, #auth);
        }),
        None => None,
    }
}

fn setup_response_headers(ty: &ReturnType<'_>, request: &TokenStream) -> TokenStream {
    match ty {
        ReturnType::None => quote! {
            conjure_http::private::encode_empty_response_headers(&mut #request);
        },
        ReturnType::Json(_) => {
            quote! {
                conjure_http::private::encode_serializable_response_headers(&mut #request);
            }
        }
        ReturnType::Binary | &ReturnType::OptionalBinary => quote! {
            conjure_http::private::encode_binary_response_headers(&mut #request);
        },
    }
}

fn setup_endpoint_extension(
    ctx: &Context,
    service: &ServiceDefinition,
    endpoint: &EndpointDefinition,
    request: &TokenStream,
) -> TokenStream {
    let service = service.service_name().name();
    let version = match ctx.version() {
        Some(version) => quote! {
            conjure_http::private::Option::Some(#version)
        },
        None => quote! {
            conjure_http::private::Option::None
        },
    };
    let name = &***endpoint.endpoint_name();
    let path = &***endpoint.http_path();

    quote! {
        #request.extensions_mut().insert(conjure_http::client::Endpoint::new(
            #service,
            #version,
            #name,
            #path,
        ));
    }
}

fn setup_decode_response(
    ctx: &Context,
    ty: &ReturnType<'_>,
    style: Style,
    response: &TokenStream,
) -> TokenStream {
    match (ty, style) {
        (ReturnType::None, Style::Sync) => quote! {
            conjure_http::private::decode_empty_response(#response)
        },
        (ReturnType::None, Style::Async) => quote! {
            conjure_http::private::async_decode_empty_response(#response).await
        },
        (ReturnType::Json(ty), Style::Sync) => {
            if ctx.is_iterable(ty) {
                quote! {
                    conjure_http::private::decode_default_serializable_response(#response)
                }
            } else {
                quote! {
                    conjure_http::private::decode_serializable_response(#response)
                }
            }
        }
        (ReturnType::Json(ty), Style::Async) => {
            if ctx.is_iterable(ty) {
                quote! {
                    conjure_http::private::async_decode_default_serializable_response(#response).await
                }
            } else {
                quote! {
                    conjure_http::private::async_decode_serializable_response(#response).await
                }
            }
        }
        (ReturnType::Binary, _) => quote! {
            conjure_http::private::decode_binary_response(#response)
        },
        (ReturnType::OptionalBinary, _) => quote! {
            conjure_http::private::decode_optional_binary_response(#response)
        },
    }
}

enum ReturnType<'a> {
    None,
    Json(&'a Type),
    Binary,
    OptionalBinary,
}
