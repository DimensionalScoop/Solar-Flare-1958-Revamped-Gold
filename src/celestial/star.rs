use super::Body;
use sys::Colour;

pub struct Star {
	size: f32,
	mass: f32,
	brightness: f32,
	/// The surface temperature in Kelvin.
	surface_temp: u32,
	colour: Colour
}

impl Star {
	pub fn new(size: f32, mass: f32, brightness: f32, surface_temp: u32) -> Star {

		// Roughly determine the colour the star must have using the surface temperature.
		let colour = Colour::from_rgb(match surface_temp {
			0...3700      => (254, 90, 81),
			3701...5200   => (255, 163, 80),
			5201...6000   => (255, 243, 161),
			6001...7500   => (252, 255, 212),
			7501...10000  => (248, 246, 255),
			10001...30000 => (203, 213, 255),
			_             => (154, 175, 255)
		});

		Star {
			size: size,
			mass: mass,
			brightness: brightness,
			surface_temp: surface_temp,
			colour: colour
		}
	}
}

impl Body for Star {
	fn size(&self) -> f32 { self.size }
	fn mass(&self) -> f32 { self.mass }
	fn brightness(&self) -> f32 { self.brightness }
	fn reflectiveness(&self) -> f32 { 0.0 } // To keep it simple, stars only emit light, but don't reflect it.
	fn colour(&self) -> &Colour { &self.colour }
}
