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
use proc_macro2::TokenStream;
use quote::quote;

use crate::context::Context;
use crate::types::AliasDefinition;

pub fn generate(ctx: &Context, def: &AliasDefinition) -> TokenStream {
    let conjure = ctx.conjure_path();
    let name = ctx.type_name(def.type_name().name());
    let alias = ctx.rust_type(def.type_name(), def.alias());
    let result = ctx.result_ident(def.type_name());
    let docs = ctx.docs(def.docs());

    let mut derives = vec!["Debug", "Clone", "PartialEq", "PartialOrd"];
    if ctx.is_copy(def.alias()) {
        derives.push("Copy");
    }
    if !ctx.has_double(def.alias()) {
        derives.push("Eq");
        derives.push("Ord");
        derives.push("Hash");
    }
    if ctx.is_default(def.alias()) {
        derives.push("Default");
    }
    let derives = derives.iter().map(|s| s.parse::<TokenStream>().unwrap());

    quote! {
        use #conjure::serde::{ser, de};

        #docs
        #[derive(#(#derives),*)]
        pub struct #name(pub #alias);

        impl std::ops::Deref for #name {
            type Target = #alias;

            #[inline]
            fn deref(&self) -> &#alias {
                &self.0
            }
        }

        impl std::ops::DerefMut for #name {
            #[inline]
            fn deref_mut(&mut self) -> &mut #alias {
                &mut self.0
            }
        }

        impl ser::Serialize for #name {
            fn serialize<S_>(&self, s: S_) -> #result<S_::Ok, S_::Error>
            where
                S_: ser::Serializer
            {
                self.0.serialize(s)
            }
        }

        impl<'de> de::Deserialize<'de> for #name {
            fn deserialize<D_>(d: D_) -> #result<#name, D_::Error>
            where
                D_: de::Deserializer<'de>
            {
                de::Deserialize::deserialize(d).map(#name)
            }
        }
    }
}
