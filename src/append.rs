/// Trait to append some value to the end of a tuple.
///
/// # Examples
///
/// ```
/// # use static_assertions::assert_type_eq_all;
/// # use tuple_traits::Append;
/// #
/// // Append a value to the unit value.
/// assert_type_eq_all!(<() as Append>::Append<usize>, (usize,));
///
/// // Append a value to a single value tuple.
/// assert_type_eq_all!(<(usize,) as Append>::Append<char>, (usize, char));
///
/// // Append a value to a multi-value tuple.
/// assert_type_eq_all!(<(usize, char) as Append>::Append<bool>, (usize, char, bool));
/// ```
///
/// The trait can also be used to modify the return value based on some generic.
///
/// ```no_run
/// # use static_assertions::assert_type_eq_all;
/// # use tuple_traits::Append;
/// #
/// fn append_usize<T>(value: T) -> T::Append<usize>
/// where
///     T: Append
/// {
///     // ...
/// #    todo!()
/// }
///
/// let result: (char, bool, usize) = append_usize(('a', true));
/// ```
pub trait Append {
    /// Append operation, which will be the type of the result.
    type Append<T>;
}

macro_rules! impl_tuples {
    // Base case, appending to unit (empty tuple).
    () => {
        impl Append for () {
            type Append<T> = (T,);
        }
    };

    // Recursive case, implement for a tuple, and recurse with the head removed.
    ($T:ident $(, $($Tail:ident),* )?) => {
        impl<$T, $($($Tail),*)?> Append for ($T, $($($Tail),*)?) {
            type Append<T> = ($T, $($($Tail,)*)? T);
        }

        impl_tuples!($($($Tail),*)?);
    };
}

crate::tuple_list!(impl_tuples);

#[cfg(test)]
mod test {
    use static_assertions::assert_type_eq_all;

    use super::*;

    assert_type_eq_all!(<() as Append>::Append<usize>, (usize,));
    assert_type_eq_all!(<(bool,) as Append>::Append<usize>, (bool, usize));
    assert_type_eq_all!(<(bool, char) as Append>::Append<usize>, (bool, char, usize));
}
