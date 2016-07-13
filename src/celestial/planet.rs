use super::{Body, System};
use sys::Colour;
// XXX: Make sure, that there is always just one instance of a Planet around.
// This can be done by checking against a very simple database, when creating a Planet.
// This is necessary, since system simply compares the pointers to determine if a Planet is the
// one that is beeing looked for..
pub struct Planet {
	name: String,
	size: f32,
	mass: f32,
	colour: Colour
}

impl Planet {
	/// Create a new planet with the values provided.
	pub fn new(name: &str, size: f32, mass: f32, colour: Colour) -> Planet {
		Planet {
			name: name.to_string(),
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
			name: "Kummerliebe".to_string(),
			size: 6378.16,
			mass: 1.0,
			colour: Colour::from_rgb((92, 159, 222))
		}
	}

	// Get the name of the planet.
	pub fn name(&self) -> String { self.name.clone() }
}

impl Body for Planet {
	fn size(&self) -> f32 { self.size }
	fn mass(&self) -> f32 { self.mass }
	// TODO: We might want to adjust brightness later, when the planet is inhabited by a
	// sufficiently developed civilisation.
	fn brightness(&self) -> f32 { 0.0 }
	fn reflectiveness(&self) -> f32 { 0.35 } // Modeled after the albedo coefficient of earth.
	fn colour(&self) -> &Colour { &self.colour }
}
