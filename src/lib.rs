#![doc = include_str!("../README.md")]
//! TLists: Type-level linked lists.
//!
//! These are useful if you need to keep track of a _list_ of types inside your type,
//! and manipulate them in generic ways (like taking the first type, reversing the list, etc.)
//!
//! The easiest way to build a TList is to use the [`TList!`] macro:
//!
//! ```rust
//! use tlist::*;
//! use typenum::consts::*;
//!
//! type MyList = TList![U10, U20, U100];
//! ```

mod typenum_ext;
mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

use typenum_ext::UnsignedExt;

use core::marker::PhantomData;

#[doc(hidden)]
pub trait TListImpl {
    type Last<X>;
    type Inits<X>: TList;
}

/// Type-level lists.
///
/// This trait is a "type-level enum".
/// [trait@TList], [TNil] and [TCons] are the type-level equivalent of the following value-level enum:
///
/// ```ignore
/// pub enum List<H, T> {
///   Nil,
///   Cons(H, T),
/// }
/// ```
///
/// This trait can be considered a 'marker' trait:
/// It has no methods to be called at runtime.
/// It only has some GATs.
///
/// Rather than calling these GATs directly,
/// it is recommended to instead use their type aliases,
/// as calling those is less verbose:
///
/// ```rust
/// use tlist::*;
///
/// // This is possible...
/// type Fine = <TList![u8, u16] as TList>::Concat<TList![u32, u64]>;
///
/// // But this is more readable:
/// type Nice = Concat<TList![u8, u16], TList![u32, u64]>;
///
/// static_assertions::assert_type_eq_all!(Fine, Nice);
/// ```
pub trait TList: Sealed + TListImpl {
    /// Implementation of [type@Concat].
    type Concat<Rhs: TList>: TList;

    /// Implementation of [type@Reverse].
    type Reverse: TList;

    /// Implementation of [type@IsEmpty].
    type IsEmpty: Bit;

    /// Implementation of [type@Len].
    type Len: UnsignedExt;
}

/// The empty [trait@TList].
///
/// Only [TNil] implements this constraining trait.
///
/// See also [IsEmpty] if you want work with both [Empty] and [NonEmpty]
/// lists generically.
pub trait Empty: TList + Sealed {}

/// Non-empty [trait@TList]s.
///
/// Any [trait@TList] except [TNil] implements this constraining trait.
/// (In other words: Any [`TCons<H, T>`], regardless of what `H` or `T` it contains, implements it.)
///
/// Quite a number of operations are only defined for non-empty [trait@TList]s,
/// so this constraint is used a lot in the library itself as well.
///
/// See also [IsEmpty] if you want work with both [Empty] and [NonEmpty]
/// lists generically.
pub trait NonEmpty: TList + Sealed {
    /// Implementation of [type@First].
    type First;
    /// Implementation of [type@Rest].
    type Rest: TList;
    /// Implementation of [type@Last].
    type Last;
    /// Implementation of [type@Inits].
    type Inits: TList;
}

/// The empty TList.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TNil;

/// A non-empty TList whose first element is `H` and whose tail is the TList `T`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TCons<H, T: TList>(PhantomData<(H, T)>);

impl Sealed for TNil {}
impl<H, T: TList> Sealed for TCons<H, T> {}

impl TListImpl for TNil {
    type Last<X> = X;
    type Inits<X> = TNil;
}
impl TList for TNil {
    type Concat<Rhs: TList> = Rhs;
    type Reverse = TNil;
    type IsEmpty = B1;
    type Len = U0;
}

impl Empty for TNil {}

impl<H, T: TList> TListImpl for TCons<H, T> {
    type Last<X> = T::Last<H>;
    type Inits<X> = TCons<X, T::Inits<H>>;
}
impl<H, T: TList> TList for TCons<H, T> {
    type Concat<Rhs: TList> = TCons<H, T::Concat<Rhs>>;
    type Reverse = Concat<T::Reverse, TCons<H, TNil>>;
    type IsEmpty = B0;
    type Len = <T::Len as UnsignedExt>::Succ;
}

impl<H, T: TList> NonEmpty for TCons<H, T> {
    type First = H;
    type Rest = T;
    type Last = T::Last<H>;
    type Inits = T::Inits<H>;
}

/// Shorthand macro to construct TList types.
///
/// This is usually much more readable than writing out the nesting of
/// [TCons] and [TNil] by hand.
///
/// ```rust
/// use tlist::*;
///
/// use static_assertions::assert_type_eq_all as type_eq;
/// use typenum::consts::{U1, U2, U3, U4, U42};
///
/// type_eq!(TList![], TNil);
///
/// type_eq!(TList![U42], TCons<U42, TNil>);
///
/// type_eq!(TList![U1, U2, U3], TCons<U1, TCons<U2, TCons<U3, TNil>>>);
///
/// // You can also use `...Rest` for the last argument:
/// type Rest = TList![U3, U4];
/// type_eq!(TList![U1, U2, ...Rest], TCons<U1, TCons<U2, Rest>>);
/// ```
#[macro_export]
// Implementation based on the frunk crate's HList! macro.
macro_rules! TList {
    () => { $crate::TNil };
    (...$Rest:ty) => { $Rest };
    ($A:ty) => { $crate::TList![$A,] };
    ($A:ty, $($tok:tt)*) => {
        $crate::TCons<$A, $crate::TList![$($tok)*]>
    };
}

/// Type-level 'function' to return the first element of a TList
///
/// Only implemented for non-empty TLists.
///
/// ```rust
/// use tlist::*;
/// use typenum::consts::{U1, U2, U3};
/// use static_assertions::assert_type_eq_all as assert_type_eq;
///
/// assert_type_eq!(First<TList![U1, U2, U3]>, U1);
///
/// assert_type_eq!(First<TList![i8, usize, i32, u64]>, i8);
/// ```
pub type First<List> = <List as NonEmpty>::First;

/// Type-level 'function' to return the first element of a TList
///
/// Only implemented for non-empty TLists.
///
/// ```rust
/// use tlist::*;
/// use typenum::consts::{U1, U2, U3};
/// use static_assertions::assert_type_eq_all as assert_type_eq;
///
/// assert_type_eq!(Rest<TList![U1, U2, U3]>, TList![U2, U3]);
///
/// assert_type_eq!(Rest<TList![i8, usize, i32, u64]>, TList![usize, i32, u64]);
/// ```
pub type Rest<List> = <List as NonEmpty>::Rest;

/// Type-level 'function' to return the all elements but the last element of a TList
///
/// Only implemented for non-empty TLists.
/// ```rust
/// use tlist::*;
/// use typenum::consts::{U1, U2, U3};
/// use static_assertions::assert_type_eq_all as assert_type_eq;
///
/// assert_type_eq!(Last<TList![U1, U2, U3]>, U3);
///
/// assert_type_eq!(Last<TList![i8, usize, i32, u64]>, u64);
/// ```
pub type Last<List> = <List as NonEmpty>::Last;

/// Type-level 'function' to return the all elements but the last element of a TList
///
/// Only implemented for non-empty TLists.
/// ```rust
/// use tlist::*;
/// use typenum::consts::{U1, U2, U3};
/// use static_assertions::assert_type_eq_all as assert_type_eq;
///
/// assert_type_eq!(Inits<TList![U1, U2, U3]>, TList![U1, U2]);
///
/// assert_type_eq!(Inits<TList![i8, usize, i32, u64]>, TList![i8, usize, i32]);
/// ```
pub type Inits<List> = <List as NonEmpty>::Inits;

/// Type-level 'function' to concatenate two TLists.
///
///
/// ```rust
/// use tlist::*;
/// use typenum::consts::{U1, U2, U3, U4, U5};
/// use static_assertions::assert_type_eq_all as assert_type_eq;
///
/// assert_type_eq!(Concat<TList![], TList![]>, TList![]);
///
/// assert_type_eq!(Concat<TList![U1], TList![]>, TList![U1]);
///
/// assert_type_eq!(Concat<TList![U2], TList![]>, TList![U2]);
///
/// assert_type_eq!(Concat<TList![U1, U2], TList![U3, U4, U5]>, TList![U1, U2, U3, U4, U5]);
/// ```
///
pub type Concat<Lhs, Rhs> = <Lhs as TList>::Concat<Rhs>;

/// Type-level 'function' to reverse a TList.
///
/// ```rust
/// use tlist::*;
/// use typenum::consts::{U1, U2, U3, U4, U5};
/// use static_assertions::assert_type_eq_all as assert_type_eq;
///
/// assert_type_eq!(Reverse<TList![]>, TList![]);
///
/// assert_type_eq!(Reverse<TList![U3, U2, U1]>, TList![U1, U2, U3]);
/// ```
///
pub type Reverse<List> = <List as TList>::Reverse;

use typenum::consts::U0;
use typenum::{Bit, B0, B1};
/// Type-level 'function' to calculate the length of a TList.
///
/// You can turn the result into a `usize` using `Len<List>::USIZE` or `Len<List>::to_usize()`.
///
/// (See [`typenum::Unsigned`].)
pub type Len<List> = <List as TList>::Len;

/// Type-level 'function' returning [`typenum::B1`] when the list is empty; [`typenum::B0`] otherwise.
///
/// You can turn the result into a `bool` using `IsEmpty<List>::BOOL` or `IsEmpty<List>::to_bool()`.
///
/// (See [`typenum::Bit`] for more on this.)
/// ```rust
/// use tlist::*;
/// use typenum::{B0, B1, Bit};
/// use static_assertions::assert_type_eq_all as assert_type_eq;
///
/// assert_type_eq!(IsEmpty<TList![]>, B1);
/// assert_type_eq!(IsEmpty<TList![i32]>, B0);
/// assert_type_eq!(IsEmpty<TList![u32, i64]>, B0);
///
/// assert_eq!(IsEmpty::<TList![]>::BOOL, true);
/// assert_eq!(IsEmpty::<TList![&'static str]>::BOOL, false);
/// ```
///
/// [IsEmpty] is a type-level function that works for any [trait@TList], returning a type-level boolean.
/// If you want to _constrain_ what kind of [trait@TList] is allowed for a certain operation,
/// use the [Empty] or [NonEmpty] constraining traits.
pub type IsEmpty<List> = <List as TList>::IsEmpty;

/// Constraint which only holds if a TList is a prefix of `Other`.
///
/// This is not a type-level 'function', but rather a constraint you can use to make compiler errors more readable.
///
/// Since this is a constraint, and only holds for a small subset of [trait@TList]s, it _is_
/// implemented as a separate trait and not as a GAT.
///
/// ```rust
/// use tlist::*;
/// use typenum::consts::{U1, U2, U3, U4, U42};
///
/// static_assertions::assert_impl_all!(TList![U1, U2]: Prefix<TList![U1, U2, U3, U4]>);
/// static_assertions::assert_not_impl_any!(TList![U42]: Prefix<TList![U1, U2, U3, U4]>);
/// ```
///
/// Also see [EitherPrefix].
pub trait Prefix<Other: TList> {}

// prefix [] _ = true
impl<Other: TList> Prefix<Other> for TNil {}

// prefix (h : ls) (h : rs) == prefix ls rs
impl<H, Ls: TList, Rs: TList> Prefix<TCons<H, Rs>> for TCons<H, Ls> where Ls: Prefix<Rs> {}

/// Constraint which either holds if a [trait@TList] is a prefix of `Other` or if `Other` is a prefix of this [trait@TList].
///
/// Relaxed variant of [Prefix].
///
/// ```rust
/// use tlist::*;
/// use typenum::consts::{U1, U2, U3, U4, U42};
///
/// static_assertions::assert_impl_all!(TList![U1, U2]: EitherPrefix<TList![U1, U2, U3, U4]>);
/// static_assertions::assert_impl_all!(TList![U1, U2, U3, U4]: EitherPrefix<TList![U1, U2]>);
/// static_assertions::assert_not_impl_any!(TList![U42]: EitherPrefix<TList![U1, U2, U3, U4]>);
/// static_assertions::assert_not_impl_any!(TList![U1, U2, U3, U4]: EitherPrefix<TList![U4, U3, U2, U1]>);
/// ```
pub trait EitherPrefix<Other: TList> {}
// eitherPrefix [] [] == true
impl EitherPrefix<TNil> for TNil {}

// eitherPrefix [] (f : gs) == true
impl<F, GS: TList> EitherPrefix<TCons<F, GS>> for TNil {}

// eitherPrefix (f : fs) [] == true
impl<F, FS: TList> EitherPrefix<TNil> for TCons<F, FS> {}

// eitherPrefix (f : fs) (g : gs) == true
impl<F, FS: TList, GS: TList> EitherPrefix<TCons<F, GS>> for TCons<F, FS> where FS: EitherPrefix<GS> {}

#[cfg(test)]
pub mod tests {
    // Since all of this is type-level code,
    // these tests run at compile-time :-).
    use super::*;
    use static_assertions::assert_type_eq_all as assert_type_eq;
    use typenum::consts::*;

    #[test]
    fn first() {
        assert_type_eq!(U1, First<TList![U1, U2]>);
    }

    #[test]
    fn rest() {
        assert_type_eq!(TList![U2], Rest<TList![U1, U2]>);
    }

    #[test]
    fn last() {
        assert_type_eq!(U2, Last<TList![U1, U2]>);
        assert_type_eq!(U1, Last<TList![U1]>);
    }

    #[test]
    fn inits() {
        assert_type_eq!(TList![U1, U2], Inits<TList![U1, U2, U3]>);
        assert_type_eq!(TList![], Inits<TList![U10]>);
    }

    #[test]
    fn concat() {
        assert_type_eq!(TList![U1, U2, U3], Concat<TList![U1], TList![U2, U3]>);
    }

    #[test]
    fn reverse() {
        assert_type_eq!(TCons<U3, TCons<U2, TCons<U1, TNil>>>, Reverse<TCons<U1, TCons<U2, TCons<U3, TNil>>>>);
    }

    #[test]
    fn len() {
        assert_type_eq!(U0, Len<TList![]>);
        assert_type_eq!(U1, Len<TList![usize]>);
        assert_type_eq!(U2, Len<TList![i32, usize]>);
        assert_type_eq!(
            U10,
            Len<TList![char, char, char, char, char, char, char, char, char, char]>
        );
    }

    #[test]
    fn is_empty() {
        assert_type_eq!(B1, IsEmpty<TList![]>);
        assert_type_eq!(B0, IsEmpty<TList![i32]>);
    }
}
