use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use crate::common::{game::*, screen::*};

static PADDLE_HEIGHT: u8 = 3;
static PADDLE_SPEED: f32 = 1.5;
static PI: f32 = 3.14159;

#[derive(PartialEq, Eq)]
enum VerticalDirection {
	Up,
	Down
}

#[derive(PartialEq, Eq)]
enum HorizontalDirection {
	Left,
	Right
}

struct Paddle {
	y: u16,
	direction: VerticalDirection,
	score: u16,
}

struct GameState {
	bounds: (u16, u16),
	finished: bool,
	left_paddle: Paddle,
	right_paddle: Paddle,
	ball_pos: (f32, f32),
	ball_direction: (HorizontalDirection, f32),
}

fn write_screen(state: &GameState) {
	let term_size = termion::terminal_size().unwrap();
	clear_screen(term_size.1);
	draw_border(state.bounds, term_size);
	draw_paddles(state, term_size);
	draw_ball(state, term_size);
	write_game_text(state, term_size)
}


fn draw_paddles(state: &GameState, term_size: (u16, u16)) {
	let paddle_char = "█";
	for (paddle_i, paddle) in [&state.left_paddle, &state.right_paddle].into_iter().enumerate() {
		for i in 1..=PADDLE_HEIGHT {
			let x: i16 = if paddle_i == 0 {2} else {state.bounds.0 as i16 + 1};
			print_text_at(paddle_char, (x, ((paddle.y + i as u16) as i16)), state.bounds, term_size)
		}
	}
}

fn write_game_text(game_state: &GameState, term_size: (u16, u16)) {
	let text: String;
	if game_state.finished {
		let win_text = if game_state.left_paddle.score > game_state.right_paddle.score {"You win"} else {"Computer wins"};
		text = format!("{}, score: {} - {}, press any key to continue", win_text, game_state.left_paddle.score, game_state.right_paddle.score);
	} else {
		text = format!("Score: {} - {}", game_state.left_paddle.score, game_state.right_paddle.score);
	}
	write_text(&text, game_state.bounds, term_size);
}

fn draw_ball(game_state: &GameState, term_size: (u16, u16)) {
	let ball_char = "o";
	print_text_at(ball_char, (game_state.ball_pos.0 as i16 + 1, game_state.ball_pos.1 as i16 + 1), game_state.bounds, term_size)
}

fn get_initial_state() -> GameState {
	let (w, h) = termion::terminal_size().unwrap();
	let bounds = (std::cmp::min(w-3, 50), std::cmp::min(h-5, 15));
	GameState {
		bounds,
		finished: false,
		left_paddle: Paddle {
			direction: VerticalDirection::Down,
			y: 1,
			score: 0
		},
		right_paddle: Paddle {
			direction: VerticalDirection::Down,
			y: 1,
			score: 0
		},
		ball_pos: (2.0, 1.0),
		ball_direction: (HorizontalDirection::Right, PI / 6.0)
	}
}

fn update_paddles(state: &mut GameState) {
	for paddle in [&mut state.left_paddle, &mut state.right_paddle].into_iter() {
		match paddle.direction {
			VerticalDirection::Down => paddle.y += 1,
			VerticalDirection::Up => paddle.y -= 1
		}
		paddle.y = std::cmp::min(paddle.y, state.bounds.1 - 2);
		paddle.y = std::cmp::max(paddle.y, 1);
	}
}

fn update_ball(state: &mut GameState) {
	let x_move = state.ball_direction.1.cos() * PADDLE_SPEED;
	let y_move = state.ball_direction.1.sin() * PADDLE_SPEED;

	match state.ball_direction.0 {
		HorizontalDirection::Left => state.ball_pos.0 -= x_move,
		HorizontalDirection::Right => state.ball_pos.0 += x_move
	}

	state.ball_pos.1 += y_move;

	if state.ball_pos.1 > (state.bounds.1 as f32 - y_move) {
		state.ball_direction.1 *= -1.0;
		state.ball_pos.1 -= y_move;
	} else if state.ball_pos.1 < 1.0 {
		state.ball_direction.1 *= -1.0;
		state.ball_pos.1 -= y_move
	}

	if state.ball_pos.0 < 1.0 {
		if state.ball_pos.1.floor() < state.left_paddle.y as f32 || state.ball_pos.1.floor() > (state.left_paddle.y + PADDLE_HEIGHT as u16) as f32 {
			state.right_paddle.score += 1;
			state.ball_direction = (HorizontalDirection::Right, PI / 6.0);
			state.ball_pos = (2.0, 1.0);
		} else {
			let paddle_center_frac = (state.ball_pos.1 - state.left_paddle.y as f32) / (PADDLE_HEIGHT as f32 / 2.0);
			state.ball_direction.1 = paddle_center_frac * PI / 6.0;
			state.ball_direction.0 = HorizontalDirection::Right;
		}
	}
	if state.ball_pos.0 > (state.bounds.0 - 1) as f32 {
		if state.ball_pos.1.floor() < state.right_paddle.y as f32 || state.ball_pos.1.floor() > (state.right_paddle.y + PADDLE_HEIGHT as u16) as f32 {
			state.left_paddle.score += 1;
			state.ball_direction = (HorizontalDirection::Left, PI / 6.0);
			state.ball_pos = ((state.bounds.0 - 1) as f32, 1.0);
		} else {
			let paddle_center_frac = (state.ball_pos.1 - state.left_paddle.y as f32) / (PADDLE_HEIGHT as f32 / 2.0);
			state.ball_direction.1 = paddle_center_frac * PI / 6.0;
			state.ball_direction.0 = HorizontalDirection::Left;
		}
	}
}

fn update_cpu(state: &mut GameState) {
	if state.ball_pos.0 > (state.bounds.0 / 2) as f32 && state.ball_direction.0 == HorizontalDirection::Right {
		state.right_paddle.direction = if state.ball_pos.1 > state.right_paddle.y as f32 + (PADDLE_HEIGHT as f32 / 2.0) {VerticalDirection::Down} else {VerticalDirection::Up};
	}
}

fn update_state(state: &mut GameState) {
	update_cpu(state);
	update_paddles(state);
	update_ball(state);
}

fn handle_input(locked_state: Arc<Mutex<GameState>>, input: char) -> Arc<Mutex<GameState>> {
	{
		let mut state = locked_state.lock().unwrap();
		match input.to_ascii_lowercase() {
			'w' => state.left_paddle.direction = VerticalDirection::Up,
			's' => state.left_paddle.direction = VerticalDirection::Down,
			_ => {}
		};
	}
	locked_state
}

fn is_over(state: &GameState) -> bool {
	state.left_paddle.score == 10 || state.right_paddle.score == 10
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
				update_state(state_ref);
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