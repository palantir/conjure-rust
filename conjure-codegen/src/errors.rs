// Copyright 2019 Palantir Technologies, Inc.
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
use crate::objects;
use crate::types::{ErrorDefinition, ObjectDefinition};

pub fn generate(ctx: &Context, def: &ErrorDefinition) -> TokenStream {
    let object = ObjectDefinition::builder()
        .type_name(def.error_name().clone())
        .fields(def.safe_args().iter().chain(def.unsafe_args()).cloned())
        .docs(def.docs().cloned())
        .build();
    let object_def = objects::generate(ctx, &object);
    let error_type = generate_error_type(ctx, def);

    quote! {
        #object_def
        #error_type
    }
}

fn generate_error_type(ctx: &Context, def: &ErrorDefinition) -> TokenStream {
    let type_name = ctx.type_name(def.error_name().name());
    let code = ctx.type_name(def.code().as_str());
    let name = format!("{}:{}", def.namespace(), def.error_name().name());

    let mut safe_args = def
        .safe_args()
        .iter()
        .map(|f| &f.field_name().0)
        .collect::<Vec<_>>();
    safe_args.sort();

    quote! {
        impl conjure_error::ErrorType for #type_name {
            #[inline]
            fn code(&self) -> conjure_error::ErrorCode {
                conjure_error::ErrorCode::#code
            }

            #[inline]
            fn name(&self) -> &str {
                #name
            }

            #[inline]
            fn safe_args(&self) -> &'static [&'static str] {
                &[#(#safe_args,)*]
            }
        }
    }
}
