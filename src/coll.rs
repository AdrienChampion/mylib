//! Helpers on collections.

use std::iter::Iterator ;




/// Adds one element at the end of an iterator.
pub struct ChainOne<Elem, I> {
  // The iterator.
  iter: I,
  // The element at the end of it.
  and_then: Option<Elem>,
}
impl<Elem, I> Iterator for ChainOne<Elem, I>
where I: Iterator<Item = Elem> {
  type Item = Elem ;
  fn next(& mut self) -> Option<Elem> {
    let next = self.iter.next() ;
    if next.is_some() { next } else {
      let mut res = None ;
      ::std::mem::swap( & mut self.and_then, & mut res ) ;
      res
    }
  }
}
/// Adds `chain_one` to iterators.
pub trait ChainOneExt<Elem>: Sized {
  /// Chains one element at the end of an iterator.
  ///
  /// ```
  /// # use mylib::coll::ChainOneExt ;
  /// let mut data = vec![ 7, 5, 3 ] ;
  /// data = data.into_iter().chain_one(2).collect() ;
  /// assert_eq!( vec![ 7, 5, 3, 2 ], data )
  /// ```
  ///
  /// ```
  /// # use mylib::coll::ChainOneExt ;
  /// let data = vec![ 7, 5, 3 ] ;
  /// let two = 2 ;
  /// let ref_data: Vec<_> = data.iter().chain_one(& two).collect() ;
  /// assert_eq!( vec![ & 7, & 5, & 3, & 2 ], ref_data )
  /// ```
  fn chain_one(self, Elem) -> ChainOne<Elem, Self> ;
}
impl<Elem, T> ChainOneExt<Elem> for T
where T: Iterator<Item = Elem> {
  fn chain_one(self, elem: Elem) -> ChainOne<Elem, Self> {
    ChainOne { iter: self, and_then: Some(elem) }
  }
}