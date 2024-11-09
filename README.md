# map! macro Rust crate

This crate provides `map!` macros to create map collections and
insert key-value pairs. This is inspired by the `vec!` macro.

## map! macro

Create a new map collection and insert key-value pairs.

Example with tuple syntax:

```rust
let m = map!((1, 2), (3, 4));
```

Example with arrow syntax:

```rust
let m = map!(1 => 2, 3 => 4);
```

Example with multiple lines and tuple syntax:

```rust
let m = map!(
    (1, 2),
    (3, 4),
);
```

Example with multiple lines and arrow syntax:

```rust
let m = map!(
    1 => 2,
    3 => 4,
);
```

Equivalent Rust standard code:

```rust
let mut m = HashMap::new();
m.insert(1, 2);
m.insert(3, 4);
```

## map_insert! macro

Use an existing map collection and insert key-value pairs.

Example with tuple syntax:

```rust
let mut m = HashMap::new();
map_insert!(m, (1, 2), (3, 4));
```

Example with arrow syntax:

```rust
let mut m = HashMap::new();
map_insert!(m, 1 => 2, 3 => 4);
```

Example with multiple lines and tuple syntax:

```rust
let mut m = HashMap::new();
map_insert!(
    m,
    (1, 2),
    (3, 4),
);
```

Example with multiple lines and arrow syntax:

```rust
let mut m = HashMap::new();
map_insert!(
    m,
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
