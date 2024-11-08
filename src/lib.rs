//! This crate provides a `map!` macro that creates a map collection then
//! inserts key-value pairs. The macro uses std::collections::HashMap.

/// Create a map collection then insert key-value pairs.
///
/// Standard Rust looks like this:
///
/// ```
/// let mut m = std::collections::HashMap::new();
/// m.insert(1, 2);
/// m.insert(3, 4);
/// ```
///
/// The `map!` macro provides this syntax with parentheses:
///
/// ```
/// # use map::*;
/// let m = map!(
///     (1, 2),
///     (3, 4),
/// );
/// ```
///
/// The `map!` macro provides this syntax with arrows:
///
/// ```
/// # use map::*;
/// let m = map!(
///     1 => 2,
///     3 => 4,
/// );
/// ```
///
/// You can use multiple lines or one line, as you prefer.
///
/// You can use trailing commas or not, as you prefer.
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! map {
    // Parentheses syntax
    ( $(( $k:expr, $v:expr )),* $(,)?) => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($k, $v);
            )*
            m
        }
    };
    // Arrows syntax
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

    mod parentheses_syntax {

        mod one_line {

            #[test]
            fn trailing_comma() {
                let x = map!((1, 2), (3, 4),);
                assert_eq!(x.get(&1).unwrap(), &2);
            }

            #[test]
            fn without_trailing_comma() {
                let x = map!((1, 2), (3, 4));
                assert_eq!(x.get(&1).unwrap(), &2);
            }
        }

        mod multiple_lines {

            #[test]
            fn with_trailing_comma() {
                let x = map!(
                    (1, 2),
                    (3, 4),
                );
                assert_eq!(x.get(&1).unwrap(), &2);
            }

            #[test]
            fn without_trailing_comma() {
                let x = map!(
                    (1, 2),
                    (3, 4)
                );
                assert_eq!(x.get(&1).unwrap(), &2);
            }
        }
    }

    mod arrow_syntax {

        mod one_line {

            #[test]
            fn with_trailing_comma() {
                let x = map!(1 => 2, 3 => 4,);
                assert_eq!(x.get(&1).unwrap(), &2);
            }

            #[test]
            fn without_trailing_comma() {
                let x = map!(1 => 2, 3 => 4);
                assert_eq!(x.get(&1).unwrap(), &2);
            }
        }

        mod multiple_lines {

            #[test]
            fn with_trailing_comma() {
                let x = map!(
                    1 => 2,
                    3 => 4,
                );
                assert_eq!(x.get(&1).unwrap(), &2);
            }

            #[test]
            fn without_trailing_comma() {
                let x = map!(
                    1 => 2,
                    3 => 4
                );
                assert_eq!(x.get(&1).unwrap(), &2);
            }
        }
    }

}
