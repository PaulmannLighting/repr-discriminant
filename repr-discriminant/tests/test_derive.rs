//! Tests for the `ReprDiscriminant` derive macro.
#![cfg(test)]
#![cfg(feature = "derive")]

use std::fmt::Debug;

use repr_discriminant::ReprDiscriminant;

#[repr(u8)]
#[derive(ReprDiscriminant)]
#[allow(dead_code)]
enum TestEnum {
    Foo = 1,
    Bar(usize, f64) = 2,
    Spam { x: i32, y: i32 } = 3,
}

const FOO: TestEnum = TestEnum::Foo;
const BAR: TestEnum = TestEnum::Bar(4, 5.1);
const SPAM: TestEnum = TestEnum::Spam { x: -32, y: 1337 };

#[test]
#[allow(dead_code)]
fn test_const_discriminant() {
    assert_eq!(FOO.discriminant(), 1u8);
    assert_eq!(BAR.discriminant(), 2u8);
    assert_eq!(SPAM.discriminant(), 3u8);
}

#[test]
#[allow(dead_code)]
fn test_trait_discriminant() {
    assert_eq!(FOO.repr_discriminant(), 1u8);
    assert_eq!(BAR.repr_discriminant(), 2u8);
    assert_eq!(SPAM.repr_discriminant(), 3u8);
}

#[test]
#[allow(dead_code)]
fn test_generic() {
    assert_discriminant(FOO, 1u8);
    assert_discriminant(BAR, 2u8);
    assert_discriminant(SPAM, 3u8);
}

fn assert_discriminant<T, U>(value: T, expected: U)
where
    T: ReprDiscriminant<Repr = U>,
    U: PartialEq + Debug,
{
    assert_eq!(value.repr_discriminant(), expected);
}
