//! Strongly-typed, zero-cost indices wrapping integers.
//!
//! Nothing in this module is meant to be used directly. The
//! [`wrap_usize`](../../macro.wrap_usize.html) does all the work.
//!
//! Typically used when storing values of some type, say `Term`, in an array.
//! Usually some things will be associated with these terms (like the term's
//! *free variables*), which one would usually put in another array understood
//! as using the same indices as the term array.
//!
//! But when doing this for more than one type, there is a strong risk of
//! using a *term index* for something else. This module wraps `usize`s and
//! gives specialized collections making it impossible to mix indices 
//! statically.
//!
//! **NB**: the wrappers use the trivial hash function for speed since this
//! library was not written for doing web-oriented things.

use std::hash::Hash ;
use common::hash::* ;

use self::hash::BuildHashUsize ;
// use self::hash::{ BuildHashUsize, BuildHashU64 } ;

/// Optimal trivial hash for `usize`s and `u64`s. The former is used for
/// wrapped indices, the latter for hashconsed things.
///
/// **NEVER USE THIS MODULE DIRECTLY. ONLY THROUGH THE `wrap_usize` MACRO.**
///
/// This is kind of unsafe, in a way. The hasher will cause logic errors if
/// asked to hash anything else than what it was supposed to hash.
///
/// In `debug`, this is actually checked each time something is hashed. This
/// check is of course deactivated in `release`.
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
    #[cfg(target_pointer_width = "64")]
    fn finish(& self) -> u64 {
      unsafe {
        ::std::mem::transmute(self.buf)
      }
    }
    #[cfg(target_pointer_width = "32")]
    fn finish(& self) -> u64 {
      unsafe {
        let int: u32 = ::std::mem::transmute(self.buf) ;
        int.into()
      }
    }
    fn write(& mut self, bytes: & [u8]) {
      Self::test_bytes(bytes) ;
      for n in 0..usize_bytes {
        self.buf[n] = bytes[n]
      }
    }
  }
}



/// Trait implemented by wrappers.
///
/// Implementing this trait iff the `usize` returned is a unique identifier
/// for `self`.
pub trait IntWrap {
  fn inner(& self) -> usize ;
}

use std::ops::{ Deref, DerefMut } ;

/// Wraps a hash set with a trivial hasher.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntHSet<Int: IntWrap + Hash + Eq> {
  set: HashSet<Int, BuildHashUsize>
}
impl<Int: IntWrap + Hash + Eq> Default for IntHSet<Int> {
  fn default() -> Self {
    IntHSet { set: HashSet::default() }
  }
}
impl<Int: IntWrap + Hash + Eq> IntHSet<Int> {
  /// Empty hash set.
  pub fn new() -> IntHSet<Int> {
    IntHSet {
      set: HashSet::with_hasher(BuildHashUsize {})
    }
  }
  /// Empty hash set with some capacity.
  pub fn with_capacity(capa: usize) -> IntHSet<Int> {
    IntHSet {
      set: HashSet::with_capacity_and_hasher(capa, BuildHashUsize {})
    }
  }
  /// An iterator visiting all elements.
  #[inline]
  pub fn iter(& self) -> ::std::collections::hash_set::Iter<Int> {
    self.set.iter()
  }
}
impl<'a, Int> IntoIterator for & 'a IntHSet<Int>
where Int: IntWrap + Hash + Eq {
  type Item = & 'a Int ;
  type IntoIter = ::std::collections::hash_set::Iter<'a, Int> ;
  fn into_iter(self) -> Self::IntoIter {
    (& self.set).into_iter()
  }
}
impl<Int> IntoIterator for IntHSet<Int>
where Int: IntWrap + Hash + Eq {
  type Item = Int ;
  type IntoIter = ::std::collections::hash_set::IntoIter<Int> ;
  fn into_iter(self) -> Self::IntoIter {
    self.set.into_iter()
  }
}
impl<Int> ::std::iter::FromIterator<Int> for IntHSet<Int>
where Int: IntWrap + Hash + Eq {
  fn from_iter<I: IntoIterator<Item = Int>>(iter: I) -> Self {
    IntHSet {
      set: HashSet::from_iter(iter)
    }
  }
}
impl<Int> ::std::iter::Extend<Int> for IntHSet<Int>
where Int: IntWrap + Hash + Eq {
  fn extend<I: IntoIterator<Item = Int>>(& mut self, iter: I) {
    self.set.extend(iter)
  }
}
impl<'a, Int> ::std::iter::Extend<& 'a Int> for IntHSet<Int>
where Int: 'a + IntWrap + Hash + Eq + Copy {
  fn extend<I: IntoIterator<Item = & 'a Int>>(& mut self, iter: I) {
    self.set.extend(iter)
  }
}
impl<Int> Deref for IntHSet<Int>
where Int: IntWrap + Hash + Eq {
  type Target = HashSet<Int, BuildHashUsize> ;
  fn deref(& self) -> & HashSet<Int, BuildHashUsize> {
    & self.set
  }
}
impl<Int> DerefMut for IntHSet<Int>
where Int: IntWrap + Hash + Eq {
  fn deref_mut(& mut self) -> & mut HashSet<Int, BuildHashUsize> {
    & mut self.set
  }
}

/// Wraps a hash map with a trivial hasher.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntHMap<Int: IntWrap + Hash + Eq, V> {
  map: HashMap<Int, V, BuildHashUsize>
}
impl<Int: IntWrap + Hash + Eq, V> Default for IntHMap<Int, V> {
  fn default() -> Self {
    IntHMap { map: HashMap::default() }
  }
}
impl<Int: IntWrap + Hash + Eq, V: Hash> Hash for IntHMap<Int, V> {
  fn hash<H>(& self, state: & mut H) where H: ::std::hash::Hasher {
    for (key, val) in self {
      state.write_usize( key.inner() ) ;
      val.hash(state)
    }
  }
}
impl<Int: IntWrap + Hash + Eq, V> IntHMap<Int, V> {
  /// Empty hash map.
  pub fn new() -> IntHMap<Int, V> {
    IntHMap {
      map: HashMap::with_hasher(BuildHashUsize {})
    }
  }
  /// Empty hash map with some capacity.
  pub fn with_capacity(capa: usize) -> IntHMap<Int, V> {
    IntHMap {
      map: HashMap::with_capacity_and_hasher(capa, BuildHashUsize {})
    }
  }
  /// An iterator visiting all elements.
  #[inline]
  pub fn iter(& self) -> ::std::collections::hash_map::Iter<
    Int, V
  > {
    self.map.iter()
  }
  /// An iterator visiting all elements.
  #[inline]
  pub fn iter_mut(& mut self) -> ::std::collections::hash_map::IterMut<
    Int, V
  > {
    self.map.iter_mut()
  }
}
impl<'a, Int, V> IntoIterator for & 'a IntHMap<Int, V>
where Int: IntWrap + Hash + Eq {
  type Item = (& 'a Int, & 'a V) ;
  type IntoIter = ::std::collections::hash_map::Iter<'a, Int, V> ;
  fn into_iter(self) -> Self::IntoIter {
    (& self.map).into_iter()
  }
}
impl<'a, Int, V> IntoIterator for & 'a mut IntHMap<Int, V>
where Int: IntWrap + Hash + Eq {
  type Item = (& 'a Int, & 'a mut V) ;
  type IntoIter = ::std::collections::hash_map::IterMut<'a, Int, V> ;
  fn into_iter(self) -> Self::IntoIter {
    (& mut self.map).into_iter()
  }
}
impl<Int, V> IntoIterator for IntHMap<Int, V>
where Int: IntWrap + Hash + Eq {
  type Item = (Int, V) ;
  type IntoIter = ::std::collections::hash_map::IntoIter<Int, V> ;
  fn into_iter(self) -> Self::IntoIter {
    self.map.into_iter()
  }
}
impl<Int, V> ::std::iter::FromIterator<(Int, V)> for IntHMap<Int, V>
where Int: IntWrap + Hash + Eq {
  fn from_iter<I: IntoIterator<Item = (Int, V)>>(iter: I) -> Self {
    IntHMap {
      map: HashMap::from_iter(iter)
    }
  }
}
impl<Int, V> ::std::iter::Extend<(Int, V)> for IntHMap<Int, V>
where Int: IntWrap + Hash + Eq {
  fn extend<I: IntoIterator<Item = (Int, V)>>(& mut self, iter: I) {
    self.map.extend(iter)
  }
}
impl<Int, V> Deref for IntHMap<Int, V>
where Int: IntWrap + Hash + Eq {
  type Target = HashMap<Int, V, BuildHashUsize> ;
  fn deref(& self) -> & HashMap<Int, V, BuildHashUsize> {
    & self.map
  }
}
impl<Int, V> DerefMut for IntHMap<Int, V>
where Int: IntWrap + Hash + Eq {
  fn deref_mut(& mut self) -> & mut HashMap<Int, V, BuildHashUsize> {
    & mut self.map
  }
}


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
    pub type $set = $crate::safe::int::IntHSet<$t> ;
    wrap_usize!{ |internal| $t $($tail)* }
  ) ;

  // Hash map (internal).
  ( |internal| $t:ident #[$cmt:meta] hash map: $map:ident $($tail:tt)* ) => (
    #[$cmt]
    pub type $map<T> = $crate::safe::int::IntHMap<$t, T> ;
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
      pub fn new<
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
      /// Creates an empty map from an existing one.
      #[inline]
      pub fn of(vec: Vec<T>) -> Self {
        $map { vec: vec }
      }
      /// Creates an empty map.
      #[inline]
      pub fn new() -> Self {
        $map { vec: Vec::new() }
      }
      /// Creates an empty map with some capacity.
      #[inline]
      pub fn with_capacity(capacity: usize) -> Self {
        $map { vec: Vec::with_capacity(capacity) }
      }
      /// Clears a map.
      #[inline]
      pub fn clear(& mut self) {
        self.vec.clear()
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
        $iter::new(self)
      }
      /// Iterates over the elements (mutable version).
      #[inline]
      pub fn iter_mut(& mut self) -> ::std::slice::IterMut<T> {
        self.vec.iter_mut()
      }
      /// Shrinks the capacity as much as possible.
      #[inline]
      pub fn shrink_to_fit(& mut self) {
        self.vec.shrink_to_fit()
      }
      /// Swap from `Vec`.
      #[inline]
      pub fn swap(& mut self, a: $t, b: $t) {
        self.vec.swap(* a, *b)
      }
      // Swap remove from `Vec`.
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
        for elem in & self.vec {
          elem.hash(state)
        }
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
    #[derive(Clone)]
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
      fn new(mut map: $map<T>) -> Self {
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
    impl $crate::safe::int::IntWrap for $t {
      fn inner(& self) -> usize { self.val }
    }
    impl $t {
      /// Wraps an int.
      #[inline]
      pub fn new(val: usize) -> Self {
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
        $t::new(val)
      }
    }
    impl<'a> ::std::convert::From<& 'a usize> for $t {
      #[inline]
      fn from(val: & 'a usize) -> Self {
        $t::new(* val)
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