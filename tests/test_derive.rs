//! Tests for the `ReprDiscriminant` derive macro.

#![cfg(test)]
use repr_discriminant::ReprDiscriminant;

#[test]
fn test_repr_discriminant() {
    #[repr(u8)]
    #[derive(ReprDiscriminant, Debug, PartialEq, Eq)]
    enum TestEnum {
        A = 1,
        B = 2,
        C = 3,
    }

    let a = TestEnum::A;
    let b = TestEnum::B;
    let c = TestEnum::C;

    assert_eq!(a.discriminant(), 1);
    assert_eq!(b.discriminant(), 2);
    assert_eq!(c.discriminant(), 3);
}
