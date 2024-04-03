use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};
use rand::prelude::*;
use crate::common::{
	game::*,
	screen::{boxes::BoxPrint, util::*}
};

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
	offset: (u16, u16),
	finished: bool,
	score: u32,
	inputs_to_handle: Vec<char>
}

fn draw_snakes(state: &GameState) {
	let body_char: &str = "█";
	for n in 0..state.bodies.len() {
		let (x, y) = state.bodies[n];
		print_at(body_char, (
			state.offset.0 + x - 1,
			state.offset.1 + y - 1
		));
	}
}

fn draw_food(state: &GameState) {
	let body_char: &str = "█";
	for n in 0..state.food_locations.len() {
		let (x, y) = state.food_locations[n];
		print_at_with_cols(
			body_char, (
				state.offset.0 + x - 1,
				state.offset.1 + y - 1
			),
			Some(&termion::color::Red),
			None
		);
	}	
}

fn write_screen(game_state: &GameState) {
	clear_screen();
	BoxPrint::new((game_state.bounds.0 + 2, game_state.bounds.1 + 2))
		.at_coords((game_state.offset.0 - 1, game_state.offset.1 - 1))
		.print();
	draw_snakes(game_state);
	draw_food(game_state);
	write_game_text(&game_state);
	cursor_to_end();
}

fn write_game_text(game_state: &GameState) {
	let text: String;
	if game_state.finished {
		text = format!("Game over, score: {}, press any key to continue", game_state.score);
	} else {
		text = format!("Score: {}", game_state.score);
	}
	print_at(&text, (
		game_state.offset.0 + game_state.bounds.0 / 2 - (text.len() / 2) as u16,
		game_state.offset.1 + game_state.bounds.1 + 1
	));
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
	if head.0 > state.bounds.0 || head.0 == 0 {return true};
	if head.1 > state.bounds.1 || head.1 == 0 {return true};
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
		offset: get_centered_coords(bounds),
		food_locations: vec![generate_new_food(bounds, &vec![(1,1)])],
		finished: false,
		score: 0,
		inputs_to_handle: vec![]
	}
}

fn handle_input(state: &mut GameState) {
	if state.inputs_to_handle.len() == 0 {return};
	match state.inputs_to_handle[0].to_ascii_lowercase() {
		'w' => {if state.head_direction != Direction::Down {state.head_direction = Direction::Up;}},
		'a' => {if state.head_direction != Direction::Right {state.head_direction = Direction::Left;}},
		's' => {if state.head_direction != Direction::Up {state.head_direction = Direction::Down;}},
		'd' => {if state.head_direction != Direction::Left {state.head_direction = Direction::Right}},
		_ => {}
	};
	state.inputs_to_handle.remove(0);
}

fn add_input_to_handle(locked_state: Arc<Mutex<GameState>>, input: char) -> Arc<Mutex<GameState>> {
	{
		let mut state = locked_state.lock().unwrap();
		state.inputs_to_handle.push(input);
	}
	locked_state	
}

pub struct Snake {}
impl<'a> Game<'a> for Snake {
	fn run(&self) -> GameReturn {
		let locked_state = Arc::from(Mutex::from(get_initial_state()));
		let mut state_clone = locked_state.clone();
		let mut game_return = GameReturn::default();
		let input_handler = thread::spawn(move || {
			let term = console::Term::stdout();
			loop {
				let char = term.read_char().unwrap();
				if state_clone.lock().unwrap().finished {break;}
				state_clone = add_input_to_handle(state_clone, char);
			}
		});

		loop {
			let mut sleep_ms: f64 = 50.0;
			{
				let mut state = locked_state.lock().unwrap();
				let state_ref = &mut state;
				handle_input(state_ref);
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
			game_return = GameReturn {
				score: Score::SinglePlayer(state_ref.score as f32),
				win_state: WinState::Lose
			};
			write_screen(state_ref);
		}
		let _ = input_handler.join();
		game_return
	}
}

pub fn get_game_instance<'a>() -> GameInstance<'a> {
	(Box::new(Snake {}), "snake")
}