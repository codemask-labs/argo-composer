use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Deserialize)]
pub fn derive_yaml_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics YamlDeserializer for #name #ty_generics #where_clause {

        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Serialize)]
pub fn derive_yaml_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics YamlSerializer for #name #ty_generics #where_clause {

        }
    };

    TokenStream::from(expanded)
}
