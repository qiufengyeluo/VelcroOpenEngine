
#![allow(clippy::manual_unwrap_or_default)]

use darling::*;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use velcro_utils::{UUID, hasder::*};

mod typeinfo;
mod reflect;
mod vobject;
mod context;


#[proc_macro_derive(Context, attributes(context))]
pub fn context(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(context::impl_context(ast))
}

/// Implements `Reflect` trait
#[proc_macro_derive(Reflect, attributes(reflect))]
pub fn reflect(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut ty_args = reflect::args::TypeArgs::from_derive_input(&ast).unwrap();
    ty_args.validate();

    let reflect_impl = reflect::impl_reflect(&ty_args);
    let prop_key_impl = reflect::impl_prop_constants(&ty_args);

    TokenStream::from(quote::quote! {
        #reflect_impl
        #prop_key_impl
    })
}

/// Implements `Reflect` by analyzing derive input, without adding property constants
///
/// This is used to implement the `Reflect` trait for external types.
#[proc_macro]
pub fn impl_reflect(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut ty_args = reflect::args::TypeArgs::from_derive_input(&ast).unwrap();
    ty_args.validate();

    let reflect_impl = reflect::impl_reflect(&ty_args);

    TokenStream::from(reflect_impl)
}

#[proc_macro]
pub fn impl_context(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(context::impl_context(ast))
}


/// Implements `TypeUuidProvider` trait
///
/// User has to import `TypeUuidProvider` trait to use this macro.
#[proc_macro_derive(TypeUuidProvider, attributes(type_uuid))]
pub fn type_uuid(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(typeinfo::impl_type_uuid_provider(ast))
}

/// Implements `VObjectProvider` trait
///
/// User has to import `VObjectProvider` trait to use this macro.
#[proc_macro_derive(VObjectProvider, attributes(vobject))]
pub fn vobject(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(vobject::impl_type_uuid_provider(ast))
}