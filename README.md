# Getset

`Getset` provides derive macros for generating getters and setters for your structs.

Import the `Getters` and `Setters` traits from the `getset` crate and annotate your struct with `#[derive(Getters, Setters)]` to automatically generate getters and setters for its fields.

```rust
use getset::{Getters, Setters};

#[derive(Getters, Setters)]
struct Example<T> {
    field: T
}

fn main() {
    let example = Example { field: String::from("example") };
    println!(example.get_field());
}
```
