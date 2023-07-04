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
use crate::{is_async, parse_path, Errors};
use conjure_codegen_shared::client::{
    self, ArgType, AuthArg, BodyArg, Endpoint, HeaderArg, PathArg, QueryArg, Service,
};
use conjure_codegen_shared::path::PathSegment;
use http::HeaderName;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::collections::HashMap;
use structmeta::StructMeta;
use syn::spanned::Spanned as _;
use syn::{
    parse_macro_input, Error, FnArg, GenericParam, ItemTrait, LitStr, Meta, Pat, PatType,
    ReturnType, TraitItem, TraitItemFn, Type,
};

pub fn generate(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as ItemTrait);
    let service = match parse_service(&mut item) {
        Ok(service) => service,
        Err(e) => return e.into_compile_error().into(),
    };
    let client = client::generate(&service);

    quote! {
        #item

        #client
    }
    .into()
}

fn parse_service(trait_: &mut ItemTrait) -> Result<Service, Error> {
    let name = Ident::new(&format!("{}Client", trait_.ident), trait_.ident.span());
    let version = syn::parse2(quote!(conjure_http::private::Option::Some(
        conjure_http::private::env!("CARGO_PKG_VERSION")
    )))
    .unwrap();

    let mut errors = Errors::new();
    let mut endpoints = vec![];
    for item in &trait_.items {
        match parse_endpoint(item) {
            Ok(endpoint) => endpoints.push(endpoint),
            Err(e) => errors.push(e),
        }
    }

    let is_async = match is_async(trait_) {
        Ok(is_async) => Some(is_async),
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

    Ok(Service::builder()
        .vis(trait_.vis.clone())
        .name(name)
        .version(version)
        .generics(trait_.generics.clone())
        .r#async(is_async.unwrap())
        .request_writer_param(request_writer_param)
        .response_body_param(response_body_param)
        .r#trait(trait_.ident.clone())
        .endpoints(endpoints)
        .build())
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
    let GenericParam::Type(param) = param else { return };

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

fn parse_endpoint(item: &TraitItem) -> Result<Endpoint, Error> {
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

        match parse_arg(arg) {
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

    let path = match params.as_ref().map(|p| parse_path(&p.path)).transpose() {
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

    let params = params.unwrap();

    let accept = params.accept.unwrap_or_else(|| {
        syn::parse2(quote!(conjure_http::client::UnitResponseDeserializer)).unwrap()
    });

    Ok(Endpoint::builder()
        .method(params.method)
        .path(params.path)
        .accept(accept)
        .name(item.sig.ident.clone())
        .return_type(ret_ty.unwrap())
        .path_segments(path.unwrap())
        .args(args.into_iter().map(|a| a.value))
        .build())
}

fn validate_args(
    args: &[Spanned<ArgType>],
    path: Option<&LitStr>,
    path_components: Option<&[PathSegment]>,
) -> Result<(), Error> {
    let mut errors = Errors::new();

    let mut body_args = args.iter().filter(|a| matches!(a.value, ArgType::Body(_)));
    if body_args.next().is_some() {
        for arg in body_args {
            errors.push(Error::new(arg.span, "duplicate `#[body]` arg"));
        }
    }

    let mut auth_args = args.iter().filter(|a| matches!(a.value, ArgType::Auth(_)));
    if auth_args.next().is_some() {
        for arg in auth_args {
            errors.push(Error::new(arg.span, "duplicate `#[auth]` arg"));
        }
    }

    for arg in args {
        let ArgType::Header(header_arg) = &arg.value else { continue };
        if let Err(e) = header_arg.header_name().value().parse::<HeaderName>() {
            errors.push(Error::new(arg.span, e));
        }
    }

    if let (Some(path), Some(path_components)) = (path, path_components) {
        let mut path_params = args
            .iter()
            .filter_map(|a| match &a.value {
                ArgType::Path(arg) => Some((arg.name().to_string(), a.span)),
                _ => None,
            })
            .collect::<HashMap<_, _>>();

        for component in path_components {
            let PathSegment::Parameter { name, ..} = component else { continue };

            if path_params.remove(name).is_none() {
                errors.push(Error::new_spanned(
                    path,
                    format!("invalid path parameter `{name}`"),
                ));
            }
        }

        for span in path_params.values() {
            errors.push(Error::new(*span, "unused path parameter"));
        }
    }

    errors.build()
}

fn parse_arg(arg: &PatType) -> Result<Spanned<ArgType>, Error> {
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
            type_ = Some(ArgType::Path(
                PathArg::builder()
                    .name(ident.clone())
                    .r#type((*arg.ty).clone())
                    .encoder(attr.encoder.unwrap_or_else(|| {
                        syn::parse2(quote!(conjure_http::client::DisplayEncoder)).unwrap()
                    }))
                    .build(),
            ));
        } else if attr.path().is_ident("query") {
            let attr = attr.parse_args::<ParamAttr>()?;
            type_ = Some(ArgType::Query(
                QueryArg::builder()
                    .name(ident.clone())
                    .r#type((*arg.ty).clone())
                    .key(attr.name)
                    .encoder(attr.encoder.unwrap_or_else(|| {
                        syn::parse2(quote!(conjure_http::client::DisplayEncoder)).unwrap()
                    }))
                    .build(),
            ));
        } else if attr.path().is_ident("header") {
            let attr = attr.parse_args::<ParamAttr>()?;
            type_ = Some(ArgType::Header(
                HeaderArg::builder()
                    .name(ident.clone())
                    .r#type((*arg.ty).clone())
                    .header_name(attr.name)
                    .encoder(attr.encoder.unwrap_or_else(|| {
                        syn::parse2(quote!(conjure_http::client::DisplayEncoder)).unwrap()
                    }))
                    .build(),
            ));
        } else if attr.path().is_ident("auth") {
            let attr = match attr.meta {
                Meta::Path(_) => AuthAttr { cookie_name: None },
                _ => attr.parse_args()?,
            };
            type_ = Some(ArgType::Auth(
                AuthArg::builder()
                    .name(ident.clone())
                    .r#type((*arg.ty).clone())
                    .cookie_name(attr.cookie_name)
                    .build(),
            ))
        } else if attr.path().is_ident("body") {
            let attr = match attr.meta {
                Meta::Path(_) => BodyAttr { serializer: None },
                _ => attr.parse_args()?,
            };
            type_ = Some(ArgType::Body(
                BodyArg::builder()
                    .name(ident.clone())
                    .r#type((*arg.ty).clone())
                    .serializer(attr.serializer.unwrap_or_else(|| {
                        syn::parse2(quote!(conjure_http::client::ConjureRequestSerializer)).unwrap()
                    }))
                    .build(),
            ));
        }
    }

    let type_ = type_.ok_or_else(|| Error::new_spanned(arg, "missing parameter type attribute"))?;

    Ok(Spanned {
        value: type_,
        span: arg.span(),
    })
}

#[derive(StructMeta)]
struct EndpointParams {
    method: Ident,
    path: LitStr,
    accept: Option<Type>,
}

struct Spanned<T> {
    value: T,
    span: Span,
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
