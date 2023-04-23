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
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    parenthesized, parse_macro_input, Error, FnArg, ItemTrait, LitStr, Pat, ReturnType, Token,
    TraitItem, TraitItemMethod, Type,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(method);
    custom_keyword!(path);
    custom_keyword!(accept);
}

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
            TraitItem::Method(meth) => Some(meth),
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

fn generate_client_method(trait_name: &Ident, method: &mut TraitItemMethod) -> TokenStream {
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
    let add_path = add_path(&request, &endpoint);
    let add_accept = add_accept(&request, &endpoint, &method.sig.output);
    let add_endpoint = add_endpoint(trait_name, method, &endpoint, &request);
    let handle_response = handle_response(&endpoint, &response);

    quote! {
        fn #name(#args) #ret {
            #create_request
            *#request.method_mut() = conjure_http::private::Method::#http_method;
            #add_path
            #add_accept
            #add_endpoint
            let #response = conjure_http::client::Client::send(&self.client, #request)?;
            #handle_response
        }
    }
}

fn create_request(request: &TokenStream, args: &[ArgType]) -> TokenStream {
    // FIXME handle multiple body params
    let body_arg = args.iter().find_map(|a| match a {
        ArgType::Body(arg) => Some(arg),
        _ => None,
    });

    match body_arg {
        Some(arg) => {
            let converter = arg.converter.as_ref().map_or_else(
                || quote!(conjure_http::client::JsonToRequestBody),
                |t| quote!(#t),
            );
            let pat = &arg.pat;

            quote! {
                let __content_type = <
                    #converter as conjure_http::client::ToRequestBody<_, C::BodyWriter>
                >::content_type(&#pat);
                let __content_length = <
                    #converter as conjure_http::client::ToRequestBody<_, C::BodyWriter>
                >::content_length(&#pat);
                let __body = <
                    #converter as conjure_http::client::ToRequestBody<_, C::BodyWriter>
                >::to_body(#pat);

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
        None => quote! {
            let mut #request = conjure_http::private::Request::new(
                conjure_http::client::RequestBody::Empty,
            );
        },
    }
}

fn add_path(request: &TokenStream, endpoint: &EndpointConfig) -> TokenStream {
    let path = &endpoint.path;

    quote! {
        let mut __path = conjure_http::private::UriBuilder::new();
        __path.push_literal(#path);
        *#request.uri_mut() = __path.build();
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
        #request.headers_mut().insert(
            conjure_http::private::header::ACCEPT,
            <#accept as conjure_http::client::FromResponse<
                <#ret as conjure_http::private::ExtractOk>::Ok,
                C::ResponseBody,
            >>::accept(),
        );
    }
}

fn add_endpoint(
    trait_name: &Ident,
    method: &TraitItemMethod,
    endpoint: &EndpointConfig,
    request: &TokenStream,
) -> TokenStream {
    let service = format!("{trait_name}");
    let name = format!("{}", method.sig.ident);
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
            <#accept as conjure_http::client::FromResponse<_, _>>::from_response(#response)
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
    fn new(endpoint: &TraitItemMethod) -> syn::Result<Self> {
        let mut method = None;
        let mut path = None;
        let mut accept = None;

        for attr in &endpoint.attrs {
            if !attr.path.is_ident("endpoint") {
                continue;
            }

            let overrides = attr.parse_args_with(|p: ParseStream<'_>| {
                p.parse_terminated::<_, Token![,]>(EndpointArg::parse)
            })?;

            for override_ in overrides {
                match override_ {
                    EndpointArg::Method(v) => method = Some(v),
                    EndpointArg::Path(v) => path = Some(v),
                    EndpointArg::Accept(v) => accept = Some(v),
                }
            }
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

enum EndpointArg {
    Method(Ident),
    Path(LitStr),
    Accept(Type),
}

impl Parse for EndpointArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::method) {
            input.parse::<kw::method>()?;
            input.parse::<Token![=]>()?;
            let method = input.parse()?;
            Ok(EndpointArg::Method(method))
        } else if lookahead.peek(kw::path) {
            input.parse::<kw::path>()?;
            input.parse::<Token![=]>()?;
            let path = input.parse()?;
            Ok(EndpointArg::Path(path))
        } else if lookahead.peek(kw::accept) {
            input.parse::<kw::accept>()?;
            input.parse::<Token![=]>()?;
            let ty = input.parse()?;
            Ok(EndpointArg::Accept(ty))
        } else {
            Err(lookahead.error())
        }
    }
}

enum ArgType {
    Body(BodyArg),
}

struct BodyArg {
    // FIXME we should extract the raw ident
    pat: Pat,
    converter: Option<Type>,
}

impl ArgType {
    fn new(arg: &mut FnArg) -> syn::Result<Option<Self>> {
        let FnArg::Typed(pat_type) = arg else { return Ok(None); };

        let mut arg_type = None;

        // FIXME detect multiple attrs
        for attr in &pat_type.attrs {
            if attr.path.is_ident("body") {
                let attr = syn::parse2::<BodyAttr>(attr.tokens.clone())?;
                arg_type = Some(ArgType::Body(BodyArg {
                    pat: (*pat_type.pat).clone(),
                    converter: attr.converter,
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
        !["path_param", "query_param", "header", "body"]
            .iter()
            .any(|v| attr.path.is_ident(v))
    });
}

struct BodyAttr {
    converter: Option<Type>,
}

impl Parse for BodyAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut arg = BodyAttr { converter: None };

        if input.is_empty() {
            return Ok(arg);
        }

        let content;
        parenthesized!(content in input);

        arg.converter = Some(content.parse()?);

        Ok(arg)
    }
}
