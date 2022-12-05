#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
pub use alloc::vec::Vec;
#[cfg(not(feature = "alloc"))]
pub use vec_64::Vec;

mod vec_64;
