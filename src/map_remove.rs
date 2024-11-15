/// Use an existing map collection and remove key-value pairs.
///
/// Example:
///
/// ```
/// # use ::map::*;
/// # use ::std::collections::HashMap;
/// let mut m = HashMap::from([(1, 2), (3, 4)]);
/// map_remove!(m, &1, &3);
/// ```
///
/// Equivalent Rust standard code:
///
/// ```
/// # use std::collections::HashMap;
/// let mut m = HashMap::from([(1, 2), (3, 4)]);
/// m.remove(&1);
/// m.remove(&3);
/// ```
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! map_remove {
    ($map:ident, $( $k:expr ),* $(,)?) => {
        {
            $(
                $map.remove($k);
            )*
        }
    };
}

#[cfg(test)]
mod test {

    mod one_line {
        use std::collections::HashMap;

        #[test]
        fn trailing_comma() {
            let mut x = HashMap::from([(1, 2), (3, 4), (5, 6), (7, 8)]);
            map_remove!(x, &3, &5,);
            assert_eq!(x, HashMap::from([(1, 2), (7, 8)]));
        }

        #[test]
        fn no_trailing_comma() {
            let mut x = HashMap::from([(1, 2), (3, 4), (5, 6), (7, 8)]);
            map_remove!(x, &3, &5);
            assert_eq!(x, HashMap::from([(1, 2), (7, 8)]));
        }
    }

    mod multiple_lines {
        use std::collections::HashMap;

        #[test]
        fn trailing_comma() {
            let mut x = HashMap::from([(1, 2), (3, 4), (5, 6), (7, 8)]);
            map_remove!(
                x,
                &3,
                &5,
            );
            assert_eq!(x, HashMap::from([(1, 2), (7, 8)]));
        }

        #[test]
        fn no_trailing_comma() {
            let mut x = HashMap::from([(1, 2), (3, 4), (5, 6), (7, 8)]);
            map_remove!(
                x,
                &3,
                &5
            );
            assert_eq!(x, HashMap::from([(1, 2), (7, 8)]));
        }
    }

}
