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
    ArgumentDefinition, AuthType, EndpointDefinition, ParameterType, ServiceDefinition, Type,
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
    let where_ = where_(ctx, body_arg);

    let method = endpoint
        .http_method()
        .as_str()
        .parse::<TokenStream>()
        .unwrap();

    let path = &**endpoint.http_path();

    let path_params = quote!(path_params_);
    let setup_path_params = setup_path_params(ctx, endpoint, &path_params);

    let query_params = quote!(query_params_);
    let setup_query_params = setup_query_params(ctx, endpoint, &query_params);

    let headers = quote!(headers_);
    let setup_headers = setup_headers(ctx, endpoint, &headers, &auth);

    let body = quote!(body_);
    let setup_body = setup_body(ctx, body_arg, &body);

    let response_visitor = quote!(response_visitor_);
    let setup_response_visitor = setup_response_visitor(ctx, &ret, &response_visitor);

    quote! {
        #docs
        #deprecated
        pub fn #name #params(&self #auth_arg #(, #args)*) -> #result<#ret_name, conjure_http::private::Error>
        #where_
        {
            #setup_path_params
            #setup_query_params
            #setup_headers
            #setup_body
            #setup_response_visitor

            self.0.request(
                conjure_http::private::http::Method::#method,
                #path,
                #path_params,
                #query_params,
                #headers,
                #body,
                #response_visitor,
            )
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
        Some(a) if ctx.is_binary(a.type_()) => quote!(where U: conjure_http::client::WriteBody),
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

fn setup_path_params(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    path_params: &TokenStream,
) -> TokenStream {
    let mut parameters = vec![];

    for argument in endpoint.args() {
        match argument.param_type() {
            ParameterType::Path(_) => {}
            _ => continue,
        }

        let key = &**argument.arg_name();
        let name = ctx.field_name(key);

        let parameter = quote! {
            conjure_http::private::encode_path_param(&mut #path_params, #key, #name);
        };
        parameters.push(parameter);
    }

    let mutability = if parameters.is_empty() {
        quote!()
    } else {
        quote!(mut)
    };

    quote! {
        let #mutability #path_params = conjure_http::PathParams::new();
        #(#parameters)*
    }
}

fn setup_query_params(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    query_params: &TokenStream,
) -> TokenStream {
    let mut parameters = vec![];

    for argument in endpoint.args() {
        let query = match argument.param_type() {
            ParameterType::Query(query) => query,
            _ => continue,
        };

        let key = &**query.param_id();
        let name = ctx.field_name(argument.arg_name());

        let parameter = if ctx.is_optional(argument.type_()).is_some() {
            quote! {
                conjure_http::private::encode_optional_query_param(
                    &mut #query_params,
                    #key,
                    &#name,
                );
            }
        } else if ctx.is_list(argument.type_()) {
            quote! {
                conjure_http::private::encode_list_query_param(
                    &mut #query_params,
                    #key,
                    &#name,
                );
            }
        } else if ctx.is_set(argument.type_()) {
            quote! {
                conjure_http::private::encode_set_query_param(
                    &mut #query_params,
                    #key,
                    &#name,
                );
            }
        } else {
            quote! {
                conjure_http::private::encode_query_param(
                    &mut #query_params,
                    #key,
                    #name,
                );
            }
        };
        parameters.push(parameter);
    }

    let mutability = if parameters.is_empty() {
        quote!()
    } else {
        quote!(mut)
    };

    quote! {
        let #mutability #query_params = conjure_http::QueryParams::new();
        #(#parameters)*
    }
}

fn setup_headers(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    headers: &TokenStream,
    auth: &TokenStream,
) -> TokenStream {
    let mut parameters = vec![];

    if let Some(parameter) = auth_header(endpoint, headers, auth) {
        parameters.push(parameter);
    }

    for argument in endpoint.args() {
        let header = match argument.param_type() {
            ParameterType::Header(header) => header,
            _ => continue,
        };

        // HeaderName::from_static expects http2-style lowercased headers
        let header = header.param_id().to_lowercase();
        let param = &**argument.arg_name();
        let name = ctx.field_name(argument.arg_name());

        let parameter = if ctx.is_optional(argument.type_()).is_some() {
            quote! {
                conjure_http::private::encode_optional_header(
                    &mut #headers,
                    #param,
                    #header,
                    &#name,
                )?;
            }
        } else {
            quote! {
                conjure_http::private::encode_header(
                    &mut #headers,
                    #param,
                    #header,
                    #name,
                )?;
            }
        };

        parameters.push(parameter);
    }

    let mutability = if parameters.is_empty() {
        quote!()
    } else {
        quote!(mut)
    };

    quote! {
        let #mutability #headers = conjure_http::private::http::HeaderMap::new();
        #(#parameters)*
    }
}

fn auth_header(
    endpoint: &EndpointDefinition,
    headers: &TokenStream,
    auth: &TokenStream,
) -> Option<TokenStream> {
    match endpoint.auth() {
        Some(AuthType::Cookie(cookie)) => {
            let prefix = format!("{}=", cookie.cookie_name());
            Some(quote! {
                conjure_http::private::encode_cookie_auth(&mut #headers, #prefix, #auth);
            })
        }
        Some(AuthType::Header(_)) => Some(quote! {
            conjure_http::private::encode_header_auth(&mut #headers, #auth);
        }),
        None => None,
    }
}

fn setup_body(
    ctx: &Context,
    body_arg: Option<&ArgumentDefinition>,
    body: &TokenStream,
) -> TokenStream {
    let expr = match body_arg {
        Some(body_arg) => {
            let name = ctx.field_name(body_arg.arg_name());
            if ctx.is_binary(body_arg.type_()) {
                quote! {
                    conjure_http::private::BinaryRequestBody(#name)
                }
            } else {
                quote! {
                    conjure_http::private::SerializableRequestBody(#name)
                }
            }
        }
        None => {
            quote! {
                conjure_http::private::EmptyRequestBody
            }
        }
    };

    quote! {
        let #body = #expr;
    }
}

fn setup_response_visitor(
    ctx: &Context,
    ty: &ReturnType<'_>,
    response_visitor: &TokenStream,
) -> TokenStream {
    let visitor = match ty {
        ReturnType::None => quote!(EmptyResponseVisitor),
        ReturnType::Json(ty) => {
            if ctx.is_iterable(ty) {
                quote!(DefaultSerializableResponseVisitor::new())
            } else {
                quote!(SerializableResponseVisitor::new())
            }
        }
        ReturnType::Binary => quote!(BinaryResponseVisitor),
        ReturnType::OptionalBinary => quote!(OptionalBinaryResponseVisitor),
    };

    quote! {
        let #response_visitor = conjure_http::private::#visitor;
    }
}

enum ReturnType<'a> {
    None,
    Json(&'a Type),
    Binary,
    OptionalBinary,
}
