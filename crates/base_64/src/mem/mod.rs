#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(feature = "alloc", not(feature = "mini_vec")))]
pub use alloc::vec::Vec;

#[cfg(not(feature = "alloc"))]
pub use vec_64::Vec;

mod vec_64;
