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
use crate::path::PathComponent;
use http::HeaderName;
use percent_encoding::AsciiSet;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashMap;
use syn::{
    parse_macro_input, Attribute, Error, FnArg, ItemTrait, LitStr, Meta, Pat, ReturnType,
    TraitItem, TraitItemFn, Type,
};

mod path;

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

#[proc_macro_attribute]
pub fn service(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as ItemTrait);

    let client = generate_client(&mut item);

    quote! {
        #item

        #client
    }
    .into()
}

/// A no-op attribute macro required due to technical limitations of Rust's macro system.
#[proc_macro_attribute]
pub fn endpoint(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

fn generate_client(trait_: &mut ItemTrait) -> TokenStream {
    let vis = &trait_.vis;
    let trait_name = &trait_.ident;
    let type_name = Ident::new(&format!("{}Client", trait_name), trait_name.span());

    let methods = trait_
        .items
        .iter_mut()
        .filter_map(|item| match item {
            TraitItem::Fn(meth) => Some(meth),
            _ => None,
        })
        .map(|m| generate_client_method(trait_name, m));

    quote! {
        #vis struct #type_name<C> {
            client: C,
        }

        impl<C> conjure_http::client::Service<C> for #type_name<C> {
            fn new(client: C) -> Self {
                #type_name { client }
            }
        }

        impl<C> #trait_name for #type_name<C>
        where
            C: conjure_http::client::Client,
        {
            #(#methods)*
        }
    }
}

fn generate_client_method(trait_name: &Ident, method: &mut TraitItemFn) -> TokenStream {
    let endpoint = match EndpointConfig::new(method) {
        Ok(c) => c,
        Err(e) => return e.into_compile_error(),
    };

    let request_args = match method
        .sig
        .inputs
        .iter_mut()
        .flat_map(|a| ArgType::new(a).transpose())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(request_args) => request_args,
        Err(e) => return e.into_compile_error(),
    };

    let name = &method.sig.ident;
    let args = &method.sig.inputs;
    let ret = &method.sig.output;

    let request = quote!(__request);
    let response = quote!(__response);
    let http_method = &endpoint.method;

    let create_request = create_request(&request, &request_args);
    let add_path = add_path(&request, &request_args, &endpoint);
    let add_accept = add_accept(&request, &endpoint, &method.sig.output);
    let add_auth = add_auth(&request, &request_args);
    let add_headers = add_headers(&request, &request_args);
    let add_endpoint = add_endpoint(trait_name, method, &endpoint, &request);
    let handle_response = handle_response(&endpoint, &response);

    quote! {
        fn #name(#args) #ret {
            #create_request
            *#request.method_mut() = conjure_http::private::Method::#http_method;
            #add_path
            #add_accept
            #add_auth
            #add_headers
            #add_endpoint
            let #response = conjure_http::client::Client::send(&self.client, #request)?;
            #handle_response
        }
    }
}

fn create_request(request: &TokenStream, args: &[ArgType]) -> TokenStream {
    let mut it = args.iter().filter_map(|a| match a {
        ArgType::Body(arg) => Some(arg),
        _ => None,
    });
    let Some(arg) = it.next() else {
        return quote! {
            let mut #request = conjure_http::private::Request::new(
                conjure_http::client::RequestBody::Empty,
            );
        };
    };

    if let Some(arg) = it.next() {
        return Error::new_spanned(&arg.ident, "only one #[body] argument allowed")
            .into_compile_error();
    }

    let serializer = arg.serializer.as_ref().map_or_else(
        || quote!(conjure_http::client::JsonRequestSerializer),
        |t| quote!(#t),
    );
    let ident = &arg.ident;

    quote! {
        let __content_type = <
            #serializer as conjure_http::client::SerializeRequest<_, C::BodyWriter>
        >::content_type(&#ident);
        let __content_length = <
            #serializer as conjure_http::client::SerializeRequest<_, C::BodyWriter>
        >::content_length(&#ident);
        let __body = <
            #serializer as conjure_http::client::SerializeRequest<_, C::BodyWriter>
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

fn add_path(
    request: &TokenStream,
    request_args: &[ArgType],
    endpoint: &EndpointConfig,
) -> TokenStream {
    let builder = quote!(__path);

    let path_writes = add_path_components(&endpoint.path, &builder, request_args);

    let query_params = request_args
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

fn add_path_components(
    path_lit: &LitStr,
    builder: &TokenStream,
    request_args: &[ArgType],
) -> TokenStream {
    let path = match path::parse(path_lit) {
        Ok(path) => path,
        Err(e) => return e.into_compile_error(),
    };

    let path_params = request_args
        .iter()
        .filter_map(|a| match a {
            ArgType::Path(param) => Some((param.ident.to_string(), param)),
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    let mut path_writes = vec![];
    let mut literal_buf = String::new();
    for component in path {
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

                let Some(param) = path_params.get(&param) else {
                    path_writes.push(
                        Error::new_spanned(
                            path_lit,
                            format_args!("invalid path parameter `{param}`"),
                        ).into_compile_error(),
                    );
                    continue;
                };

                let ident = &param.ident;
                let encoder = param.encoder.as_ref().map_or_else(
                    || quote!(conjure_http::client::DisplayParamEncoder),
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

    quote! {
        #(#path_writes)*
    }
}

fn add_query_arg(builder: &TokenStream, arg: &ParamArg) -> TokenStream {
    let ident = &arg.ident;
    let name = percent_encoding::percent_encode(arg.name.value().as_bytes(), COMPONENT).to_string();
    let encoder = arg.encoder.as_ref().map_or_else(
        || quote!(conjure_http::client::DisplayParamEncoder),
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
    request: &TokenStream,
    endpoint: &EndpointConfig,
    ret_ty: &ReturnType,
) -> TokenStream {
    let Some(accept) = &endpoint.accept else {
        return quote!();
    };

    let ret = match ret_ty {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, ty) => quote!(#ty),
    };

    quote! {
        let __accept = <#accept as conjure_http::client::DeserializeResponse<
            <#ret as conjure_http::private::ExtractOk>::Ok,
            C::ResponseBody,
        >>::accept();
        if let Some(__accept) = __accept {
            #request.headers_mut().insert(conjure_http::private::header::ACCEPT, __accept);
        }
    }
}

fn add_auth(request: &TokenStream, args: &[ArgType]) -> TokenStream {
    let mut it = args.iter().filter_map(|a| match a {
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

    match &auth_param.cookie_name {
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

fn add_headers(request: &TokenStream, args: &[ArgType]) -> TokenStream {
    let add_headers = args
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

fn add_header(request: &TokenStream, arg: &ParamArg) -> TokenStream {
    if let Err(e) = arg.name.value().parse::<HeaderName>() {
        return Error::new_spanned(&arg.name, e).into_compile_error();
    }

    let ident = &arg.ident;
    let name = &arg.name;
    let encoder = arg.encoder.as_ref().map_or_else(
        || quote!(conjure_http::client::DisplayHeaderEncoder),
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

fn add_endpoint(
    trait_name: &Ident,
    method: &TraitItemFn,
    endpoint: &EndpointConfig,
    request: &TokenStream,
) -> TokenStream {
    let service = trait_name.to_string();
    let name = method.sig.ident.to_string();
    let path = &endpoint.path;

    quote! {
        #request.extensions_mut().insert(conjure_http::client::Endpoint::new(
            #service,
            std::option::Option::Some(std::env!("CARGO_PKG_VERSION")),
            #name,
            #path,
        ));
    }
}

fn handle_response(endpoint: &EndpointConfig, response: &TokenStream) -> TokenStream {
    match &endpoint.accept {
        Some(accept) => quote! {
            <#accept as conjure_http::client::DeserializeResponse<_, _>>::deserialize(#response)
        },
        None => quote!(conjure_http::private::Result::Ok(())),
    }
}

struct EndpointConfig {
    method: Ident,
    path: LitStr,
    accept: Option<Type>,
}

impl EndpointConfig {
    fn new(endpoint: &TraitItemFn) -> syn::Result<Self> {
        let mut method = None;
        let mut path = None;
        let mut accept = None;

        for attr in &endpoint.attrs {
            if !attr.path().is_ident("endpoint") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("method") {
                    let value = meta.value()?;
                    method = Some(value.parse()?);
                } else if meta.path.is_ident("path") {
                    let value = meta.value()?;
                    path = Some(value.parse()?);
                } else if meta.path.is_ident("accept") {
                    let value = meta.value()?;
                    accept = Some(value.parse()?);
                } else {
                    return Err(meta.error("unsupported attribute"));
                }

                Ok(())
            })?;
        }

        Ok(EndpointConfig {
            method: method
                .ok_or_else(|| Error::new_spanned(endpoint, "#[endpoint(method=...) missing"))?,
            path: path
                .ok_or_else(|| Error::new_spanned(endpoint, "#[endpoint(path=...)] missing"))?,
            accept,
        })
    }
}

enum ArgType {
    Path(PathArg),
    Query(ParamArg),
    Header(ParamArg),
    Auth(AuthArg),
    Body(BodyArg),
}

struct PathArg {
    ident: Ident,
    encoder: Option<Type>,
}

struct ParamArg {
    ident: Ident,
    name: LitStr,
    encoder: Option<Type>,
}

impl ParamArg {
    fn new(ident: &Ident, attr: &Attribute) -> Result<Self, Error> {
        let mut name = None;
        let mut encoder = None;

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                let value = meta.value()?;
                name = Some(value.parse()?);
            } else if meta.path.is_ident("encoder") {
                let value = meta.value()?;
                encoder = Some(value.parse()?);
            } else {
                return Err(meta.error("unsupported attribute"));
            }

            Ok(())
        })?;

        Ok(ParamArg {
            ident: ident.clone(),
            name: name.ok_or_else(|| Error::new_spanned(attr, "`name` entry missing"))?,
            encoder,
        })
    }
}

struct AuthArg {
    ident: Ident,
    cookie_name: Option<LitStr>,
}

struct BodyArg {
    ident: Ident,
    serializer: Option<Type>,
}

impl ArgType {
    fn new(arg: &mut FnArg) -> syn::Result<Option<Self>> {
        let FnArg::Typed(pat_type) = arg else { return Ok(None); };

        let ident = match &*pat_type.pat {
            Pat::Ident(pat_ident) => &pat_ident.ident,
            _ => {
                return Err(Error::new_spanned(
                    &pat_type.pat,
                    "expected an ident pattern",
                ))
            }
        };

        let mut arg_type = None;

        // FIXME detect multiple attrs
        for attr in &pat_type.attrs {
            if attr.path().is_ident("path") {
                let mut encoder = None;
                if !(matches!(attr.meta, Meta::Path(_))) {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("encoder") {
                            let value = meta.value()?;
                            encoder = Some(value.parse()?);
                        } else {
                            return Err(meta.error("unsupported attribute"));
                        }

                        Ok(())
                    })?;
                }

                arg_type = Some(ArgType::Path(PathArg {
                    ident: ident.clone(),
                    encoder,
                }))
            } else if attr.path().is_ident("query") {
                arg_type = Some(ArgType::Query(ParamArg::new(ident, attr)?));
            } else if attr.path().is_ident("header") {
                arg_type = Some(ArgType::Header(ParamArg::new(ident, attr)?));
            } else if attr.path().is_ident("auth") {
                let mut cookie_name = None;
                if !(matches!(attr.meta, Meta::Path(_))) {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("cookie_name") {
                            let value = meta.value()?;
                            cookie_name = Some(value.parse()?);
                            Ok(())
                        } else {
                            Err(meta.error("unsupported attribute"))
                        }
                    })?;
                }

                arg_type = Some(ArgType::Auth(AuthArg {
                    ident: ident.clone(),
                    cookie_name,
                }))
            } else if attr.path().is_ident("body") {
                let mut serializer = None;
                if !matches!(attr.meta, Meta::Path(_)) {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("serializer") {
                            let value = meta.value()?;
                            serializer = Some(value.parse()?);
                            Ok(())
                        } else {
                            Err(meta.error("unsupported attribute"))
                        }
                    })?;
                }

                arg_type = Some(ArgType::Body(BodyArg {
                    ident: ident.clone(),
                    serializer,
                }));
            }
        }

        let Some(arg_type) = arg_type else {
            return Err(Error::new_spanned(arg, "missing argument type annotation"));
        };

        // Rust doesn't support "helper" attributes in attribute macros, so we need to strip out our
        // helper attributes on arguments.
        strip_arg_attrs(arg);
        Ok(Some(arg_type))
    }
}

fn strip_arg_attrs(arg: &mut FnArg) {
    let FnArg::Typed(arg) = arg else { return };

    arg.attrs.retain(|attr| {
        !["path", "query", "header", "body", "auth"]
            .iter()
            .any(|v| attr.path().is_ident(v))
    });
}
