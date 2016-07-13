use std::u8;

/// Colour structure
///
/// This represents a colour with an optional alpha-channel.
#[derive(PartialEq)]
pub struct Colour {
	/// The red component
	r: u8,
	/// The green component
	g: u8,
	/// The blue component
	b: u8,
	/// The alpha component
	a: u8
}

impl Colour {
	/// Create a colour
	///
	/// Takes red, green and blue component of the color as argument with an optional space to
	/// satisfy the alpha-channel.
	/// In case no alpha value is provided, the colour defaults to full opacity.
	pub fn new(r: u8, g: u8, b: u8, a: Option<u8>) -> Colour {
		Colour {
			r: r,
			g: g,
			b: b,
			a: match a {
				Some(a) => a,
				None => u8::MAX
			}
		}
	}

	/// Create a new colour from an (r, g, b)-tuple.
	///
	/// This is simply a convenience function to accomodate uses where using the normal `new()`
	/// function would counteract readability.
	/// The opacity will be 100%.
	pub fn from_rgb((r, g, b): (u8, u8, u8)) -> Colour {
		Colour {
			r: r,
			g: g,
			b: b,
			a: u8::MAX
		}
	}

	/// Create a new colour from an (r, g, b, a)-tuple.
	///
	/// This is simply a convenience function to accomodate uses where using the normal `new()`
	/// function would counteract readability.
	pub fn from_rgba((r, g, b, a): (u8, u8, u8, u8)) -> Colour {
		Colour {
			r: r,
			g: g,
			b: b,
			a: a
		}
	}

	/// New colour from html Code
	///
	/// This creates a new colour object from the html representation. Using this representation,
	/// the opacity currently cannot be set, and the colour is created with full opacity.
	///
	/// # Failures
	/// In case the hml-code given is not a valid hexadecimal code, the function will return
	/// `None` in place of the Colour-Object.
	///
	/// # Examples
	///
	/// ```
	/// use colour::Colour;
	///
	/// let c = Colour::from_html("#00ab24");
	/// assert_eq!(c.rgb(), (0, 171, 36));
	/// ```
	pub fn from_html(code: &str) -> Option<Colour> {
		// Since the html code always starts with a leading '#', the total length is
		// 1 + 6 for 3*8 Bits
		if code.len() != 7 {
			return None;
		}

		let mut code = code.to_string();
		code.remove(0);
		let (r, rest) = code.split_at(2);
		let (g, b) = rest.split_at(2);

		let r = u8::from_str_radix(&r, 16);
		let g = u8::from_str_radix(&g, 16);
		let b = u8::from_str_radix(&b, 16);

		if r.is_ok() && g.is_ok() && b.is_ok() {
			Some (Colour {
				r: r.unwrap(),
				g: g.unwrap(),
				b: b.unwrap(),
				a: u8::MAX
			})
		}
		else {
			None
		}
	}

	/// Get the rgb tuple representing this colour.
	///
	/// # Failures
	/// This function will never fail in release mode. However, when running in debug mode, this
	/// function will panic if the colour is not at 100% opacity.
	pub fn rgb(&self) -> (u8, u8, u8) {
		debug_assert_eq!(self.a, u8::MAX);

		(self.r, self.g, self.b)
	}

	/// Get the rgba tuple representing this colour.
	pub fn rgba(&self) -> (u8, u8, u8, u8) {
		(self.r, self.g, self.b, self.a)
	}

	/// Get the html code representating this colour.
	pub fn html(&self) -> String {
		format!("#{:x}{:x}{:x}", self.r, self.g, self.b)
	}
}
