use std::marker::PhantomData;

use crate::cons::Cons;

/// Index value to indicate the target is in the left of a [`Cons`].
pub enum Here {}
/// Index value to indicate that the target is at some other index of a [`Cons`].
pub struct There<T>(PhantomData<T>);

/// Determine whether a target is located at some index. This trait will be implemented for all
/// types that contain the target type.
///
/// Currently, there is no mechanism to retrieve the target value, only determine if it does exist
/// or not. This allows for using zero-sized types to be used as flags within the type system.
///
/// If `Index` is allowed to be inferred, it's possible to check if some arbitrary value contains
/// `Target` at any location. With very long tuples there is a chance that the type checker will
/// raise a error due to too much recursion, however it should be fine to raise this limit if your
/// needs require it..
///
/// # Examples
///
/// Allowing `Index` to be inferred allows for searching at any depth.
///
/// ```
/// # use tuple_traits::Contains;
/// #
/// struct A;
/// struct B;
/// struct C;
///
/// fn requires_c<T, Index>(value: T)
/// where
///     T: Contains<C, Index>
/// {
/// }
///
/// requires_c((A, B, C));
/// ```
///
/// The trait bound will not be satisfied for any tuples that do not contain the target value.
///
/// ```compile_fail
/// # use tuple_traits::Contains;
/// #
/// # struct A;
/// # struct B;
/// # struct C;
/// #
/// # fn requires_c<T, Index>(value: T)
/// # where
/// #     T: Contains<C, Index>
/// # {
/// # }
/// #
/// requires_c((A, B));
/// ```
///
/// The target value can also be a generic on the function without a parameter, however this isn't
/// recommended due to Rust requiring all generics to be listed by the caller.
/// ```
/// # use tuple_traits::Contains;
/// #
/// # struct A;
/// # struct B;
/// # struct C;
/// #
/// fn requires_c<T, Index>()
/// where
///     T: Contains<C, Index>
/// {
/// }
///
/// requires_c::<(A, B, C), _>();
/// ```
///
/// Technique based off of [Lloyd's blog post] on type-level recursion in Rust.
///
/// [Lloyd's blog post]: https://beachape.com/blog/2017/03/12/gentle-intro-to-type-level-recursion-in-Rust-from-zero-to-frunk-hlist-sculpting
#[diagnostic::on_unimplemented(
    label = "`{Target}` must appear within here",
    message = "`{Target}` does not appear within `{Self}`",
    note = "ensure that `{Target}` is present on `{Self}` in order to use it here"
)]
pub trait Contains<Target, Index> {}

/// Base implementation, where the target is on the left of a cons.
impl<Target, C> Contains<Target, Here> for C where C: Cons<Left = Target> {}

/// Recursive implementation, advancing the index to try resolve the trait again.
impl<Tail, Target, TailIndex, C> Contains<Target, There<TailIndex>> for C
where
    C: Cons<Right = Tail>,
    Tail: Contains<Target, TailIndex>,
{
}
