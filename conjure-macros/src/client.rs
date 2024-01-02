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
use crate::{Asyncness, Errors};
use http::HeaderName;
use percent_encoding::AsciiSet;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use structmeta::StructMeta;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Error, FnArg, GenericParam, Generics, ItemTrait, LitStr, Meta, Pat, PatType,
    ReturnType, TraitItem, TraitItemFn, Type, Visibility,
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
    let service = match Service::new(&mut item) {
        Ok(service) => service,
        Err(e) => return e.into_compile_error().into(),
    };
    let client = generate_client(&service);

    quote! {
        #item

        #client
    }
    .into()
}

fn generate_client(service: &Service) -> TokenStream {
    let vis = &service.vis;
    let trait_name = &service.name;
    let type_name = Ident::new(&format!("{}Client", trait_name), trait_name.span());

    let service_trait = match service.asyncness {
        Asyncness::Sync => quote!(Service),
        Asyncness::Async => quote!(AsyncService),
    };

    let impl_attrs = match service.asyncness {
        Asyncness::Sync => quote!(),
        Asyncness::Async => quote!(#[conjure_http::private::async_trait]),
    };

    let (_, type_generics, _) = service.generics.split_for_impl();

    let mut impl_generics = service.generics.clone();

    let client_param = quote!(__C);
    impl_generics.params.push(syn::parse2(quote!(__C)).unwrap());

    let where_clause = impl_generics.make_where_clause();
    let client_trait = match service.asyncness {
        Asyncness::Sync => quote!(Client),
        Asyncness::Async => quote!(AsyncClient),
    };
    let mut client_bindings = vec![];
    if let Some(param) = &service.request_writer_param {
        client_bindings.push(quote!(BodyWriter = #param));
    }
    if let Some(param) = &service.response_body_param {
        client_bindings.push(quote!(ResponseBody = #param));
    }
    let extra_client_predicates = match service.asyncness {
        Asyncness::Sync => quote!(),
        Asyncness::Async => quote!(+ conjure_http::private::Sync + conjure_http::private::Send),
    };
    where_clause.predicates.push(
        syn::parse2(quote! {
            #client_param: conjure_http::client::#client_trait<#(#client_bindings),*> #extra_client_predicates
        })
        .unwrap(),
    );
    if let Asyncness::Async = service.asyncness {
        where_clause.predicates.push(
            syn::parse2(quote!(#client_param::ResponseBody: 'static + conjure_http::private::Send))
                .unwrap(),
        );
    }

    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();

    let methods = service
        .endpoints
        .iter()
        .map(|endpoint| generate_client_method(&client_param, service, endpoint));

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
        impl #impl_generics #trait_name #type_generics for #type_name<#client_param>
        #where_clause
        {
            #(#methods)*
        }
    }
}

fn generate_client_method(
    client_param: &TokenStream,
    service: &Service,
    endpoint: &Endpoint,
) -> TokenStream {
    let async_ = match service.asyncness {
        Asyncness::Sync => quote!(),
        Asyncness::Async => quote!(async),
    };

    let client_trait = match service.asyncness {
        Asyncness::Sync => quote!(Client),
        Asyncness::Async => quote!(AsyncClient),
    };

    let await_ = match service.asyncness {
        Asyncness::Sync => quote!(),
        Asyncness::Async => quote!(.await),
    };

    let name = &endpoint.ident;
    let args = endpoint.args.iter().map(|a| {
        let ident = a.ident();
        let ty = a.ty();
        quote!(#ident: #ty)
    });
    let ret_ty = &endpoint.ret_ty;

    let request = quote!(__request);
    let response = quote!(__response);
    let http_method = &endpoint.params.method;

    let create_request = create_request(client_param, &request, service, endpoint);
    let add_path = add_path(&request, endpoint);
    let add_accept = add_accept(client_param, &request, service, endpoint);
    let add_auth = add_auth(&request, endpoint);
    let add_headers = add_headers(&request, endpoint);
    let add_endpoint = add_endpoint(&request, service, endpoint);
    let handle_response = handle_response(&response, service, endpoint);

    quote! {
        #async_ fn #name(&self #(, #args)*) -> #ret_ty {
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

fn create_request(
    client_param: &TokenStream,
    request: &TokenStream,
    service: &Service,
    endpoint: &Endpoint,
) -> TokenStream {
    let arg = endpoint.args.iter().find_map(|a| match a {
        ArgType::Body(arg) => Some(arg),
        _ => None,
    });
    let Some(arg) = arg else {
        let body = match service.asyncness {
            Asyncness::Sync => quote!(RequestBody),
            Asyncness::Async => quote!(AsyncRequestBody),
        };
        return quote! {
            let mut #request = conjure_http::private::Request::new(
                conjure_http::client::#body::Empty,
            );
        };
    };

    let trait_ = match service.asyncness {
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
            #serializer as conjure_http::client::#trait_<_, #client_param::BodyWriter>
        >::content_type(&#ident);
        let __content_length = <
            #serializer as conjure_http::client::#trait_<_, #client_param::BodyWriter>
        >::content_length(&#ident);
        let __body = <
            #serializer as conjure_http::client::#trait_<_, #client_param::BodyWriter>
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

fn add_path(request: &TokenStream, endpoint: &Endpoint) -> TokenStream {
    let builder = quote!(__path);

    let path_writes = add_path_components(&builder, endpoint);

    let query_params = endpoint
        .args
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

fn add_path_components(builder: &TokenStream, endpoint: &Endpoint) -> TokenStream {
    let path_params = endpoint
        .args
        .iter()
        .filter_map(|a| match a {
            ArgType::Path(param) => Some((param.ident.to_string(), param)),
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    let mut path_writes = vec![];
    let mut literal_buf = String::new();
    for component in &endpoint.path {
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

                let param = path_params[param];

                let ident = &param.ident;
                let encoder = param.attr.encoder.as_ref().map_or_else(
                    || quote!(conjure_http::client::DisplayEncoder),
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
        || quote!(conjure_http::client::DisplayEncoder),
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
    client_param: &TokenStream,
    request: &TokenStream,
    service: &Service,
    endpoint: &Endpoint,
) -> TokenStream {
    let Some(accept) = &endpoint.params.accept else {
        return quote!();
    };

    let trait_ = match service.asyncness {
        Asyncness::Sync => quote!(DeserializeResponse),
        Asyncness::Async => quote!(AsyncDeserializeResponse),
    };

    let ret_ty = &endpoint.ret_ty;

    quote! {
        let __accept = <#accept as conjure_http::client::#trait_<
            <#ret_ty as conjure_http::private::ExtractOk>::Ok,
            #client_param::ResponseBody,
        >>::accept();
        if let Some(__accept) = __accept {
            #request.headers_mut().insert(conjure_http::private::header::ACCEPT, __accept);
        }
    }
}

fn add_auth(request: &TokenStream, endpoint: &Endpoint) -> TokenStream {
    let mut it = endpoint.args.iter().filter_map(|a| match a {
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

fn add_headers(request: &TokenStream, endpoint: &Endpoint) -> TokenStream {
    let add_headers = endpoint
        .args
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
    let ident = &arg.ident;
    let name = arg.attr.name.value().to_ascii_lowercase();
    let encoder = arg.attr.encoder.as_ref().map_or_else(
        || quote!(conjure_http::client::DisplayEncoder),
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

fn add_endpoint(request: &TokenStream, service: &Service, endpoint: &Endpoint) -> TokenStream {
    let service = service.name.to_string();
    let name = endpoint.ident.to_string();
    let path = &endpoint.params.path;

    quote! {
        #request.extensions_mut().insert(conjure_http::client::Endpoint::new(
            #service,
            conjure_http::private::Option::Some(conjure_http::private::env!("CARGO_PKG_VERSION")),
            #name,
            #path,
        ));
    }
}

fn handle_response(response: &TokenStream, service: &Service, endpoint: &Endpoint) -> TokenStream {
    let accept = endpoint.params.accept.as_ref().map_or_else(
        || quote!(conjure_http::client::UnitResponseDeserializer),
        |t| quote!(#t),
    );
    let trait_ = match service.asyncness {
        Asyncness::Sync => quote!(DeserializeResponse),
        Asyncness::Async => quote!(AsyncDeserializeResponse),
    };
    let await_ = match service.asyncness {
        Asyncness::Sync => quote!(),
        Asyncness::Async => quote!(.await),
    };

    quote! {
        <#accept as conjure_http::client::#trait_<_, _>>::deserialize(#response) #await_
    }
}

struct Service {
    vis: Visibility,
    name: Ident,
    generics: Generics,
    request_writer_param: Option<Ident>,
    response_body_param: Option<Ident>,
    asyncness: Asyncness,
    endpoints: Vec<Endpoint>,
}

impl Service {
    fn new(trait_: &mut ItemTrait) -> Result<Self, Error> {
        let mut errors = Errors::new();
        let mut endpoints = vec![];
        for item in &trait_.items {
            match Endpoint::new(item) {
                Ok(endpoint) => endpoints.push(endpoint),
                Err(e) => errors.push(e),
            }
        }

        let asyncness = match Asyncness::resolve(trait_) {
            Ok(asyncness) => Some(asyncness),
            Err(e) => {
                errors.push(e);
                None
            }
        };

        let mut request_writer_param = None;
        let mut response_body_param = None;
        for param in &trait_.generics.params {
            let GenericParam::Type(param) = param else {
                errors.push(Error::new_spanned(param, "unexpected parameter"));
                continue;
            };

            for attr in &param.attrs {
                if attr.path().is_ident("request_writer") {
                    request_writer_param = Some(param.ident.clone());
                } else if attr.path().is_ident("response_body") {
                    response_body_param = Some(param.ident.clone());
                }
            }
        }

        strip_trait(trait_);
        errors.build()?;

        Ok(Service {
            vis: trait_.vis.clone(),
            name: trait_.ident.clone(),
            generics: trait_.generics.clone(),
            request_writer_param,
            response_body_param,
            asyncness: asyncness.unwrap(),
            endpoints,
        })
    }
}

// Rust doesn't support helper attributes in attribute macros so we need to manually strip them out
fn strip_trait(trait_: &mut ItemTrait) {
    for param in &mut trait_.generics.params {
        strip_param(param);
    }

    for item in &mut trait_.items {
        if let TraitItem::Fn(fn_) = item {
            strip_fn(fn_);
        }
    }
}

fn strip_param(param: &mut GenericParam) {
    let GenericParam::Type(param) = param else {
        return;
    };

    param.attrs.retain(|attr| {
        !["request_writer", "response_body"]
            .iter()
            .any(|v| attr.path().is_ident(v))
    });
}

fn strip_fn(fn_: &mut TraitItemFn) {
    for arg in &mut fn_.sig.inputs {
        strip_arg(arg);
    }
}

fn strip_arg(arg: &mut FnArg) {
    let FnArg::Typed(arg) = arg else { return };

    arg.attrs.retain(|attr| {
        !["path", "query", "header", "body", "auth"]
            .iter()
            .any(|v| attr.path().is_ident(v))
    });
}

struct Endpoint {
    ident: Ident,
    args: Vec<ArgType>,
    ret_ty: Type,
    params: EndpointParams,
    path: Vec<PathComponent>,
}

impl Endpoint {
    fn new(item: &TraitItem) -> Result<Self, Error> {
        let TraitItem::Fn(item) = item else {
            return Err(Error::new_spanned(
                item,
                "Conjure traits may only contain methods",
            ));
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

        let ret_ty = match &item.sig.output {
            ReturnType::Default => {
                errors.push(Error::new_spanned(
                    &item.sig.output,
                    "expected a return type",
                ));
                None
            }
            ReturnType::Type(_, ty) => Some((**ty).clone()),
        };

        let path = match params.as_ref().map(|p| path::parse(&p.path)).transpose() {
            Ok(path) => path,
            Err(e) => {
                errors.push(e);
                None
            }
        };

        if let Err(e) = validate_args(&args, params.as_ref().map(|p| &p.path), path.as_deref()) {
            errors.push(e);
        }

        errors.build()?;

        Ok(Endpoint {
            ident: item.sig.ident.clone(),
            args,
            ret_ty: ret_ty.unwrap(),
            params: params.unwrap(),
            path: path.unwrap(),
        })
    }
}

fn validate_args(
    args: &[ArgType],
    path: Option<&LitStr>,
    path_components: Option<&[PathComponent]>,
) -> Result<(), Error> {
    let mut errors = Errors::new();

    let mut body_args = args.iter().filter(|a| matches!(a, ArgType::Body(_)));
    if body_args.next().is_some() {
        for arg in body_args {
            errors.push(Error::new(arg.span(), "duplicate `#[body]` arg"));
        }
    }

    let mut auth_args = args.iter().filter(|a| matches!(a, ArgType::Auth(_)));
    if auth_args.next().is_some() {
        for arg in auth_args {
            errors.push(Error::new(arg.span(), "duplicate `#[auth]` arg"));
        }
    }

    for arg in args {
        let ArgType::Header(arg) = arg else { continue };
        if let Err(e) = arg.attr.name.value().parse::<HeaderName>() {
            errors.push(Error::new(arg.span, e));
        }
    }

    if let (Some(path), Some(path_components)) = (path, path_components) {
        let mut path_params = args
            .iter()
            .filter_map(|a| match a {
                ArgType::Path(arg) => Some((arg.ident.to_string(), arg.span)),
                _ => None,
            })
            .collect::<HashMap<_, _>>();

        for component in path_components {
            let PathComponent::Parameter(param) = component else {
                continue;
            };

            if path_params.remove(param).is_none() {
                errors.push(Error::new_spanned(
                    path,
                    format!("invalid path parameter `{param}`"),
                ));
            }
        }

        for span in path_params.values() {
            errors.push(Error::new(*span, "unused path parameter"));
        }
    }

    errors.build()
}

#[derive(StructMeta)]
struct EndpointParams {
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

impl ArgType {
    fn ident(&self) -> &Ident {
        match self {
            ArgType::Path(arg) => &arg.ident,
            ArgType::Query(arg) => &arg.ident,
            ArgType::Header(arg) => &arg.ident,
            ArgType::Auth(arg) => &arg.ident,
            ArgType::Body(arg) => &arg.ident,
        }
    }

    fn ty(&self) -> &Type {
        match self {
            ArgType::Path(arg) => &arg.ty,
            ArgType::Query(arg) => &arg.ty,
            ArgType::Header(arg) => &arg.ty,
            ArgType::Auth(arg) => &arg.ty,
            ArgType::Body(arg) => &arg.ty,
        }
    }

    fn span(&self) -> Span {
        match self {
            ArgType::Path(arg) => arg.span,
            ArgType::Query(arg) => arg.span,
            ArgType::Header(arg) => arg.span,
            ArgType::Auth(arg) => arg.span,
            ArgType::Body(arg) => arg.span,
        }
    }
}

struct Arg<T> {
    ident: Ident,
    ty: Type,
    span: Span,
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
    fn new(arg: &PatType) -> syn::Result<Self> {
        // FIXME we should probably just rename the arguments in our impl?
        let ident = match &*arg.pat {
            Pat::Ident(pat_ident) => &pat_ident.ident,
            _ => return Err(Error::new_spanned(&arg.pat, "expected an ident pattern")),
        };

        let mut type_ = None;

        // FIXME detect multiple attrs
        for attr in &arg.attrs {
            if attr.path().is_ident("path") {
                let attr = match attr.meta {
                    Meta::Path(_) => PathAttr { encoder: None },
                    _ => attr.parse_args()?,
                };
                type_ = Some(ArgType::Path(Arg {
                    ident: ident.clone(),
                    ty: (*arg.ty).clone(),
                    span: arg.span(),
                    attr,
                }));
            } else if attr.path().is_ident("query") {
                type_ = Some(ArgType::Query(Arg {
                    ident: ident.clone(),
                    ty: (*arg.ty).clone(),
                    span: arg.span(),
                    attr: attr.parse_args()?,
                }));
            } else if attr.path().is_ident("header") {
                type_ = Some(ArgType::Header(Arg {
                    ident: ident.clone(),
                    ty: (*arg.ty).clone(),
                    span: arg.span(),
                    attr: attr.parse_args()?,
                }));
            } else if attr.path().is_ident("auth") {
                let attr = match attr.meta {
                    Meta::Path(_) => AuthAttr { cookie_name: None },
                    _ => attr.parse_args()?,
                };
                type_ = Some(ArgType::Auth(Arg {
                    ident: ident.clone(),
                    ty: (*arg.ty).clone(),
                    span: arg.span(),
                    attr,
                }));
            } else if attr.path().is_ident("body") {
                let attr = match attr.meta {
                    Meta::Path(_) => BodyAttr { serializer: None },
                    _ => attr.parse_args()?,
                };
                type_ = Some(ArgType::Body(Arg {
                    ident: ident.clone(),
                    ty: (*arg.ty).clone(),
                    span: arg.span(),
                    attr,
                }));
            }
        }

        type_.ok_or_else(|| Error::new_spanned(arg, "missing parameter type attribute"))
    }
}
