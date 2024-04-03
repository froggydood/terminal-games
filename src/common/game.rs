use std::fmt::format;

pub enum Score {
	SinglePlayer(f32),
	TwoPlayer(f32, f32),
	None
}

pub enum WinState {
	Win,
	Lose,
	Draw,
	None
}

pub struct GameReturn {
	pub score: Score,
	pub win_state: WinState
}
impl GameReturn {
	pub fn default() -> GameReturn {
		GameReturn {
			score: Score::SinglePlayer(0.0),
			win_state: WinState::Lose
		}
	}
	pub fn get_end_text(self: &Self) -> String {
		let game_over_text = match self.win_state {
			WinState::None => "Game over.",
			WinState::Draw => "Game over, draw.",
			WinState::Lose => "Game over, you lose.",
			WinState::Win => "Game over, you win."
		};
		let score_text: String = match self.score {
			Score::None => "".to_owned(),
			Score::SinglePlayer(score) => format!("Score: {}", score),
			Score::TwoPlayer(you, cpu) => format!("Score: {} - {}", you, cpu)
		};
		let title = format!("{} {}", game_over_text, score_text);
		title
	}
}

pub trait Game<'a> {
	fn run(&self) -> GameReturn;
}

pub type GameInstance<'a> = (Box<dyn Game<'a>>, &'a str);