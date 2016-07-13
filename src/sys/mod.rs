//! The system module
//!
//! This module contains all the relevant types and traits that are used throughout the project
//! and do not serve a purpose outside the inner workings of the program.
//! This means, types or traits that have any representation in the real or simulated world are
//! not part of this module.
pub mod colour;
pub use self::colour::Colour;

pub mod consts;
