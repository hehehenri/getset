# Getset

`Getset` provides derive macros for generating getters and setters for your structs.

Import the `Getters` and `Setters` traits from the `getset` crate and annotate your struct with `#[derive(Getters, Setters)]` to automatically generate getters and setters for its fields.

```rust
use getset::{Getters, Setters};

#[derive(Getters, Setters, Default)]
struct Example<T>
where
    T: Copy + Clone + Default
{
    field: T,
    #[getset(skip)]
    skipped_field: T,
    #[getset(skip_getter, skip_setter)]
    another_skipped_field: T,
}

fn main() {
    let example = Example::default();

    example.set_field("something");
    println!(example.get_field());
}
```
