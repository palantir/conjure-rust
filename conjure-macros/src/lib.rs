use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Error, FnArg, ItemTrait, Pat, TraitItem, TraitItemMethod, Type};

#[proc_macro_attribute]
pub fn service(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as ItemTrait);

    let client = generate_client(&item);

    strip_trait_attrs(&mut item);
    quote! {
        #item

        #client
    }
    .into()
}

// Rust doesn't support "helper" attributes in attribute macros, so we need to strip out our helper
// attributes on arguments.
fn strip_trait_attrs(trait_: &mut ItemTrait) {
    for item in &mut trait_.items {
        if let TraitItem::Method(method) = item {
            strip_method_attrs(method);
        }
    }
}

fn strip_method_attrs(method: &mut TraitItemMethod) {
    for arg in &mut method.sig.inputs {
        strip_arg_attrs(arg);
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

/// A no-op attribute macro required due to technical limitations of Rust's macro system.
#[proc_macro_attribute]
pub fn endpoint(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

fn generate_client(trait_: &ItemTrait) -> TokenStream {
    let vis = &trait_.vis;
    let trait_name = &trait_.ident;
    let type_name = Ident::new(&format!("{}Client", trait_name), trait_name.span());

    let methods = trait_
        .items
        .iter()
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

fn generate_client_method(method: &TraitItemMethod) -> TokenStream {
    let name = &method.sig.ident;
    let args = &method.sig.inputs;
    let ret = &method.sig.output;

    let request_args = match args
        .iter()
        .flat_map(|a| ResolvedFnArg::new(a).transpose())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(request_args) => request_args,
        Err(e) => return e.into_compile_error(),
    };

    quote! {
        fn #name(#args) #ret {
            panic!()
        }
    }
}

enum ArgType {
    Body(BodyArg),
}

struct ResolvedFnArg<'a> {
    // FIXME we should extract the raw ident
    pat: &'a Pat,
    arg_type: ArgType,
}

impl<'a> ResolvedFnArg<'a> {
    fn new(arg: &'a FnArg) -> syn::Result<Option<Self>> {
        let FnArg::Typed(pat_type) = arg else { return Ok(None); };

        let mut arg_type = None;

        // FIXME detect multiple attrs
        for attr in &pat_type.attrs {
            if attr.path.is_ident("body") {
                let arg = attr.parse_args()?;
                arg_type = Some(ArgType::Body(arg));
            }
        }

        let Some(arg_type) = arg_type else {
            return Err(Error::new_spanned(arg, "missing argument type annotation"));
        };

        Ok(Some(ResolvedFnArg {
            pat: &pat_type.pat,
            arg_type,
        }))
    }
}

struct BodyArg {
    converter: Option<Type>,
}

impl Parse for BodyArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut arg = BodyArg { converter: None };

        if !input.is_empty() {
            arg.converter = Some(input.parse()?);
        }

        Ok(arg)
    }
}
