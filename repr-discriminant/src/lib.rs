//! A library to obtain the representation of an enum's discriminant.

/// A trait to obtain the representation of an enum's discriminant.
///
/// # Safety
///
/// The implementor must ensure that the enum is annotated with `#[repr(Self::Repr)]`
#[expect(unsafe_code)]
pub unsafe trait ReprDiscriminant {
    /// The representation type of the discriminant.
    type Repr;

    /// Returns the representation of the enum's discriminant.
    fn repr_discriminant(&self) -> Self::Repr;
}

#[cfg(feature = "derive")]
pub use repr_discriminant_derive::ReprDiscriminant;
