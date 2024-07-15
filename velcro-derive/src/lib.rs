
#![allow(clippy::manual_unwrap_or_default)]

//use darling::*;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod typeinfo;
mod reflect;
mod utils;

/// Implements `TypeUuidProvider` trait
///
/// User has to import `TypeUuidProvider` trait to use this macro.
#[proc_macro_derive(TypeUuidProvider, attributes(type_uuid))]
pub fn type_uuid(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(typeinfo::impl_type_uuid_provider(ast))
}