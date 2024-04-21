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
use crate::context::{CollectionSetterBounds, CollectionType, Context, SetterBounds};
use crate::types::{FieldDefinition, ObjectDefinition};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::HashSet;

mod staged;

pub fn generate(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    staged::generate(ctx, def)
}

fn field_names(ctx: &Context, def: &ObjectDefinition) -> HashSet<String> {
    def.fields()
        .iter()
        .map(|f| ctx.field_name(f.field_name()).to_string())
        .collect()
}

struct Setter {
    name: Ident,
    params: TokenStream,
    args: Vec<SetterArg>,
    where_: TokenStream,
    op: SetterOp,
}

struct SetterArg {
    name: TokenStream,
    type_: TokenStream,
}

enum SetterOp {
    Assign { rhs: TokenStream },
    Call { call: TokenStream },
}

fn field_setters(
    ctx: &Context,
    def: &ObjectDefinition,
    field: &FieldDefinition,
    field_names: &HashSet<String>,
) -> Vec<Setter> {
    let name = ctx.field_name(field.field_name());

    match ctx.setter_bounds(def.type_name(), field.type_(), quote!(#name)) {
        SetterBounds::Simple {
            argument_type,
            assign_rhs,
        } => {
            vec![Setter {
                params: quote!(),
                args: vec![SetterArg {
                    name: quote!(#name),
                    type_: argument_type,
                }],
                where_: quote!(),
                op: SetterOp::Assign { rhs: assign_rhs },
                name,
            }]
        }
        SetterBounds::Generic {
            argument_bound,
            assign_rhs,
        } => {
            vec![Setter {
                params: quote!(<T>),
                args: vec![SetterArg {
                    name: quote!(#name),
                    type_: quote!(T),
                }],
                where_: quote!(where T: #argument_bound),
                op: SetterOp::Assign { rhs: assign_rhs },
                name,
            }]
        }
        SetterBounds::Collection {
            argument_bound,
            type_,
        } => {
            let mut extend_name = format!("extend_{}", name);
            if field_names.contains(&extend_name) {
                extend_name.push('_');
            }
            let extend_name = Ident::new(&extend_name, name.span());

            vec![
                Setter {
                    name: name.clone(),
                    params: quote!(<T>),
                    args: vec![SetterArg {
                        name: quote!(#name),
                        type_: quote!(T),
                    }],
                    where_: quote!(where T: #argument_bound),
                    op: SetterOp::Assign {
                        rhs: quote!(#name.into_iter().collect()),
                    },
                },
                Setter {
                    name: extend_name,
                    params: quote!(<T>),
                    args: vec![SetterArg {
                        name: quote!(#name),
                        type_: quote!(T),
                    }],
                    where_: quote!(where T: #argument_bound),
                    op: SetterOp::Call {
                        call: quote!(extend(#name)),
                    },
                },
                single_setter(field_names, &name, type_),
            ]
        }
    }
}

fn single_setter(field_names: &HashSet<String>, name: &Ident, type_: CollectionType) -> Setter {
    match type_ {
        CollectionType::List { value } => collection_push(field_names, name, value, quote!(push)),
        CollectionType::Set { value } => collection_push(field_names, name, value, quote!(insert)),
        CollectionType::Map { key, value } => {
            let mut single_name = format!("insert_{}", name);
            if field_names.contains(&single_name) {
                single_name.push('_');
            }
            let single_name = Ident::new(&single_name, name.span());

            let mut params = vec![];
            let mut wheres = vec![];

            let (key_type, key_assign_rhs) = match key {
                CollectionSetterBounds::Simple {
                    argument_type,
                    assign_rhs,
                } => (argument_type, assign_rhs),
                CollectionSetterBounds::Generic {
                    argument_bound,
                    assign_rhs,
                } => {
                    params.push(quote!(K));
                    wheres.push(quote!(K: #argument_bound));
                    (quote!(K), assign_rhs)
                }
            };

            let (value_type, value_assign_rhs) = match value {
                CollectionSetterBounds::Simple {
                    argument_type,
                    assign_rhs,
                } => (argument_type, assign_rhs),
                CollectionSetterBounds::Generic {
                    argument_bound,
                    assign_rhs,
                } => {
                    params.push(quote!(V));
                    wheres.push(quote!(V: #argument_bound));
                    (quote!(V), assign_rhs)
                }
            };

            let params = if params.is_empty() {
                quote!()
            } else {
                quote!(<#(#params),*>)
            };
            let where_ = if wheres.is_empty() {
                quote!()
            } else {
                quote!(where #(#wheres),*)
            };

            Setter {
                name: single_name,
                params,
                args: vec![
                    SetterArg {
                        name: quote!(key),
                        type_: key_type,
                    },
                    SetterArg {
                        name: quote!(value),
                        type_: value_type,
                    },
                ],
                where_,
                op: SetterOp::Call {
                    call: quote!(insert(#key_assign_rhs, #value_assign_rhs)),
                },
            }
        }
    }
}

fn collection_push(
    field_names: &HashSet<String>,
    name: &Ident,
    bounds: CollectionSetterBounds,
    op: TokenStream,
) -> Setter {
    let mut single_name = format!("{}_{}", op, name);
    if field_names.contains(&single_name) {
        single_name.push('_');
    }
    let single_name = Ident::new(&single_name, name.span());
    let (params, type_, where_, assign_rhs) = match bounds {
        CollectionSetterBounds::Simple {
            argument_type,
            assign_rhs,
        } => (quote!(), argument_type, quote!(), assign_rhs),
        CollectionSetterBounds::Generic {
            argument_bound,
            assign_rhs,
        } => (
            quote!(<T>),
            quote!(T),
            quote!(where T: #argument_bound),
            assign_rhs,
        ),
    };

    Setter {
        name: single_name,
        params,
        args: vec![SetterArg {
            name: quote!(value),
            type_,
        }],
        where_,
        op: SetterOp::Call {
            call: quote!(
                #op(#assign_rhs)
            ),
        },
    }
}
