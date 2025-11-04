# repr-discriminant

Trait to retrieve an enum's discriminant.

## Usage

The feature `derive` is recommended to automatically implement the trait for your enums.

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

    assert_eq!(foo.repr_discriminant(), 1);
    assert_eq!(bar.repr_discriminant(), 2);
    assert_eq!(spam.repr_discriminant(), 3);
}
```