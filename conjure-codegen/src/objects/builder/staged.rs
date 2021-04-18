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
use crate::context::{CollectionSetterBounds, Context, SetterBounds};
use crate::objects;
use crate::types::{FieldDefinition, ObjectDefinition};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashSet;

pub fn generate(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());
    let docs = format!("A builder for the `{}` type.", name);
    let builder_type = objects::builder_type(ctx, def);
    let stage0 = stage_name(ctx, def, 0);

    let traits = generate_traits(ctx, def);

    quote! {
        #[doc = #docs]
        #[derive(Debug, Clone)]
        pub struct #builder_type<S>(S);

        impl Default for #builder_type<#stage0> {
            #[inline]
            fn default() -> Self {
                #builder_type(#stage0::new())
            }
        }

        #traits
    }
}

fn stage_name(ctx: &Context, def: &ObjectDefinition, stage: usize) -> Ident {
    let mut name = format!("Stage{}", stage);
    if ctx.type_name(def.type_name().name()) == name {
        name.push('_');
    }

    Ident::new(&name, Span::call_site())
}

fn trait_name(ctx: &Context, def: &ObjectDefinition, field: &FieldDefinition) -> Ident {
    let name = ctx.type_name(field.field_name());
    let mut name = format!("Set{}", name);
    if ctx.type_name(def.type_name().name()) == name {
        name.push('_');
    }

    Ident::new(&name, Span::call_site())
}

fn generate_traits(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let field_names = def
        .fields()
        .iter()
        .map(|f| ctx.field_name(f.field_name()).to_string())
        .collect();

    let traits = def
        .fields()
        .iter()
        .filter(|f| ctx.is_required(f.type_()))
        .map(|f| generate_trait(ctx, def, f, &field_names));

    quote! {
        #(#traits)*
    }
}

fn generate_trait(
    ctx: &Context,
    def: &ObjectDefinition,
    field: &FieldDefinition,
    field_names: &HashSet<String>,
) -> TokenStream {
    panic!()
}
