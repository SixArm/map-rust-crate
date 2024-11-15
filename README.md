# map! macro Rust crate

This crate provides the `map!` macro and related macros to create map
collections, insert key-value pairs, and remove them. Inspired by `vec!`.

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

Equivalent Rust standard code:

```rust
let mut m = HashMap::new();
m.insert(1, 2);
m.insert(3, 4);
```

## map_remove! macro

Use an existing map collection and remove key-value pairs.

Example with tuple syntax:

```rust
let mut m = HashMap::from([(1, 2), (3, 4)]);
map_remove!(m, &1, &3);
```

Equivalent Rust standard code:

```rust
let mut m = HashMap::from([(1, 2), (3, 4)]);
m.remove(&1);
m.remove(&3);
```
