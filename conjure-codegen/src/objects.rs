// Copyright 2018 Palantir Technologies, Inc.
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
use std::collections::HashSet;
use std::iter;

use crate::context::{CollectionSetterBounds, CollectionType, Context, SetterBounds};
use crate::types::{FieldDefinition, ObjectDefinition};

pub fn generate(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let object = generate_object(ctx, def);
    let builder = generate_builder(ctx, def);
    let serialize = generate_serialize(ctx, def);
    let deserialize = generate_deserialize(ctx, def);
    let field = generate_field(ctx, def);

    quote! {
        use conjure_object::serde::{ser, de};
        use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
        use std::fmt;

        #object
        #builder
        #serialize
        #deserialize
        #field
    }
}

fn generate_object(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let docs = ctx.docs(def.docs());
    let name = ctx.type_name(def.type_name().name());
    let default = ctx.default_ident(def.type_name());

    let mut derives = vec!["Debug", "Clone", "PartialEq", "PartialOrd"];
    if !def.fields().iter().any(|v| ctx.has_double(v.type_())) {
        derives.push("Eq");
        derives.push("Ord");
        derives.push("Hash");
    }
    if def.fields().iter().all(|v| ctx.is_copy(&v.type_())) {
        derives.push("Copy");
    }
    let derives = derives.iter().map(|s| s.parse::<TokenStream>().unwrap());

    let fields = &fields(ctx, def);
    let boxed_types = &def
        .fields()
        .iter()
        .map(|s| ctx.boxed_rust_type(def.type_name(), s.type_()))
        .collect::<Vec<_>>();

    let constructor = if fields.len() < 4 {
        generate_constructor(ctx, def)
    } else {
        quote!()
    };

    let accessors = def.fields().iter().map(|s| {
        let docs = ctx.docs(s.docs());
        let name = ctx.field_name(s.field_name());
        let ret_type = ctx.borrowed_rust_type(def.type_name(), s.type_());
        let borrow = ctx.borrow_rust_type(quote!(self.#name), s.type_());

        quote!(
            #docs
            #[inline]
            pub fn #name(&self) -> #ret_type {
                #borrow
            }
        )
    });

    let builder_method = if fields.iter().any(|f| f == "builder") {
        quote!(builder_)
    } else {
        quote!(builder)
    };

    let builder_type = builder_type(ctx, def);

    quote! {
        #docs
        #[derive(#(#derives),*)]
        pub struct #name {
            #(
                #fields: #boxed_types,
            )*
        }

        impl #name {
            #constructor

            /// Returns a new builder.
            #[inline]
            pub fn #builder_method() -> #builder_type {
                #default::default()
            }

            #(#accessors)*
        }
    }
}

fn fields(ctx: &Context, def: &ObjectDefinition) -> Vec<Ident> {
    def.fields()
        .iter()
        .map(|f| ctx.field_name(f.field_name()))
        .collect()
}

fn builder_type(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    if ctx.type_name(def.type_name().name()) == "Builder" {
        quote!(Builder_)
    } else {
        quote!(Builder)
    }
}

fn generate_constructor(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let some = ctx.some_ident(def.type_name());
    let name = ctx.type_name(def.type_name().name());
    let mut param_it = vec![quote!(T), quote!(U), quote!(V)].into_iter();

    let mut parameters = vec![];
    let mut arguments = vec![];
    let mut where_clauses = vec![];
    let mut assignments = vec![];

    for field in def.fields() {
        let (field_type, optional) = match ctx.option_inner_type(field.type_()) {
            Some(field_type) => (field_type, true),
            None => (field.type_(), false),
        };
        let arg_name = ctx.field_name(field.field_name());
        match ctx.setter_bounds(def.type_name(), field_type, quote!(#arg_name)) {
            SetterBounds::Simple {
                argument_type,
                mut assign_rhs,
            } => {
                arguments.push(quote!(#arg_name: #argument_type));
                if optional {
                    assign_rhs = quote!(#some(#assign_rhs));
                }
                assignments.push(quote!(#arg_name: #assign_rhs));
            }
            SetterBounds::Generic {
                argument_bound,
                mut assign_rhs,
            } => {
                let param = param_it.next().unwrap();
                parameters.push(param.clone());
                arguments.push(quote!(#arg_name: #param));
                where_clauses.push(quote!(#param: #argument_bound));
                if optional {
                    assign_rhs = quote!(#some(#assign_rhs));
                }
                assignments.push(quote!(#arg_name: #assign_rhs));
            }
            SetterBounds::Collection { argument_bound, .. } => {
                let param = param_it.next().unwrap();
                parameters.push(param.clone());
                arguments.push(quote!(#arg_name: #param));
                where_clauses.push(quote!(#param: #argument_bound));
                let mut assign_rhs = quote!(#arg_name.into_iter().collect());
                if optional {
                    assign_rhs = quote!(#some(#assign_rhs));
                }
                assignments.push(quote!(#arg_name: #assign_rhs));
            }
        }
    }

    let parameters = if parameters.is_empty() {
        quote!()
    } else {
        quote!(<#(#parameters,)*>)
    };

    let where_clauses = if where_clauses.is_empty() {
        quote!()
    } else {
        quote!(where #(#where_clauses,)*)
    };

    let new_ = if def.fields().iter().any(|f| **f.field_name() == "new") {
        quote!(new_)
    } else {
        quote!(new)
    };

    quote! {
        /// Constructs a new instance of the type.
        #[inline]
        pub fn #new_ #parameters(#(#arguments,)*) -> #name
        #where_clauses
        {
            #name {
                #(#assignments),*
            }
        }
    }
}

fn generate_builder(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());
    let builder_type = builder_type(ctx, def);
    let option = ctx.option_ident(def.type_name());
    let some = ctx.some_ident(def.type_name());
    let from = ctx.from_ident(def.type_name());

    let fields = &fields(ctx, def);
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
                pub fn #name<T>(&mut self, #name: T) -> &mut Self
                where
                    T: #argument_bound
                {
                    self.#name = #name.into_iter().collect();
                    self
                }

                #docs
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

fn generate_serialize(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());
    let result = ctx.result_ident(def.type_name());

    let name_str = name.to_string();
    let size = def.fields().len();

    let struct_mut = if def.fields().is_empty() {
        quote!()
    } else {
        quote!(mut)
    };

    let serialize_calls = def.fields().iter().map(|field| {
        let field_name = ctx.field_name(field.field_name());
        let key = &field.field_name().0;

        let mut serialize_field = quote! {
            s.serialize_field(#key, &self.#field_name)?;
        };

        if let Some(is_empty) = ctx.is_empty_method(field.type_()) {
            serialize_field = quote! {
                if self.#field_name.#is_empty() {
                    s.skip_field(#key)?;
                } else {
                    #serialize_field
                }
            };
        }

        serialize_field
    });

    quote! {
        impl ser::Serialize for #name {
            fn serialize<S>(&self, s: S) -> #result<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                let #struct_mut s = s.serialize_struct(#name_str, #size)?;
                #(#serialize_calls)*
                s.end()
            }
        }
    }
}

fn generate_deserialize(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());
    let result = ctx.result_ident(def.type_name());
    let ok = ctx.ok_ident(def.type_name());
    let err = ctx.err_ident(def.type_name());
    let some = ctx.some_ident(def.type_name());
    let none = ctx.none_ident(def.type_name());

    let name_str = name.to_string();

    let fields = &def
        .fields()
        .iter()
        .map(|f| ctx.field_name(f.field_name()))
        .collect::<Vec<_>>();
    let fields2 = fields;

    let field_names = def.fields().iter().map(|f| &f.field_name().0);

    let field_variants = def.fields().iter().map(|f| ctx.type_name(f.field_name()));

    let repeat_none = iter::repeat(&none);
    let repeat_some = iter::repeat(&some);

    let repeat_none2 = iter::repeat(&none);
    let repeat_some2 = iter::repeat(&some);

    let missing_fields = def.fields().iter().map(|f| {
        if ctx.is_required(f.type_()) {
            let field_name = &f.field_name().0;
            quote!(return #err(de::Error::missing_field(#field_name)))
        } else {
            let default = ctx.default_ident(def.type_name());
            quote!(#default::default())
        }
    });

    quote! {
        impl<'de> de::Deserialize<'de> for #name {
            fn deserialize<D>(d: D) -> #result<#name, D::Error>
            where
                D: de::Deserializer<'de>
            {
                d.deserialize_struct(#name_str, &[#(#field_names, )*], Visitor_)
            }
        }

        struct Visitor_;

        impl<'de> de::Visitor<'de> for Visitor_ {
            type Value = #name;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("map")
            }

            fn visit_map<A>(self, mut map_: A) -> #result<#name, A::Error>
            where
                A: de::MapAccess<'de>
            {
                #(
                    let mut #fields = #repeat_none;
                )*

                while let #some(field_) = map_.next_key()? {
                    match field_ {
                        #(
                            Field_::#field_variants => #fields = #repeat_some(map_.next_value()?),
                        )*
                        Field_::Unknown_ => {
                            map_.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                #(
                    let #fields = match #fields2 {
                        #repeat_some2(v) => v,
                        #repeat_none2 => #missing_fields,
                    };
                )*

                #ok(#name {
                    #(#fields,)*
                })
            }
        }
    }
}

fn generate_field(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let result = ctx.result_ident(def.type_name());
    let ok = ctx.ok_ident(def.type_name());

    let field_variants = &def
        .fields()
        .iter()
        .map(|f| ctx.type_name(f.field_name()))
        .collect::<Vec<_>>();

    let keys = &def
        .fields()
        .iter()
        .map(|f| &f.field_name().0)
        .collect::<Vec<_>>();

    quote! {
        enum Field_ {
            #(#field_variants,)*
            Unknown_
        }

        impl<'de> de::Deserialize<'de> for Field_ {
            fn deserialize<D>(d: D) -> #result<Field_, D::Error>
            where
                D: de::Deserializer<'de>
            {
                d.deserialize_str(FieldVisitor_)
            }
        }

        struct FieldVisitor_;

        impl<'de> de::Visitor<'de> for FieldVisitor_ {
            type Value = Field_;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> #result<Field_, E>
            where
                E: de::Error
            {
                let v = match value {
                    #(
                        #keys => Field_::#field_variants,
                    )*
                    _ => Field_::Unknown_,
                };

                #ok(v)
            }
        }
    }
}
