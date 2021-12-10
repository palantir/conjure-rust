use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::context::Context;
use crate::types::{
    ArgumentDefinition, AuthType, EndpointDefinition, ParameterType, ServiceDefinition, Type,
};

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
        use conjure_http::server::{Response as _, AsyncResponse as _};

        #sync_trait
        #async_trait

        #resource
    }
}

fn generate_trait(ctx: &Context, def: &ServiceDefinition, style: Style) -> TokenStream {
    let docs = ctx.docs(def.docs());
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
    let name = ctx.field_name(endpoint.endpoint_name());
    let (param, lt) = match style {
        Style::Async => (quote!(<'life0, 'async_trait>), quote!('life0)),
        Style::Sync => (quote!(), quote!()),
    };
    let auth_arg = auth_arg(endpoint);
    let args = endpoint.args().iter().map(|a| arg(ctx, def, a));
    let result = ctx.result_ident(def.service_name());
    let ret_ty = rust_return_type(ctx, def, endpoint, &return_type(ctx, endpoint));
    let mut ret_ty = quote!(#result<#ret_ty, conjure_http::private::Error>);
    if let Style::Async = style {
        let box_ = ctx.box_ident(def.service_name());
        let send = ctx.send_ident(def.service_name());
        ret_ty = quote! {
            conjure_http::private::Pin<
                #box_<dyn conjure_http::private::Future<Output = #ret_ty> + 'async_trait + #send>,
            >
        }
    }
    let where_ = match style {
        Style::Async => quote! {
            where
                'life0: 'async_trait,
                Self: 'life0,
        },
        Style::Sync => quote!(),
    };

    // ignore deprecation since the endpoint has to be implemented regardless
    quote! {
        #docs
        fn #name #param(&#lt self #auth_arg #(, #args)*) -> #ret_ty #where_;
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
    let name = resource_name(ctx, def);
    let sync_resource_impl = generate_resource_impl(ctx, def, Style::Sync);
    let async_resource_impl = generate_resource_impl(ctx, def, Style::Async);

    quote! {
        pub struct #name<T>(T);

        impl<T> #name<T> {
            /// Creates a new resource.
            pub fn new(handler: T) -> #name<T> {
                #name(handler)
            }
        }

        #sync_resource_impl
        #async_resource_impl
    }
}

fn resource_name(ctx: &Context, def: &ServiceDefinition) -> Ident {
    ctx.type_name(&format!("{}Resource", def.service_name().name()))
}

fn generate_resource_impl(ctx: &Context, def: &ServiceDefinition, style: Style) -> TokenStream {
    let name = resource_name(ctx, def);
    let resource_trait_name = match style {
        Style::Async => quote!(AsyncResource),
        Style::Sync => quote!(Resource),
    };
    let trait_name = trait_name(ctx, def, style);
    let params = params(ctx, def);
    let trait_where = match style {
        Style::Async => {
            let sync = ctx.sync_ident(def.service_name());
            let send = ctx.send_ident(def.service_name());
            quote! {
                where
                    T: #trait_name #params + #sync + #send,
                    I: #send,
            }
        }
        Style::Sync => quote! {
            where
                T: #trait_name #params,
        },
    };
    let name_str = def.service_name().name();
    let vec = ctx.vec_ident(def.service_name());
    let endpoint_name = match style {
        Style::Async => quote!(AsyncEndpoint),
        Style::Sync => quote!(Endpoint),
    };
    let endpoints_where = match style {
        Style::Async => {
            let send = ctx.send_ident(def.service_name());
            quote! {
                where
                    B: conjure_http::server::RequestBody<BinaryBody = I> + #send,
                    R: conjure_http::server::AsyncVisitResponse<BinaryWriter = O> + #send,
            }
        }
        Style::Sync => quote! {
            where
                B: conjure_http::server::RequestBody<BinaryBody = I>,
                R: conjure_http::server::VisitResponse<BinaryWriter = O>,
        },
    };

    let handler_service_trait_params = handler_service_trait_params(ctx, def);

    let handlers = def
        .endpoints()
        .iter()
        .map(|e| generate_handler(ctx, def, e, &handler_service_trait_params, style));

    let endpoints = def
        .endpoints()
        .iter()
        .map(|e| generate_endpoint(ctx, e, style));

    quote! {
        #(#handlers)*

        impl<T, I, O> conjure_http::server::#resource_trait_name<I, O> for #name<T>
        #trait_where
        {
            const NAME: &'static str = #name_str;

            fn endpoints<B, R>() -> #vec<conjure_http::server::#endpoint_name<Self, B, R>>
            #endpoints_where
            {
                vec![
                    #(#endpoints,)*
                ]
            }
        }
    }
}

fn handler_service_trait_params(ctx: &Context, def: &ServiceDefinition) -> TokenStream {
    let mut params = vec![];
    if service_has_binary_request_body(ctx, def) {
        params.push(quote!(B::BinaryBody));
    }
    if service_has_binary_response_body(ctx, def) {
        params.push(quote!(R::BinaryWriter));
    }

    if params.is_empty() {
        quote!()
    } else {
        quote!(<#(#params,)*>)
    }
}

fn generate_handler(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
    trait_params: &TokenStream,
    style: Style,
) -> TokenStream {
    let name = handler_name(ctx, endpoint, style);
    let handler_trait_name = match style {
        Style::Async => quote!(AsyncHandler),
        Style::Sync => quote!(Handler),
    };
    let resource_name = resource_name(ctx, def);
    let result = ctx.result_ident(def.service_name());
    let trait_name = trait_name(ctx, def, style);
    let trait_where = match style {
        Style::Async => {
            let sync = ctx.sync_ident(def.service_name());
            let send = ctx.send_ident(def.service_name());
            quote! {
                where
                    T: #trait_name #trait_params + #sync + #send,
                    B: conjure_http::server::RequestBody + #send,
                    B::BinaryBody: #send,
                    R: conjure_http::server::AsyncVisitResponse + #send,
            }
        }
        Style::Sync => {
            quote! {
                where
                    T: #trait_name #trait_params,
                    B: conjure_http::server::RequestBody,
                    R: conjure_http::server::VisitResponse,
            }
        }
    };

    let (params, lt) = match style {
        Style::Async => (quote!(<'a>), quote!('a)),
        Style::Sync => (quote!(), quote!()),
    };
    let service = quote!(service_);
    let mut path_params = quote!(path_params_);
    let mut query_params = quote!(query_params_);
    let mut headers = quote!(headers_);
    let body = quote!(body_);
    let response_visitor = quote!(response_visitor_);
    let auth = quote!(auth_);
    let response = quote!(response);
    let mut return_type = quote!(#result<R::Output, conjure_http::private::Error>);
    if let Style::Async = style {
        let box_ = ctx.box_ident(def.service_name());
        let send = ctx.send_ident(def.service_name());
        return_type = quote! {
            conjure_http::private::Pin<
                #box_<dyn conjure_http::private::Future<Output = #return_type> + #send + #lt>,
            >
        };
    }

    let handle_where = match style {
        Style::Async => {
            quote! {
                where
                    T: #lt,
                    B: #lt,
                    R: #lt,
            }
        }
        Style::Sync => quote!(),
    };

    let extract_path_params = extract_path_params(ctx, endpoint, &path_params);
    if extract_path_params.is_empty() {
        path_params = quote!(_);
    }

    let extract_query_params = extract_query_params(ctx, def, endpoint, &query_params);
    if extract_query_params.is_empty() {
        query_params = quote!(_);
    }

    let extract_headers = extract_headers(ctx, def, endpoint, &headers, &auth);
    if extract_headers.is_empty() {
        headers = quote!(_);
    }

    let extract_body = extract_body(ctx, endpoint, &body);

    let assign_response = assign_response(endpoint, &response);
    let handle = handle(ctx, endpoint, &auth, &service, style);

    let visit_response = visit_response(ctx, endpoint, &response_visitor, &response, style);

    let mut logic = quote! {
        #(#extract_path_params)*
        #(#extract_query_params)*
        #(#extract_headers)*
        #extract_body

        #assign_response #handle?;
        #visit_response
    };
    if let Style::Async = style {
        let box_ = ctx.box_ident(def.service_name());
        logic = quote! {
            #box_::pin(async move {
                #logic
            })
        };
    }

    quote! {
        struct #name;

        impl<T, B, R> conjure_http::server::#handler_trait_name<#resource_name<T>, B, R> for #name
        #trait_where
        {
            fn handle #params(
                &self,
                #service: &#lt #resource_name<T>,
                #path_params: &#lt conjure_http::PathParams,
                #query_params: &#lt conjure_http::QueryParams,
                #headers: &#lt conjure_http::private::http::HeaderMap,
                #body: B,
                #response_visitor: R,
            ) -> #return_type
            #handle_where
            {
                #logic
            }
        }
    }
}

fn handler_name(ctx: &Context, endpoint: &EndpointDefinition, style: Style) -> Ident {
    let name = ctx.type_name(endpoint.endpoint_name());
    let suffix = match style {
        Style::Async => "Async",
        Style::Sync => "",
    };
    Ident::new(&format!("{}Handler{}_", name, suffix), name.span())
}

fn extract_path_params(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    path_params: &TokenStream,
) -> Vec<TokenStream> {
    let mut params = vec![];

    for arg in endpoint.args() {
        match arg.param_type() {
            ParameterType::Path(_) => {}
            _ => continue,
        }

        let name = ctx.field_name(arg.arg_name());
        let id = &**arg.arg_name();

        let param = quote! {
            let #name = conjure_http::private::parse_path_param(#path_params, #id)?;
        };
        params.push(param);
    }

    params
}

fn extract_query_params(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
    query_params: &TokenStream,
) -> Vec<TokenStream> {
    let mut params = vec![];

    for arg in endpoint.args() {
        let query = match arg.param_type() {
            ParameterType::Query(query) => query,
            _ => continue,
        };

        let name = ctx.field_name(arg.arg_name());
        let param_name = &**arg.arg_name();
        let id = &**query.param_id();
        let ty = ctx.rust_type(def.service_name(), arg.type_());
        let default = ctx.default_ident(def.service_name());

        let param = if ctx.is_optional(arg.type_()).is_some() {
            quote! {
                let mut #name: #ty = #default::default();
                conjure_http::private::parse_optional_query_param(#query_params, #param_name, #id, &mut #name)?;
            }
        } else if ctx.is_list(arg.type_()) {
            quote! {
                let mut #name: #ty = #default::default();
                conjure_http::private::parse_list_query_param(#query_params, #param_name, #id, &mut #name)?;
            }
        } else if ctx.is_set(arg.type_()) {
            quote! {
                let mut #name: #ty = #default::default();
                conjure_http::private::parse_set_query_param(#query_params, #param_name, #id, &mut #name)?;
            }
        } else {
            quote! {
                let #name = conjure_http::private::parse_query_param(#query_params, #param_name, #id)?;
            }
        };
        params.push(param);
    }

    params
}

fn extract_headers(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
    headers: &TokenStream,
    auth: &TokenStream,
) -> Vec<TokenStream> {
    let mut params = vec![];

    for arg in endpoint.args() {
        let header = match arg.param_type() {
            ParameterType::Header(header) => header,
            _ => continue,
        };

        let name = ctx.field_name(arg.arg_name());
        let id = &**header.param_id();

        let arg_name = &**arg.arg_name();

        let param = if ctx.is_optional(arg.type_()).is_some() {
            let ty = ctx.rust_type(def.service_name(), arg.type_());
            let default = ctx.default_ident(def.service_name());
            // we're passing in a reference to punch through alias layers.
            quote! {
                let mut #name: #ty = #default::default();
                conjure_http::private::parse_optional_header(#headers, #arg_name, #id, &mut #name)?;
            }
        } else {
            quote! {
                let #name = conjure_http::private::parse_required_header(#headers, #arg_name, #id)?;
            }
        };
        params.push(param)
    }

    if let Some(param) = extract_auth(endpoint, headers, auth) {
        params.push(param);
    }

    params
}

fn extract_auth(
    endpoint: &EndpointDefinition,
    headers: &TokenStream,
    auth: &TokenStream,
) -> Option<TokenStream> {
    let parser = match endpoint.auth()? {
        AuthType::Cookie(cookie) => {
            let prefix = format!("{}=", cookie.cookie_name());
            quote!(parse_cookie_auth(#headers, #prefix))
        }
        AuthType::Header(_) => quote!(parse_header_auth(#headers)),
    };

    Some(quote! {
        let #auth = conjure_http::private::#parser?;
    })
}

fn extract_body(ctx: &Context, endpoint: &EndpointDefinition, body: &TokenStream) -> TokenStream {
    let arg = endpoint
        .args()
        .iter()
        .find(|a| matches!(a.param_type(), ParameterType::Body(_)));

    let arg = match arg {
        Some(arg) => arg,
        None => {
            return quote! {
                #body.accept(conjure_http::private::EmptyRequestBodyVisitor)?;
            }
        }
    };

    let name = ctx.field_name(arg.arg_name());

    if ctx.is_optional(arg.type_()).is_some() {
        quote! {
            let #name = #body.accept(conjure_http::private::DefaultSerializableRequestBodyVisitor::new())?;
        }
    } else if ctx.is_binary(arg.type_()) {
        quote! {
            let #name = #body.accept(conjure_http::private::BinaryRequestBodyVisitor)?;
        }
    } else {
        quote! {
            let #name = #body.accept(conjure_http::private::SerializableRequestBodyVisitor::new())?;
        }
    }
}

fn assign_response(endpoint: &EndpointDefinition, response: &TokenStream) -> TokenStream {
    if endpoint.returns().is_some() {
        quote!(let #response = )
    } else {
        quote!()
    }
}

fn handle(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    auth: &TokenStream,
    service: &TokenStream,
    style: Style,
) -> TokenStream {
    let name = ctx.field_name(endpoint.endpoint_name());

    let auth = if endpoint.auth().is_some() {
        quote!(#auth,)
    } else {
        quote!()
    };

    let args = endpoint.args().iter().map(|a| ctx.field_name(a.arg_name()));

    let await_ = match style {
        Style::Async => quote!(.await),
        Style::Sync => quote!(),
    };

    quote! {
        #service.0 .#name(#auth #(#args),*) #await_
    }
}

fn visit_response(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    response_visitor: &TokenStream,
    response: &TokenStream,
    style: Style,
) -> TokenStream {
    let returns = return_type(ctx, endpoint);

    let ty_name = |name: &str| {
        let name = match style {
            Style::Async => format!("Async{}", name),
            Style::Sync => name.to_string(),
        };
        let name = Ident::new(&name, Span::call_site());
        quote!(conjure_http::private::#name)
    };

    let response = match returns {
        ReturnType::None => ty_name("EmptyResponse"),
        ReturnType::Json(ty) => {
            let ty = if ctx.is_iterable(ty) {
                ty_name("DefaultSerializableResponse")
            } else {
                ty_name("SerializableResponse")
            };
            quote!(#ty(#response))
        }
        ReturnType::Binary => {
            let ty = ty_name("BinaryResponse");
            quote!(#ty(#response))
        }
        ReturnType::OptionalBinary => {
            let ty = ty_name("OptionalBinaryResponse");
            quote!(#ty(#response))
        }
    };

    quote! {
        #response.accept(#response_visitor)
    }
}

fn generate_endpoint(ctx: &Context, endpoint: &EndpointDefinition, style: Style) -> TokenStream {
    let endpoint_name = match style {
        Style::Async => quote!(AsyncEndpoint),
        Style::Sync => quote!(Endpoint),
    };
    let name = &**endpoint.endpoint_name();
    let method = endpoint
        .http_method()
        .as_str()
        .parse::<TokenStream>()
        .unwrap();
    let path = &**endpoint.http_path();
    let handler = handler_name(ctx, endpoint, style);
    let parameters = parameters(ctx, endpoint);
    let deprecated = endpoint.deprecated().is_some();

    quote! {
        conjure_http::server::#endpoint_name {
            metadata: conjure_http::server::Metadata::new(
                #name,
                conjure_http::private::http::Method::#method,
                #path,
                #parameters,
                #deprecated,
            ),
            handler: &#handler,
        }
    }
}

fn parameters(ctx: &Context, endpoint: &EndpointDefinition) -> TokenStream {
    let parameters = endpoint
        .args()
        .iter()
        .flat_map(|a| parameter(ctx, a))
        .collect::<Vec<_>>();

    if parameters.is_empty() {
        quote! {
            &[]
        }
    } else {
        quote! {
            {
                const PARAMS: &[conjure_http::server::Parameter] = &[
                    #(#parameters,)*
                ];
                PARAMS
            }
        }
    }
}

fn parameter(ctx: &Context, argument: &ArgumentDefinition) -> Option<TokenStream> {
    let name = &**argument.arg_name();

    let type_ = match argument.param_type() {
        ParameterType::Path(_) => quote! {
            conjure_http::server::ParameterType::Path(
                conjure_http::server::PathParameter::new(),
            )
        },
        ParameterType::Query(query) => {
            let key = &**query.param_id();
            quote! {
                conjure_http::server::ParameterType::Query(
                    conjure_http::server::QueryParameter::new(#key),
                )
            }
        }
        ParameterType::Header(header) => {
            let header = &**header.param_id();
            quote! {
                conjure_http::server::ParameterType::Header(
                    conjure_http::server::HeaderParameter::new(#header),
                )
            }
        }
        ParameterType::Body(_) => return None,
    };

    let safe = if argument.tags().iter().any(|s| s == "safe")
        || argument.markers().iter().any(|a| ctx.is_safe_arg(a))
    {
        quote! {
            .with_safe(true)
        }
    } else {
        quote!()
    };

    Some(quote! {
        conjure_http::server::Parameter::new(#name, #type_)
        #safe
    })
}
