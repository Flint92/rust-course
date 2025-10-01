use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod enum_from;

#[proc_macro_derive(EnumFrom)]
pub fn proc_macro_derive_enum_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    enum_from::process_enum_from(input)
}
