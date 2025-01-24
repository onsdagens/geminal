#![no_std]

pub mod gemtext;
pub use gemtext::*;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
pub mod ui;
