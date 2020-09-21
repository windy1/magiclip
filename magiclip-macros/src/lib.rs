extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput, Ident};

#[proc_macro_derive(FromRaw)]
pub fn from_raw_macro_derive(input: TokenStream) -> TokenStream {
    impl_from_raw(&syn::parse(input).unwrap())
}

fn impl_from_raw(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let gen = quote! {
        impl #generics crate::ffi::FromRaw<#name #generics> for #name #generics {}
    };

    gen.into()
}

#[proc_macro_derive(CloneRaw)]
pub fn clone_raw_macro_derive(input: TokenStream) -> TokenStream {
    impl_clone_raw(&syn::parse(input).unwrap())
}

fn impl_clone_raw(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let gen = quote! {
        impl #generics crate::ffi::CloneRaw<#name #generics> for #name #generics {}
    };

    gen.into()
}

#[proc_macro_derive(BuilderDelegate)]
pub fn builder_delegate_macro_derive(input: TokenStream) -> TokenStream {
    impl_builder_delegate(&syn::parse(input).unwrap())
}

fn impl_builder_delegate(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let builder: Ident = syn::parse_str(&format!("{}Builder", name)).unwrap();
    let generics = &ast.generics;

    let gen = quote! {
        impl #generics crate::builder::BuilderDelegate<#builder #generics> for #name #generics {}
    };

    gen.into()
}
