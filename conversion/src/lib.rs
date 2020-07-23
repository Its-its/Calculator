#![warn(warnings, rust_2018_idioms, unsafe_code, dead_code)]
#![warn(clippy::all)]
#![allow(clippy::missing_docs_in_private_items)]

pub mod error;
pub mod units;
pub mod quantity;

pub use error::{Error, Result};
pub use units::*;
pub use quantity::*;