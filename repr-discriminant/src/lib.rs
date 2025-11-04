//! A library to obtain the representation of an enum's discriminant.

/// A trait to obtain the representation of an enum's discriminant.
pub trait ReprDiscriminant {
    /// The representation type of the discriminant.
    type Repr;

    /// Returns the representation of the enum's discriminant.
    fn repr_discriminant(&self) -> Self::Repr;
}

#[cfg(feature = "derive")]
pub use repr_discriminant_derive::ReprDiscriminant;
