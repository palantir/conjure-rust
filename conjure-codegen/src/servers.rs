use crate::context::Context;
use crate::types::{
    ArgumentDefinition, AuthType, EndpointDefinition, ParameterType, ServiceDefinition, Type,
};
use heck::ToUpperCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[derive(Copy, Clone)]
enum Style {
    Async,
    Sync,
}

pub fn generate(ctx: &Context, def: &ServiceDefinition) -> TokenStream {
    let sync_trait = generate_trait(ctx, def, Style::Sync);
    let async_trait = generate_trait(ctx, def, Style::Async);

    quote! {
        use conjure_http::endpoint;

        #sync_trait
        #async_trait
    }
}

fn generate_trait(ctx: &Context, def: &ServiceDefinition, style: Style) -> TokenStream {
    let docs = ctx.docs(def.docs());
    let service_name = def.service_name().name();
    let name = trait_name(ctx, def, style);
    let params = params(ctx, def);

    let binary_types = def
        .endpoints()
        .iter()
        .flat_map(|e| generate_binary_type(ctx, def, e, style));

    let endpoints = def
        .endpoints()
        .iter()
        .map(|e| generate_trait_endpoint(ctx, def, e, style));

    quote! {
        #docs
        #[conjure_http::conjure_endpoints(name = #service_name)]
        pub trait #name #params {
            #(#binary_types)*

            #(#endpoints)*
        }
    }
}

fn trait_name(ctx: &Context, def: &ServiceDefinition, style: Style) -> Ident {
    match style {
        Style::Async => ctx.type_name(&format!("Async{}", def.service_name().name())),
        Style::Sync => ctx.type_name(def.service_name().name()),
    }
}

fn params(ctx: &Context, def: &ServiceDefinition) -> TokenStream {
    let mut params = vec![];
    if service_has_binary_request_body(ctx, def) {
        params.push(quote! {
                #[request_body]
                I
        });
    }
    if service_has_binary_response_body(ctx, def) {
        params.push(quote! {
                #[response_writer]
                O
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

fn service_has_binary_response_body(ctx: &Context, def: &ServiceDefinition) -> bool {
    def.endpoints()
        .iter()
        .any(|e| endpoint_has_binary_response_body(ctx, e))
}

fn endpoint_has_binary_response_body(ctx: &Context, endpoint: &EndpointDefinition) -> bool {
    match return_type(ctx, endpoint) {
        ReturnType::Binary | ReturnType::OptionalBinary => true,
        ReturnType::None | ReturnType::Json(_) => false,
    }
}

fn generate_binary_type(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
    style: Style,
) -> Option<TokenStream> {
    if endpoint_has_binary_response_body(ctx, endpoint) {
        let docs = format!(
            "The body type returned by the `{}` method.",
            ctx.field_name(endpoint.endpoint_name())
        );
        let name = binary_type(endpoint);
        let bounds = match style {
            Style::Async => {
                let send = ctx.send_ident(def.service_name());
                quote!(conjure_http::server::AsyncWriteBody<O> + 'static + #send)
            }
            Style::Sync => quote!(conjure_http::server::WriteBody<O> + 'static),
        };
        Some(quote! {
            #[doc = #docs]
            type #name: #bounds;
        })
    } else {
        None
    }
}

fn binary_type(endpoint: &EndpointDefinition) -> TokenStream {
    format!("{}Body", endpoint.endpoint_name().to_upper_camel_case())
        .parse()
        .unwrap()
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
    let path = &**endpoint.http_path();
    let endpoint_name = &**endpoint.endpoint_name();
    let async_ = match style {
        Style::Async => quote!(async),
        Style::Sync => quote!(),
    };
    let name = ctx.field_name(endpoint.endpoint_name());
    let produces = match endpoint.returns() {
        Some(ty) => {
            let produces = produces(ctx, ty);
            quote!(, produces = #produces)
        }
        None => quote!(),
    };

    let auth_arg = auth_arg(endpoint);
    let args = endpoint.args().iter().map(|a| arg(ctx, def, a));
    let request_context_arg = request_context_arg(endpoint);

    let result = ctx.result_ident(def.service_name());

    let ret_ty = rust_return_type(ctx, def, endpoint, &return_type(ctx, endpoint));
    let ret_ty = quote!(#result<#ret_ty, conjure_http::private::Error>);

    // ignore deprecation since the endpoint has to be implemented regardless
    quote! {
        #docs
        #[endpoint(method = #method, path = #path, name = #endpoint_name #produces)]
        #async_ fn #name(&self #auth_arg #(, #args)* #request_context_arg) -> #ret_ty;
    }
}

fn produces(ctx: &Context, ty: &Type) -> TokenStream {
    match ctx.is_optional(ty) {
        Some(inner) if ctx.is_binary(inner) => {
            quote!(conjure_http::server::conjure::OptionalBinaryResponseSerializer)
        }
        _ if ctx.is_binary(ty) => quote!(conjure_http::server::conjure::BinaryResponseSerializer),
        _ if ctx.is_iterable(ty) => {
            quote!(conjure_http::server::conjure::CollectionResponseSerializer)
        }
        _ => quote!(conjure_http::server::StdResponseSerializer),
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
            quote!(, #[auth #params] auth_: conjure_object::BearerToken)
        }
        None => quote!(),
    }
}

fn arg(ctx: &Context, def: &ServiceDefinition, arg: &ArgumentDefinition) -> TokenStream {
    let name = ctx.field_name(arg.arg_name());

    let log_as = if name == **arg.arg_name() {
        quote!()
    } else {
        let log_as = &**arg.arg_name();
        quote!(, log_as = #log_as)
    };

    let safe = if ctx.is_safe_arg(arg) {
        quote!(, safe)
    } else {
        quote!()
    };

    let attr = match arg.param_type() {
        ParameterType::Body(_) => {
            let deserializer = if ctx.is_optional(arg.type_()).is_some() {
                let mut decoder =
                    quote!(conjure_http::server::conjure::OptionalRequestDeserializer);
                let dealiased = ctx.dealiased_type(arg.type_());
                if dealiased != arg.type_() {
                    let dealiased = ctx.rust_type(def.service_name(), dealiased);
                    decoder =
                        quote!(conjure_http::server::FromRequestDeserializer<#decoder, #dealiased>)
                }
                decoder
            } else if ctx.is_binary(arg.type_()) {
                quote!(conjure_http::server::conjure::BinaryRequestDeserializer)
            } else {
                quote!(conjure_http::server::StdRequestDeserializer)
            };
            quote!(#[body(deserializer = #deserializer #log_as #safe)])
        }
        ParameterType::Header(header) => {
            let name = &**header.param_id();
            let decoder = if ctx.is_optional(arg.type_()).is_some() {
                optional_decoder(ctx, def, arg.type_())
            } else {
                quote!(conjure_http::server::conjure::FromPlainDecoder)
            };
            quote!(#[header(name = #name, decoder = #decoder #log_as #safe)])
        }
        ParameterType::Path(_) => {
            let name = &**arg.arg_name();
            quote! {
                #[path(
                    name = #name,
                    decoder = conjure_http::server::conjure::FromPlainDecoder
                    #log_as
                    #safe
                )]
            }
        }
        ParameterType::Query(query) => {
            let name = &**query.param_id();
            let decoder = if ctx.is_optional(arg.type_()).is_some() {
                optional_decoder(ctx, def, arg.type_())
            } else if ctx.is_iterable(arg.type_()) {
                quote!(conjure_http::server::conjure::FromPlainSeqDecoder<_>)
            } else {
                quote!(conjure_http::server::conjure::FromPlainDecoder)
            };
            quote!(#[query(name = #name, decoder = #decoder #log_as #safe)])
        }
    };

    let ty = if ctx.is_binary(arg.type_()) {
        quote!(I)
    } else {
        ctx.rust_type(def.service_name(), arg.type_())
    };
    quote!(#attr #name: #ty)
}

fn optional_decoder(ctx: &Context, def: &ServiceDefinition, ty: &Type) -> TokenStream {
    let mut decoder = quote!(conjure_http::server::conjure::FromPlainOptionDecoder);
    let dealiased = ctx.dealiased_type(ty);
    if dealiased != ty {
        let dealiased = ctx.rust_type(def.service_name(), dealiased);
        decoder = quote!(conjure_http::server::FromDecoder<#decoder, #dealiased>)
    }
    decoder
}

fn request_context_arg(endpoint: &EndpointDefinition) -> TokenStream {
    if has_request_context(endpoint) {
        quote!(, #[context] request_context_: conjure_http::server::RequestContext<'_>)
    } else {
        quote!()
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

fn rust_return_type(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
    ty: &ReturnType<'_>,
) -> TokenStream {
    match ty {
        ReturnType::None => quote!(()),
        ReturnType::Json(ty) => ctx.rust_type(def.service_name(), ty),
        ReturnType::Binary => {
            let name = binary_type(endpoint);
            quote!(Self::#name)
        }
        ReturnType::OptionalBinary => {
            let name = binary_type(endpoint);
            let option = ctx.option_ident(def.service_name());
            quote!(#option<Self::#name>)
        }
    }
}

enum ReturnType<'a> {
    None,
    Json(&'a Type),
    Binary,
    OptionalBinary,
}

fn has_request_context(endpoint: &EndpointDefinition) -> bool {
    endpoint
        .tags()
        .iter()
        .any(|t| t == "server-request-context")
}
