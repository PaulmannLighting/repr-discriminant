//! Tests for the `ReprDiscriminant` derive macro.

#![cfg(test)]
use repr_discriminant::ReprDiscriminant;

#[test]
#[allow(dead_code)]
fn test_repr_discriminant() {
    #[repr(u8)]
    #[derive(ReprDiscriminant)]
    enum TestEnum {
        Foo = 1,
        Bar(usize, f64) = 2,
        Spam { x: i32, y: i32 } = 3,
    }

    let foo = TestEnum::Foo;
    let bar = TestEnum::Bar(4, 5.1);
    let spam = TestEnum::Spam { x: -32, y: 1337 };

    assert_eq!(foo.discriminant(), 1u8);
    assert_eq!(bar.discriminant(), 2u8);
    assert_eq!(spam.discriminant(), 3u8);
}
