use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};
use rand::prelude::*;
use crate::common::game::*;
use crate::common::screen::*;

#[derive(PartialEq)]
enum Direction {
	Left,
	Right,
	Up,
	Down
}

struct GameState {
	head_direction: Direction,
	bodies: Vec<(u16, u16)>,
	food_locations: Vec<(u16, u16)>,
	bounds: (u16, u16),
	finished: bool,
	score: u32
}

fn draw_snakes(bodies: &Vec<(u16, u16)>, bounds: (u16, u16), term_size: (u16, u16)) {
	let body_char: &str = "█";
	for n in 0..bodies.len() {
		let (x, y) = bodies[n];
		print_text_at(body_char, ((x+1) as i16, (y+1) as i16), bounds, term_size);
	}
}

fn draw_food(food: &Vec<(u16, u16)>, bounds: (u16, u16), term_size: (u16, u16)) {
	let body_char: &str = "█";
	for n in 0..food.len() {
		let (x, y) = food[n];
		print_text_at_with_color(body_char, (x+1, y+1), &termion::color::Red, bounds, term_size);
	}	
}

fn write_screen(game_state: &GameState) {
	let term_size = termion::terminal_size().unwrap();
	clear_screen( term_size.1);
	draw_border(game_state.bounds, term_size);
	draw_snakes(&game_state.bodies, game_state.bounds, term_size);
	draw_food(&game_state.food_locations, game_state.bounds, term_size);
	write_game_text(&game_state, game_state.bounds, term_size);
	println!("{}", termion::cursor::Goto(1, term_size.1-1))
}

fn write_game_text(game_state: &GameState, bounds: (u16, u16), term_size: (u16, u16)) {
	let text: String;
	if game_state.finished {
		text = format!("Game over, score: {}, press any key to continue", game_state.score);
	} else {
		text = format!("Score: {}", game_state.score);
	}
	write_text(&text, bounds, term_size);
}

fn update_state(game_state: &mut GameState) {

	let last_index = game_state.bodies.len() - 1;
	
	for i in 0..(game_state.bodies.len()-1) {
		game_state.bodies[i].0 = game_state.bodies[i+1].0;
		game_state.bodies[i].1 = game_state.bodies[i+1].1;
	}

	match game_state.head_direction {
		Direction::Left => {game_state.bodies[last_index].0 -= 1;},
		Direction::Right => {game_state.bodies[last_index].0 += 1;},
		Direction::Down => {game_state.bodies[last_index].1 += 1;},
		Direction::Up => {game_state.bodies[last_index].1 -= 1;}
	}
}

fn check_on_food(state: &mut GameState) {
	let (x, y) = state.bodies.last().unwrap();
	let mut add = false;
	for i in 0..state.food_locations.len() {
		if *x == state.food_locations[i].0 && *y == state.food_locations[i].1 {
			add = true
		}
	}
	if add {
		let last = state.bodies[0];
		state.bodies.reverse();
		state.bodies.append(&mut vec![last.clone()]);
		state.bodies.reverse();
		state.food_locations = vec![generate_new_food(state.bounds, &state.bodies)];
		state.score += 1;
	}
}

fn generate_new_food(bounds: (u16, u16), ignore_locations: &Vec<(u16, u16)>) -> (u16, u16) {
	let (w, h) = bounds;
	let mut rng = rand::thread_rng();
	let mut x: u16;
	let mut y: u16;
	loop {
		x = rng.gen_range(1..=w);
		y = rng.gen_range(1..=h);
		let mut found = false;
		for i in 0..ignore_locations.len() {
			if ignore_locations[i].0 == x && ignore_locations[i].1 == y {
				found = true
			}
		}
		if !found {break;}
	}
	(x, y)
}

fn is_over(state: &GameState) -> bool {
	let head = state.bodies.last().unwrap();
	let same_as_head_vec = state.bodies.iter().filter(|body| {body.0 == head.0 && body.1 == head.1}).collect::<Vec<&(u16, u16)>>();
	if same_as_head_vec.len() > 1 {return true};
	if head.0 > state.bounds.0 || head.0 < 1 {return true};
	if head.1 > state.bounds.1 || head.1 < 1 {return true};
	return false
}

fn get_initial_state() -> GameState {
	let (w, h) = termion::terminal_size().unwrap();
	let bounds = (std::cmp::min(w-3, 30), std::cmp::min(h-5, 10));
	let bodies = vec![(1, 1)];
	GameState {
		head_direction: Direction::Right,
		bodies,
		bounds: bounds,
		food_locations: vec![generate_new_food(bounds, &vec![(1,1)])],
		finished: false,
		score: 0
	}
}

fn handle_input(locked_state: Arc<Mutex<GameState>>, input: char) -> Arc<Mutex<GameState>> {
	{
		let lock = locked_state.lock();
		let mut state = lock.unwrap();
		match input.to_ascii_lowercase() {
			'w' => {if state.head_direction != Direction::Down {state.head_direction = Direction::Up;}},
			'a' => {if state.head_direction != Direction::Right {state.head_direction = Direction::Left;}},
			's' => {if state.head_direction != Direction::Up {state.head_direction = Direction::Down;}},
			'd' => {if state.head_direction != Direction::Left {state.head_direction = Direction::Right}},
			_ => {}
		};
	}
	locked_state
}

pub struct Snake {}
impl<'a> Game<'a> for Snake {
	fn run(&self) {
		let locked_state = Arc::from(Mutex::from(get_initial_state()));
		let mut state_clone = locked_state.clone();
		let input_handler = thread::spawn(move || {
			let term = console::Term::stdout();
			loop {
				let char = term.read_char().unwrap();
				if state_clone.lock().unwrap().finished {break;}
				state_clone = handle_input(state_clone, char);
			}
		});

		loop {
			let mut sleep_ms: f64 = 50.0;
			{
				let mut state = locked_state.lock().unwrap();
				let state_ref = &mut state;
				check_on_food(state_ref);
				update_state(state_ref);
				write_screen(state_ref);
				let over = is_over(&state);
				if over {
					state.finished = true;
					break;
				}
				if state.head_direction == Direction::Up || state.head_direction == Direction::Down {sleep_ms = sleep_ms * 1.75};
			}
			sleep(Duration::from_millis(sleep_ms.floor() as u64));
		}
		{
			let mut state = locked_state.lock().unwrap();
			let state_ref = &mut state;
			write_screen(state_ref);
		}
		let _ = input_handler.join();
	}
}

pub fn get_game_instance<'a>() -> GameInstance<'a> {
	(Box::new(Snake {}), "snake")
}