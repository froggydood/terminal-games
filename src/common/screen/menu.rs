use std::ops::Range;

use console::Key;

use super::util::*;
use super::boxes::*;
use super::text::*;

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

pub fn get_window_scroll_items_range<T>(items: &Vec<T>, selected_index: usize, max_window_size: usize) -> Range<usize> {
	let selected_offset = max_window_size / 2;
	let windows = items.windows(max_window_size);
	let window_len = windows.len();
	let mut window_i = 0;
	for _ in windows {
		if selected_index < selected_offset as usize && window_i == 0 {
			return window_i..(window_i + max_window_size);
		} else if selected_index >= selected_offset as usize && window_i == selected_index - selected_offset as usize {
			return window_i..(window_i + max_window_size);
		} else if selected_index + selected_offset >= items.len() - 1 && window_i == window_len - 1 {
			return window_i..(window_i + max_window_size);
		};
		window_i += 1;
	};
	0..items.len()
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
		Some(item) => item.label.len() as u16 + 4,
		None => 5
	};
	if title.len() as u16 > max_length {max_length = title.len() as u16 + 2};
	clear_screen();

	let max_window_items = 7;
	let size = (
		std::cmp::min(max_length + 2, term_size.0 - 4),
		std::cmp::min(std::cmp::min(max_window_items, items.len()) as u16 + 3, term_size.1)
	);

	let items_window_range = get_window_scroll_items_range(items, selected_index, max_window_items);
	let min = items_window_range.clone().min().unwrap();
	let items_window = &items[items_window_range];

	let coords = get_centered_coords(size);
	let mut box_print = BoxPrint::new(size);
	box_print
		.at_coords(coords)
		.set_border(true);
	box_print.print();

	TextPrint::new(title, (coords.0 + 2, coords.1 + 1))
		.add_prefix(&termion::style::Bold)
		.add_prefix(&termion::style::Underline)
		.print();
	let mut i = 0;
	for item in items_window {
		let item_index = i + min;
		let label: &str;
		let unselected_label = format!("  {}", item.label);
		let selected_label = format!("{}{} {}{}", termion::style::Bold, ">", &item.label, termion::style::Reset);
		if item_index == selected_index  {
			label = &selected_label
		} else {
			label = &unselected_label
		}
		print_at(label, (box_print.coords.0 + 2, box_print.coords.1 + 2 + i as u16));
		i += 1;
	}
	cursor_to_end();
}

pub fn draw_menu(items: &Vec<MenuItem>, title: &str) -> String {
	let mut selected_index = 0;
	let term = console::Term::stdout();
	loop {
		draw_menu_render(items, title, selected_index);
		let input = term.read_key();
		match input {
			Ok(Key::ArrowUp) => {
				let new_index: usize;
				if selected_index == 0 {new_index = 0}
				else {new_index = selected_index - 1}
				selected_index = new_index
			},
			Ok(Key::ArrowDown) => {
				let new_index: usize;
				if selected_index == items.len() - 1 {new_index = selected_index}
				else {new_index = selected_index + 1}
				selected_index = new_index
			},
			Ok(Key::Enter) => {
				break;
			}
			_ => {}
		}
	};
	items[selected_index].value.to_owned()
}