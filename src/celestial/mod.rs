//! # Celestial Module
//!
//! This module contains the parts necessary to create, query and maintain planetary systems,
//! the planets themselves or any other celestial bodies.
pub mod body;
pub use self::body::Body;

pub mod planet;
pub use self::planet::Planet;

pub mod star;
pub use self::star::Star;

pub mod system;
pub use self::system::System;
