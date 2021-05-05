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
use crate::context::Context;
use crate::objects;
use crate::objects::builder::{self, SetterOp};
use crate::types::{FieldDefinition, ObjectDefinition};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashSet;

pub fn generate(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let name = ctx.type_name(def.type_name().name());
    let docs = format!("A builder for the `{}` type.", name);
    let builder_type = objects::builder_type(ctx, def);

    let field_names = builder::field_names(ctx, def);

    let default_impl = generate_default_impl(ctx, def);
    let from_impl = generate_from_impl(ctx, def);
    let builder_impls = generate_builder_impls(ctx, def, &field_names);
    let traits = generate_traits(ctx, def, &field_names);
    let stages = generate_stages(ctx, def, &field_names);

    quote! {
        #[doc = #docs]
        #[derive(Debug, Clone)]
        pub struct #builder_type<S>(S);

        #default_impl
        #from_impl
        #builder_impls
        #traits
        #stages
    }
}

fn trait_name(ctx: &Context, def: &ObjectDefinition, field: &FieldDefinition) -> Ident {
    let name = ctx.type_name(field.field_name());
    let mut name = format!("Set{}", name);
    if ctx.type_name(def.type_name().name()) == name {
        name.push('_');
    }

    Ident::new(&name, Span::call_site())
}

fn generate_default_impl(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let default = ctx.default_ident(def.type_name());
    let builder = objects::builder_type(ctx, def);
    let stage0 = objects::stage_name(ctx, def, 0);

    let has_required_fields = def.fields().iter().any(|f| ctx.is_required(f.type_()));

    let inits = if has_required_fields {
        vec![]
    } else {
        def.fields()
            .iter()
            .filter(|f| !ctx.is_required(f.type_()))
            .map(|f| {
                let name = ctx.field_name(f.field_name());
                quote!(#name: #default::default())
            })
            .collect()
    };

    quote! {
        impl #default for #builder<#stage0> {
            #[inline]
            fn default() -> Self {
                #builder(#stage0 {
                    #(#inits,)*
                })
            }
        }
    }
}

fn generate_from_impl(ctx: &Context, def: &ObjectDefinition) -> TokenStream {
    let from = ctx.from_ident(def.type_name());
    let builder = objects::builder_type(ctx, def);

    let stage = def
        .fields()
        .iter()
        .filter(|f| ctx.is_required(f.type_()))
        .count();
    let stage = objects::stage_name(ctx, def, stage);

    let type_ = ctx.type_name(def.type_name().name());

    let fields = def.fields().iter().map(|f| {
        let name = ctx.field_name(f.field_name());
        quote!(#name: value.#name)
    });

    let value = if def.fields().is_empty() {
        quote!(_)
    } else {
        quote!(value)
    };

    quote! {
        impl #from<#type_> for #builder<#stage> {
            #[inline]
            fn from(#value: #type_) -> Self {
                #builder(#stage {
                    #(#fields,)*
                })
            }
        }
    }
}

fn generate_builder_impls(
    ctx: &Context,
    def: &ObjectDefinition,
    field_names: &HashSet<String>,
) -> TokenStream {
    let required_impls = def
        .fields()
        .iter()
        .filter(|f| ctx.is_required(f.type_()))
        .map(|f| generate_required_impl(ctx, def, f, field_names));

    let final_impl = generate_final_impl(ctx, def, field_names);

    quote! {
        #(#required_impls)*
        #final_impl
    }
}

fn generate_required_impl(
    ctx: &Context,
    def: &ObjectDefinition,
    field: &FieldDefinition,
    field_names: &HashSet<String>,
) -> TokenStream {
    let builder = objects::builder_type(ctx, def);
    let trait_name = trait_name(ctx, def, field);

    let docs = ctx.docs(field.docs());
    let deprecated = ctx.deprecated(field.deprecated());

    let setters = builder::field_setters(ctx, def, field, field_names)
        .into_iter()
        .map(|setter| {
            let args = setter.args.iter().map(|arg| {
                let name = &arg.name;
                let type_ = &arg.type_;
                quote!(#name: #type_)
            });

            let method = setter.name;
            let params = setter.params;
            let where_ = setter.where_;

            let call_args = setter.args.iter().map(|arg| &arg.name);

            quote! {
                #docs
                #deprecated
                #[inline]
                pub fn #method #params(self, #(#args),*) -> #builder<S::Stage> #where_ {
                    #builder(self.0.#method(#(#call_args),*))
                }
            }
        });

    quote! {
        impl<S> #builder<S>
        where
            S: #trait_name,
        {
            #(#setters)*
        }
    }
}

fn generate_final_impl(
    ctx: &Context,
    def: &ObjectDefinition,
    field_names: &HashSet<String>,
) -> TokenStream {
    let builder = objects::builder_type(ctx, def);
    let stage = def
        .fields()
        .iter()
        .filter(|f| ctx.is_required(f.type_()))
        .count();
    let stage = objects::stage_name(ctx, def, stage);

    let setters = def
        .fields()
        .iter()
        .filter(|f| !ctx.is_required(f.type_()))
        .flat_map(|field| {
            let docs = ctx.docs(field.docs());
            let deprecated = ctx.deprecated(field.deprecated());

            builder::field_setters(ctx, def, field, field_names)
                .into_iter()
                .map(|setter| {
                    let args = setter.args.iter().map(|arg| {
                        let name = &arg.name;
                        let type_ = &arg.type_;
                        quote!(#name: #type_)
                    });

                    let field = ctx.field_name(field.field_name());
                    let update = match setter.op {
                        SetterOp::Assign { rhs } => quote!(self.0.#field = #rhs),
                        SetterOp::Call { call } => quote!(self.0.#field.#call),
                    };

                    let method = setter.name;
                    let params = setter.params;
                    let where_ = setter.where_;

                    quote! {
                        #docs
                        #deprecated
                        #[inline]
                        pub fn #method #params(mut self, #(#args),*) -> Self #where_ {
                            #update;
                            self
                        }
                    }
                })
                .collect::<Vec<_>>()
        });

    let build_method = if objects::fields(ctx, def).iter().any(|f| f == "build") {
        quote!(build_)
    } else {
        quote!(build)
    };

    let object = ctx.type_name(def.type_name().name());

    let build_fields = def.fields().iter().map(|f| {
        let name = ctx.field_name(f.field_name());
        quote!(#name: self.0.#name)
    });

    quote! {
        impl #builder<#stage> {
            #(#setters)*

            /// Consumes the builder, constructing a new instance of the type.
            #[inline]
            pub fn #build_method(self) -> #object {
                #object {
                    #(#build_fields,)*
                }
            }
        }
    }
}

fn generate_traits(
    ctx: &Context,
    def: &ObjectDefinition,
    field_names: &HashSet<String>,
) -> TokenStream {
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
    let trait_name = trait_name(ctx, def, field);

    let setters = builder::field_setters(ctx, def, field, field_names)
        .into_iter()
        .map(|setter| {
            let args = setter.args.iter().map(|arg| {
                let name = &arg.name;
                let type_ = &arg.type_;
                quote!(#name: #type_)
            });

            let method = setter.name;
            let params = setter.params;
            let where_ = setter.where_;

            quote! {
                fn #method #params(self, #(#args),*) -> Self::Stage #where_;
            }
        });

    quote! {
        pub trait #trait_name {
            type Stage;

            #(#setters)*
        }
    }
}

fn generate_stages(
    ctx: &Context,
    def: &ObjectDefinition,
    field_names: &HashSet<String>,
) -> TokenStream {
    let (required_fields, optional_fields) = def
        .fields()
        .iter()
        .partition::<Vec<_>, _>(|f| ctx.is_required(f.type_()));

    let stages = (0..=required_fields.len()).map(|idx| {
        generate_stage(
            ctx,
            def,
            field_names,
            &required_fields,
            &optional_fields,
            idx,
        )
    });

    quote!(#(#stages)*)
}

fn generate_stage(
    ctx: &Context,
    def: &ObjectDefinition,
    field_names: &HashSet<String>,
    required_fields: &[&FieldDefinition],
    optional_fields: &[&FieldDefinition],
    stage: usize,
) -> TokenStream {
    let name = objects::stage_name(ctx, def, stage);

    let final_stage = stage == required_fields.len();
    let included_required_fields = &required_fields[..stage];
    let included_optional_fields = if final_stage { optional_fields } else { &[] };

    let fields = included_required_fields
        .iter()
        .chain(included_optional_fields)
        .map(|f| {
            let name = ctx.field_name(f.field_name());
            let type_ = ctx.boxed_rust_type(def.type_name(), f.type_());
            quote!(#name: #type_)
        });

    let impls = if final_stage {
        included_required_fields
            .iter()
            .map(|f| generate_in_place_stage_impl(ctx, def, field_names, f, stage))
            .collect()
    } else {
        let new_optional_fields = if stage + 1 == required_fields.len() {
            optional_fields
        } else {
            &[]
        };
        generate_next_stage_impl(
            ctx,
            def,
            field_names,
            included_required_fields,
            &required_fields[stage],
            new_optional_fields,
            stage,
        )
    };

    quote! {
        #[derive(Debug, Clone)]
        pub struct #name {
            #(#fields,)*
        }

        #impls
    }
}

fn generate_next_stage_impl(
    ctx: &Context,
    def: &ObjectDefinition,
    field_names: &HashSet<String>,
    existing_fields: &[&FieldDefinition],
    field: &FieldDefinition,
    optional_fields: &[&FieldDefinition],
    stage: usize,
) -> TokenStream {
    let trait_name = trait_name(ctx, def, field);
    let stage_name = objects::stage_name(ctx, def, stage);
    let new_stage_name = objects::stage_name(ctx, def, stage + 1);

    let setters = builder::field_setters(ctx, def, field, field_names)
        .into_iter()
        .map(|setter| {
            let existing_inits = existing_fields.iter().map(|f| {
                let name = ctx.field_name(f.field_name());
                quote!(#name: self.#name)
            });

            let field_name = ctx.field_name(field.field_name());
            let field_init = match &setter.op {
                SetterOp::Assign { rhs } => quote!(#field_name: #rhs),
                SetterOp::Call { .. } => unreachable!("required fields use assign"),
            };

            let default = ctx.default_ident(def.type_name());
            let optional_inits = optional_fields.iter().map(|f| {
                let name = ctx.field_name(f.field_name());
                quote!(#name: #default::default())
            });

            let body = quote! {
                #new_stage_name {
                    #(#existing_inits,)*
                    #field_init,
                    #(#optional_inits,)*
                }
            };

            let args = setter.args.iter().map(|arg| {
                let name = &arg.name;
                let type_ = &arg.type_;
                quote!(#name: #type_)
            });

            let method = setter.name;
            let params = setter.params;
            let where_ = setter.where_;

            quote! {
                #[inline]
                fn #method #params(self, #(#args),*) -> Self::Stage #where_ {
                    #body
                }
            }
        });

    quote! {
        impl #trait_name for #stage_name {
            type Stage = #new_stage_name;

            #(#setters)*
        }
    }
}

fn generate_in_place_stage_impl(
    ctx: &Context,
    def: &ObjectDefinition,
    field_names: &HashSet<String>,
    field: &FieldDefinition,
    stage: usize,
) -> TokenStream {
    let trait_name = trait_name(ctx, def, field);
    let stage_name = objects::stage_name(ctx, def, stage);
    let field_name = ctx.field_name(field.field_name());

    let setters = builder::field_setters(ctx, def, field, field_names)
        .into_iter()
        .map(|setter| {
            let args = setter.args.iter().map(|arg| {
                let name = &arg.name;
                let type_ = &arg.type_;
                quote!(#name: #type_)
            });

            let rhs = match setter.op {
                SetterOp::Assign { rhs } => rhs,
                SetterOp::Call { .. } => unreachable!("required fields use assign"),
            };

            let method = setter.name;
            let params = setter.params;
            let where_ = setter.where_;

            quote! {
                #[inline]
                fn #method #params(mut self, #(#args),*) -> Self::Stage #where_ {
                    self.#field_name = #rhs;
                    self
                }
            }
        });

    quote! {
        impl #trait_name for #stage_name {
            type Stage = Self;

            #(#setters)*
        }
    }
}
