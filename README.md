# repr-discriminant

Derive macro to add primitive representations to enums and allow parsing their discriminant.

## Usage

```rust
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

    assert_eq!(foo.discriminant(), 1);
    assert_eq!(bar.discriminant(), 2);
    assert_eq!(spam.discriminant(), 3);
}
```