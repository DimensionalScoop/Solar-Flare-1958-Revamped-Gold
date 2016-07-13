use super::{Body, Planet, Star};
use std::cmp::Ordering;
use sys::consts::*;

pub struct System {
	/// The planets of the system. The first parameter is the average distance from the systems
	/// center in astronomical units.
	// The planets are internally sorted, by their distance from the gravitational center.
	planets: Vec<(f32, Planet)>,
	star: Vec<Star>,
	clockwise_prograde: bool
}

impl System {
	/// Create a new System. The vector for "star" represents the possibly multiple objects that
	/// shall make up the center of the system.
	/// Clockwise prograde determines, if the prograde of this system is going to be clockwise
	/// as viewed from the north-pole of the star.
	///
	/// # Failures
	/// If the star for this system does not at least contain one celestial body, the system cannot
	/// be created and the function returns `None`.
	pub fn new(star: Vec<Star>, clockwise_prograde: bool) -> Option<System> {
		if star.len() < 1 {
			return None;
		}

		Some (System {
			planets: Vec::new(),
			star: star,
			clockwise_prograde: clockwise_prograde
		})
	}


	/// Add a planet to the system. Returns true, if the addition was successful, or false if the
	/// planet is not compatible with the system.
	/// `distance` describes the average radius of the orbit relative to the gravitational center
	/// of the system.
	pub fn add_planet(&mut self, planet: Planet, distance: f32) -> bool {
		// Search the placement of the planet, in case it can theoretically be inserted.
		if let Err(index) = self.planets.binary_search_by(|&(a, _)| a.partial_cmp(&distance).unwrap_or(Ordering::Equal)) {
			// No planet on this orbit yet, but could it clash with the planets right next to it.
			if let Some(&(ref o, ref p)) = self.planets.get(index - 1) {
				// The planet with the lower orbit could connect with the side nearer to the center.
				if o * f32::AU + p.size() >= distance * f32::AU - planet.size() {
					return false;
				}
			}
			if let Some(&(ref o, ref p)) = self.planets.get(index) {
				// The planet with the higher orbit could connect with the side further from the center.
				if o * f32::AU - p.size() <= distance * f32::AU + planet.size() {
					return false;
				}
			}

			// The planet can be added to the system without problem.
			self.planets.insert(index, (distance, planet));
			true
		}
		else {
			// There is a planet that already has the exact same orbit.
			false
		}
	}

	/// Get a planet of the system by its name.
	/// Returns `None` in case the planet does not exist, or is not part of this system.
	pub fn planet(&self, name: &str) -> Option<&Planet> {
		// TODO: It would probably better, to sort the planets of this system by name, not by size,
		// in case this function gets called more often. Inserting a planet would still just be O(n).
		for &(_, ref planet) in &self.planets {
			if &planet.name() == name {
				return Some(&planet);
			}
		}

		None
	}

	/// Get the height of a planet's orbit.
	pub fn orbit(&self, planet: &Planet) -> Option<f32> {
		for &(ref o, ref p) in &self.planets {
			if p as *const Planet == planet as *const Planet {
				return Some(*o);
			}
		}

		None
	}

	/// Get the orbital speed of a planet. The speed is given in m/s
	pub fn speed(&self, planet: &Planet) -> Option<f32> {
		for &(ref o, ref p) in &self.planets {
			if p as *const Planet == planet as *const Planet {
				// Calculate the complete mass all involved bodies have when added together.
				let mut mass: f64 = planet.mass() as f64;
				for ref s in &self.star {
					mass += s.mass() as f64;
				}

				// sqrt( G*M/r ) 
				return Some((f64::G * mass / (*o as f64 * f64::AU * 1000.0)).sqrt() as f32);
			}
		}

		None
	}
}
