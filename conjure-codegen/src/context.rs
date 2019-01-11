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
use heck::{CamelCase, SnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::cell::Cell;
use std::collections::HashMap;

use crate::types::{
    ConjureDefinition, Documentation, PrimitiveType, Type, TypeDefinition, TypeName,
};

struct TypeContext {
    def: TypeDefinition,
    has_double: Cell<Option<bool>>,
    is_copy: Cell<Option<bool>>,
}

pub struct Context {
    types: HashMap<TypeName, TypeContext>,
    exhaustive: bool,
}

impl Context {
    pub fn new(defs: &ConjureDefinition, exhaustive: bool) -> Context {
        let mut context = Context {
            types: HashMap::new(),
            exhaustive,
        };

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
                },
            );
        }

        context
    }

    pub fn exhaustive(&self) -> bool {
        self.exhaustive
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
                PrimitiveType::DOUBLE => true,
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
                PrimitiveType::STRING
                | PrimitiveType::BINARY
                | PrimitiveType::ANY
                | PrimitiveType::RID
                | PrimitiveType::BEARERTOKEN => false,
                PrimitiveType::DATETIME
                | PrimitiveType::INTEGER
                | PrimitiveType::DOUBLE
                | PrimitiveType::SAFELONG
                | PrimitiveType::BOOLEAN
                | PrimitiveType::UUID => true,
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
                PrimitiveType::STRING
                | PrimitiveType::INTEGER
                | PrimitiveType::DOUBLE
                | PrimitiveType::SAFELONG
                | PrimitiveType::BINARY
                | PrimitiveType::BOOLEAN => true,
                PrimitiveType::DATETIME
                | PrimitiveType::ANY
                | PrimitiveType::UUID
                | PrimitiveType::RID
                | PrimitiveType::BEARERTOKEN => false,
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

    pub fn rust_type(&self, this_type: &TypeName, def: &Type) -> TokenStream {
        match def {
            Type::Primitive(def) => match *def {
                PrimitiveType::STRING => self.string_ident(this_type),
                PrimitiveType::DATETIME => quote!(conjure_types::DateTime<conjure_types::Utc>),
                PrimitiveType::INTEGER => quote!(i32),
                PrimitiveType::DOUBLE => quote!(f64),
                PrimitiveType::SAFELONG => quote!(conjure_types::SafeLong),
                PrimitiveType::BINARY => quote!(conjure_types::ByteBuf),
                PrimitiveType::ANY => quote!(conjure_types::Value),
                PrimitiveType::BOOLEAN => quote!(bool),
                PrimitiveType::UUID => quote!(conjure_types::Uuid),
                PrimitiveType::RID => quote!(conjure_types::ResourceIdentifier),
                PrimitiveType::BEARERTOKEN => quote!(conjure_types::BearerToken),
            },
            Type::Optional(def) => {
                let option = self.option_ident(this_type);
                let item = self.rust_type(this_type, def.item_type());
                quote!(#option<#item>)
            }
            Type::List(def) => {
                let vec = self.vec_ident(this_type);
                let item = self.rust_type(this_type, def.item_type());
                quote!(#vec<#item>)
            }
            Type::Set(def) => {
                let item = self.rust_type(this_type, def.item_type());
                quote!(std::collections::BTreeSet<#item>)
            }
            Type::Map(def) => {
                let key = self.rust_type(this_type, def.key_type());
                let value = self.rust_type(this_type, def.value_type());
                quote!(std::collections::BTreeMap<#key, #value>)
            }
            Type::Reference(def) => {
                let name = self.type_name(def.name());
                quote!(super::#name)
            }
            Type::External(def) => self.rust_type(this_type, def.fallback()),
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

        let unboxed = self.type_name(name.name());
        if needs_box {
            let box_ = self.box_ident(name);
            quote!(#box_<super::#unboxed>)
        } else {
            quote!(super::#unboxed)
        }
    }

    pub fn borrowed_rust_type(&self, this_type: &TypeName, def: &Type) -> TokenStream {
        match def {
            Type::Primitive(def) => match *def {
                PrimitiveType::STRING => quote!(&str),
                PrimitiveType::DATETIME => quote!(conjure_types::DateTime<conjure_types::Utc>),
                PrimitiveType::INTEGER => quote!(i32),
                PrimitiveType::DOUBLE => quote!(f64),
                PrimitiveType::SAFELONG => quote!(conjure_types::SafeLong),
                PrimitiveType::BINARY => quote!(&[u8]),
                PrimitiveType::ANY => quote!(&conjure_types::Value),
                PrimitiveType::BOOLEAN => quote!(bool),
                PrimitiveType::UUID => quote!(conjure_types::Uuid),
                PrimitiveType::RID => quote!(&conjure_types::ResourceIdentifier),
                PrimitiveType::BEARERTOKEN => quote!(&conjure_types::BearerToken),
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
                let item = self.rust_type(this_type, def.item_type());
                quote!(&std::collections::BTreeSet<#item>)
            }
            Type::Map(def) => {
                let key = self.rust_type(this_type, def.key_type());
                let value = self.rust_type(this_type, def.value_type());
                quote!(&std::collections::BTreeMap<#key, #value>)
            }
            Type::Reference(def) => self.borrowed_rust_type_ref(def),
            Type::External(def) => self.borrowed_rust_type(this_type, def.fallback()),
        }
    }

    fn borrowed_rust_type_ref(&self, name: &TypeName) -> TokenStream {
        let ctx = &self.types[name];

        let type_ = self.type_name(name.name());
        let type_ = quote!(super::#type_);
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
                PrimitiveType::STRING | PrimitiveType::BINARY => quote!(&*#value),
                PrimitiveType::ANY | PrimitiveType::RID | PrimitiveType::BEARERTOKEN => {
                    quote!(&#value)
                }
                PrimitiveType::DATETIME
                | PrimitiveType::INTEGER
                | PrimitiveType::DOUBLE
                | PrimitiveType::SAFELONG
                | PrimitiveType::BOOLEAN
                | PrimitiveType::UUID => value,
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

    pub fn setter_bounds(
        &self,
        this_type: &TypeName,
        def: &Type,
        value_ident: TokenStream,
    ) -> SetterBounds {
        match def {
            Type::Primitive(primitive) => {
                let some = self.some_ident(this_type);
                match *primitive {
                    PrimitiveType::STRING => {
                        let into = self.into_ident(this_type);
                        let string = self.string_ident(this_type);
                        SetterBounds::Generic {
                            argument_bound: quote!(#into<#string>),
                            assign_rhs: quote!(#some(#value_ident.into())),
                        }
                    }
                    PrimitiveType::BINARY => {
                        let into = self.into_ident(this_type);
                        let vec = self.vec_ident(this_type);
                        SetterBounds::Generic {
                            argument_bound: quote!(#into<#vec<u8>>),
                            assign_rhs: quote!(#some(#value_ident.into().into())),
                        }
                    }
                    PrimitiveType::ANY => SetterBounds::Generic {
                        argument_bound: quote!(conjure_types::serde::Serialize),
                        assign_rhs: quote! {
                            #some(conjure_types::serde_value::to_value(#value_ident).expect("value failed to serialize"))
                        },
                    },
                    _ => SetterBounds::Simple {
                        argument_type: self.rust_type(this_type, def),
                        assign_rhs: quote!(#some(#value_ident)),
                    },
                }
            }
            Type::Optional(def) => {
                let into = self.into_ident(this_type);
                let option = self.option_ident(this_type);
                let item_type = self.rust_type(this_type, def.item_type());
                let assign_rhs = if self.needs_box(def.item_type()) {
                    let box_ = self.box_ident(this_type);
                    quote!(#value_ident.into().map(#box_::new))
                } else {
                    quote!(#value_ident.into())
                };

                SetterBounds::Generic {
                    argument_bound: quote!(#into<#option<#item_type>>),
                    assign_rhs,
                }
            }
            Type::List(def) => {
                let into_iterator = self.into_iterator_ident(this_type);
                let item_type = self.rust_type(this_type, def.item_type());
                SetterBounds::Collection {
                    argument_bound: quote!(#into_iterator<Item = #item_type>),
                }
            }
            Type::Set(def) => {
                let into_iterator = self.into_iterator_ident(this_type);
                let item_type = self.rust_type(this_type, def.item_type());
                SetterBounds::Collection {
                    argument_bound: quote!(#into_iterator<Item = #item_type>),
                }
            }
            Type::Map(def) => {
                let into_iterator = self.into_iterator_ident(this_type);
                let key_type = self.rust_type(this_type, def.key_type());
                let value_type = self.rust_type(this_type, def.value_type());
                SetterBounds::Collection {
                    argument_bound: quote!(#into_iterator<Item = (#key_type, #value_type)>),
                }
            }
            Type::Reference(def) => {
                let type_ = self.type_name(def.name());
                let mut assign_rhs = value_ident;
                if self.ref_needs_box(def) {
                    let box_ = self.box_ident(this_type);
                    assign_rhs = quote!(#box_::new(#assign_rhs));
                }
                if self.ref_is_required(def) {
                    let some = self.some_ident(this_type);
                    assign_rhs = quote!(#some(#assign_rhs))
                }

                SetterBounds::Simple {
                    argument_type: quote!(super::#type_),
                    assign_rhs,
                }
            }
            Type::External(def) => self.setter_bounds(this_type, def.fallback(), value_ident),
        }
    }

    pub fn is_empty_method(&self, def: &Type) -> Option<TokenStream> {
        match def {
            Type::Primitive(_) => None,
            Type::Optional(_) => Some(quote!(is_none)),
            Type::List(_) | Type::Set(_) | Type::Map(_) => Some(quote!(is_empty)),
            Type::Reference(def) => self.is_empty_method_ref(def),
            Type::External(def) => self.is_empty_method(def.fallback()),
        }
    }

    fn is_empty_method_ref(&self, name: &TypeName) -> Option<TokenStream> {
        let ctx = &self.types[name];

        match &ctx.def {
            TypeDefinition::Alias(def) => self.is_empty_method(def.alias()),
            TypeDefinition::Enum(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => None,
        }
    }

    pub fn docs(&self, docs: Option<&Documentation>) -> TokenStream {
        match docs {
            Some(docs) => {
                let docs = &**docs;
                quote!(#[doc = #docs])
            }
            None => TokenStream::new(),
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
    pub fn from_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "From", "std::convert::From")
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Into", "std::convert::Into")
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_iterator_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "IntoIterator", "std::iter::IntoIterator")
    }

    pub fn default_ident(&self, name: &TypeName) -> TokenStream {
        self.prelude_ident(name, "Default", "std::default::Default")
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
            "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv"
            | "typeof" | "unsized" | "virtual" | "yield" => true,
            // weak keywords
            "union" | "dyn" => true,
            _ => false,
        };

        if keyword {
            s.push('_');
        }

        s
    }

    pub fn type_name(&self, name: &str) -> Ident {
        let mut name = name.to_camel_case();

        let keyword = match &*name {
            "Self" => true,
            _ => false,
        };

        if keyword {
            name.push('_');
        }

        Ident::new(&name, Span::call_site())
    }
}

pub enum SetterBounds {
    Simple {
        argument_type: TokenStream,
        assign_rhs: TokenStream,
    },
    Generic {
        argument_bound: TokenStream,
        assign_rhs: TokenStream,
    },
    Collection {
        argument_bound: TokenStream,
    },
}
