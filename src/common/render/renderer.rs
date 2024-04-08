use std::borrow::Borrow;
use termion::cursor::Goto;
use termion::*;
use termion::color::*;
use super::scene::*;
use rand::Rng;

pub struct Rasterized<'a> {
	pub screen_width: u16,
	pub screen_height: u16,
	pub image_data: &'a [u8]
}

pub trait Renderer {
	fn rasterize<'a>(self: &'a Self, scene: &'a Scene, image_buffer: &'a mut Vec<u8>) -> Rasterized;
	fn render(self: &Self, rasterized: Rasterized);
}

pub struct TerminalRenderer {
	pub screen_width: u16,
	pub screen_height: u16,
	pub color_map: [&'static dyn termion::color::Color; 256]
}

const COLORS: [(&'static dyn Color, u8); 16] = [
	(&Black, 0b00000000), (&Blue, 0b00001100),
	(&Cyan, 0b00111100), (&Green, 0b00110000),
	(&Magenta, 0b11001100), (&Red, 0b11000000),
	(&White, 0b11111100), (&Yellow, 0b11110000),
	(&LightBlue, 0b10101100), (&LightBlack, 0b01010100),
	(&LightCyan, 0b10111100), (&LightGreen, 0b10111000),
	(&LightMagenta, 0b11101100), (&LightRed, 0b11101100),
	(&LightWhite, 0b11111100), (&LightYellow, 0b11111000),
];

impl Default for TerminalRenderer {
	fn default() -> Self {
		let (w, h) = terminal_size().unwrap();
		let mut color_map: [&'static dyn Color; 256] = [&White; 256];

		(0..=255).for_each(|i| {
			let r = (i & 0b11000000) >> 6;
			let g = (i & 0b00110000) >> 4;
			let b = (i & 0b00001100) >> 2;
			let mut color = COLORS[0].0;
			let mut min_dist = 100;
			COLORS.iter().for_each(|(col, col_value)| {
				let cr = (col_value & 0b11000000) >> 6;
				let cg = (col_value & 0b00110000) >> 4;
				let cb = (col_value & 0b00001100) >> 2;
				let dist = cr.abs_diff(r) + cg.abs_diff(g) + cb.abs_diff(b);
				if dist < min_dist {
					min_dist = dist;
					color = col
				}
			});
			color_map[i as usize] = color
		});

		TerminalRenderer {
			screen_height: h,
			screen_width: w,
			color_map: color_map
		}
	}
}

impl Renderer for TerminalRenderer {
	fn rasterize<'a>(self: &'a Self, scene: &'a Scene, image_buffer: &'a mut Vec<u8>) -> Rasterized {
		let mut rng = rand::thread_rng();
		for x in 0..self.screen_width {
			for y in 0..self.screen_height {
				let data_index = (x + (x % self.screen_width) * y) as usize;
				image_buffer[data_index] = rng.gen_range(0..=255);
			}
		}
		Rasterized {
			screen_width: self.screen_width,
			screen_height: self.screen_height,
			image_data: image_buffer
		}
	}

	fn render(self: &Self, rasterized: Rasterized) {
		let mut text = "".to_owned();
		for (i, val) in rasterized.image_data.iter().enumerate() {
			let x = i % rasterized.screen_width as usize;
			let y = i / rasterized.screen_width as usize;
			let col = self.color_map[*val as usize];
			text = format!("{}{}{}", text, Fg(col), "#");
		}
		print!("{}{}", Goto(1, 1), text)
	}
}