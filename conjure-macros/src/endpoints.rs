use crate::path::{self, PathComponent};
use heck::ToUpperCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use structmeta::StructMeta;
use syn::{
    parse_macro_input, Error, FnArg, ItemTrait, LitStr, Meta, Pat, PatType, TraitItem, TraitItemFn,
    Type, Visibility,
};

pub fn generate(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as ItemTrait);
    let service = Service::new(&item);
    strip_trait(&mut item);
    let service = match service {
        Ok(service) => service,
        Err(e) => return e.into_compile_error().into(),
    };
    let endpoints = generate_endpoints(&service);

    quote! {
        #item

        #endpoints
    }
    .into()
}

// Rust doesn't support helper attributes in attribute macros so we need to manually strip them out
fn strip_trait(trait_: &mut ItemTrait) {
    for item in &mut trait_.items {
        if let TraitItem::Fn(fn_) = item {
            strip_fn(fn_);
        }
    }
}

fn strip_fn(fn_: &mut TraitItemFn) {
    for arg in &mut fn_.sig.inputs {
        strip_arg(arg);
    }
}

fn strip_arg(arg: &mut FnArg) {
    let FnArg::Typed(arg) = arg else { return };

    arg.attrs.retain(|attr| {
        !["path", "query", "header", "auth", "body", "context"]
            .iter()
            .any(|v| attr.path().is_ident(v))
    })
}

fn generate_endpoints(service: &Service) -> TokenStream {
    let vis = &service.vis;
    let trait_name = &service.name;
    let type_name = Ident::new(&format!("{}Endpoints", service.name), service.name.span());

    let endpoints = service
        .endpoints
        .iter()
        .map(|e| generate_endpoint(service, e));

    let endpoint_values = service.endpoints.iter().map(|e| {
        let name = endpoint_name(e);
        quote!(conjure_http::private::Box::new(#name(self.0.clone())))
    });

    quote! {
        #vis struct #type_name<T>(conjure_http::private::Arc<T>);

        impl<T> #type_name<T> {
            /// Creates a new resource.
            pub fn new(handler: T) -> Self {
                #type_name(conjure_http::private::Arc::new(handler))
            }
        }

        impl<T, I, O> conjure_http::server::Service<I, O> for #type_name<T>
        where
            T: #trait_name + 'static + Sync + Send,
            I: conjure_http::private::Iterator<
                Item = conjure_http::private::Result<
                    conjure_http::private::Bytes,
                    conjure_http::private::Error,
                >,
            >,
        {
            fn endpoints(
                &self,
            ) -> conjure_http::private::Vec<
                conjure_http::private::Box<dyn conjure_http::server::Endpoint<I, O> + Sync + Send>,
            > {
                #(#endpoints)*

                vec![#(#endpoint_values,)*]
            }
        }
    }
}

fn endpoint_name(endpoint: &Endpoint) -> Ident {
    Ident::new(
        &format!(
            "__{}Endpoint",
            endpoint.ident.to_string().to_upper_camel_case()
        ),
        endpoint.ident.span(),
    )
}

fn generate_endpoint(service: &Service, endpoint: &Endpoint) -> TokenStream {
    let name = endpoint_name(endpoint);

    let metadata = generate_endpoint_metadata(service, endpoint);
    let handler = generate_endpoint_handler(service, endpoint);

    quote! {
        struct #name<T>(conjure_http::private::Arc<T>);

        #metadata
        #handler
    }
}

fn generate_endpoint_metadata(service: &Service, endpoint: &Endpoint) -> TokenStream {
    let struct_name = endpoint_name(endpoint);
    let method = &endpoint.params.method;
    let path = endpoint.path.iter().map(|c| match c {
        PathComponent::Literal(lit) => {
            quote! {
                conjure_http::server::PathSegment::Literal(
                    conjure_http::private::Cow::Borrowed(#lit),
                )
            }
        }
        PathComponent::Parameter(param) => {
            quote! {
                conjure_http::server::PathSegment::Parameter {
                    name: conjure_http::private::Cow::Borrowed(#param),
                    regex: conjure_http::private::Option::None,
                }
            }
        }
    });
    let template = &endpoint.params.path;
    let service_name = service.name.to_string();
    let name = endpoint.ident.to_string();

    quote! {
        impl<T> conjure_http::server::EndpointMetadata for #struct_name<T> {
            fn method(&self) -> conjure_http::private::Method {
                conjure_http::private::Method::#method
            }

            fn path(&self) -> &[conjure_http::server::PathSegment] {
                &[#(#path,)*]
            }

            fn template(&self) -> &str {
                #template
            }

            fn service_name(&self) -> &str {
                #service_name
            }

            fn name(&self) -> &str {
                #name
            }

            fn deprecated(&self) -> conjure_http::private::Option<&str> {
                conjure_http::private::Option::None
            }
        }
    }
}

fn generate_endpoint_handler(service: &Service, endpoint: &Endpoint) -> TokenStream {
    let struct_name = endpoint_name(endpoint);
    let trait_name = &service.name;

    let request = quote!(__request);
    let response_extensions = quote!(__response_extensions);
    let parts = quote!(__parts);
    let body = quote!(__body);
    let query_params = quote!(__query_params);
    let safe_params = quote!(__safe_params);
    let response = quote!(__response);
    let method = &endpoint.ident;

    let generate_query_params = if has_query_params(endpoint) {
        quote! {
            let #query_params = conjure_http::private::parse_query_params(&#parts);
        }
    } else {
        quote!()
    };

    let generate_safe_params = if has_safe_params(endpoint) {
        quote! {
            #response_extensions.insert(conjure_http::SafeParams::new());
            let #safe_params = #response_extensions.get_mut::<conjure_http::SafeParams>().unwrap();
        }
    } else {
        quote!()
    };

    let generate_args = endpoint.args.iter().map(|arg| {
        generate_arg(
            &parts,
            &body,
            &query_params,
            &response_extensions,
            &safe_params,
            arg,
        )
    });

    let args = endpoint.args.iter().map(|arg| arg.ident());

    let generate_response = generate_response(&parts, &response, endpoint);

    quote! {
        impl<T, I, O> conjure_http::server::Endpoint<I, O> for #struct_name<T>
        where
            T: #trait_name + 'static + Sync + Send,
            I: conjure_http::private::Iterator<
                Item = conjure_http::private::Result<
                    conjure_http::private::Bytes,
                    conjure_http::private::Error,
                >,
            >,
        {
            fn handle(
                &self,
                #request: conjure_http::private::Request<I>,
                #response_extensions: &mut conjure_http::private::Extensions,
            ) -> conjure_http::private::Result<
                conjure_http::private::Response<conjure_http::server::ResponseBody<O>>,
                conjure_http::private::Error,
            > {
                let (#parts, #body) = #request.into_parts();
                #generate_query_params
                #generate_safe_params
                #(#generate_args)*
                let #response = self.0.#method(#(#args)*)?;
                #generate_response
            }
        }
    }
}

fn has_safe_params(endpoint: &Endpoint) -> bool {
    endpoint.args.iter().any(|a| a.safe())
}

fn has_query_params(endpoint: &Endpoint) -> bool {
    endpoint.args.iter().any(|a| matches!(a, ArgType::Query(_)))
}

fn generate_arg(
    parts: &TokenStream,
    body: &TokenStream,
    query_params: &TokenStream,
    response_extensions: &TokenStream,
    safe_params: &TokenStream,
    arg: &ArgType,
) -> TokenStream {
    let generate_arg = match arg {
        ArgType::Path(arg) => generate_path_arg(parts, arg),
        ArgType::Query(arg) => generate_query_arg(query_params, arg),
        ArgType::Header(arg) => generate_header_arg(parts, arg),
        ArgType::Auth(arg) => generate_auth_arg(parts, arg),
        ArgType::Body(arg) => generate_body_arg(parts, body, arg),
        ArgType::Context(arg) => generate_context_arg(parts, response_extensions, arg),
    };

    let safe_log = if arg.safe() {
        let name = &arg.ident();
        let key = name.to_string();
        quote! {
            #safe_params.insert(#key, &#name);
        }
    } else {
        quote!()
    };

    quote! {
        #generate_arg
        #safe_log
    }
}

fn generate_path_arg(parts: &TokenStream, arg: &Arg<PathArg>) -> TokenStream {
    let name = &arg.ident;
    let param = arg.ident.to_string();
    let decoder = arg.params.decoder.as_ref().map_or_else(
        || quote!(conjure_http::server::FromStrDecoder),
        |d| quote!(#d),
    );
    quote! {
        let #name = conjure_http::private::path_param::<_, #decoder>(&#parts, #param)?;
    }
}

fn generate_query_arg(query_params: &TokenStream, arg: &Arg<ParamArg>) -> TokenStream {
    let name = &arg.ident;
    let key = &arg.params.name;
    let param = arg.ident.to_string();
    let decoder = arg.params.decoder.as_ref().map_or_else(
        || quote!(conjure_http::server::FromStrDecoder),
        |d| quote!(#d),
    );
    quote! {
        let #name = conjure_http::private::query_param::<_, #decoder>(&#query_params, #key, #param)?;
    }
}

fn generate_header_arg(parts: &TokenStream, arg: &Arg<ParamArg>) -> TokenStream {
    let name = &arg.ident;
    let header = &arg.params.name;
    let param = arg.ident.to_string();
    let decoder = arg.params.decoder.as_ref().map_or_else(
        || quote!(conjure_http::server::FromStrDecoder),
        |d| quote!(#d),
    );
    quote! {
        let #name = conjure_http::private::header_param::<_, #decoder>(&#parts, #header, #param)?;
    }
}

fn generate_auth_arg(parts: &TokenStream, arg: &Arg<AuthArg>) -> TokenStream {
    let name = &arg.ident;
    let call = match &arg.params.cookie_name {
        Some(cookie_name) => {
            let prefix = format!("{}=", cookie_name.value());
            quote!(parse_cookie_auth(&#parts, #prefix))
        }
        None => quote!(parse_header_auth(&#parts)),
    };

    quote! {
        let #name = conjure_http::private::#call?;
    }
}

fn generate_body_arg(parts: &TokenStream, body: &TokenStream, arg: &Arg<BodyArg>) -> TokenStream {
    let name = &arg.ident;
    let deserializer = arg.params.deserializer.as_ref().map_or_else(
        || quote!(conjure_http::server::ConjureRequestDeserializer),
        |d| quote!(#d),
    );
    quote! {
        let #name = <#deserializer as conjure_http::server::DeserializeRequest<_, _>>::deserialize(
            &#parts.headers,
            #body,
        )?;
    }
}

fn generate_context_arg(
    parts: &TokenStream,
    response_extensions: &TokenStream,
    arg: &Arg<ContextArg>,
) -> TokenStream {
    let name = &arg.ident;
    quote! {
        let #name = conjure_http::server::RequestContext::new(&#parts, #response_extensions);
    }
}

fn generate_response(
    parts: &TokenStream,
    response: &TokenStream,
    endpoint: &Endpoint,
) -> TokenStream {
    let serializer = endpoint.params.produces.as_ref().map_or_else(
        || quote!(conjure_http::server::EmptyResponseSerializer),
        |s| quote!(#s),
    );

    quote! {
        <#serializer as conjure_http::server::SerializeResponse<_, _>>::serialize(
            &#parts.headers,
            #response,
        )
    }
}

struct Service {
    vis: Visibility,
    name: Ident,
    endpoints: Vec<Endpoint>,
}

impl Service {
    fn new(trait_: &ItemTrait) -> Result<Self, Error> {
        let mut error = None;
        let mut endpoints = vec![];
        for item in &trait_.items {
            match Endpoint::new(item) {
                Ok(endpoint) => endpoints.push(endpoint),
                Err(mut e) => {
                    if let Some(other) = error {
                        e.combine(other);
                    }
                    error = Some(e);
                }
            }
        }

        match error {
            Some(error) => Err(error),
            None => Ok(Service {
                vis: trait_.vis.clone(),
                name: trait_.ident.clone(),
                endpoints,
            }),
        }
    }
}

struct Endpoint {
    ident: Ident,
    args: Vec<ArgType>,
    params: EndpointParams,
    path: Vec<PathComponent>,
}

impl Endpoint {
    fn new(item: &TraitItem) -> Result<Self, Error> {
        let TraitItem::Fn(item) = item else {
            return Err(Error::new_spanned(item, "Conjure traits may only contain methods"));
        };

        let mut errors = Errors::new();

        let mut endpoint_attrs = item
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("endpoint"));
        let params = endpoint_attrs
            .next()
            .ok_or_else(|| Error::new_spanned(item, "missing #[endpoint] attribute"))
            .and_then(|a| a.parse_args::<EndpointParams>());
        let params = match params {
            Ok(params) => Some(params),
            Err(e) => {
                errors.push(e);
                None
            }
        };

        let mut args = vec![];
        for arg in &item.sig.inputs {
            // Ignore the self arg.
            let FnArg::Typed(arg) = arg else { continue };

            match ArgType::new(arg) {
                Ok(arg) => args.push(arg),
                Err(e) => errors.push(e),
            }
        }

        let path = match params.as_ref().map(|p| path::parse(&p.path)).transpose() {
            Ok(path) => path,
            Err(e) => {
                errors.push(e);
                None
            }
        };

        // FIXME validate args

        errors.build()?;

        Ok(Endpoint {
            ident: item.sig.ident.clone(),
            args,
            params: params.unwrap(),
            path: path.unwrap(),
        })
    }
}

#[derive(StructMeta)]
struct EndpointParams {
    method: Ident,
    path: LitStr,
    produces: Option<Type>,
}

enum ArgType {
    Path(Arg<PathArg>),
    Query(Arg<ParamArg>),
    Header(Arg<ParamArg>),
    Auth(Arg<AuthArg>),
    Body(Arg<BodyArg>),
    Context(Arg<ContextArg>),
}

impl ArgType {
    fn new(arg: &PatType) -> Result<Self, Error> {
        let ident = match &*arg.pat {
            Pat::Ident(pat_ident) => &pat_ident.ident,
            _ => {
                return Err(Error::new_spanned(&arg.pat, "expected an ident pattern"));
            }
        };

        let mut type_ = None;

        // FIXME detect multiple attrs
        for attr in &arg.attrs {
            if attr.path().is_ident("path") {
                let attr = match attr.meta {
                    Meta::Path(_) => PathArg::default(),
                    _ => attr.parse_args()?,
                };
                type_ = Some(ArgType::Path(Arg {
                    ident: ident.clone(),
                    params: attr,
                }));
            } else if attr.path().is_ident("query") {
                let attr = attr.parse_args()?;
                type_ = Some(ArgType::Query(Arg {
                    ident: ident.clone(),
                    params: attr,
                }));
            } else if attr.path().is_ident("header") {
                let attr = attr.parse_args()?;
                type_ = Some(ArgType::Header(Arg {
                    ident: ident.clone(),
                    params: attr,
                }));
            } else if attr.path().is_ident("auth") {
                let attr = match attr.meta {
                    Meta::Path(_) => AuthArg::default(),
                    _ => attr.parse_args()?,
                };
                type_ = Some(ArgType::Auth(Arg {
                    ident: ident.clone(),
                    params: attr,
                }));
            } else if attr.path().is_ident("body") {
                let attr = match attr.meta {
                    Meta::Path(_) => BodyArg::default(),
                    _ => attr.parse_args()?,
                };
                type_ = Some(ArgType::Body(Arg {
                    ident: ident.clone(),
                    params: attr,
                }));
            } else if attr.path().is_ident("context") {
                let attr = match attr.meta {
                    Meta::Path(_) => ContextArg::default(),
                    _ => attr.parse_args()?,
                };
                type_ = Some(ArgType::Context(Arg {
                    ident: ident.clone(),
                    params: attr,
                }));
            }
        }

        type_.ok_or_else(|| Error::new_spanned(arg, "missing paramter type attribute"))
    }

    fn ident(&self) -> &Ident {
        match self {
            ArgType::Path(arg) => &arg.ident,
            ArgType::Query(arg) => &arg.ident,
            ArgType::Header(arg) => &arg.ident,
            ArgType::Auth(arg) => &arg.ident,
            ArgType::Body(arg) => &arg.ident,
            ArgType::Context(arg) => &arg.ident,
        }
    }

    fn safe(&self) -> bool {
        match self {
            ArgType::Path(arg) => arg.params.safe,
            ArgType::Query(arg) => arg.params.safe,
            ArgType::Header(arg) => arg.params.safe,
            ArgType::Auth(_) => false,
            ArgType::Body(arg) => arg.params.safe,
            ArgType::Context(_) => false,
        }
    }
}

struct Arg<T> {
    ident: Ident,
    params: T,
}

#[derive(StructMeta, Default)]
struct PathArg {
    safe: bool,
    decoder: Option<Type>,
}

#[derive(StructMeta)]
struct ParamArg {
    safe: bool,
    name: LitStr,
    decoder: Option<Type>,
}

#[derive(StructMeta, Default)]
struct AuthArg {
    cookie_name: Option<LitStr>,
}

#[derive(StructMeta, Default)]
struct BodyArg {
    safe: bool,
    deserializer: Option<Type>,
}

#[derive(StructMeta, Default)]
struct ContextArg {}

struct Errors(Vec<Error>);

impl Errors {
    fn new() -> Self {
        Errors(vec![])
    }

    fn push(&mut self, error: Error) {
        self.0.push(error);
    }

    fn build(mut self) -> Result<(), Error> {
        let Some(mut error) = self.0.pop() else { return Ok(()) };
        for other in self.0 {
            error.combine(other);
        }
        Err(error)
    }
}
