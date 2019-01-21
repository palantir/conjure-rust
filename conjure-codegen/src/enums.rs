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
use crate::types::EnumDefinition;

pub fn generate(ctx: &Context, def: &EnumDefinition) -> TokenStream {
    let enum_ = generate_enum(ctx, def);
    let unknown = generate_unknown(ctx, def);

    quote! {
        use conjure_object::serde::{ser, de};
        use std::fmt;

        #enum_
        #unknown
    }
}

fn generate_enum(ctx: &Context, def: &EnumDefinition) -> TokenStream {
    let root_docs = ctx.docs(def.docs());
    let name = ctx.type_name(def.type_name().name());
    let result = ctx.result_ident(def.type_name());
    let ok = ctx.ok_ident(def.type_name());
    let err = ctx.err_ident(def.type_name());
    let unknown = unknown(ctx, def);

    let variants = def.values().iter().map(|v| ctx.type_name(v.value()));

    let other_variant = if ctx.exhaustive() {
        quote!()
    } else {
        quote! {
            /// An unknown variant.
            Unknown(#unknown)
        }
    };

    let as_str_arms = def.values().iter().map(|v| {
        let value = v.value();
        let variant = ctx.type_name(v.value());
        quote! {
            #name::#variant => #value,
        }
    });

    let as_str_other = if ctx.exhaustive() {
        quote!()
    } else {
        quote!(#name::Unknown(v) => &*v,)
    };

    let visit_str_arms = def.values().iter().map(|v| {
        let value = v.value();
        let variant = ctx.type_name(value);
        quote! {
            #value => #ok(#name::#variant),
        }
    });

    let values = def.values().iter().map(|v| v.value());
    let unknown_variant_error = quote! {
        #err(de::Error::unknown_variant(v, &[#(#values, )*]))
    };

    let visit_str_other = if ctx.exhaustive() {
        quote! {
            v => #unknown_variant_error,
        }
    } else {
        quote! {
            v => {
                if conjure_object::private::valid_enum_variant(v) {
                    #ok(#name::Unknown(#unknown(v.to_string().into_boxed_str())))
                } else {
                    #unknown_variant_error
                }
            }
        }
    };

    quote! {
        #root_docs
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum #name {
            #(
                #variants,
            )*
            #other_variant
        }

        impl #name {
            /// Returns the string representation of the enum.
            #[inline]
            pub fn as_str(&self) -> &str {
                match self {
                    #(#as_str_arms)*
                    #as_str_other
                }
            }
        }

        impl fmt::Display for #name {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt::Display::fmt(self.as_str(), fmt)
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

fn unknown(ctx: &Context, def: &EnumDefinition) -> TokenStream {
    if ctx.type_name(def.type_name().name()) == "Unknown" {
        quote!(Unknown_)
    } else {
        quote!(Unknown)
    }
}

fn generate_unknown(ctx: &Context, def: &EnumDefinition) -> TokenStream {
    if ctx.exhaustive() {
        return quote!();
    }

    let doc = format!(
        "An unknown variant of the {} enum.",
        ctx.type_name(def.type_name().name())
    );

    let unknown = unknown(ctx, def);
    let box_ = ctx.box_ident(def.type_name());

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct #unknown(#box_<str>);

        impl std::ops::Deref for #unknown {
            type Target = str;

            #[inline]
            fn deref(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for #unknown {
            #[inline]
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, fmt)
            }
        }
    }
}
