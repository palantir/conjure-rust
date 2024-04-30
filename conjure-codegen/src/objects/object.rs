// Copyright 2021 Palantir Technologies, Inc.
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
use crate::context::{BuilderConfig, BuilderItemConfig, Context};
use crate::objects;
use crate::types::{FieldDefinition, ObjectDefinition};
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let docs = ctx.docs(def.docs());
    let name = ctx.type_name(def.type_name().name());

    let mut type_attrs = vec![];
    let mut derives = vec!["Debug", "Clone"];

    if def.fields().iter().any(|v| ctx.has_double(v.type_())) {
        derives.push("conjure_object::private::Educe");
        type_attrs.push(quote!(#[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]));
    } else {
        derives.push("PartialEq");
        derives.push("Eq");
        derives.push("PartialOrd");
        derives.push("Ord");
        derives.push("Hash");
    }

    if def.fields().iter().all(|v| ctx.is_copy(v.type_())) {
        derives.push("Copy");
    }

    let derives = derives.iter().map(|s| s.parse::<TokenStream>().unwrap());
    // The derive attr has to be before the educe attr, so insert rather than push
    type_attrs.insert(0, quote!(#[derive(#(#derives),*)]));

    let field_attrs = def.fields().iter().map(|s| {
        let builder_attr = field_builder_attr(ctx, def, s);
        let educe_attr = if ctx.is_double(s.type_()) {
            quote! {
                #[educe(
                    PartialEq(method(conjure_object::private::DoubleOps::eq)),
                    Ord(method(conjure_object::private::DoubleOps::cmp)),
                    Hash(method(conjure_object::private::DoubleOps::hash)),
                )]
            }
        } else {
            quote!()
        };

        quote! {
            #builder_attr
            #educe_attr
        }
    });
    let fields = &objects::fields(ctx, def);
    let boxed_types = &def
        .fields()
        .iter()
        .map(|s| ctx.boxed_rust_type(def.type_name(), s.type_()))
        .collect::<Vec<_>>();

    let constructor = generate_constructor(ctx, def);

    let accessors = def.fields().iter().map(|s| {
        let docs = ctx.docs(s.docs());
        let deprecated = ctx.deprecated(s.deprecated());
        let name = ctx.field_name(s.field_name());
        let ret_type = ctx.borrowed_rust_type(def.type_name(), s.type_());
        let borrow = ctx.borrow_rust_type(quote!(self.#name), s.type_());

        quote!(
            #docs
            #deprecated
            #[inline]
            pub fn #name(&self) -> #ret_type {
                #borrow
            }
        )
    });

    quote! {
        #docs
        #(#type_attrs)*
        #[conjure_object::private::staged_builder::staged_builder]
        #[builder(
            crate = conjure_object::private::staged_builder,
            update,
            inline,
        )]
        pub struct #name {
            #(
                #field_attrs
                #fields: #boxed_types,
            )*
        }

        impl #name {
            #constructor

            #(#accessors)*
        }
    }
}

fn generate_constructor(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let required_args = def
        .fields()
        .iter()
        .filter(|f| ctx.is_required(f.type_()))
        .collect::<Vec<_>>();

    if required_args.len() > 3 {
        return quote!();
    }

    let new = if def.fields().iter().any(|f| **f.field_name() == "new") {
        quote!(new_)
    } else {
        quote!(new)
    };

    let arguments = required_args.iter().map(|f| {
        let name = ctx.field_name(f.field_name());
        let ty = match ctx.builder_config(def.type_name(), f.type_()) {
            BuilderConfig::Normal => ctx.rust_type(def.type_name(), f.type_()),
            BuilderConfig::Into => {
                let into = ctx.into_ident(def.type_name());
                let ty = ctx.rust_type(def.type_name(), f.type_());
                quote!(impl #into<#ty>)
            }
            BuilderConfig::Custom { type_, .. } => type_,
            BuilderConfig::List { .. } | BuilderConfig::Set { .. } | BuilderConfig::Map { .. } => {
                unreachable!()
            }
        };
        quote!(#name: #ty)
    });

    let setters = required_args.iter().map(|f| {
        let field = ctx.field_name(f.field_name());
        quote!(.#field(#field))
    });

    quote! {
        /// Constructs a new instance of the type.
        #[inline]
        pub fn #new(#(#arguments,)*) -> Self {
            Self::builder()
                #(#setters)*
                .build()
        }
    }
}

fn field_builder_attr(
    ctx: &Context,
    def: &ObjectDefinition,
    field: &FieldDefinition,
) -> TokenStream {
    let mut inner = match ctx.builder_config(def.type_name(), field.type_()) {
        BuilderConfig::Normal => quote!(),
        BuilderConfig::Into => quote!(into),
        BuilderConfig::Custom { type_, convert } => {
            quote!(custom(type = #type_, convert = #convert))
        }
        BuilderConfig::List { item } => {
            let item = builder_item_attr(item);
            quote!(list(item(#item)))
        }
        BuilderConfig::Set { item } => {
            let item = builder_item_attr(item);
            quote!(set(item(#item)))
        }
        BuilderConfig::Map { key, value } => {
            let key = builder_item_attr(key);
            let value = builder_item_attr(value);
            quote!(map(key(#key), value(#value)))
        }
    };

    // FIXME this is unnecessary for lists, sets, maps
    if !ctx.is_required(field.type_()) {
        inner = quote!(default, #inner);
    }

    if inner.is_empty() {
        quote!()
    } else {
        quote!(#[builder(#inner)])
    }
}

fn builder_item_attr(config: BuilderItemConfig) -> TokenStream {
    match config {
        BuilderItemConfig::Normal { type_ } => quote!(type = #type_),
        BuilderItemConfig::Into { type_ } => quote!(type = #type_, into),
        BuilderItemConfig::Custom { type_, convert } => {
            quote!(custom(type = #type_, convert = #convert))
        }
    }
}
