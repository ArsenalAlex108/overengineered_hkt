#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::alloc_instead_of_core)]
#[macro_use]
mod macros;

pub mod hkt;
pub mod marker_classification;
#[cfg(feature = "transmute")]
pub mod transmute;
#[cfg(not(feature = "transmute"))]
pub(crate) mod transmute;
pub(crate) mod utils;

/// Re-export of [either::Either]
pub(crate) use either::Either;
