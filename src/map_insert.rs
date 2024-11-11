/// Use an existing map collection and insert key-value pairs.
///
/// Example with tuple syntax:
///
/// ```
/// # use ::map::*;
/// # use ::std::collections::HashMap;
/// let mut m = HashMap::new();
/// map_insert!(m, (1, 2), (3, 4));
/// ```
///
/// Example with arrow syntax:
///
/// ```
/// # use ::map::*;
/// # use ::std::collections::HashMap;
/// let mut m = HashMap::new();
/// map_insert!(m, 1 => 2, 3 => 4);
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
#[allow(unused_macros)]
#[macro_export]
macro_rules! map_insert {
    ($map:ident, $(( $k:expr, $v:expr )),* $(,)?) => {
        {
            $(
                $map.insert($k, $v);
            )*
        }
    };
    ($map:ident, $( $k:expr => $v:expr ),* $(,)?) => {
        {
            $(
                $map.insert($k, $v);
            )*
        }
    };
}

#[cfg(test)]
mod test {

    mod tuple_syntax {

        mod one_line {
            use std::collections::HashMap;

            #[test]
            fn trailing_comma() {
                let mut x = ::std::collections::HashMap::new();
                map_insert!(x, (1, 2), (3, 4),);
                assert_eq!(x, HashMap::from([(1, 2), (3, 4)]));
            }

            #[test]
            fn no_trailing_comma() {
                let mut x = ::std::collections::HashMap::new();
                map_insert!(x, (1, 2), (3, 4));
                assert_eq!(x, HashMap::from([(1, 2), (3, 4)]));
            }
        }

        mod multiple_lines {
            use std::collections::HashMap;

            #[test]
            fn trailing_comma() {
                let mut x = ::std::collections::HashMap::new();
                map_insert!(
                    x,
                    (1, 2),
                    (3, 4),
                );
                assert_eq!(x, HashMap::from([(1, 2), (3, 4)]));
            }

            #[test]
            fn no_trailing_comma() {
                let mut x: std::collections::HashMap<i32, i32> = ::std::collections::HashMap::new();
                map_insert!(
                    x,
                    (1, 2),
                    (3, 4)
                );
                assert_eq!(x, HashMap::from([(1, 2), (3, 4)]));
            }
        }
    }

    mod arrow_syntax {

        mod one_line {
            use std::collections::HashMap;

            #[test]
            fn trailing_comma() {
                let mut x: std::collections::HashMap<i32, i32> = ::std::collections::HashMap::new();
                map_insert!(x, 1 => 2, 3 => 4,);
                assert_eq!(x, HashMap::from([(1, 2), (3, 4)]));
            }

            #[test]
            fn no_trailing_comma() {
                let mut x: std::collections::HashMap<i32, i32> = ::std::collections::HashMap::new();
                map_insert!(x, 1 => 2, 3 => 4);
                assert_eq!(x, HashMap::from([(1, 2), (3, 4)]));
            }
        }

        mod multiple_lines {
            use std::collections::HashMap;

            #[test]
            fn trailing_comma() {
                let mut x: std::collections::HashMap<i32, i32> = ::std::collections::HashMap::new();
                map_insert!(
                    x,
                    1 => 2,
                    3 => 4,
                );
                assert_eq!(x, HashMap::from([(1, 2), (3, 4)]));
            }

            #[test]
            fn no_trailing_comma() {
                let mut x: std::collections::HashMap<i32, i32> = ::std::collections::HashMap::new();
                map_insert!(
                    x,
                    1 => 2,
                    3 => 4
                );
                assert_eq!(x, HashMap::from([(1, 2), (3, 4)]));
            }
        }
    }
}
