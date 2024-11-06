//! This crate provides a `map!` macro that creates a map collection then
//! inserts key-value pairs. The macro uses std::collections::HashMap.

/// Create a map collection then insert elements.
///
/// Examples:
///
/// ```
/// use map::*;
///
/// let m = map!("a" => "b", "c" => "d");
///
/// let m = map!(
///     "a" => "b",
///     "c" => "d",
/// );
///
/// let m: std::collections::HashMap<i32, i32> = map!(
///     1 => 2,
///     3 => 4,
/// );
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! map {
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

    #[test]
    fn test_map_macro_with_numbers_with_one_line_with_trailing_comma() {
        let x = map!(1 => 2, 3 => 4,);
        assert_eq!(x.get(&1).unwrap(), &2);
    }

    #[test]
    fn test_map_macro_with_one_line_without_trailing_comma() {
        let x = map!(1 => 2, 3 => 4);
        assert_eq!(x.get(&1).unwrap(), &2);
    }

    #[test]
    fn test_map_macro_with_multiple_lines_with_trailing_comma() {
        let x = map!(
            1 => 2,
            3 => 4,
        );
        assert_eq!(x.get(&1).unwrap(), &2);
    }

    #[test]
    fn test_map_macro_with_multiple_lines_without_trailing_comma() {
        let x = map!(
            1 => 2,
            3 => 4
        );
        assert_eq!(x.get(&1).unwrap(), &2);
    }
}
