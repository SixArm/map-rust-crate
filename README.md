# map! macro Rust crate

This crate provides the `map!` macro to create a new map collection, 
then insert key-value pairs. This is inspired by the `vec!` macro.
See below for equivalent Rust standard library code.

Example with parenthesis syntax:

```rust
let m = map!((1, 2), (3, 4));
```

Example with arrow syntax:

```rust
let m = map!(1 => 2, 3 => 4);
```

Example with tuple syntax and multiple lines:

```rust
let m = map!(
    (1, 2),
    (3, 4),
);
```

Example with arrows and multiple lines:

```rust
let m = map!(
    1 => 2,
    3 => 4,
);
```

Equivalent Rust std code with method `insert``:

```rust
let mut m = HashMap::new();
m.insert(1, 2);
m.insert(3, 4);
```

Equivalent Rust std code with method `from`:

```rust
let m = HashMap::from([(1, 2), (3, 4)]);
```

Equivalent Rust std code with method `into`:

```rust
let m: HashMap<_, _> = [(1, 2), (3, 4)].into();
```
