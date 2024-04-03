use termion::*;
use termion::color::*;
use super::util::*;
use super::text::*;

pub struct BoxPrint<'a> {
	pub coords: (u16, u16),
	pub size: (u16, u16),
	pub has_border: bool,
	pub fill_col: Option<&'a dyn Color>
}

impl<'a> BoxPrint<'a> {
	pub fn new(size: (u16, u16)) -> BoxPrint<'a> {
		let term_size = terminal_size().unwrap();
		let new_size = (
			std::cmp::min(size.0, term_size.0),
			std::cmp::min(size.1, term_size.1),
		);
		let coords = get_centered_coords(size);
		BoxPrint {
			coords,
			size: new_size,
			has_border: true,
			fill_col: None
		}
	}
	pub fn at_coords(self: &mut Self, coords: (u16, u16)) -> &mut Self {
		self.coords = coords;
		self
	}
	pub fn set_border(self: &mut Self, has_border: bool) -> &mut Self {
		self.has_border = has_border;
		self
	}
	pub fn set_fill(self: &mut Self, fill: &'a dyn Color) -> &mut Self {
		self.fill_col = Some(fill);
		self
	}
	pub fn remove_fill(self: &mut Self) -> &mut Self {
		self.fill_col = None;
		self
	}
	pub fn draw_border(self: &Self) {
		let (w, h) = self.size;
		let (x, y) = self.coords;
		// Draw corners
		print_at_with_cols("┌", (x, y), self.fill_col, None);
		print_at_with_cols("┐", (x+w-1, y), self.fill_col, None);
		print_at_with_cols("└", (x, y+h-1), self.fill_col, None);
		print_at_with_cols("┘", (x+w-1, y+h-1), self.fill_col, None);
	
		// Draw lines
		print_at_with_cols(&repeat_str("─", w-2), (x+1, y), self.fill_col, None);
		print_at_with_cols(&repeat_str("─", w-2), (x+1, y+h-1), self.fill_col, None);
		for i in 1..=(h-2) {
			print_at_with_cols("│", (x, y+i), self.fill_col, None);
			print_at_with_cols("│", (x+w-1, y+i), self.fill_col, None);
		}
	}
	pub fn print(self: &Self) {
		for _ in 1..=(self.size.1-2) {
			print_at_with_cols(&repeat_str(" ", self.size.0-1), self.coords, self.fill_col, None);
		}
		if self.has_border {self.draw_border()};
	}
}