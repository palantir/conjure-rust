use crate::context::Context;
use crate::http_paths::{self, PathSegment};
use crate::types::{
    ArgumentDefinition, AuthType, EndpointDefinition, HeaderParameterType, ParameterType,
    QueryParameterType, ServiceDefinition, Type,
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
    let resource = generate_resource(ctx, def);

    quote! {
        #sync_trait
        #async_trait

        #resource
    }
}

fn generate_trait(ctx: &Context, def: &ServiceDefinition, style: Style) -> TokenStream {
    let docs = ctx.docs(def.docs());
    let attr = match style {
        Style::Async => quote!(#[conjure_http::private::async_trait]),
        Style::Sync => quote!(),
    };
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
        #attr
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
        params.push(quote!(I));
    }
    if service_has_binary_response_body(ctx, def) {
        params.push(quote!(O));
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
    let async_ = match style {
        Style::Async => quote!(async),
        Style::Sync => quote!(),
    };
    let name = ctx.field_name(endpoint.endpoint_name());
    let auth_arg = auth_arg(endpoint);
    let args = endpoint.args().iter().map(|a| arg(ctx, def, a));
    let request_context_arg = request_context_arg(endpoint);
    let result = ctx.result_ident(def.service_name());
    let ret_ty = rust_return_type(ctx, def, endpoint, &return_type(ctx, endpoint));
    let ret_ty = quote!(#result<#ret_ty, conjure_http::private::Error>);

    // ignore deprecation since the endpoint has to be implemented regardless
    quote! {
        #docs
        #async_ fn #name(&self #auth_arg #(, #args)* #request_context_arg) -> #ret_ty;
    }
}

fn auth_arg(endpoint: &EndpointDefinition) -> TokenStream {
    match endpoint.auth() {
        Some(_) => quote!(, auth_: conjure_object::BearerToken),
        None => quote!(),
    }
}

fn arg(ctx: &Context, def: &ServiceDefinition, arg: &ArgumentDefinition) -> TokenStream {
    let name = ctx.field_name(arg.arg_name());
    let ty = if ctx.is_binary(arg.type_()) {
        quote!(I)
    } else {
        ctx.rust_type(def.service_name(), arg.type_())
    };
    quote!(#name: #ty)
}

fn request_context_arg(endpoint: &EndpointDefinition) -> TokenStream {
    if has_request_context(endpoint) {
        quote!(, request_context_: conjure_http::server::RequestContext<'_>)
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

fn generate_resource(ctx: &Context, def: &ServiceDefinition) -> TokenStream {
    let name = service_name(ctx, def);
    let sync_service_impl = generate_service_impl(ctx, def, Style::Sync);
    let async_service_impl = generate_service_impl(ctx, def, Style::Async);

    let endpoints = def
        .endpoints()
        .iter()
        .map(|e| generate_endpoint(ctx, def, e));

    quote! {
        pub struct #name<T>(conjure_http::private::Arc<T>);

        impl<T> #name<T> {
            /// Creates a new resource.
            pub fn new(handler: T) -> #name<T> {
                #name(conjure_http::private::Arc::new(handler))
            }
        }

        #sync_service_impl
        #async_service_impl
        #(#endpoints)*
    }
}

fn service_name(ctx: &Context, def: &ServiceDefinition) -> Ident {
    ctx.type_name(&format!("{}Endpoints", def.service_name().name()))
}

fn generate_service_impl(ctx: &Context, def: &ServiceDefinition, style: Style) -> TokenStream {
    let name = service_name(ctx, def);
    let service_trait_name = match style {
        Style::Async => quote!(AsyncService),
        Style::Sync => quote!(Service),
    };
    let trait_name = trait_name(ctx, def, style);
    let params = params(ctx, def);
    let sync = ctx.sync_ident(def.service_name());
    let send = ctx.send_ident(def.service_name());
    let input_trait = input_trait(ctx, def, style);
    let i_traits = match style {
        Style::Async => quote!(+ #sync + #send),
        Style::Sync => quote!(),
    };
    let result = ctx.result_ident(def.service_name());
    let vec = ctx.vec_ident(def.service_name());
    let box_ = ctx.box_ident(def.service_name());
    let endpoint_name = endpoint_trait_name(style);

    let endpoint_instances = def.endpoints().iter().map(|e| create_endpoint(ctx, def, e));

    quote! {
        impl<T, I, O> conjure_http::server::#service_trait_name<I, O> for #name<T>
        where
            T: #trait_name #params + 'static + #sync + #send,
            I: #input_trait<Item = #result<conjure_http::private::Bytes, conjure_http::private::Error>> #i_traits,
        {
            fn endpoints(
                &self,
                _: &conjure_http::private::Arc<conjure_http::server::ConjureRuntime>,
            ) -> #vec<#box_<dyn conjure_http::server::#endpoint_name<I, O> + Sync + Send>>
            {
                vec![
                    #(#endpoint_instances,)*
                ]
            }
        }
    }
}

fn endpoint_trait_name(style: Style) -> TokenStream {
    match style {
        Style::Async => quote!(AsyncEndpoint),
        Style::Sync => quote!(Endpoint),
    }
}

fn input_trait(ctx: &Context, def: &ServiceDefinition, style: Style) -> TokenStream {
    match style {
        Style::Async => quote!(conjure_http::private::Stream),
        Style::Sync => ctx.iterator_ident(def.service_name()),
    }
}

fn generate_endpoint(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
) -> TokenStream {
    let name = endpoint_name(ctx, endpoint);
    let endpoint_metadata = generate_endpoint_metadata(ctx, def, endpoint);
    let endpoint_impl = generate_endpoint_impl(ctx, def, endpoint, Style::Sync);
    let async_endpoint_impl = generate_endpoint_impl(ctx, def, endpoint, Style::Async);

    quote! {
        struct #name<T>(conjure_http::private::Arc<T>);

        #endpoint_metadata
        #endpoint_impl
        #async_endpoint_impl
    }
}

fn generate_endpoint_metadata(
    ctx: &Context,
    service: &ServiceDefinition,
    endpoint: &EndpointDefinition,
) -> TokenStream {
    let endpoint_name = endpoint_name(ctx, endpoint);

    let some = ctx.some_ident(service.service_name());
    let none = ctx.none_ident(service.service_name());
    let option = ctx.option_ident(service.service_name());

    let method = endpoint
        .http_method()
        .as_str()
        .parse::<TokenStream>()
        .unwrap();

    let path = http_paths::parse(endpoint.http_path()).map(|segment| match segment {
        PathSegment::Literal(lit) => quote! {
            conjure_http::server::PathSegment::Literal(
                conjure_http::private::Cow::Borrowed(#lit),
            )
        },
        PathSegment::Parameter { name, regex } => {
            let regex = match regex {
                Some(regex) => {
                    quote!(#some(conjure_http::private::Cow::Borrowed(#regex)))
                }
                None => quote!(#none),
            };
            quote! {
                conjure_http::server::PathSegment::Parameter {
                    name: conjure_http::private::Cow::Borrowed(#name),
                    regex: #regex,
                }
            }
        }
    });

    let template = &***endpoint.http_path();
    let service_name = service.service_name().name();
    let name = &***endpoint.endpoint_name();

    let deprecated = match endpoint.deprecated() {
        Some(deprecated) => {
            let deprecated = &***deprecated;
            quote!(#some(#deprecated))
        }
        None => quote!(#none),
    };

    quote! {
        impl<T> conjure_http::server::EndpointMetadata for #endpoint_name<T> {
            fn method(&self) -> conjure_http::private::Method {
                conjure_http::private::Method::#method
            }

            fn path(&self) -> &[conjure_http::server::PathSegment] {
                &[#(#path,)*]
            }

            fn template(&self) -> &str {
                #template
            }

            fn service_name(&self) -> &str{
                #service_name
            }

            fn name(&self) -> &str{
                #name
            }

            fn deprecated(&self) -> #option<&str> {
                #deprecated
            }
        }
    }
}

fn endpoint_name(ctx: &Context, endpoint: &EndpointDefinition) -> TokenStream {
    let name = ctx.type_name(endpoint.endpoint_name());
    format!("{}Endpoint_", name).parse().unwrap()
}

fn generate_endpoint_impl(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
    style: Style,
) -> TokenStream {
    let attr = match style {
        Style::Async => quote!(#[conjure_http::private::async_trait]),
        Style::Sync => quote!(),
    };

    let endpoint_name = endpoint_name(ctx, endpoint);
    let endpoint_trait_name = endpoint_trait_name(style);
    let trait_name = trait_name(ctx, def, style);
    let trait_params = params(ctx, def);
    let sync = ctx.sync_ident(def.service_name());
    let send = ctx.send_ident(def.service_name());
    let input_trait = input_trait(ctx, def, style);
    let result = ctx.result_ident(def.service_name());

    let i_bounds = match style {
        Style::Async => {
            quote!(+ #sync + #send)
        }
        Style::Sync => quote!(),
    };

    let asyncness = match style {
        Style::Async => quote!(async),
        Style::Sync => quote!(),
    };
    let response_body = match style {
        Style::Async => quote!(conjure_http::server::AsyncResponseBody),
        Style::Sync => quote!(conjure_http::server::ResponseBody),
    };
    let fn_where = match style {
        Style::Async => quote!(where I: 'async_trait),
        Style::Sync => quote!(),
    };

    let parts = quote!(parts_);
    let body = quote!(body_);
    let response_extensions = if has_safe_params(ctx, endpoint) || has_request_context(endpoint) {
        quote!(response_extensions_)
    } else {
        quote!(_response_extensions)
    };
    let safe_params = quote!(safe_params_);
    let query_params = quote!(query_params_);
    let auth = quote!(auth_);
    let response = quote!(response_);

    let make_query_params = if has_query_params(endpoint) {
        quote! {
            let #query_params = conjure_http::private::parse_query_params(&#parts);
        }
    } else {
        quote!()
    };

    let make_safe_params = if has_safe_params(ctx, endpoint) {
        quote! {
            #response_extensions.insert(conjure_http::SafeParams::new());
            let #safe_params = #response_extensions.get_mut::<conjure_http::SafeParams>().unwrap();
        }
    } else {
        quote!()
    };

    let consume_empty_body = if has_body_param(endpoint) {
        quote!()
    } else {
        quote! {
            conjure_http::private::decode_empty_request(&#parts, #body)?;
        }
    };

    let args = endpoint.args().iter().map(|arg| {
        let variable = ctx.field_name(arg.arg_name());

        let parse = match arg.param_type() {
            ParameterType::Path(_) => parse_path_arg(ctx, arg, &parts),
            ParameterType::Query(param) => parse_query_arg(ctx, def, arg, param, &query_params),
            ParameterType::Header(param) => parse_header_arg(ctx, def, arg, param, &parts),
            ParameterType::Body(_) => parse_body_arg(ctx, arg, &parts, &body, style),
        };

        let put_safe = if ctx.is_safe_arg(arg) {
            let name = &***arg.arg_name();
            quote! {
                #safe_params.insert(#name, &#variable);
            }
        } else {
            quote!()
        };

        quote! {
            #parse
            #put_safe
        }
    });

    let make_auth = match endpoint.auth() {
        Some(auth_type) => parse_auth(auth_type, &parts, &auth),
        None => quote!(),
    };

    let request_context = quote!(request_context_);
    let make_request_context = if has_request_context(endpoint) {
        quote! {
            let #request_context = conjure_http::server::RequestContext::new(
                #parts,
                #response_extensions,
            );
        }
    } else {
        quote!()
    };

    let assign_response = if endpoint.returns().is_some() {
        quote!(let #response = )
    } else {
        quote!()
    };

    let handle = handle(ctx, endpoint, &auth, &request_context, style);

    let make_response = make_response(ctx, endpoint, &response, style);
    let ok = ctx.ok_ident(def.service_name());

    quote! {
        #attr
        impl<T, I, O> conjure_http::server::#endpoint_trait_name<I, O> for #endpoint_name<T>
        where
            T: #trait_name #trait_params + 'static + #sync + #send,
            I: #input_trait<Item = #result<conjure_http::private::Bytes, conjure_http::private::Error>> #i_bounds,
        {
            #asyncness fn handle(
                &self,
                request: conjure_http::private::Request<I>,
                #response_extensions: &mut conjure_http::private::Extensions,
            ) -> #result<conjure_http::private::Response<#response_body<O>>, conjure_http::private::Error>
            #fn_where
            {
                let (#parts, #body) = request.into_parts();
                #make_query_params
                #make_safe_params
                #(#args)*
                #make_auth
                #consume_empty_body
                #make_request_context

                #assign_response #handle?;

                #ok(#make_response)
            }
        }
    }
}

fn has_safe_params(ctx: &Context, endpoint: &EndpointDefinition) -> bool {
    endpoint.args().iter().any(|arg| ctx.is_safe_arg(arg))
}

fn has_query_params(endpoint: &EndpointDefinition) -> bool {
    endpoint
        .args()
        .iter()
        .any(|arg| matches!(arg.param_type(), ParameterType::Query { .. }))
}

fn has_body_param(endpoint: &EndpointDefinition) -> bool {
    endpoint
        .args()
        .iter()
        .any(|arg| matches!(arg.param_type(), ParameterType::Body { .. }))
}

fn has_request_context(endpoint: &EndpointDefinition) -> bool {
    endpoint
        .tags()
        .iter()
        .any(|t| t == "server-request-context")
}

fn parse_path_arg(ctx: &Context, arg: &ArgumentDefinition, parts: &TokenStream) -> TokenStream {
    let name = ctx.field_name(arg.arg_name());
    let param_name = &***arg.arg_name();
    quote! {
        let #name = conjure_http::private::parse_path_param(&#parts, #param_name)?;
    }
}

fn parse_query_arg(
    ctx: &Context,
    def: &ServiceDefinition,
    arg: &ArgumentDefinition,
    param: &QueryParameterType,
    query_params: &TokenStream,
) -> TokenStream {
    let name = ctx.field_name(arg.arg_name());
    let param_name = &***arg.arg_name();
    let id = &***param.param_id();
    let ty = ctx.rust_type(def.service_name(), arg.type_());
    let default = ctx.default_ident(def.service_name());

    if ctx.is_optional(arg.type_()).is_some() {
        quote! {
            let mut #name: #ty = #default::default();
            conjure_http::private::parse_optional_query_param(&#query_params, #param_name, #id, &mut #name)?;
        }
    } else if ctx.is_list(arg.type_()) {
        quote! {
            let mut #name: #ty = #default::default();
            conjure_http::private::parse_list_query_param(&#query_params, #param_name, #id, &mut #name)?;
        }
    } else if ctx.is_set(arg.type_()) {
        quote! {
            let mut #name: #ty = #default::default();
            conjure_http::private::parse_set_query_param(&#query_params, #param_name, #id, &mut #name)?;
        }
    } else {
        quote! {
            let #name = conjure_http::private::parse_query_param(&#query_params, #param_name, #id)?;
        }
    }
}

fn parse_header_arg(
    ctx: &Context,
    def: &ServiceDefinition,
    arg: &ArgumentDefinition,
    param: &HeaderParameterType,
    parts: &TokenStream,
) -> TokenStream {
    let name = ctx.field_name(arg.arg_name());
    let id = &***param.param_id();

    let arg_name = &***arg.arg_name();

    if ctx.is_optional(arg.type_()).is_some() {
        let ty = ctx.rust_type(def.service_name(), arg.type_());
        let default = ctx.default_ident(def.service_name());
        quote! {
            let mut #name: #ty = #default::default();
            conjure_http::private::parse_optional_header(&#parts, #arg_name, #id, &mut #name)?;
        }
    } else {
        quote! {
            let #name = conjure_http::private::parse_required_header(&#parts, #arg_name, #id)?;
        }
    }
}

fn parse_body_arg(
    ctx: &Context,
    arg: &ArgumentDefinition,
    parts: &TokenStream,
    body: &TokenStream,
    style: Style,
) -> TokenStream {
    let name = ctx.field_name(arg.arg_name());

    let call = if ctx.is_optional(arg.type_()).is_some() {
        match style {
            Style::Async => {
                quote!(async_decode_optional_serializable_request(&#parts, #body).await)
            }
            Style::Sync => quote!(decode_optional_serializable_request(&#parts, #body)),
        }
    } else if ctx.is_binary(arg.type_()) {
        quote!(decode_binary_request(&#parts, #body))
    } else {
        match style {
            Style::Async => quote!(async_decode_serializable_request(&#parts, #body).await),
            Style::Sync => quote!(decode_serializable_request(&#parts, #body)),
        }
    };

    quote! {
        let #name = conjure_http::private::#call?;
    }
}

fn parse_auth(auth_type: &AuthType, parts: &TokenStream, auth: &TokenStream) -> TokenStream {
    let parser = match auth_type {
        AuthType::Cookie(cookie) => {
            let prefix = format!("{}=", cookie.cookie_name());
            quote!(parse_cookie_auth(&#parts, #prefix))
        }
        AuthType::Header(_) => quote!(parse_header_auth(&#parts)),
    };

    quote! {
        let #auth = conjure_http::private::#parser?;
    }
}

fn handle(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    auth: &TokenStream,
    request_context: &TokenStream,
    style: Style,
) -> TokenStream {
    let name = ctx.field_name(endpoint.endpoint_name());

    let mut args = vec![];

    if endpoint.auth().is_some() {
        args.push(quote!(#auth));
    }

    args.extend(endpoint.args().iter().map(|a| {
        let name = ctx.field_name(a.arg_name());
        quote!(#name)
    }));

    if has_request_context(endpoint) {
        args.push(quote!(#request_context));
    }

    let await_ = match style {
        Style::Async => quote!(.await),
        Style::Sync => quote!(),
    };

    quote! {
        self.0 .#name(#(#args),*) #await_
    }
}

fn make_response(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    response: &TokenStream,
    style: Style,
) -> TokenStream {
    let call = match endpoint.returns() {
        Some(ty) => match ctx.is_optional(ty) {
            Some(inner) if ctx.is_binary(inner) => match style {
                Style::Async => {
                    quote!(async_encode_optional_binary_response(#response))
                }
                Style::Sync => {
                    quote!(encode_optional_binary_response(#response))
                }
            },
            _ if ctx.is_binary(ty) => match style {
                Style::Async => quote!(async_encode_binary_response(#response)),
                Style::Sync => quote!(encode_binary_response(#response)),
            },
            _ if ctx.is_iterable(ty) => match style {
                Style::Async => {
                    quote!(async_encode_default_serializable_response(&#response))
                }
                Style::Sync => quote!(encode_default_serializable_response(&#response)),
            },
            _ => match style {
                Style::Async => quote!(async_encode_serializable_response(&#response)),
                Style::Sync => quote!(encode_serializable_response(&#response)),
            },
        },
        None => match style {
            Style::Async => quote!(async_encode_empty_response()),
            Style::Sync => quote!(encode_empty_response()),
        },
    };

    quote!(conjure_http::private::#call)
}

fn create_endpoint(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
) -> TokenStream {
    let box_ = ctx.box_ident(def.service_name());
    let handler = endpoint_name(ctx, endpoint);

    quote! {
        #box_::new(#handler(self.0.clone()))
    }
}
