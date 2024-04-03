use console::Key;

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

pub fn print_text_at_with_color(text: &str, coords: (i16, i16), color: &dyn termion::color::Color, bounds: (u16, u16), term_size: (u16, u16)) {
	print_text_at(&format!(
		"{}{}{}",
		termion::color::Fg(color),
		text,
		termion::color::Fg(termion::color::Reset)
	), coords, bounds, term_size)
}

pub fn cursor_to_end(term_height: u16) {
	println!("{}", termion::cursor::Goto(1, term_height-1))
}

pub fn print_text_at_with_bg_color(text: &str, coords: (i16, i16), color: &dyn termion::color::Color, bounds: (u16, u16), term_size: (u16, u16)) {
	print_text_at(&format!(
		"{}{}{}",
		termion::color::Bg(color),
		text,
		termion::color::Bg(termion::color::Reset)
	), coords, bounds, term_size)
}

pub fn repeat_str(original_str: &str, num: u16) -> String {
	let mut new_str = String::with_capacity((original_str.len() as u16 * num) as usize);
	for _ in 0..num {
		new_str.push_str(original_str);
	}
	return new_str
}

pub fn write_text(text: &str, bounds: (u16, u16), term_size: (u16, u16)) {
	let coords = (bounds.0 as i16 / 2 - (text.len() as i16 / 2) + 1, bounds.1 as i16 + 3);
	print_text_at(&text, coords, bounds, term_size);
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

pub struct MenuItem<'a> {
	label: &'a str,
	value: &'a str
}
impl<'a> MenuItem<'a> {
	pub fn new(label: &'a str, value: &'a str) -> MenuItem<'a> {
		MenuItem {
			label,
			value
		}
	}
}

pub fn draw_filled_box(size: (u16, u16), color: &dyn termion::color::Color, term_size: (u16, u16)) {
	let bg_char = "█";
	for y in 1..=size.1 {
		print_text_at_with_color(&repeat_str(bg_char, size.0), (1, y as i16), color, size, term_size);
	}
}

pub fn draw_menu_render(items: &Vec<MenuItem>, title: &str, selected_index: usize) {
	let term_size = termion::terminal_size().unwrap();
	let max_length_item = items.iter().reduce(|item, acc| {
		if item.label.len() > acc.label.len() {
			return item;
		}
		acc
	});
	let mut max_length: u16 = match max_length_item {
		Some(item) => item.label.len() as u16,
		None => 5
	};
	if title.len() as u16 > max_length {max_length = title.len() as u16};
	clear_screen(term_size.1);
	let size = (
		std::cmp::min(max_length + 2, term_size.0 - 4),
		std::cmp::min(std::cmp::min(items.len() as u16, term_size.1) + 1, term_size.1 - 4)
	);
	draw_border(size, term_size);
	print_text_at(title, ((size.1) as i16 - 1 + 1, size.1 as i16 - 1), size, term_size);
	for (i, item) in items.iter().enumerate() {
		let label: &str;
		let unselected_label = "  ".to_owned() + &item.label;
		let selected_label = format!("{}{} {}{}", termion::style::Bold, ">", &item.label, termion::style::Reset);
		if i == selected_index  {
			label = &selected_label
		} else {
			label = &unselected_label
		}
		print_text_at(label, (size.1 as i16, ((i as u16 + size.1) as i16) as i16), size, term_size);
	}
	cursor_to_end(term_size.1);
}

pub fn draw_menu(items: &Vec<MenuItem>, title: &str) -> String {
	let mut selected_index = 0;
	let term = console::Term::stdout();
	loop {
		draw_menu_render(items, title, selected_index);
		let input = term.read_key();
		match input {
			Ok(Key::ArrowUp) => {
				selected_index = ((selected_index as i16 - 1 + items.len() as i16) % items.len() as i16) as usize;
			},
			Ok(Key::ArrowDown) => {
				selected_index = (selected_index + 1) % items.len();
			},
			Ok(Key::Enter) => {
				break;
			}
			_ => {}
		}
	};
	items[selected_index].value.to_owned()
}