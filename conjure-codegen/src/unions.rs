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
use std::iter;

use crate::context::Context;
use crate::types::UnionDefinition;

pub fn generate(ctx: &Context, def: &UnionDefinition) -> TokenStream {
    let enum_ = generate_enum(ctx, def);
    let serialize = generate_serialize(ctx, def);
    let deserialize = generate_deserialize(ctx, def);
    let variant = generate_variant(ctx, def);
    let unknown = generate_unknown(ctx, def);

    quote! {
        use conjure_object::serde::{ser, de};
        use conjure_object::serde::ser::SerializeMap as SerializeMap_;
        use conjure_object::private::{UnionField_, UnionTypeField_};
        use std::fmt;

        #enum_
        #serialize
        #deserialize
        #variant
        #unknown
    }
}

fn variants(ctx: &Context, def: &UnionDefinition) -> Vec<Ident> {
    def.union_()
        .iter()
        .map(|f| ctx.type_name(f.field_name()))
        .collect()
}

fn unknown(ctx: &Context, def: &UnionDefinition) -> TokenStream {
    if variants(ctx, def).iter().any(|f| f == "Unknown") {
        quote!(Unknown_)
    } else {
        quote!(Unknown)
    }
}

fn generate_enum(ctx: &Context, def: &UnionDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());

    let mut type_attrs = vec![];
    let mut derives = vec!["Debug", "Clone"];
    if def.union_().iter().any(|v| ctx.has_double(v.type_())) {
        derives.push("conjure_object::private::Educe");
        type_attrs.push(quote!(#[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]));
    } else {
        derives.push("PartialEq");
        derives.push("Eq");
        derives.push("PartialOrd");
        derives.push("Ord");
        derives.push("Hash");
    }
    let derives = derives.iter().map(|s| s.parse::<TokenStream>().unwrap());
    // The derive attr has to be before the educe attr, so insert rather than push
    type_attrs.insert(0, quote!(#[derive(#(#derives),*)]));

    let docs = def.union_().iter().map(|f| ctx.docs(f.docs()));
    let deprecated = def.union_().iter().map(|f| ctx.deprecated(f.deprecated()));

    let variants = &variants(ctx, def);

    let types = &def
        .union_()
        .iter()
        .map(|f| {
            let attr = if ctx.is_double(f.type_()) {
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

            let ty = ctx.boxed_rust_type(def.type_name(), f.type_());

            quote!(#attr #ty)
        })
        .collect::<Vec<_>>();

    let unknown = unknown(ctx, def);
    let unknown_variant = if ctx.exhaustive() {
        quote!()
    } else {
        quote! {
            /// An unknown variant.
            #unknown(#unknown),
        }
    };

    quote! {
        #(#type_attrs)*
        pub enum #name {
            #(
                #docs
                #deprecated
                #variants(#types),
            )*
            #unknown_variant
        }
    }
}

fn generate_serialize(ctx: &Context, def: &UnionDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());
    let result = ctx.result_ident(def.type_name());
    let some = ctx.some_ident(def.type_name());

    if def.union_().is_empty() && ctx.exhaustive() {
        return quote! {
            impl ser::Serialize for #name {
                fn serialize<S>(&self, _: S) -> #result<S::Ok, S::Error>
                where
                    S: ser::Serializer,
                {
                    match *self {}
                }
            }
        };
    }

    let serialize_unknown = if ctx.exhaustive() {
        quote!()
    } else {
        let unknown = unknown(ctx, def);
        quote! {
            #name::#unknown(value) => {
                map.serialize_entry(&"type", &value.type_)?;
                map.serialize_entry(&value.type_, &value.value)?;
            }
        }
    };

    let allow_deprecated = def
        .union_()
        .iter()
        .map(|f| ctx.allow_deprecated(f.deprecated()));
    let variants = &variants(ctx, def);
    let variant_strs = &def
        .union_()
        .iter()
        .map(|f| &f.field_name().0)
        .collect::<Vec<_>>();
    let variant_strs2 = variant_strs;
    let name_repeat = iter::repeat(&name);

    quote! {
        impl ser::Serialize for #name {
            fn serialize<S>(&self, s: S) -> #result<S::Ok, S::Error>
            where
                S: ser::Serializer
            {
                let mut map = s.serialize_map(#some(2))?;

                match self {
                    #(
                        #allow_deprecated
                        #name_repeat::#variants(value) => {
                            map.serialize_entry(&"type", &#variant_strs)?;
                            map.serialize_entry(&#variant_strs2, value)?;
                        }
                    )*
                    #serialize_unknown
                }

                map.end()
            }
        }
    }
}

fn generate_deserialize(ctx: &Context, def: &UnionDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());
    let result = ctx.result_ident(def.type_name());

    let expecting = format!("union {}", name);

    let some = ctx.some_ident(def.type_name());

    let variants = &variants(ctx, def);
    let variants2 = variants;
    let variants3 = variants;

    let allow_deprecated = &def
        .union_()
        .iter()
        .map(|f| ctx.allow_deprecated(f.deprecated()))
        .collect::<Vec<_>>();

    let name_repeat = iter::repeat(&name);
    let some_repeat = iter::repeat(&some);

    let unknown = unknown(ctx, def);

    let err = ctx.err_ident(def.type_name());

    let unknown_match1 = if ctx.exhaustive() {
        quote!()
    } else {
        quote! {
            (Variant_::#unknown(type_), #some(Variant_::#unknown(b))) => {
                if type_ == b {
                    let value = map.next_value()?;
                    #name::#unknown(#unknown { type_, value })
                } else {
                    return #err(de::Error::invalid_value(de::Unexpected::Str(&type_), &&*b))
                }
            }
        }
    };

    let none = ctx.none_ident(def.type_name());

    let name_repeat2 = iter::repeat(&name);

    let unknown_match2 = if ctx.exhaustive() {
        quote!()
    } else {
        quote! {
            Variant_::#unknown(ref type_) => {
                let value = map.next_value()?;
                #name::#unknown(#unknown { type_: type_.clone(), value })
            }
        }
    };

    let ok = ctx.ok_ident(def.type_name());

    let visit_map_body = if def.union_().is_empty() && ctx.exhaustive() {
        quote! {
            match map.next_key::<UnionField_<Variant_>>()? {
                #some(UnionField_::Type) => match map.next_value::<Variant_>()? {}
                #some(UnionField_::Value(variant)) => match variant {}
                #none => #err(de::Error::missing_field("type")),
            }
        }
    } else {
        let wrong_type_match = if def.union_().is_empty() {
            quote!()
        } else {
            quote! {
                (variant, #some(key)) => {
                    return #err(
                        de::Error::invalid_value(de::Unexpected::Str(key.as_str()), &variant.as_str()),
                    );
                }
            }
        };

        quote! {
            let v = match map.next_key::<UnionField_<Variant_>>()? {
                #some(UnionField_::Type) => {
                    let variant = map.next_value()?;
                    let key = map.next_key()?;
                    match (variant, key) {
                        #(
                            #allow_deprecated
                            (Variant_::#variants, #some_repeat(Variant_::#variants2)) => {
                                let value = map.next_value()?;
                                #name_repeat::#variants3(value)
                            }
                        )*
                        #unknown_match1
                        #wrong_type_match
                        (variant, #none) => return #err(de::Error::missing_field(variant.as_str())),
                    }
                }
                #some(UnionField_::Value(variant)) => {
                    let value = match variant {
                        #(
                            Variant_::#variants => {
                                let value = map.next_value()?;
                                #allow_deprecated
                                #name_repeat2::#variants2(value)
                            }
                        )*
                        #unknown_match2
                    };

                    if map.next_key::<UnionTypeField_>()?.is_none() {
                        return #err(de::Error::missing_field("type"));
                    }

                    let type_variant = map.next_value::<Variant_>()?;
                    if variant != type_variant {
                        return #err(
                            de::Error::invalid_value(de::Unexpected::Str(type_variant.as_str()), &variant.as_str()),
                        );
                    }

                    value
                }
                #none => return #err(de::Error::missing_field("type")),
            };

            if map.next_key::<UnionField_<Variant_>>()?.is_some() {
                return #err(de::Error::invalid_length(3, &"type and value fields"));
            }

            #ok(v)
        }
    };

    quote! {
        impl<'de> de::Deserialize<'de> for #name {
            fn deserialize<D>(d: D) -> #result<#name, D::Error>
            where
                D: de::Deserializer<'de>
            {
                d.deserialize_map(Visitor_)
            }
        }

        struct Visitor_;

        impl<'de> de::Visitor<'de> for Visitor_ {
            type Value = #name;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str(#expecting)
            }

            fn visit_map<A>(self, mut map: A) -> #result<#name, A::Error>
            where
                A: de::MapAccess<'de>
            {
                #visit_map_body
            }
        }
    }
}

fn generate_variant(ctx: &Context, def: &UnionDefinition) -> TokenStream {
    let variants = &variants(ctx, def);

    let unknown = unknown(ctx, def);

    let unknown_variant = if ctx.exhaustive() {
        quote!()
    } else {
        let box_ = ctx.box_ident(def.type_name());
        quote!(#unknown(#box_<str>))
    };

    let variant_strs = &def
        .union_()
        .iter()
        .map(|f| &f.field_name().0)
        .collect::<Vec<_>>();

    let unknown_as_str = if ctx.exhaustive() {
        quote!()
    } else {
        quote! {
            Variant_::#unknown(_) => "unknown variant",
        }
    };

    let result = ctx.result_ident(def.type_name());

    let unknown_de_visit_str = if ctx.exhaustive() {
        let err = ctx.err_ident(def.type_name());
        quote! {
            value => return #err(de::Error::unknown_variant(value, &[#(#variant_strs, )*])),
        }
    } else {
        quote! {
            value => Variant_::#unknown(value.to_string().into_boxed_str()),
        }
    };

    let ok = ctx.ok_ident(def.type_name());

    let de_visit_str_match = quote! {
        match value {
            #(
                #variant_strs => Variant_::#variants,
            )*
            #unknown_de_visit_str
        }
    };

    let de_visit_str_body = if def.union_().is_empty() && ctx.exhaustive() {
        de_visit_str_match
    } else {
        quote! {
            let v = #de_visit_str_match;
            #ok(v)
        }
    };

    quote! {
        #[derive(PartialEq)]
        enum Variant_ {
            #(#variants,)*
            #unknown_variant
        }

        impl Variant_ {
            fn as_str(&self) -> &'static str {
                match *self {
                    #(
                        Variant_::#variants => #variant_strs,
                    )*
                    #unknown_as_str
                }
            }
        }

        impl<'de> de::Deserialize<'de> for Variant_ {
            fn deserialize<D>(d: D) -> #result<Variant_, D::Error>
            where
                D: de::Deserializer<'de>
            {
                d.deserialize_str(VariantVisitor_)
            }
        }

        struct VariantVisitor_;

        impl<'de> de::Visitor<'de> for VariantVisitor_ {
            type Value = Variant_;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("string")
            }

            fn visit_str<E>(self, value: &str) -> #result<Variant_, E>
            where
                E: de::Error,
            {
                #de_visit_str_body
            }
        }
    }
}

fn generate_unknown(ctx: &Context, def: &UnionDefinition) -> TokenStream {
    if ctx.exhaustive() {
        return quote!();
    }

    let doc = format!(
        "An unknown variant of the {} union.",
        ctx.type_name(def.type_name().name())
    );

    let unknown = unknown(ctx, def);
    let box_ = ctx.box_ident(def.type_name());

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct #unknown {
            type_: #box_<str>,
            value: conjure_object::Any,
        }

        impl #unknown {
            /// Returns the unknown variant's type name.
            #[inline]
            pub fn type_(&self) -> &str {
                &self.type_
            }
        }
    }
}
