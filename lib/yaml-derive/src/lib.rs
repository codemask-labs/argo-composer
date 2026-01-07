use proc_macro::TokenStream;
use syn::parse_macro_input;

mod codegen;
mod utils;

use codegen::{generate_deserialize_impl, generate_serialize_impl};

#[proc_macro_derive(Deserialize, attributes(yaml))]
pub fn derive_yaml_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let expanded = generate_deserialize_impl(&input);
    TokenStream::from(expanded)
}

#[proc_macro_derive(Serialize, attributes(yaml))]
pub fn derive_yaml_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let expanded = generate_serialize_impl(&input);
    TokenStream::from(expanded)
}
