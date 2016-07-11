use super::Body;
use sys::Colour;

pub struct Planet {
	size: f64,
	mass: f64,
	colour: Colour
}

impl Planet {
	/// Create a new planet with the values provided.
	pub fn new(size: f64, mass: f64, colour: Colour) -> Planet {
		Planet {
			size: size,
			mass: mass,
			colour: colour
		}
	}

	/// Create a new planet with random values.
	pub fn random() -> Planet {

	}
}

impl Body for Planet {
	fn size(&self) -> f64 { self.size }
	fn mass(&self) -> f64 { self.mass }
	// TODO: We might want to adjust brightness later, when the planet is inhabited by a
	// sufficiently developed civilisation.
	fn brightness(&self) -> f32 { 0.0 }
	fn reflectiveness(&self) -> f32 { 0.35 } // Modeled after the albedo coefficient of earth.
	fn colour(&self) -> &Colour { &self.colour }
}
