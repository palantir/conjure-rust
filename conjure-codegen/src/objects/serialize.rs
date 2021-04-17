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

pub fn generate(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());
    let result = ctx.result_ident(def.type_name());

    let name_str = name.to_string();

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
                    if #check_name {
                        s.skip_field(#key)?;
                    } else {
                        s.serialize_field(#key, &self.#field_name)?;
                    }
                };
                serialize_calls.push(serialize_call);
            }
            None => {
                size += 1;

                let serialize_call = quote! {
                    s.serialize_field(#key, &self.#field_name)?;
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

    let struct_mut = if def.fields().is_empty() {
        quote!()
    } else {
        quote!(mut)
    };

    quote! {
        impl ser::Serialize for #name {
            fn serialize<S>(&self, s: S) -> #result<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                let #size_mut size = #size;
                #(#empty_checks)*

                let #struct_mut s = s.serialize_struct(#name_str, size)?;
                #(#serialize_calls)*
                s.end()
            }
        }
    }
}
