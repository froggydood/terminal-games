use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use crate::common::{game::*, screen::*};

static PADDLE_HEIGHT: u8 = 4;

enum Direction {
	Up,
	Down
}

struct Paddle {
	y: u16,
	direction: Direction,
	score: u16,
}

struct GameState {
	bounds: (u16, u16),
	finished: bool,
	left_paddle: Paddle,
	right_paddle: Paddle,
}

fn write_screen(state: &GameState) {
	let (w, h) = termion::terminal_size().unwrap();
	clear_screen(h);
	draw_border(state.bounds, (w, h));
	draw_paddles(state, (w, h));
	write_game_text(state, state.bounds, (w, h))
}


fn draw_paddles(state: &GameState, term_size: (u16, u16)) {
	let paddle_char = "█";
	for (paddle_i, paddle) in [&state.left_paddle, &state.right_paddle].into_iter().enumerate() {
		for i in 1..=PADDLE_HEIGHT {
			let x: i16 = if paddle_i == 0 {2} else {state.bounds.0 as i16 + 1};
			print_text_at(paddle_char, (x, ((paddle.y + i as u16) as i16) + 1), state.bounds, term_size)
		}
	}
}

fn write_game_text(game_state: &GameState, bounds: (u16, u16), term_size: (u16, u16)) {
	let text: String;
	if game_state.finished {
		text = format!("Game over, score: {} - {}, press any key to continue", game_state.left_paddle.score, game_state.right_paddle.score);
	} else {
		text = format!("Score: {} - {}", game_state.left_paddle.score, game_state.right_paddle.score);
	}
	write_text(&text, bounds, term_size);
}

fn get_initial_state() -> GameState {
	let (w, h) = termion::terminal_size().unwrap();
	let bounds = (std::cmp::min(w-3, 50), std::cmp::min(h-5, 15));
	GameState {
		bounds,
		finished: false,
		left_paddle: Paddle {
			direction: Direction::Down,
			y: 0,
			score: 0
		},
		right_paddle: Paddle {
			direction: Direction::Down,
			y: 0,
			score: 0
		}
	}
}

fn handle_input(state: Arc<Mutex<GameState>>, input: char) -> Arc<Mutex<GameState>> {
	state
}

fn is_over(state: &GameState) -> bool {
	true
}

pub struct Pong{}
impl<'a> Game<'a> for Pong {
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
			let sleep_ms: f64 = 50.0;
			{
				let mut state = locked_state.lock().unwrap();
				let state_ref = &mut state;
				write_screen(state_ref);
				let over = is_over(&state);
				if over {
					state.finished = true;
					break;
				}
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
	(Box::new(Pong {}), "pong")
}