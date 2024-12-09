#![warn(warnings)]
#![no_std]
#![doc = include_str!("../README.md")]

#[macro_export]
macro_rules! tuple_len {
    ( ($($a:expr),+ $(,)?) ) => { $crate::tuple_len!(1usize, $($a,)+) };
    ( $len:expr, $a:expr, $($rest_a:expr,)+ ) => { $crate::tuple_len!($len + 1usize, $($rest_a,)+) };
    ( $len:expr, $a:expr, ) => { $len };
    ( () ) => { 0usize };
    ( $tuple:ident ) => { $crate::len($tuple) };
    ( &$tuple:ident ) => { $crate::len(&$tuple) };
}

#[allow(clippy::len_without_is_empty)]
pub trait TupleLen {
    fn len(&self) -> usize;
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! tuple_impl {
    ($( $T:ident ),*) => {
        impl<$( $T, )*> TupleLen for ($( $T, )*) {
            #[inline]
            fn len(&self) -> usize {
                count!($( $T )*)
            }
        }

        impl<$( $T, )*> TupleLen for &($( $T, )*) {
            #[inline]
            fn len(&self) -> usize {
                count!($( $T )*)
            }
        }
    }
}

tuple_impl!();
tuple_impl!(A);
tuple_impl!(A, B);
tuple_impl!(A, B, C);
tuple_impl!(A, B, C, D);
tuple_impl!(A, B, C, D, E);
tuple_impl!(A, B, C, D, E, F);
tuple_impl!(A, B, C, D, E, F, G);
tuple_impl!(A, B, C, D, E, F, G, H);
tuple_impl!(A, B, C, D, E, F, G, H, I);
tuple_impl!(A, B, C, D, E, F, G, H, I, J);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L);

#[inline]
pub fn len(tuple: impl TupleLen) -> usize {
    tuple.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn r#macro() {
        let _x: u8;

        assert_eq!(tuple_len!(()), 0);
        assert_eq!(tuple_len!((1)), 1);
        assert_eq!(tuple_len!((1,)), 1);
        assert_eq!(tuple_len!(((1, 1))), 1);
        assert_eq!(tuple_len!((_x, _x)), 2);
        assert_eq!(tuple_len!((_x, 1, _x)), 3);
        assert_eq!(tuple_len!((_x, _x, Some("foo"), || {})), 4);
    }

    #[test]
    fn macro_ident() {
        let tuple = ();

        assert_eq!(crate::tuple_len!(tuple), 0);
        assert_eq!(crate::tuple_len!(&tuple), 0);
    }

    #[test]
    fn r#trait() {
        use crate::TupleLen;

        let _x: u8 = 0;

        assert_eq!(().len(), 0);
        assert_eq!((1,).len(), 1);
        assert_eq!((_x, _x).len(), 2);
        assert_eq!((_x, 1, _x).len(), 3);
        assert_eq!((_x, _x, Some("foo"), || {}).len(), 4);
    }

    #[test]
    fn function() {
        let _x: u8 = 0;

        assert_eq!(crate::len(()), 0);
        #[allow(clippy::needless_borrows_for_generic_args)]
        {
            assert_eq!(crate::len(&()), 0);
        }
        assert_eq!(crate::len((1,)), 1);
        assert_eq!(crate::len((_x, _x)), 2);
        assert_eq!(crate::len((_x, 1, _x)), 3);
        assert_eq!(crate::len((_x, _x, Some("foo"), || {})), 4);
    }
}
