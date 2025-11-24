#![doc = include_str!("../../README.md")]

#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::alloc_instead_of_core)]

#![cfg_attr(feature = "no-std", no_std)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod hkt;
pub mod marker_classification;
#[cfg(feature = "transmute")]
pub mod transmute;
#[cfg(not(feature = "transmute"))]
pub(crate) mod transmute;
pub mod utils;

