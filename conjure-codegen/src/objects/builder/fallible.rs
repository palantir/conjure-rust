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
use crate::context::{CollectionSetterBounds, CollectionType, Context, SetterBounds};
use crate::objects;
use crate::types::{FieldDefinition, ObjectDefinition};
use proc_macro2::{Ident, TokenStream};
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

    let field_names = fields.iter().map(Ident::to_string).collect();
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

    match ctx.setter_bounds(def.type_name(), field.type_(), quote!(#name)) {
        SetterBounds::Simple {
            argument_type,
            mut assign_rhs,
        } => {
            if ctx.is_required(field.type_()) {
                assign_rhs = quote!(#some(#assign_rhs));
            }
            quote! {
                #docs
                #required
                #deprecated
                #[inline]
                pub fn #name(&mut self, #name: #argument_type) -> &mut Self {
                    self.#name = #assign_rhs;
                    self
                }
            }
        }
        SetterBounds::Generic {
            argument_bound,
            mut assign_rhs,
        } => {
            if ctx.is_required(field.type_()) {
                assign_rhs = quote!(#some(#assign_rhs));
            }
            quote! {
                #docs
                #required
                #deprecated
                pub fn #name<T>(&mut self, #name: T) -> &mut Self
                where
                    T: #argument_bound
                {
                    self.#name = #assign_rhs;
                    self
                }
            }
        }
        SetterBounds::Collection {
            argument_bound,
            type_,
        } => {
            let mut extend_name = format!("extend_{}", name);
            if field_names.contains(&extend_name) {
                extend_name.push('_');
            }
            let extend_name = extend_name.parse::<TokenStream>().unwrap();

            let single_method = match type_ {
                CollectionType::List { value } => {
                    let mut single_name = format!("push_{}", name);
                    if field_names.contains(&single_name) {
                        single_name.push('_');
                    }
                    let single_name = single_name.parse::<TokenStream>().unwrap();
                    let (params, type_, where_, assign_rhs) = match value {
                        CollectionSetterBounds::Simple {
                            argument_type,
                            assign_rhs,
                        } => (quote!(), argument_type, quote!(), assign_rhs),
                        CollectionSetterBounds::Generic {
                            argument_bound,
                            assign_rhs,
                        } => (
                            quote!(<T>),
                            quote!(T),
                            quote!(where T: #argument_bound),
                            assign_rhs,
                        ),
                    };
                    quote! {
                        #docs
                        #deprecated
                        pub fn #single_name #params(&mut self, value: #type_) -> &mut Self
                        #where_
                        {
                            self.#name.push(#assign_rhs);
                            self
                        }
                    }
                }
                CollectionType::Set { value } => {
                    let mut single_name = format!("insert_{}", name);
                    if field_names.contains(&single_name) {
                        single_name.push('_');
                    }
                    let single_name = single_name.parse::<TokenStream>().unwrap();
                    let (params, type_, where_, assign_rhs) = match value {
                        CollectionSetterBounds::Simple {
                            argument_type,
                            assign_rhs,
                        } => (quote!(), argument_type, quote!(), assign_rhs),
                        CollectionSetterBounds::Generic {
                            argument_bound,
                            assign_rhs,
                        } => (
                            quote!(<T>),
                            quote!(T),
                            quote!(where T: #argument_bound),
                            assign_rhs,
                        ),
                    };
                    quote! {
                        #docs
                        #deprecated
                        pub fn #single_name #params(&mut self, value: #type_) -> &mut Self
                        #where_
                        {
                            self.#name.insert(#assign_rhs);
                            self
                        }
                    }
                }
                CollectionType::Map { key, value } => {
                    let mut single_name = format!("insert_{}", name);
                    if field_names.contains(&single_name) {
                        single_name.push('_');
                    }
                    let single_name = single_name.parse::<TokenStream>().unwrap();

                    let mut params = vec![];
                    let mut wheres = vec![];

                    let (key_type, key_assign_rhs) = match key {
                        CollectionSetterBounds::Simple {
                            argument_type,
                            assign_rhs,
                        } => (argument_type, assign_rhs),
                        CollectionSetterBounds::Generic {
                            argument_bound,
                            assign_rhs,
                        } => {
                            wheres.push(quote!(K: #argument_bound));
                            params.push(quote!(K));
                            (quote!(K), assign_rhs)
                        }
                    };

                    let (value_type, value_assign_rhs) = match value {
                        CollectionSetterBounds::Simple {
                            argument_type,
                            assign_rhs,
                        } => (argument_type, assign_rhs),
                        CollectionSetterBounds::Generic {
                            argument_bound,
                            assign_rhs,
                        } => {
                            wheres.push(quote!(V: #argument_bound));
                            params.push(quote!(V));
                            (quote!(V), assign_rhs)
                        }
                    };

                    let params = if params.is_empty() {
                        quote!()
                    } else {
                        quote!(<#(#params),*>)
                    };
                    let wheres = if wheres.is_empty() {
                        quote!()
                    } else {
                        quote!(where #(#wheres),*)
                    };

                    quote! {
                        #docs
                        #deprecated
                        pub fn #single_name #params(&mut self, key: #key_type, value: #value_type) -> &mut Self
                        #wheres
                        {
                            self.#name.insert(#key_assign_rhs, #value_assign_rhs);
                            self
                        }
                    }
                }
            };

            quote! {
                #docs
                #deprecated
                pub fn #name<T>(&mut self, #name: T) -> &mut Self
                where
                    T: #argument_bound
                {
                    self.#name = #name.into_iter().collect();
                    self
                }

                #docs
                #deprecated
                pub fn #extend_name<T>(&mut self, #name: T) -> &mut Self
                where
                    T: #argument_bound
                {
                    self.#name.extend(#name);
                    self
                }

                #single_method
            }
        }
    }
}
