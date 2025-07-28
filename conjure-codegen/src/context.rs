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
#![allow(clippy::match_like_matches_macro)]

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use crate::errors::error_object_definition;
use crate::types::{
    ArgumentDefinition, ConjureDefinition, Documentation, LogSafety, PrimitiveType, Type,
    TypeDefinition, TypeName,
};

enum CachedLogSafety {
    Uncomputed,
    Computed(Option<LogSafety>),
}

struct TypeContext {
    def: TypeDefinition,
    has_double: Cell<Option<bool>>,
    is_copy: Cell<Option<bool>>,
    log_safety: RefCell<CachedLogSafety>,
}

pub struct Context {
    types: HashMap<TypeName, TypeContext>,
    exhaustive: bool,
    serialize_empty_collections: bool,
    strip_prefix: Vec<String>,
    version: Option<String>,
}

impl Context {
    pub fn new(
        defs: &ConjureDefinition,
        exhaustive: bool,
        serialize_empty_collections: bool,
        strip_prefix: Option<&str>,
        version: Option<&str>,
    ) -> Context {
        let mut context = Context {
            types: HashMap::new(),
            exhaustive,
            serialize_empty_collections,
            strip_prefix: vec![],
            version: version.map(str::to_owned),
        };

        if let Some(strip_prefix) = strip_prefix {
            context.strip_prefix = context.raw_module_path(strip_prefix);
        }

        for def in defs.types() {
            let name = match &def {
                TypeDefinition::Alias(def) => def.type_name().clone(),
                TypeDefinition::Enum(def) => def.type_name().clone(),
                TypeDefinition::Object(def) => def.type_name().clone(),
                TypeDefinition::Union(def) => def.type_name().clone(),
            };

            context.types.insert(
                name,
                TypeContext {
                    def: def.clone(),
                    has_double: Cell::new(None),
                    is_copy: Cell::new(None),
                    log_safety: RefCell::new(CachedLogSafety::Uncomputed),
                },
            );
        }

        for def in defs.errors() {
            context.types.insert(
                def.error_name().clone(),
                TypeContext {
                    def: TypeDefinition::Object(error_object_definition(def)),
                    has_double: Cell::new(None),
                    is_copy: Cell::new(None),
                    log_safety: RefCell::new(CachedLogSafety::Uncomputed),
                },
            );
        }

        context
    }

    pub fn exhaustive(&self) -> bool {
        self.exhaustive
    }

    pub fn serialize_empty_collections(&self) -> bool {
        self.serialize_empty_collections
    }

    fn needs_box(&self, def: &Type) -> bool {
        match def {
            Type::Primitive(_) => false,
            Type::Optional(def) => self.needs_box(def.item_type()),
            Type::List(_) | Type::Set(_) | Type::Map(_) => false,
            Type::Reference(def) => self.ref_needs_box(def),
            Type::External(def) => self.needs_box(def.fallback()),
        }
    }

    fn ref_needs_box(&self, name: &TypeName) -> bool {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.needs_box(def.alias()),
            TypeDefinition::Enum(_) => false,
            TypeDefinition::Object(_) | TypeDefinition::Union(_) => true,
        }
    }

    pub fn has_double(&self, def: &Type) -> bool {
        match def {
            Type::Primitive(def) => match *def {
                PrimitiveType::Double => true,
                _ => false,
            },
            Type::Optional(def) => self.has_double(def.item_type()),
            Type::List(def) => self.has_double(def.item_type()),
            Type::Set(def) => self.has_double(def.item_type()),
            Type::Map(def) => self.has_double(def.key_type()) || self.has_double(def.value_type()),
            Type::Reference(def) => self.ref_has_double(def),
            Type::External(def) => self.has_double(def.fallback()),
        }
    }

    fn ref_has_double(&self, name: &TypeName) -> bool {
        let ctx = &self.types[name];

        if let Some(has_double) = ctx.has_double.get() {
            return has_double;
        }

        ctx.has_double.set(Some(false)); // break cycles
        let has_double = match &ctx.def {
            TypeDefinition::Alias(def) => self.has_double(def.alias()),
            TypeDefinition::Enum(_) => false,
            TypeDefinition::Object(def) => def.fields().iter().any(|f| self.has_double(f.type_())),
            TypeDefinition::Union(def) => def.union_().iter().any(|f| self.has_double(f.type_())),
        };

        ctx.has_double.set(Some(has_double));
        has_double
    }

    pub fn is_copy(&self, def: &Type) -> bool {
        match def {
            Type::Primitive(def) => match *def {
                PrimitiveType::String
                | PrimitiveType::Binary
                | PrimitiveType::Any
                | PrimitiveType::Rid
                | PrimitiveType::Bearertoken => false,
                PrimitiveType::Datetime
                | PrimitiveType::Integer
                | PrimitiveType::Double
                | PrimitiveType::Safelong
                | PrimitiveType::Boolean
                | PrimitiveType::Uuid => true,
            },
            Type::Optional(def) => self.is_copy(def.item_type()),
            Type::List(_) | Type::Set(_) | Type::Map(_) => false,
            Type::Reference(def) => self.ref_is_copy(def),
            Type::External(def) => self.is_copy(def.fallback()),
        }
    }

    fn ref_is_copy(&self, name: &TypeName) -> bool {
        let ctx = &self.types[name];

        if let Some(is_copy) = ctx.is_copy.get() {
            return is_copy;
        }

        let is_copy = match &ctx.def {
            TypeDefinition::Alias(def) => self.is_copy(def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => false,
        };

        ctx.is_copy.set(Some(is_copy));
        is_copy
    }

    pub fn is_required(&self, def: &Type) -> bool {
        match def {
            Type::Primitive(_) => true,
            Type::Optional(_) | Type::List(_) | Type::Set(_) | Type::Map(_) => false,
            Type::Reference(def) => self.ref_is_required(def),
            Type::External(def) => self.is_required(def.fallback()),
        }
    }

    fn ref_is_required(&self, name: &TypeName) -> bool {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.is_required(def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => true,
        }
    }

    pub fn is_default(&self, def: &Type) -> bool {
        match def {
            Type::Primitive(def) => match *def {
                PrimitiveType::String
                | PrimitiveType::Integer
                | PrimitiveType::Double
                | PrimitiveType::Safelong
                | PrimitiveType::Binary
                | PrimitiveType::Boolean => true,
                PrimitiveType::Datetime
                | PrimitiveType::Any
                | PrimitiveType::Uuid
                | PrimitiveType::Rid
                | PrimitiveType::Bearertoken => false,
            },
            Type::Optional(_) | Type::List(_) | Type::Set(_) | Type::Map(_) => true,
            Type::Reference(def) => self.ref_is_default(def),
            Type::External(def) => self.is_default(def.fallback()),
        }
    }

    fn ref_is_default(&self, name: &TypeName) -> bool {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.is_default(def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => false,
        }
    }

    pub fn is_display(&self, def: &Type) -> bool {
        match def {
            Type::Primitive(def) => match *def {
                PrimitiveType::String
                | PrimitiveType::Datetime
                | PrimitiveType::Integer
                | PrimitiveType::Double
                | PrimitiveType::Safelong
                | PrimitiveType::Boolean
                | PrimitiveType::Uuid
                | PrimitiveType::Rid => true,
                PrimitiveType::Binary | PrimitiveType::Any | PrimitiveType::Bearertoken => false,
            },
            Type::Optional(_) | Type::List(_) | Type::Set(_) | Type::Map(_) => false,
            Type::Reference(def) => self.ref_is_display(def),
            Type::External(def) => self.is_display(def.fallback()),
        }
    }

    fn ref_is_display(&self, name: &TypeName) -> bool {
        match &self.types[name].def {
            TypeDefinition::Alias(def) => self.is_display(def.alias()),
            TypeDefinition::Enum(_) => true,
            TypeDefinition::Object(_) | TypeDefinition::Union(_) => false,
        }
    }

    pub fn is_from_iter(&self, this_type: &TypeName, def: &Type) -> Option<TokenStream> {
        match def {
            Type::Primitive(_) | Type::Optional(_) => None,
            Type::Map(def) => {
                let key = self.rust_type_inner(this_type, def.key_type(), true);
                let value = self.rust_type(this_type, def.value_type());
                Some(quote!((#key, #value)))
            }
            Type::List(def) => Some(self.rust_type(this_type, def.item_type())),
            Type::Set(def) => Some(self.rust_type_inner(this_type, def.item_type(), true)),
            Type::Reference(def) => self.ref_is_from_iter(this_type, def),
            Type::External(def) => self.is_from_iter(this_type, def.fallback()),
        }
    }

    fn ref_is_from_iter(&self, this_type: &TypeName, name: &TypeName) -> Option<TokenStream> {
        match &self.types[name].def {
            TypeDefinition::Alias(def) => self.is_from_iter(this_type, def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => None,
        }
    }

    pub fn dealiased_type<'a>(&'a self, def: &'a Type) -> &'a Type {
        match def {
            Type::Primitive(_)
            | Type::Optional(_)
            | Type::List(_)
            | Type::Set(_)
            | Type::Map(_) => def,
            Type::Reference(name) => match &self.types[name].def {
                TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => {
                    def
                }
                TypeDefinition::Alias(def) => self.dealiased_type(def.alias()),
            },
            Type::External(def) => self.dealiased_type(def.fallback()),
        }
    }

    pub fn rust_type(&self, this_type: &TypeName, def: &Type) -> TokenStream {
        self.rust_type_inner(this_type, def, false)
    }

    fn rust_type_inner(&self, this_type: &TypeName, def: &Type, key: bool) -> TokenStream {
        match def {
            Type::Primitive(def) => match *def {
                PrimitiveType::String => self.string_ident(this_type),
                PrimitiveType::Datetime => quote!(conjure_object::DateTime<conjure_object::Utc>),
                PrimitiveType::Integer => quote!(i32),
                PrimitiveType::Double => {
                    if key {
                        quote!(conjure_object::DoubleKey)
                    } else {
                        quote!(f64)
                    }
                }
                PrimitiveType::Safelong => quote!(conjure_object::SafeLong),
                PrimitiveType::Binary => quote!(conjure_object::Bytes),
                PrimitiveType::Any => quote!(conjure_object::Any),
                PrimitiveType::Boolean => quote!(bool),
                PrimitiveType::Uuid => quote!(conjure_object::Uuid),
                PrimitiveType::Rid => quote!(conjure_object::ResourceIdentifier),
                PrimitiveType::Bearertoken => quote!(conjure_object::BearerToken),
            },
            Type::Optional(def) => {
                let option = self.option_ident(this_type);
                let item = self.rust_type_inner(this_type, def.item_type(), key);
                quote!(#option<#item>)
            }
            Type::List(def) => {
                let vec = self.vec_ident(this_type);
                let item = self.rust_type_inner(this_type, def.item_type(), key);
                quote!(#vec<#item>)
            }
            Type::Set(def) => {
                let item = self.rust_type_inner(this_type, def.item_type(), true);
                quote!(std::collections::BTreeSet<#item>)
            }
            Type::Map(def) => {
                let key = self.rust_type_inner(this_type, def.key_type(), true);
                let value = self.rust_type(this_type, def.value_type());
                quote!(std::collections::BTreeMap<#key, #value>)
            }
            Type::Reference(def) => self.type_path(this_type, def),
            Type::External(def) => self.rust_type_inner(this_type, def.fallback(), key),
        }
    }

    pub fn boxed_rust_type(&self, this_type: &TypeName, def: &Type) -> TokenStream {
        match def {
            Type::Optional(def) => {
                let option = self.option_ident(this_type);
                let item = self.boxed_rust_type(this_type, def.item_type());
                quote!(#option<#item>)
            }
            Type::Reference(def) => self.ref_boxed_rust_type(this_type, def),
            Type::External(def) => self.boxed_rust_type(this_type, def.fallback()),
            def => self.rust_type(this_type, def),
        }
    }

    fn ref_boxed_rust_type(&self, this_type: &TypeName, name: &TypeName) -> TokenStream {
        let ctx = &self.types[name];

        let needs_box = match &ctx.def {
            TypeDefinition::Alias(def) => self.needs_box(def.alias()),
            TypeDefinition::Enum(_) => false,
            TypeDefinition::Object(_) => match &self.types[this_type].def {
                TypeDefinition::Union(_) => false,
                _ => true,
            },
            TypeDefinition::Union(_) => true,
        };

        let unboxed = self.type_path(this_type, name);
        if needs_box {
            let box_ = self.box_ident(name);
            quote!(#box_<#unboxed>)
        } else {
            unboxed
        }
    }

    pub fn borrowed_rust_type(&self, this_type: &TypeName, def: &Type) -> TokenStream {
        match def {
            Type::Primitive(def) => match *def {
                PrimitiveType::String => quote!(&str),
                PrimitiveType::Datetime => quote!(conjure_object::DateTime<conjure_object::Utc>),
                PrimitiveType::Integer => quote!(i32),
                PrimitiveType::Double => quote!(f64),
                PrimitiveType::Safelong => quote!(conjure_object::SafeLong),
                PrimitiveType::Binary => quote!(&conjure_object::Bytes),
                PrimitiveType::Any => quote!(&conjure_object::Any),
                PrimitiveType::Boolean => quote!(bool),
                PrimitiveType::Uuid => quote!(conjure_object::Uuid),
                PrimitiveType::Rid => quote!(&conjure_object::ResourceIdentifier),
                PrimitiveType::Bearertoken => quote!(&conjure_object::BearerToken),
            },
            Type::Optional(def) => {
                let option = self.option_ident(this_type);
                let item = self.borrowed_rust_type(this_type, def.item_type());
                quote!(#option<#item>)
            }
            Type::List(def) => {
                let item = self.rust_type(this_type, def.item_type());
                quote!(&[#item])
            }
            Type::Set(def) => {
                let item = self.rust_type_inner(this_type, def.item_type(), true);
                quote!(&std::collections::BTreeSet<#item>)
            }
            Type::Map(def) => {
                let key = self.rust_type_inner(this_type, def.key_type(), true);
                let value = self.rust_type(this_type, def.value_type());
                quote!(&std::collections::BTreeMap<#key, #value>)
            }
            Type::Reference(def) => self.borrowed_rust_type_ref(this_type, def),
            Type::External(def) => self.borrowed_rust_type(this_type, def.fallback()),
        }
    }

    fn borrowed_rust_type_ref(&self, this_type: &TypeName, name: &TypeName) -> TokenStream {
        let ctx = &self.types[name];

        let type_ = self.type_path(this_type, name);
        match &ctx.def {
            TypeDefinition::Alias(def) => {
                if self.is_copy(def.alias()) {
                    type_
                } else {
                    quote!(&#type_)
                }
            }
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => {
                quote!(&#type_)
            }
        }
    }

    pub fn borrow_rust_type(&self, value: TokenStream, def: &Type) -> TokenStream {
        match def {
            Type::Primitive(def) => match *def {
                PrimitiveType::String => quote!(&*#value),
                PrimitiveType::Any
                | PrimitiveType::Rid
                | PrimitiveType::Bearertoken
                | PrimitiveType::Binary => {
                    quote!(&#value)
                }
                PrimitiveType::Datetime
                | PrimitiveType::Integer
                | PrimitiveType::Double
                | PrimitiveType::Safelong
                | PrimitiveType::Boolean
                | PrimitiveType::Uuid => value,
            },
            Type::Optional(def) => {
                let borrow_item = self.borrow_rust_type(quote!(*o), def.item_type());
                quote!(#value.as_ref().map(|o| #borrow_item))
            }
            Type::List(_) => quote!(&*#value),
            Type::Set(_) | Type::Map(_) => quote!(&#value),
            Type::Reference(def) => self.borrow_rust_type_ref(value, def),
            Type::External(def) => self.borrow_rust_type(value, def.fallback()),
        }
    }

    fn borrow_rust_type_ref(&self, value: TokenStream, name: &TypeName) -> TokenStream {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => {
                if self.needs_box(def.alias()) {
                    quote!(&*#value)
                } else if self.is_copy(def.alias()) {
                    value
                } else {
                    quote!(&#value)
                }
            }
            TypeDefinition::Enum(_) => quote!(&#value),
            TypeDefinition::Object(_) | TypeDefinition::Union(_) => quote!(&*#value),
        }
    }

    pub fn builder_config(&self, this_type: &TypeName, def: &Type) -> BuilderConfig {
        match def {
            Type::Primitive(def) => match def {
                PrimitiveType::String | PrimitiveType::Binary => BuilderConfig::Into,
                PrimitiveType::Any => BuilderConfig::Custom {
                    type_: quote!(impl conjure_object::serde::Serialize),
                    convert: quote!(
                        |v| conjure_object::Any::new(v).expect("value failed to serialize")
                    ),
                },
                _ => BuilderConfig::Normal,
            },
            Type::Optional(def) => {
                if self.needs_box(def.item_type()) {
                    let into = self.into_ident(this_type);
                    let option = self.option_ident(this_type);
                    let item_type = self.rust_type(this_type, def.item_type());
                    let box_ = self.box_ident(this_type);
                    BuilderConfig::Custom {
                        type_: quote!(impl #into<#option<#item_type>>),
                        convert: quote!(|v| v.into().map(#box_::new)),
                    }
                } else {
                    BuilderConfig::Into
                }
            }
            Type::List(def) => BuilderConfig::List {
                item: self.builder_item_config(this_type, def.item_type(), false),
            },
            Type::Set(def) => BuilderConfig::Set {
                item: self.builder_item_config(this_type, def.item_type(), true),
            },
            Type::Map(def) => BuilderConfig::Map {
                key: self.builder_item_config(this_type, def.key_type(), true),
                value: self.builder_item_config(this_type, def.value_type(), false),
            },
            Type::Reference(def) => {
                if self.ref_needs_box(def) {
                    let box_ = self.box_ident(this_type);
                    BuilderConfig::Custom {
                        type_: self.type_path(this_type, def),
                        convert: quote!(#box_::new),
                    }
                } else {
                    BuilderConfig::Normal
                }
            }
            Type::External(def) => self.builder_config(this_type, def.fallback()),
        }
    }

    fn builder_item_config(
        &self,
        this_type: &TypeName,
        def: &Type,
        key: bool,
    ) -> BuilderItemConfig {
        match def {
            Type::Primitive(primitive) => match primitive {
                PrimitiveType::String => BuilderItemConfig::Into {
                    type_: self.string_ident(this_type),
                },
                PrimitiveType::Binary => BuilderItemConfig::Into {
                    type_: quote!(conjure_object::Bytes),
                },
                PrimitiveType::Any => BuilderItemConfig::Custom {
                    type_: quote!(impl conjure_object::serde::Serialize),
                    convert: quote!(
                        |v| conjure_object::Any::new(v).expect("value failed to serialize")
                    ),
                },
                _ => BuilderItemConfig::Normal {
                    type_: self.rust_type_inner(this_type, def, key),
                },
            },
            Type::Optional(def) => {
                let option = self.option_ident(this_type);
                let item_type = self.rust_type(this_type, def.item_type());
                BuilderItemConfig::Into {
                    type_: quote!(#option<#item_type>),
                }
            }
            Type::List(def) => {
                let into_iterator = self.into_iterator_ident(this_type);
                let item_type = self.rust_type_inner(this_type, def.item_type(), key);
                BuilderItemConfig::Custom {
                    type_: quote!(impl #into_iterator<Item = #item_type>),
                    convert: quote!(|v| v.into_iter().collect()),
                }
            }
            Type::Set(def) => {
                let into_iterator = self.into_iterator_ident(this_type);
                let item_type = self.rust_type_inner(this_type, def.item_type(), true);
                BuilderItemConfig::Custom {
                    type_: quote!(impl #into_iterator<Item = #item_type>),
                    convert: quote!(|v| v.into_iter().collect()),
                }
            }
            Type::Map(def) => {
                let into_iterator = self.into_iterator_ident(this_type);
                let key_type = self.rust_type_inner(this_type, def.key_type(), true);
                let value_type = self.rust_type(this_type, def.value_type());
                BuilderItemConfig::Custom {
                    type_: quote!(impl #into_iterator<Item = (#key_type, #value_type)>),
                    convert: quote!(|v| v.into_iter().collect()),
                }
            }
            Type::Reference(def) => BuilderItemConfig::Normal {
                type_: self.type_path(this_type, def),
            },
            Type::External(def) => self.builder_item_config(this_type, def.fallback(), key),
        }
    }

    pub fn is_empty_method(&self, this_type: &TypeName, def: &Type) -> Option<String> {
        match def {
            Type::Primitive(_) => None,
            Type::Optional(_) => {
                let option = self.option_ident(this_type);
                Some(format!("{option}::is_none"))
            }
            Type::List(_) => {
                let vec = self.vec_ident(this_type);
                Some(format!("{vec}::is_empty"))
            }
            Type::Set(_) => Some("std::collections::BTreeSet::is_empty".to_string()),
            Type::Map(_) => Some("std::collections::BTreeMap::is_empty".to_string()),
            Type::Reference(def) => self.is_empty_method_ref(this_type, def),
            Type::External(def) => self.is_empty_method(this_type, def.fallback()),
        }
    }

    fn is_empty_method_ref(&self, this_type: &TypeName, name: &TypeName) -> Option<String> {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.is_empty_method(this_type, def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => None,
        }
    }

    pub fn is_binary(&self, def: &Type) -> bool {
        match def {
            Type::Primitive(PrimitiveType::Binary) => true,
            Type::Primitive(_)
            | Type::Optional(_)
            | Type::List(_)
            | Type::Set(_)
            | Type::Map(_) => false,
            Type::Reference(def) => self.is_binary_ref(def),
            Type::External(def) => self.is_binary(def.fallback()),
        }
    }

    fn is_binary_ref(&self, name: &TypeName) -> bool {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.is_binary(def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => false,
        }
    }

    pub fn is_plain(&self, def: &Type) -> bool {
        match def {
            Type::Primitive(primitive) => match primitive {
                PrimitiveType::String
                | PrimitiveType::Datetime
                | PrimitiveType::Integer
                | PrimitiveType::Double
                | PrimitiveType::Safelong
                | PrimitiveType::Binary
                | PrimitiveType::Boolean
                | PrimitiveType::Uuid
                | PrimitiveType::Rid
                | PrimitiveType::Bearertoken => true,
                PrimitiveType::Any => false,
            },
            Type::Optional(_) | Type::List(_) | Type::Set(_) | Type::Map(_) => false,
            Type::Reference(def) => self.is_plain_ref(def),
            Type::External(def) => self.is_plain(def.fallback()),
        }
    }

    fn is_plain_ref(&self, name: &TypeName) -> bool {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.is_plain(def.alias()),
            TypeDefinition::Enum(_) => true,
            TypeDefinition::Object(_) | TypeDefinition::Union(_) => false,
        }
    }

    pub fn is_iterable(&self, def: &Type) -> bool {
        match def {
            Type::Primitive(_) => false,
            Type::Optional(_) | Type::List(_) | Type::Set(_) | Type::Map(_) => true,
            Type::Reference(def) => self.is_iterable_ref(def),
            Type::External(def) => self.is_iterable(def.fallback()),
        }
    }

    fn is_iterable_ref(&self, name: &TypeName) -> bool {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.is_iterable(def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => false,
        }
    }

    pub fn is_optional<'a>(&'a self, def: &'a Type) -> Option<&'a Type> {
        match def {
            Type::Primitive(_) | Type::List(_) | Type::Set(_) | Type::Map(_) => None,
            Type::Optional(def) => Some(def.item_type()),
            Type::Reference(def) => self.is_optional_ref(def),
            Type::External(def) => self.is_optional(def.fallback()),
        }
    }

    fn is_optional_ref(&self, name: &TypeName) -> Option<&Type> {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.is_optional(def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => None,
        }
    }

    pub fn is_list(&self, def: &Type) -> bool {
        match def {
            Type::List(_) => true,
            Type::Primitive(_) | Type::Optional(_) | Type::Set(_) | Type::Map(_) => false,
            Type::Reference(def) => self.is_list_ref(def),
            Type::External(def) => self.is_list(def.fallback()),
        }
    }

    fn is_list_ref(&self, name: &TypeName) -> bool {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.is_list(def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => false,
        }
    }

    pub fn is_set(&self, def: &Type) -> bool {
        match def {
            Type::Set(_) => true,
            Type::Primitive(_) | Type::Optional(_) | Type::List(_) | Type::Map(_) => false,
            Type::Reference(def) => self.is_set_ref(def),
            Type::External(def) => self.is_set(def.fallback()),
        }
    }

    fn is_set_ref(&self, name: &TypeName) -> bool {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.is_set(def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => false,
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    pub fn is_double(&self, def: &Type) -> bool {
        match def {
            Type::Primitive(PrimitiveType::Double) => true,
            Type::Optional(def) => self.is_double(def.item_type()),
            Type::List(def) => self.is_double(def.item_type()),
            Type::Map(def) => self.is_double(def.value_type()),
            Type::Primitive(_) | Type::Set(_) | Type::Reference(_) => false,
            Type::External(def) => self.is_double(def.fallback()),
        }
    }

    pub fn docs(&self, docs: Option<&Documentation>) -> TokenStream {
        match docs {
            Some(docs) => {
                let docs = docs
                    .lines()
                    // Add a leading space to align with standard doc comments.
                    .map(|s| format!(" {s}"))
                    // Docs may have code blocks, which will turn into rust doctests by default,
                    // and probably not build. Sticking a `ignore` info string onto the opening
                    // fence makes rustdoc skip them.
                    .map({
                        let mut in_code_block = false;
                        move |mut s| {
                            if !s.trim().starts_with("```") {
                                return s;
                            }

                            if in_code_block {
                                in_code_block = false;
                                return s;
                            }

                            if s.trim() == "```" {
                                s.push_str("ignore");
                            }
                            in_code_block = true;

                            s
                        }
                    });
                quote!(#(#[doc = #docs])*)
            }
            None => quote!(),
        }
    }

    pub fn deprecated(&self, deprecated: Option<&Documentation>) -> TokenStream {
        match deprecated {
            Some(docs) => {
                let docs = &**docs;
                quote!(#[deprecated(note = #docs)])
            }
            None => quote!(),
        }
    }

    pub fn allow_deprecated(&self, deprecated: Option<&Documentation>) -> TokenStream {
        match deprecated {
            Some(_) => quote!(#[allow(deprecated)]),
            None => quote!(),
        }
    }

    pub fn box_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Box", "std::boxed::Box")
    }

    pub fn result_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Result", "std::result::Result")
    }

    pub fn ok_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Ok", "Result::Ok")
    }

    pub fn err_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Err", "Result::Err")
    }

    pub fn option_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Option", "std::option::Option")
    }

    pub fn some_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Some", "Option::Some")
    }

    pub fn none_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "None", "Option::None")
    }

    pub fn string_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "String", "std::string::String")
    }

    pub fn vec_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Vec", "std::vec::Vec")
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Into", "std::convert::Into")
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_iterator_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "IntoIterator", "std::iter::IntoIterator")
    }

    pub fn send_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Send", "std::marker::Send")
    }

    fn prelude_ident(&self, name: &TypeName, short: &str, long: &str) -> TokenStream {
        let s = if self.type_name(name.name()) == short {
            long
        } else {
            short
        };

        s.parse().unwrap()
    }

    pub fn module_name(&self, name: &TypeName) -> String {
        self.ident_name(name.name())
    }

    pub fn field_name(&self, s: &str) -> Ident {
        Ident::new(&self.ident_name(s), Span::call_site())
    }

    fn ident_name(&self, s: &str) -> String {
        let mut s = s.to_snake_case();

        let keyword = match &*s {
            // strict keywords
            "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern"
            | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod"
            | "move" | "mut" | "pub" | "ref" | "return" | "self" | "static" | "struct"
            | "super" | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" => true,
            // reserved keywords
            "abstract" | "async" | "become" | "box" | "do" | "final" | "macro" | "override"
            | "priv" | "typeof" | "unsized" | "virtual" | "yield" => true,
            // weak keywords
            "union" | "dyn" => true,
            // builder pattern methods
            "build" | "builder" | "new" => true,
            _ => false,
        };

        if keyword {
            s.push('_');
        }

        s
    }

    pub fn type_name(&self, name: &str) -> Ident {
        let mut name = name.to_upper_camel_case();

        let keyword = match &*name {
            "Self" => true,
            _ => false,
        };

        if keyword {
            name.push('_');
        }

        Ident::new(&name, Span::call_site())
    }

    pub fn module_path(&self, name: &TypeName) -> Vec<String> {
        let raw = self.raw_module_path(name.package());

        if raw.starts_with(&self.strip_prefix) {
            raw[self.strip_prefix.len()..].to_vec()
        } else {
            raw
        }
    }

    fn raw_module_path(&self, package: &str) -> Vec<String> {
        package.split('.').map(|s| self.ident_name(s)).collect()
    }

    fn type_path(&self, this_type: &TypeName, other_type: &TypeName) -> TokenStream {
        let this_module_path = self.module_path(this_type);
        let other_module_path = self.module_path(other_type);

        let shared_prefix = this_module_path
            .iter()
            .zip(&other_module_path)
            .take_while(|(a, b)| a == b)
            .count();

        // one super to get out of this type's module
        let mut components = vec![quote!(super)];

        // one for each part of this type's unique module prefix
        for _ in 0..this_module_path.len() - shared_prefix {
            components.push(quote!(super));
        }

        // then the path to the other type's module
        for component in &other_module_path[shared_prefix..] {
            components.push(component.parse().unwrap());
        }

        let other_type_name = self.type_name(other_type.name());

        quote!(#(#components::)* #other_type_name)
    }

    // https://github.com/palantir/conjure-java/blob/develop/conjure-java-core/src/main/java/com/palantir/conjure/java/types/SafetyEvaluator.java
    pub fn is_safe_arg(&self, arg: &ArgumentDefinition) -> bool {
        if let Some(log_safety) = arg.safety() {
            return *log_safety == LogSafety::Safe;
        }

        if self.is_legacy_safe_arg(arg) {
            return true;
        }

        self.type_log_safety(arg.type_()) == Some(LogSafety::Safe)
    }

    fn is_legacy_safe_arg(&self, arg: &ArgumentDefinition) -> bool {
        arg.tags().iter().any(|s| s == "safe")
            || arg.markers().iter().any(|a| self.is_legacy_safe_marker(a))
    }

    fn is_legacy_safe_marker(&self, ty: &Type) -> bool {
        match ty {
            Type::External(def) => {
                let name = def.external_reference();
                name.package() == "com.palantir.logsafe" && name.name() == "Safe"
            }
            _ => false,
        }
    }

    fn type_log_safety(&self, ty: &Type) -> Option<LogSafety> {
        match ty {
            Type::Primitive(primitive) => self.primitive_log_safety(primitive),
            Type::Optional(optional) => self.type_log_safety(optional.item_type()),
            Type::List(list) => self.type_log_safety(list.item_type()),
            Type::Set(set) => self.type_log_safety(set.item_type()),
            Type::Map(map) => self.combine_safety(
                self.type_log_safety(map.key_type()),
                self.type_log_safety(map.value_type()),
            ),
            Type::Reference(def) => self.type_log_safety_ref(def),
            Type::External(_) => None,
        }
    }

    fn primitive_log_safety(&self, primitive: &PrimitiveType) -> Option<LogSafety> {
        match primitive {
            PrimitiveType::Bearertoken => Some(LogSafety::DoNotLog),
            _ => None,
        }
    }

    fn type_log_safety_ref(&self, name: &TypeName) -> Option<LogSafety> {
        let ctx = &self.types[name];

        if let CachedLogSafety::Computed(safety) = &*ctx.log_safety.borrow() {
            return safety.clone();
        }

        // temporarily treat it as safe in case of recursive type definitions.
        *ctx.log_safety.borrow_mut() = CachedLogSafety::Computed(Some(LogSafety::Safe));

        let safety = match &ctx.def {
            TypeDefinition::Alias(alias) => alias
                .safety()
                .cloned()
                .or_else(|| self.type_log_safety(alias.alias())),
            // We consider enums to be safe even when not compiled as exhaustive, since we assume
            // unknown variants are simply from a future definition.
            TypeDefinition::Enum(_) => Some(LogSafety::Safe),
            TypeDefinition::Object(object) => object
                .fields()
                .iter()
                .map(|f| {
                    f.safety()
                        .cloned()
                        .or_else(|| self.type_log_safety(f.type_()))
                })
                .try_fold(LogSafety::Safe, |a, b| self.combine_safety(Some(a), b)),
            TypeDefinition::Union(union_) => union_
                .union_()
                .iter()
                .map(|f| {
                    f.safety()
                        .cloned()
                        .or_else(|| self.type_log_safety(f.type_()))
                })
                // The unknown variant is unsafe to log, and we don't want log safety to vary based
                // on the type generation configuration. However, like conjure-java we're going to
                // treat the unknown variant as unannotated for now to ease the rollout.
                .fold(None, |a, b| self.combine_safety(a, b)),
        };

        *ctx.log_safety.borrow_mut() = CachedLogSafety::Computed(safety.clone());
        safety
    }

    fn combine_safety(&self, a: Option<LogSafety>, b: Option<LogSafety>) -> Option<LogSafety> {
        match (a, b) {
            (Some(LogSafety::DoNotLog), _) | (_, Some(LogSafety::DoNotLog)) => {
                Some(LogSafety::DoNotLog)
            }
            (Some(LogSafety::Unsafe), _) | (_, Some(LogSafety::Unsafe)) => Some(LogSafety::Unsafe),
            (Some(LogSafety::Safe), Some(LogSafety::Safe)) => Some(LogSafety::Safe),
            // nb: we notably do not combine safe + unknown to safe
            (Some(LogSafety::Safe), None) | (None, Some(LogSafety::Safe)) | (None, None) => None,
        }
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

pub enum BuilderConfig {
    Normal,
    Into,
    Custom {
        type_: TokenStream,
        convert: TokenStream,
    },
    List {
        item: BuilderItemConfig,
    },
    Set {
        item: BuilderItemConfig,
    },
    Map {
        key: BuilderItemConfig,
        value: BuilderItemConfig,
    },
}

pub enum BuilderItemConfig {
    Normal {
        type_: TokenStream,
    },
    Into {
        type_: TokenStream,
    },
    Custom {
        type_: TokenStream,
        convert: TokenStream,
    },
}
