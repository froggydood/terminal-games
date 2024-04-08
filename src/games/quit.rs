use crate::common::game::{Game, GameInstance, GameReturn};

pub struct Quit{}
impl<'a> Game<'a> for Quit {
	fn run(self: &Self) -> GameReturn {
		std::process::exit(0);
	}
}
pub fn get_game_instance<'a>() -> GameInstance<'a> {
	(Box::new(Quit {}), "Quit")
}