//! # `map!` macro Rust crate
//!
//! This crate provides the `map!` macro to create a new map collection, 
//! then insert key-value pairs. This is inspired by the `vec!` macro.
//! See below for equivalent Rust standard library code.
//!
//! Example with tuple syntax:
//!
//! ```
//! # use map::*;
//! let m = map!((1, 2), (3, 4));
//! ```
//!
//! Example with arrow syntax:
//!
//! ```
//! # use map::*;
//! let m = map!(1 => 2, 3 => 4);
//! ```
//! 
//! Example with multiple lines and tuple syntax:
//!
//! ```
//! # use map::*;
//! let m = map!(
//!     (1, 2),
//!     (3, 4),
//! );
//! ```
//!
//! Example with multiple lines and arrow syntax:
//!
//! ```rust
//! # use map::*;
//! let m = map!(
//!     1 => 2,
//!     3 => 4,
//! );
//! ```
//! 
//! Equivalent Rust std code with method `insert``:
//!
//! ```
//! # use std::collections::HashMap;
//! let mut m = HashMap::new();
//! m.insert(1, 2);
//! m.insert(3, 4);
//! ```
//! 
//! Equivalent Rust std code with method `from`:
//!
//! ```
//! # use std::collections::HashMap;
//! let m = HashMap::from([(1, 2), (3, 4)]);
//! ```
//! 
//! Equivalent Rust std code with method `into`:
//!
//! ```
//! # use std::collections::HashMap;
//! let m: HashMap<_, _> = [(1, 2), (3, 4)].into();
//! ```

/// Create a map collection then insert key-value pairs.
///
///
/// Example with tuple syntax:
///
/// ```
/// # use map::*;
/// let m = map!((1, 2), (3, 4));
/// ```
///
/// Example with arrow syntax:
///
/// ```
/// # use map::*;
/// let m = map!(1 => 2, 3 => 4);
/// ```
/// 
/// Example with multiple lines and tuple syntax:
///
/// ```
/// # use map::*;
/// let m = map!(
///     (1, 2),
///     (3, 4),
/// );
/// ```
///
/// Example with multiple lines and arrow syntax:
///
/// ```rust
/// # use map::*;
/// let m = map!(
///     1 => 2,
///     3 => 4,
/// );
/// ```
/// 
/// Equivalent Rust std code with method `insert``:
///
/// ```
/// # use std::collections::HashMap;
/// let mut m = HashMap::new();
/// m.insert(1, 2);
/// m.insert(3, 4);
/// ```
/// 
/// Equivalent Rust std code with method `from`:
///
/// ```
/// # use std::collections::HashMap;
/// let m = HashMap::from([(1, 2), (3, 4)]);
/// ```
/// 
/// Equivalent Rust std code with method `into`:
///
/// ```
/// # use std::collections::HashMap;
/// let m: HashMap<_, _> = [(1, 2), (3, 4)].into();
/// ```
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! map {
    // tuple syntax
    ( $(( $k:expr, $v:expr )),* $(,)?) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($k, $v);
            )*
            m
        }
    };
    // arrow syntax
    ( $( $k:expr => $v:expr ),* $(,)?) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($k, $v);
            )*
            m
        }
    };
}

#[cfg(test)]
mod tests {

    mod tuple_syntax {

        mod one_line {

            #[test]
            fn trailing_comma() {
                let x = map!((1, 2), (3, 4),);
                assert_eq!(x.get(&1).unwrap(), &2);
            }

            #[test]
            fn no_trailing_comma() {
                let x = map!((1, 2), (3, 4));
                assert_eq!(x.get(&1).unwrap(), &2);
            }
        }

        mod multiple_lines {

            #[test]
            fn trailing_comma() {
                let x = map!((1, 2), (3, 4),);
                assert_eq!(x.get(&1).unwrap(), &2);
            }

            #[test]
            fn no_trailing_comma() {
                let x = map!((1, 2), (3, 4));
                assert_eq!(x.get(&1).unwrap(), &2);
            }
        }
    }

    mod arrow_syntax {

        mod one_line {

            #[test]
            fn trailing_comma() {
                let x = map!(1 => 2, 3 => 4,);
                assert_eq!(x.get(&1).unwrap(), &2);
            }

            #[test]
            fn no_trailing_comma() {
                let x = map!(1 => 2, 3 => 4);
                assert_eq!(x.get(&1).unwrap(), &2);
            }
        }

        mod multiple_lines {

            #[test]
            fn trailing_comma() {
                let x = map!(
                    1 => 2,
                    3 => 4,
                );
                assert_eq!(x.get(&1).unwrap(), &2);
            }

            #[test]
            fn no_trailing_comma() {
                let x = map!(
                    1 => 2,
                    3 => 4
                );
                assert_eq!(x.get(&1).unwrap(), &2);
            }
        }
        
    }

}
