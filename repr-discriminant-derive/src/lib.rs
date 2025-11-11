//! Attribute macro to implement a discriminant method for enums with a specific representation type.

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Data, DeriveInput, Ident, ImplGenerics, Type, TypeGenerics, WhereClause, parse_macro_input,
};

const SUPPORTED_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
];

/// Attribute macro to implement a discriminant method for enums with a specific representation type.
///
/// # Panics
///
/// This macro will panic if the input type is not an enum with a valid `#[repr(T)]`.
#[proc_macro_derive(ReprDiscriminant)]
pub fn repr_discriminant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let repr_type: Type = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("repr"))
        .find_map(|attr| attr.parse_args().ok())
        .expect("`#[repr(T)]` is required");

    assert!(
        SUPPORTED_TYPES.contains(&repr_type.to_token_stream().to_string().as_str()),
        "`ReprDiscriminant` can only be used with the following types: {SUPPORTED_TYPES:?}"
    );

    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name = &input.ident;

    if let Data::Enum(_) = input.data {
        let const_impl = impl_const(&impl_generics, name, &ty_generics, where_clause, &repr_type);
        let trait_impl = impl_trait(&impl_generics, name, &ty_generics, where_clause, &repr_type);
        quote! {
            #const_impl

            #trait_impl
        }
        .into()
    } else {
        unimplemented!("`ReprDiscriminant` can only be derived for enums")
    }
}

fn impl_const(
    impl_generics: &ImplGenerics<'_>,
    name: &Ident,
    ty_generics: &TypeGenerics<'_>,
    where_clause: Option<&WhereClause>,
    repr_type: &Type,
) -> proc_macro2::TokenStream {
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
}

fn impl_trait(
    impl_generics: &ImplGenerics<'_>,
    name: &Ident,
    ty_generics: &TypeGenerics<'_>,
    where_clause: Option<&WhereClause>,
    repr_type: &Type,
) -> proc_macro2::TokenStream {
    quote! {
        impl #impl_generics ::repr_discriminant::ReprDiscriminant for #name #ty_generics #where_clause {
            type Repr = #repr_type;

            fn repr_discriminant(&self) -> Self::Repr {
                self.discriminant()
            }
        }
    }
}
