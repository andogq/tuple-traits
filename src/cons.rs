/// A base type that can be implemented by any type, which is used to build operations on top of.
/// With this format it can be implemented to an infinite length, meaning that any operations built
/// from it will work for any given length.
pub trait Cons {
    /// Left value of the cons, ideally the next value.
    type Left;
    /// Right value of the cons, another cons or unit if there are no more values.
    type Right;
}

/// Helper macro to implement cons for all tuples of some length.
///
/// Tuple of length 0 (unit) must not implement cons, as it has neither a value nor a right side.
macro_rules! impl_tuples {
    // Base case, implement for when there is a single generic value.
    ($T:ident) => {
        impl<$T> Cons for ($T,) {
            type Left = $T;
            type Right = ();
        }
    };

    // Recursive case, implement for when the first value is the left side, and recurse with the
    // right side and rest of the list.
    ($T:ident, $($Tail:ident),*) => {
        impl<$T, $($Tail),*> Cons for ($T, $($Tail),*) {
            type Left = $T;
            type Right = ($($Tail),*,);
        }

        impl_tuples!($($Tail),*);
    };
}

crate::tuple_list!(impl_tuples);

#[cfg(test)]
mod test {
    use static_assertions::{assert_impl_all, assert_not_impl_all};

    use super::*;

    assert_impl_all!((usize,): Cons<Left = usize, Right = ()>);
    assert_impl_all!((usize, usize): Cons<Left = usize, Right = (usize,)>);
    assert_impl_all!((usize, usize, usize): Cons<Left = usize, Right = (usize, usize)>);

    assert_not_impl_all!((): Cons);
}
