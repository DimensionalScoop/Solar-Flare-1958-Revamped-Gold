use super::{Body, System};
use sys::Colour;

pub struct Planet {
	size: f64,
	mass: f32,
	colour: Colour
}

impl Planet {
	/// Create a new planet with the values provided.
	pub fn new(size: f64, mass: f32, colour: Colour) -> Planet {
		Planet {
			size: size,
			mass: mass,
			colour: colour
		}
	}

	/// Create a new planet with random values.
	///
	/// This requires a system in order for the planet to
	pub fn random(system: &System) -> Planet {
		// XXX: This is simply an approximation of earth, which is quite clearly *not* a randomly
		// generated planet.
		Planet {
			size: 6378.16,
			mass: 1.0,
			colour: Colour::from_rgb((92, 159, 222))
		}
	}
}

impl Body for Planet {
	fn size(&self) -> f64 { self.size }
	fn mass(&self) -> f32 { self.mass }
	// TODO: We might want to adjust brightness later, when the planet is inhabited by a
	// sufficiently developed civilisation.
	fn brightness(&self) -> f32 { 0.0 }
	fn reflectiveness(&self) -> f32 { 0.35 } // Modeled after the albedo coefficient of earth.
	fn colour(&self) -> &Colour { &self.colour }
}
