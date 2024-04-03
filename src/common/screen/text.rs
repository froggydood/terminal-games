use std::fmt::Display;

use termion::color::*;
use super::util::*;

pub struct TextPrint<'a> {
	pub text: &'a str,
	pub coords: (u16, u16),
	pub prefixes: Vec<&'a dyn Display>,
	pub fg_col: Option<&'a dyn Color>,
	pub bg_col: Option<&'a dyn Color>,
}

impl<'a> TextPrint<'a> {
	pub fn new(text: &'a str, coords: (u16, u16)) -> TextPrint<'a> {
		TextPrint {
			text,
			coords,
			prefixes: vec![],
			bg_col: None,
			fg_col: None
		}
	}
	pub fn color_fg(self: &mut Self, col: &'a dyn Color) -> &mut Self {
		self.fg_col = Some(col);
		self
	}
	pub fn color_bg(self: &mut Self, col: &'a dyn Color) -> &mut Self {
		self.bg_col = Some(col);
		self
	}
	pub fn add_prefix(self: &mut Self, prefix: &'a dyn Display) -> &mut Self {
		self.prefixes.push(prefix);
		self
	}
	pub fn print(self: &Self) {
		let mut prefix_str: String = "".to_owned();
		self.prefixes.iter().for_each(|prefix| {
			prefix_str = format!("{}{}", prefix_str, prefix);
		});
		print_at_with_cols(&format!("{}{}{}", prefix_str, self.text, termion::style::Reset), self.coords, self.bg_col, self.fg_col);
	}
}