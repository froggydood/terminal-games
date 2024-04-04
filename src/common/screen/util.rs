use termion::cursor::*;
use termion::color::*;

pub fn repeat_str(original_str: &str, num: u16) -> String {
	let mut new_str = String::with_capacity((original_str.len() as u16 * num) as usize);
	for _ in 0..num {
		new_str.push_str(original_str);
	}
	return new_str
}

pub fn get_centered_coords(size: (u16, u16)) -> (u16, u16) {
	let (w, h) = termion::terminal_size().unwrap();
	return (
		w / 2 - size.0 / 2,
		h / 2 - size.1 / 2,
	)
}

pub fn print_at_with_cols(text: &str, coords: (u16, u16), bg: Option<&dyn Color>, fg: Option<&dyn Color>) {
	let mut bg_col: &dyn termion::color::Color = &termion::color::Reset;
	if let Some(col) = bg {
		bg_col = col;
	}
	let mut fg_col: &dyn termion::color::Color = &termion::color::Reset;
	if let Some(col) = fg {
		fg_col = col;
	}
	println!("{}{}{}{}{}{}", Goto(coords.0, coords.1), Bg(bg_col), Fg(fg_col), text, Bg(Reset), Fg(Reset));
}

pub fn print_at(text: &str, coords: (u16, u16)) {
	println!("{}{}", Goto(coords.0, coords.1), text);
}

pub fn clear_screen() {
	let term_size = termion::terminal_size().unwrap();
	for i in 1..=(term_size.1-1) {
		println!("{}{}", termion::cursor::Goto(1, i), termion::clear::CurrentLine);
	}
}

pub fn cursor_to_end() {
	let term_size = termion::terminal_size().unwrap();
	println!("{}", termion::cursor::Goto(1, term_size.1 - 1))
}