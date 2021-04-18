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
use crate::context::Context;
use crate::objects;
use crate::objects::builder::{self, SetterOp};
use crate::types::{FieldDefinition, ObjectDefinition};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;

pub fn generate(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());
    let docs = format!("A builder for the `{}` type.", name);
    let builder_type = objects::builder_type(ctx, def);
    let option = ctx.option_ident(def.type_name());
    let some = ctx.some_ident(def.type_name());
    let from = ctx.from_ident(def.type_name());

    let fields = &objects::fields(ctx, def);
    let boxed_types = def.fields().iter().map(|f| {
        let type_ = ctx.boxed_rust_type(def.type_name(), f.type_());
        if ctx.is_required(f.type_()) {
            quote!(#option<#type_>)
        } else {
            type_
        }
    });

    let field_names = builder::field_names(ctx, def);
    let setters = def
        .fields()
        .iter()
        .map(|f| generate_setter(ctx, def, f, &field_names));

    let build_method = if fields.iter().any(|f| f == "build") {
        quote!(build_)
    } else {
        quote!(build)
    };

    let build_rhs = def.fields().iter().map(|f| {
        let var = ctx.field_name(f.field_name());
        if ctx.is_required(f.type_()) {
            let msg = format!("field {} was not set", var);
            quote!(self.#var.clone().expect(#msg))
        } else {
            quote!(self.#var.clone())
        }
    });

    let from_rhs = def.fields().iter().map(|f| {
        let var = ctx.field_name(f.field_name());
        if ctx.is_required(f.type_()) {
            quote!(#some(_v.#var))
        } else {
            quote!(_v.#var)
        }
    });

    quote! {
        #[doc = #docs]
        #[derive(Debug, Clone, Default)]
        pub struct #builder_type {
            #(
                #fields: #boxed_types,
            )*
        }

        impl #builder_type {
            #(#setters)*

            /// Constructs a new instance of the type.
            ///
            /// # Panics
            ///
            /// Panics if a required field was not set.
            #[inline]
            pub fn #build_method(&self) -> #name {
                #name {
                    #(
                        #fields: #build_rhs,
                    )*
                }
            }
        }

        impl #from<#name> for #builder_type {
            #[inline]
            fn from(_v: #name) -> #builder_type {
                #builder_type {
                    #(
                        #fields: #from_rhs,
                    )*
                }
            }
        }
    }
}

fn generate_setter(
    ctx: &Context,
    def: &ObjectDefinition,
    field: &FieldDefinition,
    field_names: &HashSet<String>,
) -> TokenStream {
    let some = ctx.some_ident(def.type_name());

    let docs = ctx.docs(field.docs());

    let required = if ctx.is_required(field.type_()) {
        quote! {
            ///
            /// Required.
        }
    } else {
        quote!()
    };

    let deprecated = ctx.deprecated(field.deprecated());

    let name = ctx.field_name(field.field_name());

    let setters = builder::field_setters(ctx, def, field, field_names)
        .into_iter()
        .map(|setter| {
            let update = match setter.op {
                SetterOp::Assign { mut rhs } => {
                    if ctx.is_required(field.type_()) {
                        rhs = quote!(#some(#rhs));
                    }

                    quote!(self.#name = #rhs;)
                }
                SetterOp::Call { call } => {
                    quote!(self.#name.#call;)
                }
            };

            let method = setter.name;
            let params = setter.params;
            let args = setter.args;
            let where_ = setter.where_;

            quote! {
                #docs
                #required
                #deprecated
                #[inline]
                pub fn #method #params(&mut self, #args) -> &mut Self #where_ {
                    #update
                    self
                }
            }
        });

    quote!(#(#setters)*)
}
