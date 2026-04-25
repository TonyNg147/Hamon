use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AllowStep, attributes(from))]
pub fn derive_from_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // 1. Look for #[from(any)] or #[from(SomeState)]
    // This is a simplified version of attribute parsing
    let attr = input.attrs.iter().find(|a| a.path().is_ident("from"));

    if let Some(attr) = attr {
        let meta = attr.parse_args::<syn::Ident>().unwrap();

        if meta == "any" {
            // If it's "any", implement the universal marker
            quote! {
                impl ::hamon::builder::AnyStep for #name {}
            }
        } else {
            // Otherwise, implement the specific transition
            quote! {
                // The permit needed to form the bridge for next step.
                impl ::hamon::utils::FromStep<#meta> for #name {}
            }
        }
    } else {
        quote! {} // No attribute, no permission generated
    }
    .into()
}
