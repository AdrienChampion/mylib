#![doc = r#"
Strongly-typed, zero-cost indices wrapping integers.

Nothing in this module is meant to be used directly. The
[`wrap_usize`](../../macro.wrap_usize.html) does all the work.

Typically used when storing values of some type, say `Term`, in an array.
Usually some things will be associated with these terms (like the term's *free
variables*), which one would usually put in another array understood as using
the same indices as the term array.

But when doing this for more than one type, there is a strong risk of using a
*term index* for something else. This module wraps `usize`s and gives
specialized collections making it impossible to mix indices statically.

**NB**: the wrappers use the trivial hash function for speed since this library
was not written for doing web-oriented things.

"#]

use std::hash::Hash ;
use common::hash::* ;

use self::hash::BuildHashUsize ;
// use self::hash::{ BuildHashUsize, BuildHashU64 } ;

#[doc = r#"Optimal trivial hash for `usize`s and `u64`s. The former is used for
wrapped indices, the latter for hashconsed things.

**NEVER USE THIS MODULE DIRECTLY. ONLY THROUGH THE `wrap_usize` MACRO.**

This is kind of unsafe, in a way. The hasher will cause logic errors if asked
to hash anything else than what it was supposed to hash.

In `debug`, this is actually checked each time something is hashed. This check
is of course deactivated in `release`.
"#]
mod hash {
  use std::hash::{ Hasher, BuildHasher } ;

  use consts::usize_bytes ;

  /// Empty struct used to build `HashUsize`.
  #[derive(Clone)]
  pub struct BuildHashUsize {}
  impl BuildHasher for BuildHashUsize {
    type Hasher = HashUsize ;
    fn build_hasher(& self) -> HashUsize {
      HashUsize { buf: [0 ; usize_bytes] }
    }
  }
  impl Default for BuildHashUsize {
    fn default() -> Self {
      BuildHashUsize {}
    }
  }

  /// Trivial hasher for `usize`. **This hasher is only for hashing `usize`s**.
  pub struct HashUsize {
    buf: [u8 ; usize_bytes]
  }
  impl HashUsize {
    /// Checks that a slice of bytes has the length of a `usize`. Only active
    /// in debug.
    #[cfg(debug)]
    #[inline(always)]
    fn test_bytes(bytes: & [u8]) {
      if bytes.len() != usize_bytes {
        panic!(
          "[illegal] `HashUsize::hash` \
          called with non-`usize` argument ({} bytes, expected {})",
          bytes.len(), usize_bytes
        )
      }
    }
    /// Checks that a slice of bytes has the length of a `usize`. Only active
    /// in debug.
    #[cfg( not(debug) )]
    #[inline(always)]
    fn test_bytes(_: & [u8]) {}
  }
  impl Hasher for HashUsize {
    fn finish(& self) -> u64 {
      unsafe {
        ::std::mem::transmute(self.buf)
      }
    }
    fn write(& mut self, bytes: & [u8]) {
      Self::test_bytes(bytes) ;
      for n in 0..usize_bytes {
        self.buf[n] = bytes[n]
      }
    }
  }

  // /// Empty struct used to build `HashU64`.
  // #[derive(Clone)]
  // pub struct BuildHashU64 {}
  // impl BuildHasher for BuildHashU64 {
  //   type Hasher = HashU64 ;
  //   fn build_hasher(& self) -> HashU64 {
  //     HashU64 { buf: [0 ; 8] }
  //   }
  // }
  // impl Default for BuildHashU64 {
  //   fn default() -> Self {
  //     BuildHashU64 {}
  //   }
  // }

  // /// Trivial hasher for `usize`. **This hasher is only for hashing `usize`s**.
  // pub struct HashU64 {
  //   buf: [u8 ; 8]
  // }
  // impl HashU64 {
  //   /// Checks that a slice of bytes has the length of a `usize`. Only active
  //   /// in debug.
  //   #[cfg(debug)]
  //   #[inline(always)]
  //   fn test_bytes(bytes: & [u8]) {
  //     if bytes.len() != 8 {
  //       panic!(
  //         "[illegal] `HashU64::hash` \
  //         called with non-`u64` argument ({} bytes, expected {})",
  //         bytes.len(), 8
  //       )
  //     }
  //   }
  //   /// Checks that a slice of bytes has the length of a `usize`. Only active
  //   /// in debug.
  //   #[cfg( not(debug) )]
  //   #[inline(always)]
  //   fn test_bytes(_: & [u8]) {}
  // }
  // impl Hasher for HashU64 {
  //   fn finish(& self) -> u64 {
  //     unsafe {
  //       ::std::mem::transmute(self.buf)
  //     }
  //   }
  //   fn write(& mut self, bytes: & [u8]) {
  //     Self::test_bytes(bytes) ;
  //     for n in 0..8 {
  //       self.buf[n] = bytes[n]
  //     }
  //   }
  // }
}

/// Trait letting `HashMap` and `HashSet` be created with `HashUsize`.
pub trait CanNew {
  /// Creates a new thing.
  #[inline]
  fn new() -> Self ;
  /// Creates a new thing with a capacity.
  #[inline]
  fn with_capacity(capacity: usize) -> Self ;
}
impl<K: PartialEq + Eq + Hash, V> CanNew for HashMap<K, V, BuildHashUsize> {
  fn new() -> Self {
    Self::with_hasher(BuildHashUsize {})
  }
  fn with_capacity(capacity: usize) -> Self {
    Self::with_capacity_and_hasher(capacity, BuildHashUsize {})
  }
}
impl<T: PartialEq + Eq + Hash> CanNew for HashSet<T, BuildHashUsize> {
  fn new() -> Self {
    Self::with_hasher(BuildHashUsize {})
  }
  fn with_capacity(capacity: usize) -> Self {
    Self::with_capacity_and_hasher(capacity, BuildHashUsize {})
  }
}
// impl<K: PartialEq + Eq + Hash, V> CanNew for HashMap<K, V, BuildHashU64> {
//   fn new() -> Self {
//     Self::with_hasher(BuildHashU64 {})
//   }
//   fn with_capacity(capacity: usize) -> Self {
//     Self::with_capacity_and_hasher(capacity, BuildHashU64 {})
//   }
// }
// impl<T: PartialEq + Eq + Hash> CanNew for HashSet<T, BuildHashU64> {
//   fn new() -> Self {
//     Self::with_hasher(BuildHashU64 {})
//   }
//   fn with_capacity(capacity: usize) -> Self {
//     Self::with_capacity_and_hasher(capacity, BuildHashU64 {})
//   }
// }


pub type IntSet<Int> = HashSet<Int, BuildHashUsize> ;
pub type IntHashMap<Int, V> = HashMap<Int, V, BuildHashUsize> ;


#[doc = r#"Wraps a `usize` into a struct (zero-cost). Also generates the
relevant collections indexed by the wrapper.

- implements `Deref` and `From` for `usize`,
- implements `Debug`, `Clone`, `Copy`, `PartialOrd`, `Ord`, `PartialEq`,
  `Eq`, `Hash` and `Display`.

Can also generate a range structure allowing to iterate over a range of
indices.

For more details see [the example](safe/int/examples/index.html).

# Usage

Most basic use-case is simply to wrap something:

```
# #[macro_use]
# extern crate mylib ;
# fn main() {}
wrap_usize!{
  #[doc = "Arity."]
  Arity
}
```

After the mandatory comment and wrapper identifier `Id`, one can add any
combination of the following tags using the syntax
`#[doc = <comment>] <tag>: <ident>` (see example below):

- `range`: structure to iterate between two `Id`s,
- `set`: alias type for a set `Id`s with 0-cost hashing,
- `hash map`: alias type for a hash map from `Id` to something with 0-cost
  hashing,
- `map`: wrapper around a vector forcing to use `Id` instead of `usize` to
  access elements.

Here is an example:

```
# #[macro_use]
# extern crate mylib ;
# fn main() {}
wrap_usize!{
  #[doc = "Index of a non-terminal."]
  NtIndex
  #[doc = "Range over `NtIndex`."]
  range: NtRange
  #[doc = "Set of non-terminal indices."]
  set: NtSet
  #[doc = "Map of non-terminal indices."]
  hash map: NtHMap
  #[doc = "Vector indexed by non-terminal indices."]
  map: NtMap with iter: NtMapIter
}
```
"#]
#[macro_export]
macro_rules! wrap_usize {
  // // Vector (internal).
  // ( internal $t:ident #[$cmt:meta] vec: $vec:ident $($tail:tt)* ) => (
  //   #[$cmt]
  //   pub type $vec = Vec<$t> ;
  //   wrap_usize!{ internal $t $($tail)* }
  // ) ;

  // Set (internal).
  ( |internal| $t:ident #[$cmt:meta] set: $set:ident $($tail:tt)* ) => (
    #[$cmt]
    pub type $set = $crate::safe::int::IntSet<$t> ;
    wrap_usize!{ |internal| $t $($tail)* }
  ) ;

  // Hash map (internal).
  ( |internal| $t:ident #[$cmt:meta] hash map: $map:ident $($tail:tt)* ) => (
    #[$cmt]
    pub type $map<T> = $crate::safe::int::IntHashMap<$t, T> ;
    wrap_usize!{ |internal| $t $($tail)* }
  ) ;

  // Range (internal).
  ( |internal| $t:ident #[$cmt:meta] range: $range:ident $($tail:tt)* ) => (
    #[$cmt]
    #[derive(Debug)]
    pub struct $range {
      start: $t,
      end: $t,
    }
    impl $range {
      /// Creates a new range.
      pub fn mk<
        T1: ::std::convert::Into<$t>,
        T2: ::std::convert::Into<$t>
      >(start: T1, end: T2) -> Self {
        $range { start: start.into(), end: end.into() }
      }
      /// Creates a range from `0` to something.
      pub fn zero_to<T: ::std::convert::Into<$t>>(end: T) -> Self {
        $range { start: 0.into(), end: end.into() }
      }
    }
    impl ::std::iter::Iterator for $range {
      type Item = $t ;
      fn next(& mut self) -> Option<$t> {
        if self.start >= self.end { None } else {
          let res = Some(self.start) ;
          self.start.val += 1 ;
          res
        }
      }
    }
    wrap_usize!{ |internal| $t $($tail)* }
  ) ;

  // Map: vector indexed by `$t` (internal).
  (
    |internal| $t:ident #[$cmt:meta]
    map: $map:ident with iter: $iter:ident
    $($tail:tt)*
  ) => (
    #[$cmt]
    #[derive(Debug)]
    pub struct $map<T> {
      vec: Vec<T>
    }
    impl<T: Clone> Clone for $map<T> {
      fn clone(& self) -> Self {
        $map { vec: self.vec.clone() }
      }
    }
    impl<T> $map<T> {
      /// Creates an empty vector from an existing one.
      #[inline]
      pub fn of(vec: Vec<T>) -> Self {
        $map { vec: vec }
      }
      /// Creates an empty vector with some capacity.
      #[inline]
      pub fn with_capacity(capacity: usize) -> Self {
        $map { vec: Vec::with_capacity(capacity) }
      }
      /// Number of elements in the map.
      #[inline]
      pub fn len(& self) -> usize {
        self.vec.len()
      }
      /// Capacity of the map.
      #[inline]
      pub fn capacity(& self) -> usize {
        self.vec.capacity()
      }
      /// The next free index (wrapped `self.len()`).
      #[inline]
      pub fn next_index(& self) -> $t {
        self.len().into()
      }
      /// Pushes an element.
      #[inline]
      pub fn push(& mut self, elem: T) {
        self.vec.push(elem)
      }
      /// Pops an element.
      #[inline]
      pub fn pop(& mut self) -> Option<T> {
        self.vec.pop()
      }
      /// Iterates over the elements.
      #[inline]
      pub fn iter(& self) -> ::std::slice::Iter<T> {
        self.vec.iter()
      }
      /// Iterates over the elements with the index.
      #[inline]
      pub fn index_iter<'a>(& 'a self) -> $iter<& 'a $map<T>>
      where T: 'a {
        $iter::mk_ref(self)
      }
      /// Iterates over the elements with the index.
      #[inline]
      pub fn into_index_iter(self) -> $iter<$map<T>> {
        $iter::mk(self)
      }
      /// Iterates over the elements (mutable version).
      #[inline]
      pub fn iter_mut(& mut self) -> ::std::slice::IterMut<T> {
        self.vec.iter_mut()
      }
      /// Shrinks the capacity as much as possible.
      #[inline]
      pub fn shrink(& mut self) {
        self.vec.shrink_to_fit()
      }
      /// Swap remove operation lifted from `Vec`.
      #[inline]
      pub fn swap_remove(& mut self, idx: $t) -> T {
        self.vec.swap_remove(* idx)
      }
    }
    impl<T: Clone> $map<T> {
      /// Creates an empty vector with some capacity.
      #[inline]
      pub fn of_elems(elem: T, size: usize) -> Self {
        $map { vec: vec![ elem ; size ] }
      }
    }
    impl<T: PartialEq> PartialEq for $map<T> {
      #[inline]
      fn eq(& self, other: & Self) -> bool {
        self.vec.eq( & other.vec )
      }
    }
    impl<T: Eq> Eq for $map<T> {}
    impl<T: ::std::hash::Hash> ::std::hash::Hash for $map<T> {
      #[inline]
      fn hash<H: ::std::hash::Hasher>(& self, state: & mut H) {
        self.vec.hash(state)
      }
    }
    impl<T> ::std::convert::From< Vec<T> > for $map<T> {
      #[inline]
      fn from(vec: Vec<T>) -> Self {
        $map { vec }
      }
    }
    impl<T> ::std::iter::IntoIterator for $map<T> {
      type Item = T ;
      type IntoIter = ::std::vec::IntoIter<T> ;
      #[inline]
      fn into_iter(self) -> ::std::vec::IntoIter<T> {
        self.vec.into_iter()
      }
    }
    impl<'a, T> ::std::iter::IntoIterator for & 'a $map<T> {
      type Item = & 'a T ;
      type IntoIter = ::std::slice::Iter<'a, T> ;
      #[inline]
      fn into_iter(self) -> ::std::slice::Iter<'a, T> {
        self.iter()
      }
    }
    impl<'a, T> ::std::iter::IntoIterator for & 'a mut $map<T> {
      type Item = & 'a mut T ;
      type IntoIter = ::std::slice::IterMut<'a, T> ;
      #[inline]
      fn into_iter(self) -> ::std::slice::IterMut<'a, T> {
        self.iter_mut()
      }
    }
    impl<T> ::std::iter::FromIterator<T> for $map<T> {
      fn from_iter<
        I: ::std::iter::IntoIterator<Item = T>
      >(iter: I) -> Self {
        $map { vec: iter.into_iter().collect() }
      }
    }
    impl<T> ::std::ops::Index<$t> for $map<T> {
      type Output = T ;
      fn index(& self, index: $t) -> & T {
        & self.vec[ index.get() ]
      }
    }
    impl<T> ::std::ops::IndexMut<$t> for $map<T> {
      fn index_mut(& mut self, index: $t) -> & mut T {
        & mut self.vec[ index.get() ]
      }
    }
    impl<T> ::std::ops::Index<
      ::std::ops::Range<usize>
    > for $map<T> {
      type Output = [T] ;
      fn index(& self, index: ::std::ops::Range<usize>) -> & [T] {
        self.vec.index(index)
      }
    }
    // impl<T> ::std::ops::Index<
    //   ::std::ops::RangeInclusive<usize>
    // > for $map<T> {
    //   type Output = [T] ;
    //   fn index(& self, index: ::std::ops::RangeInclusive<usize>) -> & [T] {
    //     self.vec.index(index)
    //   }
    // }
    impl<T> ::std::ops::Index<
      ::std::ops::RangeFrom<usize>
    > for $map<T> {
      type Output = [T] ;
      fn index(& self, index: ::std::ops::RangeFrom<usize>) -> & [T] {
        self.vec.index(index)
      }
    }
    impl<T> ::std::ops::Index<
      ::std::ops::RangeTo<usize>
    > for $map<T> {
      type Output = [T] ;
      fn index(& self, index: ::std::ops::RangeTo<usize>) -> & [T] {
        self.vec.index(index)
      }
    }
    // impl<T> ::std::ops::Index<
    //   ::std::ops::RangeToInclusive<usize>
    // > for $map<T> {
    //   type Output = [T] ;
    //   fn index(& self, index: ::std::ops::RangeToInclusive<usize>) -> & [T] {
    //     self.vec.index(index)
    //   }
    // }
    impl<T> ::std::ops::Deref for $map<T> {
      type Target = Vec<T> ;
      fn deref(& self) -> & Vec<T> {
        & self.vec
      }
    }
    /// Structure allowing to iterate over the elements of a map and their
    /// index.
    pub struct $iter<T> {
      cursor: $t,
      map: T,
    }
    impl<'a, T> $iter<& 'a $map<T>> {
      /// Creates an iterator starting at 0.
      fn mk_ref(map: & 'a $map<T>) -> Self {
        $iter { cursor: $t::zero(), map: map }
      }
    }
    impl<'a, T: 'a> ::std::iter::Iterator for $iter<& 'a $map<T>> {
      type Item = ($t, & 'a T) ;
      fn next(& mut self) -> Option< ($t, & 'a T) > {
        if self.cursor >= self.map.len() {
          None
        } else {
          let res = (self.cursor, & self.map[self.cursor]) ;
          self.cursor.inc() ;
          Some(res)
        }
      }
    }
    impl<T> $iter<$map<T>> {
      /// Creates an iterator starting at 0.
      fn mk(mut map: $map<T>) -> Self {
        map.vec.reverse() ;
        $iter { cursor: $t::zero(), map: map }
      }
    }
    impl<T> ::std::iter::Iterator for $iter<$map<T>> {
      type Item = ($t, T) ;
      fn next(& mut self) -> Option< ($t, T) > {
        if let Some(elem) = self.map.pop() {
          let res = (self.cursor, elem) ;
          self.cursor.inc() ;
          Some(res)
        } else {
          None
        }
      }
    }
    wrap_usize!{ |internal| $t $($tail)* }
  ) ;

  // Terminal case (internal).
  ( |internal| $t:ident ) => () ;

  // Entry point.
  (
    #[$cmt:meta] $t:ident
    $($tail:tt)*
  ) => (
    #[$cmt]
    #[derive(
      Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash
    )]
    pub struct $t {
      val: usize
    }
    impl $t {
      /// Wraps an int.
      #[inline]
      pub fn mk(val: usize) -> Self {
        $t { val: val }
      }
      /// Zero.
      #[inline]
      pub fn zero() -> Self {
        $t { val: 0 }
      }
      /// One.
      #[inline]
      pub fn one() -> Self {
        $t { val: 1 }
      }
      /// Accessor.
      #[inline]
      pub fn get(& self) -> usize {
        self.val
      }
      /// Increments the int.
      #[inline]
      pub fn inc(& mut self) {
        self.val += 1
      }
      /// Decrements the int.
      #[inline]
      pub fn dec(& mut self) {
        self.val -= 1
      }
    }
    impl ::std::convert::From<usize> for $t {
      #[inline]
      fn from(val: usize) -> Self {
        $t::mk(val)
      }
    }
    impl<'a> ::std::convert::From<& 'a usize> for $t {
      #[inline]
      fn from(val: & 'a usize) -> Self {
        $t::mk(* val)
      }
    }
    impl ::std::convert::Into<usize> for $t {
      #[inline]
      fn into(self) -> usize {
        self.val
      }
    }
    impl<'a> ::std::convert::Into<usize> for & 'a $t {
      #[inline]
      fn into(self) -> usize {
        self.val
      }
    }
    impl<T: ::std::convert::Into<usize>> ::std::ops::AddAssign<T> for $t {
      #[inline]
      fn add_assign(& mut self, rhs: T) {
        self.val += rhs.into()
      }
    }
    impl<T: ::std::convert::Into<usize>> ::std::ops::Add<T> for $t {
      type Output = $t ;
      #[inline]
      fn add(mut self, rhs: T) -> $t {
        self.val += rhs.into() ;
        self
      }
    }
    impl ::std::ops::Deref for $t {
      type Target = usize ;
      #[inline]
      fn deref(& self) -> & usize {
        & self.val
      }
    }
    impl ::std::fmt::Display for $t {
      #[inline]
      fn fmt(& self, fmt: & mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(fmt, "{}", self.val)
      }
    }
    impl ::std::cmp::PartialEq<usize> for $t {
      #[inline]
      fn eq(& self, int: & usize) -> bool {
        self.val.eq(int)
      }
    }
    impl ::std::cmp::PartialOrd<usize> for $t {
      #[inline]
      fn partial_cmp(& self, int: & usize) -> Option<
        ::std::cmp::Ordering
      > {
        self.val.partial_cmp(int)
      }
    }
    wrap_usize!{ |internal| $t $($tail)* }
  ) ;
}



#[doc = r#"Example of zero-cost wrapping. **Do not use this.**

This module is generated by

```
#[macro_use]
extern crate mylib ;

wrap_usize!{
  #[doc = "Index of a variable."]
  VarIndex
  #[doc = "Range over `VarIndex`."]
  range: VarRange
  #[doc = "Set of variable indices."]
  set: VarSet
  #[doc = "Map of variable indices."]
  hash map: VarHMap
  #[doc = "Vector indexed by variable indices."]
  map: VarMap with iter: VarMapIter
}
fn main() {
  use std::mem::size_of ;
  assert_eq!( size_of::<VarIndex>(), size_of::<usize>() ) ;
  assert_eq!( size_of::<VarRange>(), 2 * size_of::<usize>() ) ;
  assert_eq!( size_of::<VarSet>(), size_of::<VarSet>() ) ;
  assert_eq!( size_of::<VarMap<String>>(), size_of::<Vec<String>>() ) ;
}
```
"#]
pub mod examples {
  wrap_usize!{
    #[doc = "Index of a variable."]
    VarIndex
    #[doc = "Range over `VarIndex`."]
    range: VarRange
    #[doc = "Set of variable indices."]
    set: VarSet
    #[doc = "Map of variable indices."]
    hash map: VarHMap
    #[doc = "Vector indexed by variable indices."]
    map: VarMap with iter: VarMapIter
  }
}