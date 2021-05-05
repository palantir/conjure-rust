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
use crate::types::ObjectDefinition;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter;

pub fn generate(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
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

    let field = generate_field(ctx, def);

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

        #field
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
