use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{
    parenthesized, parse_macro_input, Error, FnArg, ItemTrait, Pat, TraitItem, TraitItemMethod,
    Type,
};

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
        .map(generate_client_method);

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

fn generate_client_method(method: &mut TraitItemMethod) -> TokenStream {
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

    let create_request = create_request(&request, &request_args);

    quote! {
        fn #name(#args) #ret {
            #create_request
            conjure_http::client::Client::send(&self.client, #request)?;
            Ok(())
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
                let mut __body = <#converter as conjure_http::client::ToRequestBody<_, C::BodyWriter>>::to_request_body(#pat);
                let __content_type = conjure_http::client::TypedRequestBody::<C::BodyWriter>::content_type(&__body);
                let __content_length = conjure_http::client::TypedRequestBody::<C::BodyWriter>::content_length(&__body);

                let mut #request = conjure_http::private::Request::new(
                    conjure_http::client::TypedRequestBody::body(&mut __body),
                );
                #request.headers_mut().insert(
                    conjure_http::private::header::CONTENT_TYPE,
                    __content_type,
                );
                if let Some(__content_length) = __content_length {
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
