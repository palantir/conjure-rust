use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, ItemTrait};

pub fn generate(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item = parse_macro_input!(item as ItemTrait);

    quote! {
        #item
    }
    .into()
}

struct Service {
    name: Ident,
    endpoints: Vec<Endpoint>,
}

struct Endpoint {
    name: Ident,
    args: Vec<Arg>,
}

enum Arg {
    Receiver,
}
