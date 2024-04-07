use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(NotInScope)]
pub fn generate_not_in_scope(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    quote! {
        pub trait NotInScope {
            fn not_in_scope(&self);
        }

        impl NotInScope for #name {
            fn not_in_scope(&self) {
                println!("Hello from NotInScope");
            }
        }

        common::create_wrapper!();
    }
    .into()
}
