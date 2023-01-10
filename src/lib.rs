#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

//! # Input formatting details
//! The data structures in this crate do not store any information about the exact formatting of TAS inputs.
//! The parsers can parse any input that the Celeste TAS mod would count as valid, which is quite flexible.
//! This means whitespace at the start of input lines is not necessary. Commas as separators are not either, and input characters can be lowercase.
//!
//! When converting from the rust data structures back to strings, the formatting will always be the same as how Celeste Studio would instantly format it.
//! Any other formatting style is a very niche use case and this crate does not account for it.

pub mod freeform_input;
pub mod reasonable_input;
