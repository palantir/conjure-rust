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
use crate::context::Context;
use crate::types::ObjectDefinition;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

mod builder;
mod deserialize;
mod object;
mod serialize;

pub fn generate(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let object = object::generate(ctx, def);
    let builder = builder::generate(ctx, def);
    let serialize = serialize::generate(ctx, def);
    let deserialize = deserialize::generate(ctx, def);

    quote! {
        use conjure_object::serde::{ser, de};
        use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
        use std::fmt;

        #object
        #builder
        #serialize
        #deserialize
    }
}

fn fields(ctx: &Context, def: &ObjectDefinition) -> Vec<Ident> {
    def.fields()
        .iter()
        .map(|f| ctx.field_name(f.field_name()))
        .collect()
}

fn stage_name(ctx: &Context, def: &ObjectDefinition, stage: usize) -> Ident {
    let mut name = format!("BuilderStage{}", stage);
    if ctx.type_name(def.type_name().name()) == name {
        name.push('_');
    }

    Ident::new(&name, Span::call_site())
}

fn builder_type(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    if ctx.type_name(def.type_name().name()) == "Builder" {
        quote!(Builder_)
    } else {
        quote!(Builder)
    }
}
