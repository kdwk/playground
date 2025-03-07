use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Object)]
pub fn derive_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl Object for #name {
            fn class_name(&self) -> String {
               stringify!(#name).to_string()
            }
        }
    };
    expanded.into()
}

#[proc_macro_derive(Enum)]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let syn_item: DeriveInput = syn::parse(input).unwrap();
    let variants = match syn_item.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|variant| variant.ident),
        _ => panic!("This object is not an enum"),
    };
    let enum_name = syn_item.ident;
    let expanded = quote! {
        impl Object for #enum_name {
            fn class_name(&self) -> String {
                stringify!(#enum_name).to_string()
            }
        }
        impl Enum for #enum_name {
            fn variants() -> Vec<Self> {
                vec![#(#enum_name::#variants),*]
            }
        }
    };
    expanded.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
