use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;
use crate::context::Context;
use crate::types::ConstantDefinition;

pub fn generate_constants(ctx: &Context, defs: &Vec<&ConstantDefinition>) -> TokenStream {
    let constants: Vec<TokenStream> = defs.iter().map(|def| generate(ctx, def)).collect();
    quote! {
        #(#constants)*
    }
}

pub fn generate(ctx: &Context, def: &ConstantDefinition) -> TokenStream {
    let const_name = ctx.constant_name(def.type_name().name());
    let const_type = ctx.primitive_rust_type(def.type_name(), def.const_type(), false);
    let const_value = ctx.parse_const_value(def.const_type(), def.value());
    let constant_str = format!("pub const {}: {} = {};", const_name, const_type, const_value);

    return parse_str(&constant_str).unwrap();
}