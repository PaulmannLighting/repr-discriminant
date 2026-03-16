# repr-discriminant

Derive macro and trait to add primitive representations to enums and allow parsing their discriminant.

## Usage

```rust
use std::fmt::Display;
use repr_discriminant::ReprDiscriminant;

#[repr(u8)]
#[derive(ReprDiscriminant)]
enum TestEnum {
    Foo = 1,
    Bar(usize, f64) = 2,
    Spam { x: i32, y: i32 } = 3,
}

fn main() {
    let foo = TestEnum::Foo;
    let bar = TestEnum::Bar(4, 5.1);
    let spam = TestEnum::Spam { x: -32, y: 1337 };

    // By associated const function
    assert_eq!(foo.discriminant(), 1);
    assert_eq!(bar.discriminant(), 2);
    assert_eq!(spam.discriminant(), 3);

    // By trait method
    assert_eq!(foo.repr_discriminant(), 1);
    assert_eq!(bar.repr_discriminant(), 2);
    assert_eq!(spam.repr_discriminant(), 3);
}
```