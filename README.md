# repr-discriminant

Derive macro to add primitive representations to enums and allow parsing their discriminant.

## Usage

```rust
use repr_discriminant::ReprDiscriminant;

#[repr(u8)]
#[derive(ReprDiscriminant, Debug, PartialEq, Eq)]
enum TestEnum {
    A = 1,
    B = 2,
    C = 3,
}

fn main() {
    let a = TestEnum::A;
    let b = TestEnum::B;
    let c = TestEnum::C;

    assert_eq!(a.discriminant(), 1);
    assert_eq!(b.discriminant(), 2);
    assert_eq!(c.discriminant(), 3);
}
```