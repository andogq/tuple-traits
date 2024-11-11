#![doc = include_str!("../README.md")]

mod append;
mod cons;
mod contains;

pub use self::{
    append::Append,
    cons::Cons,
    contains::{Contains, Here, There},
};

/// Run the provided macro against the maximum size tuple.
///
/// If longer tuples are required, this is the place to change it.
macro_rules! tuple_list {
    ($macro:ident) => {
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    };
}

pub(crate) use tuple_list;
