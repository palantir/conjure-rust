// Copyright 2026 Palantir Technologies, Inc.
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

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, Type,
};

pub fn generate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let field_types = collect_field_types(&input.data);

    let mut generics = input.generics.clone();
    {
        let where_clause = generics.make_where_clause();
        for ty in &field_types {
            where_clause
                .predicates
                .push(parse_quote!(#ty: conjure_object::log_safety::LogSafe));
        }
    }
    let (ig, tg, wc) = generics.split_for_impl();

    quote! {
        impl #ig conjure_object::log_safety::LogSafe for #name #tg #wc {}
    }
    .into()
}

fn collect_field_types(data: &Data) -> Vec<Type> {
    match data {
        Data::Struct(DataStruct { fields, .. }) => fields_to_types(fields),
        Data::Enum(DataEnum { variants, .. }) => variants
            .iter()
            .flat_map(|v| fields_to_types(&v.fields))
            .collect(),
        Data::Union(_) => Vec::new(),
    }
}

fn fields_to_types(fields: &Fields) -> Vec<Type> {
    fields
        .iter()
        .filter(|f| !has_assert_is_safe(&f.attrs))
        .map(|f: &Field| f.ty.clone())
        .collect()
}

fn has_assert_is_safe(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.path().is_ident("assert_is_safe"))
}
