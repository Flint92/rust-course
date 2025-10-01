use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn process_enum_from(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data,
    } = EnumFromDarling::from_derive_input(&input).unwrap();

    println!("ident: {:?}, generics: {:?}, data: {:?}", ident, generics, data);

    quote! {}.into()
}

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<EnumVariants, ()>,
}

#[derive(Debug, FromVariant)]
#[allow(unused)]
struct EnumVariants {
    ident: syn::Ident,
    fields: darling::ast::Fields<EnumVariantsFields>,
}

#[derive(Debug, FromField)]
#[allow(unused)]
struct EnumVariantsFields {
    ty: syn::Type,
}
