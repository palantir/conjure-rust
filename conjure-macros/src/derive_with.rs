// Copyright 2025 Palantir Technologies, Inc.
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

use proc_macro2::{Span, TokenStream};
use quote::quote;
use structmeta::StructMeta;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Field, Fields, Ident, Variant};

#[derive(StructMeta)]
struct ItemAttr {
    #[struct_meta(name = "PartialEq")]
    partial_eq: bool,
    #[struct_meta(name = "Eq")]
    eq: bool,
    #[struct_meta(name = "PartialOrd")]
    partial_ord: bool,
    #[struct_meta(name = "Ord")]
    ord: bool,
    #[struct_meta(name = "Hash")]
    hash: bool,
}

#[derive(StructMeta)]
struct FieldAttr {
    with: Expr,
}

pub fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let delegate = Ident::new(&format!("__{}Delegate", input.ident), input.ident.span());
    let delegate_item = generate_delegate(&input, &delegate);
    let from_impl = generate_from_impl(&input, &delegate);

    let mut derives = vec![];
    let mut impls = vec![];

    for attr in &input.attrs {
        if attr.path().is_ident("derive_with") {
            let attr = match attr.parse_args::<ItemAttr>() {
                Ok(attr) => attr,
                Err(e) => return e.into_compile_error().into(),
            };

            if attr.partial_eq {
                derives.push(quote!(PartialEq));
                impls.push(generate_partial_eq(&input, &delegate));
            }

            if attr.eq {
                derives.push(quote!(Eq));
                impls.push(generate_eq(&input));
            }

            if attr.partial_ord {
                derives.push(quote!(PartialOrd));
                impls.push(generate_partial_ord(&input));
            }

            if attr.ord {
                derives.push(quote!(Ord));
                impls.push(generate_ord(&input, &delegate));
            }

            if attr.hash {
                derives.push(quote!(Hash));
                impls.push(generate_hash(&input, &delegate));
            }
        }
    }

    quote! {
        const _: () = {
            #[derive(#(#derives),*)]
            #delegate_item

            #from_impl

            #(#impls)*
        };
    }
    .into()
}

fn generate_delegate(input: &DeriveInput, ident: &Ident) -> TokenStream {
    let lt = quote!('__a);

    match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let idents = fields_named.named.iter().map(|f| &f.ident);
                let tys = fields_named.named.iter().map(|f| generate_field_ty(f, &lt));

                quote! {
                    struct #ident<#lt> {
                        #(#idents: #tys),*
                    }
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let tys = fields_unnamed
                    .unnamed
                    .iter()
                    .map(|f| generate_field_ty(f, &lt));

                quote! {
                    struct #ident<#lt>(#(#tys),*);
                }
            }
            Fields::Unit => quote! {
                struct #ident;
            },
        },
        Data::Enum(data_enum) => {
            let variants = data_enum.variants.iter().map(|v| generate_variant(v, &lt));

            quote! {
                enum #ident<#lt> {
                    #(#variants),*
                }
            }
        }
        Data::Union(_) => quote!(compile_error!("DeriveWith does not support unions")),
    }
}

fn extract_with(field: &Field) -> Option<TokenStream> {
    for attr in &field.attrs {
        if attr.path().is_ident("derive_with") {
            let attr = match attr.parse_args::<FieldAttr>() {
                Ok(attr) => attr,
                Err(e) => return Some(e.into_compile_error()),
            };

            let with = attr.with;
            return Some(quote!(#with));
        }
    }

    None
}

fn generate_field_ty(field: &Field, lt: &TokenStream) -> TokenStream {
    let ty = &field.ty;
    let mut ty = quote!(&#lt #ty);
    if let Some(with) = extract_with(field) {
        ty = quote!(#with<#ty>);
    }

    ty
}

fn generate_variant(variant: &Variant, lt: &TokenStream) -> TokenStream {
    let ident = &variant.ident;
    let discriminant = &variant.discriminant.as_ref().map(|(eq, e)| quote!(#eq #e));

    match &variant.fields {
        Fields::Named(fields_named) => {
            let field_idents = fields_named.named.iter().map(|f| &f.ident);
            let field_tys = fields_named.named.iter().map(|f| generate_field_ty(f, lt));
            quote!(#ident { #(#field_idents: #field_tys),* } #discriminant)
        }
        Fields::Unnamed(fields_unnamed) => {
            let field_tys = fields_unnamed
                .unnamed
                .iter()
                .map(|f| generate_field_ty(f, lt));
            quote!(#ident(#(#field_tys),*) #discriminant)
        }
        Fields::Unit => quote!(#ident #discriminant),
    }
}

fn generate_from_impl(input: &DeriveInput, delegate: &Ident) -> TokenStream {
    let lt = quote!('__a);
    let ident = &input.ident;

    let constructor = match &input.data {
        Data::Struct(data_struct) => {
            let fields = data_struct
                .fields
                .members()
                .zip(data_struct.fields.iter().map(extract_with))
                .map(|(member, with)| {
                    let mut expr = quote!(&v.#member);
                    if let Some(with) = with {
                        expr = quote!(#with(#expr));
                    }

                    quote!(#member: #expr)
                });

            quote! {
                #delegate {
                    #(#fields),*
                }
            }
        }
        Data::Enum(data_enum) => {
            let variants = data_enum
                .variants
                .iter()
                .map(|variant| generate_variant_arm(&input.ident, variant));

            quote! {
                match v {
                    #(#variants),*
                }
            }
        }
        Data::Union(_) => quote!(compile_error!("DeriveWith does not support unions")),
    };

    quote! {
        impl<#lt> std::convert::From<&#lt #ident> for #delegate<#lt> {
            #[inline]
            #[allow(deprecated)]
            fn from(v: &#lt #ident) -> Self {
                #constructor
            }
        }
    }
}

fn generate_variant_arm(ident: &Ident, variant: &Variant) -> TokenStream {
    let variant_ident = &variant.ident;
    match &variant.fields {
        Fields::Named(fields_named) => {
            let names = fields_named.named.iter().map(|f| &f.ident);
            let constructor = fields_named.named.iter().map(|f| {
                let name = &f.ident;
                let mut expr = quote!(#name);
                if let Some(with) = extract_with(f) {
                    expr = quote!(#with(#expr));
                }
                expr
            });

            quote! {
                #ident::#variant_ident { #(#names),* } => Self::#variant_ident { #(#constructor),* }
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            let names = fields_unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| Ident::new(&format!("__f{i}"), Span::call_site()));
            let constructor = fields_unnamed.unnamed.iter().enumerate().map(|(i, f)| {
                let name = Ident::new(&format!("__f{i}"), Span::call_site());
                let mut expr = quote!(#name);
                if let Some(with) = extract_with(f) {
                    expr = quote!(#with(#expr));
                }
                expr
            });

            quote! {
                #ident::#variant_ident(#(#names),*) => Self::#variant_ident(#(#constructor),*)
            }
        }
        Fields::Unit => quote!(#ident::#variant_ident => Self::#variant_ident),
    }
}

fn generate_partial_eq(input: &DeriveInput, delegate: &Ident) -> TokenStream {
    let ident = &input.ident;

    quote! {
        impl std::cmp::PartialEq for #ident {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                std::cmp::PartialEq::eq(
                    &<#delegate as std::convert::From<_>>::from(self),
                    &<#delegate as std::convert::From<_>>::from(other),
                )
            }
        }
    }
}

fn generate_eq(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    quote! {
        impl std::cmp::Eq for #ident {}
    }
}

fn generate_partial_ord(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    quote! {
        impl std::cmp::PartialOrd for #ident {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
                std::option::Option::Some(std::cmp::Ord::cmp(self, other))
            }
        }
    }
}

fn generate_ord(input: &DeriveInput, delegate: &Ident) -> TokenStream {
    let ident = &input.ident;

    quote! {
        impl std::cmp::Ord for #ident {
            #[inline]
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                std::cmp::Ord::cmp(
                    &<#delegate as std::convert::From<_>>::from(self),
                    &<#delegate as std::convert::From<_>>::from(other),
                )
            }
        }
    }
}

fn generate_hash(input: &DeriveInput, delegate: &Ident) -> TokenStream {
    let ident = &input.ident;

    quote! {
        impl std::hash::Hash for #ident {
            #[inline]
            fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
                std::hash::Hash::hash(&<#delegate as std::convert::From<_>>::from(self), hasher)
            }
        }
    }
}
