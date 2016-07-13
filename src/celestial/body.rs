use sys::Colour;
use std::f64::consts::PI;

/// Celestial body
///
/// This trait should be implemented by any object that is contained in a star system or
/// generally happily floating about in space.
pub trait Body {
	/// The size of the celestial body. Return the radius in kilometers.
	fn size(&self) -> f32;
	/// The mass of the body in earth masses.
	fn mass(&self) -> f32;

	/// The volume of this body in km^3.
	// Usually it should be fine to leave the default and not implement this function seperately.
	fn volume(&self) -> f32 {
		// For our purposes, it will suffice to assume, the body is a *perfect sphere*.
		((4.0/3.0) * PI * self.size().powi(3) as f64) as f32
	}

	/// The average density of the body in kg/m^3.
	// Usually it should be fine to leave the default and not implement this function seperately.
	fn density(&self) -> f32 {
		// `self.mass()` yields the weight earth masses. To get kg we multiply by 5.972*10^24kg
		// `self.volume()` yields the volume in km^3. To get m^3 we multiply it by 1000.0^3
		// Hence we multiply by 5.972*10^12 kg in total
		(self.mass() as f64 / self.volume() as f64 * (5.972 as f64).powi(12)) as f32
	}

	/// The amount of light the body emits on itself.
	fn brightness(&self) -> f32;
	/// The percentage of light that is reflected back when it hits the body.
	fn reflectiveness(&self) -> f32;
	/// The colour of light leaving the body.
	fn colour(&self) -> &Colour;
}
