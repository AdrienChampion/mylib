//! Things I find useful.


/// Convenient re-exports.
pub mod common {
  /// Hash related things.
  pub mod hash {
    pub use std::collections::{ HashMap, HashSet } ;
  }
  /// IO related things.
  pub mod io {
    pub use std::fs::{ File, OpenOptions } ;
    pub use std::io::{ Read, Write, BufRead, BufReader } ;
    pub use std::io::Error as IOError ;
  }
}

/// Private module for constants used in the lib.
mod consts {
  #![allow(non_upper_case_globals)]
  /// Number of bytes in a `usize` (32 bit version).
  #[cfg(target_pointer_width = "32")]
  pub const usize_bytes: usize = 4 ;
  /// Number of bytes in a `usize` (64 bit version).
  #[cfg(target_pointer_width = "64")]
  pub const usize_bytes: usize = 8 ;
}

pub mod safe ;