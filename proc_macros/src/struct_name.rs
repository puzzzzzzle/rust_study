//! 对一个struct添加get_name实现

use proc_macro;
use quote;
use syn;

pub fn struct_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = input.ident;
    let expanded = quote::quote! {
        impl StructName for #name {
            fn struct_name() -> String {
                stringify!(#name).to_string()
           }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
