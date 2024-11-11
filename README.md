<div align="center">
    <h1>tuple-traits</h1>
    <p>Additional traits to enable some <del>cursed</del> <i>ergonomic</i> types.</p>
    <!-- -->
    <a href="https://crates.io/crates/tuple-traits"><img src="https://img.shields.io/crates/v/tuple-traits" alt="crates.io" /></a>
    <a href="https://docs.rs/tuple-traits/latest/tuple-traits"><img src="https://img.shields.io/docsrs/tuple-traits" alt="docs.rs" /></a>
    <a href="https://github.com/andogq/tuple-traits/actions/workflows/checks.yml"><img src="https://github.com/andogq/tuple-traits/actions/workflows/checks.yml/badge.svg" alt="checks" /></a>
</div>

# Traits

## [`Append`](https://docs.rs/tuple-traits/latest/tuple-traits/trait.Append.html)

Append a type to a tuple.

```rust
static_assertions::assert_type_eq_all!(
    <(usize, char) as tuple_traits::Append>::Append<bool>,
    (usize, char, bool)
);
```

## [`Cons`](https://docs.rs/tuple-traits/latest/tuple-traits/trait.Cons.html)

Represent a tuple as a cons (*ish*) value, with the first value on the left, and the rest of the
tuple on the right.

```rust
static_assertions::assert_impl_all!(
    (usize, usize, usize): tuple_traits::Cons<Left = usize, Right = (usize, usize)>
);
```

## [`Contains`](https://docs.rs/tuple-traits/latest/tuple-traits/trait.Contains.html)

A trait that will only be implement for a given target if it is present within a given type.

```rust
struct A;
struct B;
struct C;

fn requires_c<T, Index>(value: T)
where
    T: tuple_traits::Contains<C, Index>
{
}

// Works!
requires_c((A, B, C));

// Compiler error: `C` does not appear within `(A, B)`
// requires_c((A, B));
```

# Example

Check out [`./examples/buffer-flags.rs`](./examples/buffer-flags.rs) for a full example!
