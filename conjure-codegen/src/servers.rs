use heck::CamelCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::context::Context;
use crate::types::{
    ArgumentDefinition, AuthType, EndpointDefinition, ParameterType, ServiceDefinition, Type,
};

pub fn generate(ctx: &Context, def: &ServiceDefinition) -> TokenStream {
    let trait_ = generate_trait(ctx, def);
    let resource = generate_resource(ctx, def);

    quote! {
        use conjure_http::server::Response as _;

        #trait_

        #resource
    }
}

fn generate_trait(ctx: &Context, def: &ServiceDefinition) -> TokenStream {
    let docs = ctx.docs(def.docs());
    let name = ctx.type_name(def.service_name().name());
    let param = param(ctx, def);

    let binary_types = def
        .endpoints()
        .iter()
        .flat_map(|e| generate_binary_type(ctx, e));

    let endpoints = def
        .endpoints()
        .iter()
        .map(|e| generate_trait_endpoint(ctx, def, e));

    quote! {
        #docs
        pub trait #name #param {
            #(#binary_types)*

            #(#endpoints)*
        }
    }
}

fn param(ctx: &Context, def: &ServiceDefinition) -> TokenStream {
    if service_has_binary_body(ctx, def) {
        quote!(<T>)
    } else {
        quote!()
    }
}

fn service_has_binary_body(ctx: &Context, def: &ServiceDefinition) -> bool {
    def.endpoints()
        .iter()
        .any(|e| endpoint_has_binary_body(ctx, e))
}

fn endpoint_has_binary_body(ctx: &Context, endpoint: &EndpointDefinition) -> bool {
    endpoint.args().iter().any(|a| match a.param_type() {
        ParameterType::Body(_) => ctx.is_binary(a.type_()),
        _ => false,
    })
}

fn generate_binary_type(ctx: &Context, endpoint: &EndpointDefinition) -> Option<TokenStream> {
    match return_type(ctx, endpoint) {
        ReturnType::Binary | ReturnType::OptionalBinary => {
            let docs = format!(
                "The body type returned by the `{}` method.",
                ctx.field_name(endpoint.endpoint_name())
            );
            let name = binary_type(endpoint);
            Some(quote! {
                #[doc = #docs]
                type #name: conjure_http::server::WriteBody + 'static;
            })
        }
        ReturnType::None | ReturnType::Json(_) => None,
    }
}

fn binary_type(endpoint: &EndpointDefinition) -> TokenStream {
    format!("{}Body", endpoint.endpoint_name().to_camel_case())
        .parse()
        .unwrap()
}

fn generate_trait_endpoint(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
) -> TokenStream {
    let docs = ctx.docs(endpoint.docs());
    let name = ctx.field_name(endpoint.endpoint_name());
    let auth_arg = auth_arg(endpoint);
    let args = endpoint.args().iter().map(|a| arg(ctx, def, a));
    let result = ctx.result_ident(def.service_name());
    let ret_ty = rust_return_type(ctx, def, endpoint, &return_type(ctx, endpoint));

    // ignore deprecation since the endpoint has to be implemented regardless
    quote! {
        #docs
        fn #name(&self #auth_arg #(, #args)*) -> #result<#ret_ty, conjure_http::private::Error>;
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
        quote!(T)
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
    let name = ctx.type_name(&format!("{}Resource", def.service_name().name()));
    let trait_name = ctx.type_name(def.service_name().name());
    let name_str = def.service_name().name();
    let vec = ctx.vec_ident(def.service_name());

    let endpoint_fns = def
        .endpoints()
        .iter()
        .map(|e| generate_endpoint_fn(ctx, def, e));

    let endpoints = def
        .endpoints()
        .iter()
        .map(|e| generate_endpoint(ctx, def, e));

    quote! {
        pub struct #name<T>(T);

        impl<T> #name<T> {
            /// Creates a new resource.
            pub fn new(handler: T) -> #name<T> {
                #name(handler)
            }
        }

        impl<T> #name<T> {
            #(#endpoint_fns)*
        }

        impl<T, B, R> conjure_http::server::Resource<B, R> for #name<T>
        where
            T: #trait_name<B::Body>,
            B: conjure_http::server::RequestBody,
            R: conjure_http::server::VisitResponse,
        {
            const NAME: &'static str = #name_str;

            fn endpoints() -> #vec<conjure_http::server::Endpoint<Self, B, R>> {
                vec![
                    #(#endpoints,)*
                ]
            }
        }
    }
}

fn generate_endpoint_fn(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
) -> TokenStream {
    let name = ctx.field_name(endpoint.endpoint_name());
    let result = ctx.result_ident(def.service_name());
    let trait_name = ctx.type_name(def.service_name().name());

    let mut path_params = quote!(path_params_);
    let mut query_params = quote!(query_params_);
    let mut headers = quote!(headers_);
    let body = quote!(body_);
    let response_visitor = quote!(response_visitor_);
    let auth = quote!(auth_);
    let response = quote!(response);

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
    let handle = handle(ctx, endpoint, &auth);

    let visit_response = visit_response(ctx, endpoint, &response_visitor, &response);

    quote! {
        fn #name<B, R>(
            &self,
            #path_params: &conjure_http::PathParams,
            #query_params: &conjure_http::QueryParams,
            #headers: &conjure_http::private::http::HeaderMap,
            #body: B,
            #response_visitor: R,
        ) -> #result<R::Output, conjure_http::private::Error>
        where
            T: #trait_name<B::Body>,
            B: conjure_http::server::RequestBody,
            R: conjure_http::server::VisitResponse,
        {
            #(#extract_path_params)*
            #(#extract_query_params)*
            #(#extract_headers)*
            #extract_body

            #assign_response #handle?;
            #visit_response
        }
    }
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
    let arg = endpoint.args().iter().find(|a| match a.param_type() {
        ParameterType::Body(_) => true,
        _ => false,
    });

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

fn handle(ctx: &Context, endpoint: &EndpointDefinition, auth: &TokenStream) -> TokenStream {
    let name = ctx.field_name(endpoint.endpoint_name());

    let auth = if endpoint.auth().is_some() {
        quote!(#auth,)
    } else {
        quote!()
    };

    let args = endpoint.args().iter().map(|a| ctx.field_name(a.arg_name()));

    quote! {
        (self.0).#name(#auth #(#args),*)
    }
}

fn visit_response(
    ctx: &Context,
    endpoint: &EndpointDefinition,
    response_visitor: &TokenStream,
    response: &TokenStream,
) -> TokenStream {
    let returns = return_type(ctx, endpoint);

    let response = match returns {
        ReturnType::None => quote!(conjure_http::private::EmptyResponse),
        ReturnType::Json(ty) => {
            if ctx.is_iterable(ty) {
                quote! {
                    conjure_http::private::DefaultSerializableResponse(#response)
                }
            } else {
                quote! {
                    conjure_http::private::SerializableResponse(#response)
                }
            }
        }
        ReturnType::Binary => quote! {
            conjure_http::private::BinaryResponse(#response)
        },
        ReturnType::OptionalBinary => quote! {
            conjure_http::private::OptionalBinaryResponse(#response)
        },
    };

    quote! {
        #response.accept(#response_visitor)
    }
}

fn generate_endpoint(
    ctx: &Context,
    def: &ServiceDefinition,
    endpoint: &EndpointDefinition,
) -> TokenStream {
    let name = &**endpoint.endpoint_name();
    let method = endpoint
        .http_method()
        .as_str()
        .parse::<TokenStream>()
        .unwrap();
    let path = &**endpoint.http_path();
    let resource = ctx.type_name(&format!("{}Resource", def.service_name().name()));
    let handler = ctx.field_name(endpoint.endpoint_name());
    let parameters = parameters(ctx, endpoint);

    quote! {
        conjure_http::server::Endpoint::new(
            #name,
            conjure_http::private::http::Method::#method,
            #path,
            #resource::#handler,
            #parameters,
        )
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

    let safe = if argument.markers().iter().any(|a| ctx.is_safe_arg(a)) {
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
