pub fn clear_screen(terminal_height: u16) {
	for i in 1..=(terminal_height-1) {
		println!("{}{}", termion::cursor::Goto(1, i), termion::clear::CurrentLine);
	}
}

pub fn print_text_at(text: &str, coords: (i16, i16), bounds: (u16, u16), term_size: (u16, u16)) {
	let offset_x = (term_size.0 - bounds.0) / 2;
	let offset_y = (term_size.1 - bounds.1) / 2 - 1;
	let x: u16 = std::cmp::max(coords.0 + offset_x as i16, 1) as u16;
	let y: u16 = std::cmp::max(coords.1 + offset_y as i16, 1) as u16;
	println!("{}{}", termion::cursor::Goto(x, y), text);
}

pub fn print_text_at_with_color(text: &str, coords: (u16, u16), color: &dyn termion::color::Color, bounds: (u16, u16), term_size: (u16, u16)) {
	print_text_at(&format!(
		"{}{}{}",
		termion::color::Fg(color),
		text,
		termion::color::Fg(termion::color::Reset)
	), (coords.0 as i16, coords.1 as i16), bounds, term_size)
}

pub fn repeat_str(original_str: &str, num: u16) -> String {
	let mut new_str = String::with_capacity((original_str.len() as u16 * num) as usize);
	for _ in 0..num {
		new_str.push_str(original_str);
	}
	return new_str
}

pub fn draw_border(bounds: (u16, u16), term_size: (u16, u16)) {
	let (w, h) = bounds;
	// Draw corners
	print_text_at("┌", (1, 1), bounds, term_size);
	print_text_at("┐", ((w+2) as i16, 1), bounds, term_size);
	print_text_at("└", (1, (h+2) as i16), bounds, term_size);
	print_text_at("┘", ((w+2) as i16, (h+2) as i16), bounds, term_size);

	print_text_at(&repeat_str("─", w), (2, 1), bounds, term_size);
	print_text_at(&repeat_str("─", w), (2, (h+2) as i16), bounds, term_size);
	for i in 2..=(h+1) {
		print_text_at("│", (1, i as i16), bounds, term_size);
		print_text_at("│", ((w+2) as i16, i as i16), bounds, term_size);
	}
}