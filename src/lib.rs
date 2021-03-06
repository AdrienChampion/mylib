//! Things I find useful.

#![forbid(missing_docs)]

/// Convenient re-exports.
pub mod common {
    /// Hash related things.
    pub mod hash {
        pub use std::collections::{HashMap, HashSet};
    }
    /// IO related things.
    pub mod io {
        pub use std::fs::{File, OpenOptions};
        pub use std::io::Error as IOError;
        pub use std::io::{BufRead, BufReader, Read, Write};
    }
}

/// Private module for constants used in the lib.
mod consts {
    #![allow(non_upper_case_globals)]
    /// Number of bytes in a `usize` (32 bit version).
    #[cfg(target_pointer_width = "32")]
    pub const usize_bytes: usize = 4;
    /// Number of bytes in a `usize` (64 bit version).
    #[cfg(target_pointer_width = "64")]
    pub const usize_bytes: usize = 8;
}

pub mod coll;
pub mod safe;

/// Performs something special on the first element of an iterator, and then works on the rest.
///
/// ```
/// #[macro_use]
/// extern crate mylib ;
///
/// fn main() {
///     let input = "lines\nseveral\nover\nmsg" ;
///
///     let expected = "msg\nover\nseveral\nlines" ;
///     let mut buff ;
///     let result = for_first!(
///         input.lines() => {
///             |fst_line| buff = format!("{}", fst_line),
///             then |nxt_line| buff = format!("{}\n{}", nxt_line, buff),
///             yild buff
///         } else "".into()
///     ) ;
///
///     assert_eq!( result, expected ) ;
///
///     // Alternatively:
///     let new_expected = "msg:over:one:line" ;
///     let mut buff = String::new() ;
///     for_first!(
///         result.lines() => {
///             |fst_line| buff.push_str(fst_line),
///             then |nxt_line| {
///                 buff.push(':') ;
///                 if nxt_line == "several" {
///                     buff.push_str("one")
///                 } else {
///                     buff.push_str(nxt_line)
///                 }
///             }
///         }
///     ) ;
///     let obsolete_s = buff.pop() ;
///     assert_eq!( obsolete_s, Some('s') ) ;
///
///     assert_eq!( buff, new_expected ) ;
/// }
/// ```
#[macro_export]
macro_rules! for_first {
    (
        $iter:expr => {
            |$fst:pat| $e_fst:expr,
            then |$thn:pat| $e_thn:expr,
            yild $e_yld:expr $(,)*
        } else $e_els:expr
    ) => {{
        let mut iter = $iter;
        if let Some($fst) = iter.next() {
            $e_fst;
            for $thn in iter {
                $e_thn
            }
            $e_yld
        } else {
            $e_els
        }
    }};
    (
        $iter:expr => {
            |$fst:pat| $e_fst:expr,
            then |$thn:pat| $e_thn:expr $(,)*
        }
    ) => {
        $crate::for_first! {
            $iter => {
                |$fst| $e_fst,
                then |$thn| $e_thn,
                yild ()
            } else ()
        }
    };
    (
        $iter:expr => {
            |$fst:pat| $e_fst:expr,
            then |$thn:pat| $e_thn:expr $(,)*
        } else $e_els:expr
    ) => {
        $crate::for_first! {
            $iter => {
                |$fst| $e_fst,
                then |$thn| $e_thn,
                yild ()
            } else $e_els
        }
    };
}

/// Helper to implement `Display` for a type.
///
/// ```
/// #[macro_use]
/// extern crate mylib ;
///
/// struct Blah { name: String, n: usize }
/// impl_fmt!{
///     Blah(self, fmt) {
///         write!(fmt, "{}({})", self.name, self.n)
///     }
/// }
///
/// fn main() {
///     let blah = Blah { name: "name".into(), n: 7 } ;
///     assert_eq!( format!("{}", blah), "name(7)" )
/// }
/// ```
#[macro_export]
macro_rules! impl_fmt {
    (
        $t:ident ($slf:ident, $fmt:ident) $b:block
    ) => (
        impl ::std::fmt::Display for $t {
            fn fmt(
                & $slf, $fmt: & mut ::std::fmt::Formatter
            ) -> ::std::fmt::Result $b
        }
    ) ;
}
