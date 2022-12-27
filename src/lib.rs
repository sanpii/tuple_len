#![warn(warnings)]
#![no_std]

#[macro_export]
macro_rules! tuple_len {
    ( ($($a:expr),+) ) => { $crate::tuple_len!(1, $($a,)+) };
    ( $len:expr, $a:expr, $($rest_a:expr,)+ ) => { $crate::tuple_len!($len + 1, $($rest_a,)+) };
    ( $len:expr, $a:expr, ) => { $len };
    ( () ) => { 0usize };
}

#[cfg(test)]
mod tests {
    #[test]
    fn tuple_len() {
        let _x: u8;

        assert_eq!(tuple_len!(()), 0);
        assert_eq!(tuple_len!((1)), 1);
        assert_eq!(tuple_len!(((1, 1))), 1);
        assert_eq!(tuple_len!((_x, _x)), 2);
        assert_eq!(tuple_len!((_x, 1, _x)), 3);
        assert_eq!(tuple_len!((_x, _x, Some("foo"), || {})), 4);
    }
}
