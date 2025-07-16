//! Attribute macro to implement a discriminant method for enums with a specific representation type.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Type};

const DEFAULT_DISCRIMINANT_METHOD_NAME: &str = "discriminant";

/// Attribute macro to implement a discriminant method for enums with a specific representation type.
///
/// # Panics
///
/// This macro will panic if the input type is not an enum or if the arguments are not specified correctly.
#[proc_macro_attribute]
pub fn repr_discriminant(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: Args = parse_macro_input!(args);
    let typ = args.typ;
    let method_name = args
        .method_name
        .unwrap_or_else(|| Ident::new(DEFAULT_DISCRIMINANT_METHOD_NAME, Span::call_site()));
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name = &input.ident;
    let body = match input.data {
        Data::Enum(_) => {
            quote! {
                pub const fn #method_name(&self) -> #typ {
                    // SAFETY: The macro guarantees that the enum is repr(#typ).
                    unsafe {
                        *::core::ptr::from_ref(self)
                            .cast::<#typ>()
                    }
                }
            }
        }
        _ => unimplemented!(),
    };

    TokenStream::from(quote! {
        #[repr(#typ)]
        #input

        impl #impl_generics #name #ty_generics #where_clause {
            #body
        }
    })
}


/// Arguments for the `repr_discriminant` macro.
struct Args {
    typ: Type,
    method_name: Option<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let typ: Type = input.parse()?;

        if input.is_empty() {
            return Ok(Self {
                typ,
                method_name: None,
            });
        };

        let _: Comma = input.parse()?;

        Ok(Self {
            typ,
            method_name: Some(input.parse()?),
        })
    }
}
