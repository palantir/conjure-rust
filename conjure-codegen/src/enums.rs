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
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::context::Context;
use crate::types::EnumDefinition;

pub fn generate(ctx: &Context, def: &EnumDefinition) -> TokenStream {
    let conjure_types = ctx.conjure_path();
    let root_docs = ctx.docs(def.docs());
    let name = ctx.type_name(def.type_name().name());
    let box_ = ctx.box_ident(def.type_name());
    let result = ctx.result_ident(def.type_name());
    let ok = ctx.ok_ident(def.type_name());
    let err = ctx.err_ident(def.type_name());

    let variants = def.values().iter().map(|v| ctx.type_name(v.value()));

    let other_variant = if ctx.exhaustive() {
        quote!()
    } else {
        quote!(Unknown(#box_<str>))
    };

    let constants = def.values().iter().map(|v| {
        let docs = ctx.docs(v.docs());
        let constant = Ident::new(v.value(), Span::call_site());
        let variant = ctx.type_name(v.value());
        quote! {
            #docs
            pub const #constant: #name = #name(Inner_::#variant);
        }
    });

    let as_str_arms = def.values().iter().map(|v| {
        let value = v.value();
        let variant = ctx.type_name(v.value());
        quote! {
            Inner_::#variant => #value,
        }
    });

    let as_str_other = if ctx.exhaustive() {
        quote!()
    } else {
        quote!(Inner_::Unknown(v) => v,)
    };

    let visit_str_arms = def.values().iter().map(|v| {
        let value = v.value();
        let constant = Ident::new(v.value(), Span::call_site());
        quote! {
            #value => #ok(#name::#constant),
        }
    });

    let visit_str_other = if ctx.exhaustive() {
        let values = def.values().iter().map(|v| v.value());
        quote! {
            v => #err(de::Error::unknown_variant(v, &[#(#values, )*]))
        }
    } else {
        quote! {
            v => {
                // FIXME enforce SCREAMING_SNAKE_CASE?
                #ok(#name(Inner_::Unknown(v.to_string().into_boxed_str())))
            }
        }
    };

    quote! {
        use #conjure_types::serde::{ser, de};
        use std::fmt;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum Inner_ {
            #(
                #variants,
            )*
            #other_variant
        }

        #root_docs
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct #name(Inner_);

        impl #name {
            #(#constants)*

            /// Returns the string representation of the enum.
            #[inline]
            pub fn as_str(&self) -> &str {
                match &self.0 {
                    #(#as_str_arms)*
                    #as_str_other
                }
            }
        }

        impl fmt::Display for #name {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str(self.as_str())
            }
        }

        impl ser::Serialize for #name {
            fn serialize<S_>(&self, s: S_) -> #result<S_::Ok, S_::Error>
            where
                S_: ser::Serializer,
            {
                s.serialize_str(self.as_str())
            }
        }

        impl<'de> de::Deserialize<'de> for #name {
            fn deserialize<D_>(d: D_) -> #result<#name, D_::Error>
            where
                D_: de::Deserializer<'de>
            {
                d.deserialize_str(Visitor_)
            }
        }

        struct Visitor_;

        impl<'de> de::Visitor<'de> for Visitor_ {
            type Value = #name;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("string")
            }

            fn visit_str<E_>(self, v: &str) -> #result<#name, E_>
            where
                E_: de::Error,
            {
                match v {
                    #(#visit_str_arms)*
                    #visit_str_other
                }
            }
        }
    }
}
