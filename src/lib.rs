//! Attribute macro to implement a discriminant method for enums with a specific representation type.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, parse_macro_input};

const UNSUPPORTED_REPRESENTATIONS: &[&str] = &["C", "packed", "transparent"];

/// Attribute macro to implement a discriminant method for enums with a specific representation type.
///
/// # Panics
///
/// This macro will panic if the input type is not an enum with a valid `#[repr(T)]`.
#[proc_macro_derive(ReprDiscriminant)]
pub fn repr_discriminant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let repr_type: Ident = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("repr"))
        .find_map(|attr| attr.parse_args().ok())
        .expect("`#[repr(T)]` is required");

    for &unsupported in UNSUPPORTED_REPRESENTATIONS {
        assert_ne!(
            repr_type, unsupported,
            "`ReprDiscriminant` cannot be used with `repr({unsupported})`"
        );
    }

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name = &input.ident;

    if let Data::Enum(_) = input.data {
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                /// Returns the discriminant value of the enum.
                ///
                /// # Safety
                ///
                /// This method is safe, because the macro guarantees that the enum is repr(T).
                pub const fn discriminant(&self) -> #repr_type {
                    #[allow(unsafe_code)]
                    unsafe {
                        *::core::ptr::from_ref(self)
                            .cast::<#repr_type>()
                    }
                }
            }
        }
        .into()
    } else {
        unimplemented!("`ReprDiscriminant` can only be derived for enums")
    }
}
