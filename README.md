# map! Rust crate

This crate provides a macro `map!`.

The macro creates a map collection then insert key-value pairs.

The macro uses the Rust standard library HashMap.

Examples:

```rust
use map::*;

let a = map!("a" => "b", "c" => "d");

let b = map!(
    "a" => "b", 
    "c" => "d",
);

let c: HashMap<i32, i32> = map!(
    1 => 2,
    3 => 4,
);
```
