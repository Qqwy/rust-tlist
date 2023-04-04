# TList

Type-level linked lists for Rust.


These are useful if you need to keep track of a _list_ of types inside your type,
and manipulate them in generic ways, like looking at the first type in a list, concatenating lists, reversing the list, et cetera.



The easiest way to build a TList is to use the [TList!] macro:

```rust
use tlist::*;

type MyList = TList![String, usize, bool];
```

## Type-level functions

Manipulation of a [trait@TList] is done by using one of the many type aliases defined in the crate.
These are nice and readable aliases that internally use one of the many generic associated types (GATs) which are part of the definitions of the [trait@TList] and [NonEmpty] traits.

You can think of these type aliases as the type-level version of functions. Instead of normal functions, they run at compile time, on the type level:

```rust
use tlist::TList;
use static_assertions::assert_type_eq_all as assert_type_eq;


type Things = TList![String, usize, bool];

type Sgniht = tlist::Reverse<Things>;
assert_type_eq!(Sgniht, TList![bool, usize, String]);

type MoreThings = tlist::Concat<Things, TList![u8, u8, u8]>;
assert_type_eq!(MoreThings, TList![String, usize, bool, u8, u8, u8]);
```

This means that you can use them inside the `where` clauses of any types, traits or (normal) functions you define.
TList implements [Default] wich makes it very easy to add it as a field to a struct or enum.
(It is a [ZST](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts) so it takes up no size at runtime).

```rust
use tlist::TList;

#[derive(Debug)]
pub struct MyStruct<List: TList> {
  normal_data: String,
  special_data: List,
}

let foo = MyStruct::<TList![usize, bool]> {
  normal_data: "Hello".to_string(),
  special_data: Default::default()
};
println!("{:?}", foo);
```


## Compile-time safety

Attempting to do operations only defined on [NonEmpty] TLists on [Empty] TLists
results in an error at compile time:

```compile_fail
use tlist::TList;
use static_assertions::assert_type_eq_all as assert_type_eq;

type Boom = tlist::First<TList![]>;
assert_type_eq!(Boom, u8); // <- Compile error: Trait NonEmpty is not implemented for TNil
```

_Note that the compile error only happens on the second line, where we look at the output.
Rust performs type expansion lazily, so if you never use an 'impossible' result the compiler does not complain._

## Efficiency

[trait@TList]'s two constructors, [TNil] and [TCons] are both zero-size types ([ZSTs](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts)).
This means that any TList will be zero size as well and disappear completely before your program runs.

Because all of the calculations happen at compile-time, the runtime of your program is not affected at all.

## Compatibility

### no_std

TList only depends on `core` and as such is fully `no_std` compatible.
No features need to be disabled to turn on `no_std` support.

### MSRV

TList's Minimum Supported Rust Version is 1.65: The implementation makes pervasive use of GATs.
