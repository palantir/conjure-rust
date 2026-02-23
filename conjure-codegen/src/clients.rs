// Copyright 2025 Palantir Technologies, Inc.
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
use syn::Ident;

use crate::{
    context::{BaseModule, Context},
    types::objects::{
        ArgumentDefinition, AuthType, EndpointDefinition, ParameterType, ServiceDefinition, Type,
    },
};

#[derive(Copy, Clone)]
enum Style {
    Async,
    Sync,
    Local,
}

pub fn generate(ctx: &Context, def: &ServiceDefinition) -> TokenStream {
    let sync_trait = generate_trait(ctx, def, Style::Sync);
    let async_trait = generate_trait(ctx, def, Style::Async);
    let local_trait = generate_trait(ctx, def, Style::Local);

    quote! {
        use conjure_http::endpoint;

        #sync_trait
        #async_trait
        #local_trait
    }
}

fn generate_trait(ctx: &Context, def: &ServiceDefinition, style: Style) -> TokenStream {
    let docs = ctx.docs(def.docs());
    let service_name = def.service_name().name();
    let name = trait_name(ctx, def, style);
    let version = match ctx.version() {
        Some(version) => {
            let some = ctx.some_ident(def.service_name());
            quote!(, version = #some(#version))
        }
        None => quote!(),
    };
    let local = match style {
        Style::Local => quote!(, local),
        Style::Async | Style::Sync => quote!(),
    };
    let params = params(ctx, def, style);

    let endpoints = def
        .endpoints()
        .iter()
        .map(|e| generate_trait_endpoint(ctx, def, e, style));

    quote! {
        #docs
        #[conjure_http::conjure_client(name = #service_name #version #local)]
        pub trait #name #params {
            #(#endpoints)*
        }
    }
}

fn trait_name(ctx: &Context, def: &ServiceDefinition, style: Style) -> Ident {
    match style {
        Style::Async => ctx.type_name(&format!("Async{}", def.service_name().name())),
        Style::Local => ctx.type_name(&format!("LocalAsync{}", def.service_name().name())),
        Style::Sync => ctx.type_name(def.service_name().name()),
    }
}

fn params(ctx: &Context, def: &ServiceDefinition, style: Style) -> TokenStream {
    let mut params = vec![];
    if service_has_binary_request_body(ctx, def) {
        params.push(quote! {
            #[request_writer]
            O
        })
    }

    if !def.endpoints().is_empty() {
        let result = ctx.result_ident(def.service_name());
        let trait_ = match style {
            Style::Async | Style::Local => quote!(conjure_http::private::Stream),
            Style::Sync => {
                let iterator = ctx.iterator_ident(def.service_name());
                quote!(#iterator)
            }
        };
        params.push(quote! {
            #[response_body]
            I: #trait_<Item = #result<conjure_http::private::Bytes, conjure_http::private::Error>>
        });
    }

    if params.is_empty() {
        quote!()
    } else {
        quote!(<#(#params),*>)
    }
}

fn service_has_binary_request_body(ctx: &Context, def: &ServiceDefinition) -> bool {
    def.endpoints()
        .iter()
        .any(|e| endpoint_has_binary_request_body(ctx, e))
}

fn endpoint_has_binary_request_body(ctx: &Context, endpoint: &EndpointDefinition) -> bool {
    endpoint.args().iter().any(|a| match a.param_type() {
        ParameterType::Body(_) => ctx.is_binary(a.type_()),
        _ => false,
    })
}

fn generate_trait_endpoint(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
    style: Style,
) -> TokenStream {
    let docs = ctx.docs(endpoint.docs());
    let method = endpoint
        .http_method()
        .as_str()
        .parse::<TokenStream>()
        .unwrap();
    let path = path(endpoint);
    let endpoint_name = &**endpoint.endpoint_name();
    let async_ = match style {
        Style::Async | Style::Local => quote!(async),
        Style::Sync => quote!(),
    };
    let name = ctx.field_name(endpoint.endpoint_name());
    let accept = accept(ctx, endpoint);

    let auth_arg = auth_arg(endpoint);
    let args = endpoint.args().iter().map(|a| arg(ctx, def, a, style));

    let result = ctx.result_ident(def.service_name());

    let ret_ty = rust_return_type(ctx, def, endpoint);
    let ret_ty = quote!(#result<#ret_ty, conjure_http::private::Error>);

    quote! {
        #docs
        #[endpoint(method = #method, path = #path, name = #endpoint_name, accept = #accept)]
        #async_ fn #name(&self #auth_arg #(, #args)*) -> #ret_ty;
    }
}

/// We need to strip the legacy regexes off of path params:
///
/// /foo/{bar:.*} -> /foo/{bar}
fn path(endpoint: &EndpointDefinition) -> String {
    endpoint
        .http_path()
        .split('/')
        .map(
            |segment| match segment.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                Some(segment) => format!("{{{}}}", segment.split(':').next().unwrap()),
                None => segment.to_string(),
            },
        )
        .collect::<Vec<_>>()
        .join("/")
}

fn accept(ctx: &Context, endpoint: &EndpointDefinition) -> TokenStream {
    match return_type(ctx, endpoint) {
        ReturnType::None => quote!(conjure_http::client::conjure::EmptyResponseDeserializer),
        ReturnType::Json(ty) => {
            if ctx.is_iterable(ty) {
                quote!(conjure_http::client::conjure::CollectionResponseDeserializer)
            } else {
                quote!(conjure_http::client::StdResponseDeserializer)
            }
        }
        ReturnType::Binary => quote!(conjure_http::client::conjure::BinaryResponseDeserializer),
        ReturnType::OptionalBinary => {
            quote!(conjure_http::client::conjure::OptionalBinaryResponseDeserializer)
        }
    }
}

fn auth_arg(endpoint: &EndpointDefinition) -> TokenStream {
    match endpoint.auth() {
        Some(auth) => {
            let params = match auth {
                AuthType::Header(_) => quote!(),
                AuthType::Cookie(cookie) => {
                    let name = &cookie.cookie_name();
                    quote!((cookie_name = #name))
                }
            };
            quote!(, #[auth #params] auth_: &conjure_object::BearerToken)
        }
        None => quote!(),
    }
}

fn arg(
    ctx: &Context,
    def: &ServiceDefinition,
    arg: &ArgumentDefinition,
    style: Style,
) -> TokenStream {
    let name = ctx.field_name(arg.arg_name());

    let attr = match arg.param_type() {
        ParameterType::Body(_) => {
            let serializer = if ctx.is_binary(arg.type_()) {
                quote!(conjure_http::client::conjure::BinaryRequestSerializer)
            } else {
                quote!(conjure_http::client::StdRequestSerializer)
            };
            quote!(#[body(serializer = #serializer)])
        }
        ParameterType::Header(header) => {
            let name = &**header.param_id();
            let mut encoder = if ctx.is_optional(arg.type_()).is_some() {
                quote!(conjure_http::client::conjure::PlainSeqEncoder)
            } else {
                quote!(conjure_http::client::conjure::PlainEncoder)
            };
            if ctx.is_aliased(arg.type_()) {
                let dealiased = ctx.dealiased_type(arg.type_());
                let dealiased = ctx.rust_type(BaseModule::Clients, def.service_name(), dealiased);
                encoder = quote!(conjure_http::client::AsRefEncoder<#encoder, #dealiased>)
            }
            quote!(#[header(name = #name, encoder = #encoder)])
        }
        ParameterType::Path(_) => {
            let name = &**arg.arg_name();
            quote!(#[path(name = #name, encoder = conjure_http::client::conjure::PlainEncoder)])
        }
        ParameterType::Query(query) => {
            let name = &**query.param_id();
            let mut encoder = if ctx.is_iterable(arg.type_()) {
                quote!(conjure_http::client::conjure::PlainSeqEncoder)
            } else {
                quote!(conjure_http::client::conjure::PlainEncoder)
            };
            if ctx.is_aliased(arg.type_()) {
                let dealiased = ctx.dealiased_type(arg.type_());
                let dealiased = ctx.rust_type(BaseModule::Clients, def.service_name(), dealiased);
                encoder = quote!(conjure_http::client::AsRefEncoder<#encoder, #dealiased>)
            }
            quote!(#[query(name = #name, encoder = #encoder)])
        }
    };

    let ty = if ctx.is_binary(arg.type_()) {
        match style {
            Style::Async => {
                let sync = ctx.sync_ident(def.service_name());
                let send = ctx.send_ident(def.service_name());
                quote!(impl conjure_http::client::AsyncWriteBody<O> + #sync + #send)
            }
            Style::Local => quote!(impl conjure_http::client::LocalAsyncWriteBody<O>),
            Style::Sync => quote!(impl conjure_http::client::WriteBody<O>),
        }
    } else {
        ctx.borrowed_rust_type(BaseModule::Clients, def.service_name(), arg.type_())
    };
    quote!(#attr #name: #ty)
}

fn rust_return_type(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
) -> TokenStream {
    match return_type(ctx, endpoint) {
        ReturnType::None => quote!(()),
        ReturnType::Json(ty) => ctx.rust_type(BaseModule::Clients, def.service_name(), ty),
        ReturnType::Binary => quote!(I),
        ReturnType::OptionalBinary => {
            let option = ctx.option_ident(def.service_name());
            quote!(#option<I>)
        }
    }
}

fn return_type<'a>(ctx: &Context, endpoint: &'a EndpointDefinition) -> ReturnType<'a> {
    match endpoint.returns() {
        Some(ty) => match ctx.is_optional(ty) {
            Some(inner) if ctx.is_binary(inner) => ReturnType::OptionalBinary,
            _ if ctx.is_binary(ty) => ReturnType::Binary,
            _ => ReturnType::Json(ty),
        },
        None => ReturnType::None,
    }
}

enum ReturnType<'a> {
    None,
    Json(&'a Type),
    Binary,
    OptionalBinary,
}
