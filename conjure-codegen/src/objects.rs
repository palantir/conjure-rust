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

use crate::context::{Context, SetterBounds};
use crate::types::{FieldDefinition, ObjectDefinition};

pub fn generate(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let object = generate_object(ctx, def);
    let builder = generate_builder(ctx, def);
    let serialize = generate_serialize(ctx, def);
    let deserialize = generate_deserialize(ctx, def);
    let field = generate_field(ctx, def);

    quote! {
        use conjure_types::serde::{ser, de};
        use conjure_types::serde::ser::SerializeMap as SerializeMap_;
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

    let field_names = fields.iter().map(|f| f.to_string()).collect();
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

    let param = if ctx.type_name(def.type_name().name()) == "T" {
        quote!(U)
    } else {
        quote!(T)
    };

    match ctx.setter_bounds(def.type_name(), field.type_(), quote!(#name)) {
        SetterBounds::Simple {
            argument_type,
            assign_rhs,
        } => {
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
            assign_rhs,
        } => {
            quote! {
                #docs
                #required
                pub fn #name<#param>(&mut self, #name: #param) -> &mut Self
                where
                    #param: #argument_bound
                {
                    self.#name = #assign_rhs;
                    self
                }
            }
        }
        SetterBounds::Collection { argument_bound } => {
            let mut extend_name = format!("extend_{}", name);
            if field_names.contains(&extend_name) {
                extend_name.push('_');
            }
            let extend_name = extend_name.parse::<TokenStream>().unwrap();

            quote! {
                #docs
                pub fn #name<#param>(&mut self, #name: #param) -> &mut Self
                where
                    #param: #argument_bound
                {
                    self.#name = #name.into_iter().collect();
                    self
                }

                #docs
                pub fn #extend_name<#param>(&mut self, #name: #param) -> &mut Self
                where
                    #param: #argument_bound
                {
                    self.#name.extend(#name);
                    self
                }
            }
        }
    }
}

fn generate_serialize(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());
    let result = ctx.result_ident(def.type_name());
    let some = ctx.some_ident(def.type_name());

    let mut size = 0;
    let mut empty_checks = vec![];
    let mut serialize_calls = vec![];
    for field in def.fields() {
        let field_name = ctx.field_name(field.field_name());
        let key = &field.field_name().0;

        match ctx.is_empty_method(field.type_()) {
            Some(is_empty) => {
                let check_name = format!("skip_{}", field_name)
                    .parse::<TokenStream>()
                    .unwrap();

                let check = quote! {
                    let #check_name = self.#field_name.#is_empty();
                    if !#check_name {
                        size += 1;
                    }
                };
                empty_checks.push(check);

                let serialize_call = quote! {
                    if !#check_name {
                        map.serialize_entry(&#key, &self.#field_name)?;
                    }
                };
                serialize_calls.push(serialize_call);
            }
            None => {
                size += 1;

                let serialize_call = quote! {
                    map.serialize_entry(&#key, &self.#field_name)?;
                };
                serialize_calls.push(serialize_call);
            }
        }
    }

    let size_mut = if size == def.fields().len() {
        quote!()
    } else {
        quote!(mut)
    };

    let map_mut = if def.fields().is_empty() {
        quote!()
    } else {
        quote!(mut)
    };

    quote! {
        impl ser::Serialize for #name {
            fn serialize<S_>(&self, s: S_) -> #result<S_::Ok, S_::Error>
            where
                S_: ser::Serializer,
            {
                let #size_mut size = #size;
                #(#empty_checks)*

                let #map_mut map = s.serialize_map(#some(size))?;
                #(#serialize_calls)*
                map.end()
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
            fn deserialize<D_>(d: D_) -> #result<#name, D_::Error>
            where
                D_: de::Deserializer<'de>
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

            fn visit_map<A_>(self, mut map_: A_) -> #result<#name, A_::Error>
            where
                A_: de::MapAccess<'de>
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
            fn deserialize<D_>(d: D_) -> #result<Field_, D_::Error>
            where
                D_: de::Deserializer<'de>
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

            fn visit_str<E_>(self, value: &str) -> #result<Field_, E_>
            where
                E_: de::Error
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
