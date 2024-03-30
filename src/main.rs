use common::game::{Game, GameInstance};
use clap::Parser;
use games::*;

mod games;
mod common;

#[derive(Parser, Debug)]
#[command(version, about = "Run games within the terminal", long_about = None)]
struct Args {
	game: String
}

fn main() -> Result<(), ()> {
	let mut found_game: Option<Box<dyn Game>> = None;
	let cli = Args::parse();
	let snake = snake::get_game_instance();
	{
		let games: [GameInstance; 1] = [snake];

		for game_instance in games {
			let game = game_instance.0;
			let name = game_instance.1;
			if name == cli.game {
				found_game = Some(game);
			}
		}
	}

	if let Some(game) = found_game {
		game.run();
	} else {
		println!("No such game: \"{}\"", cli.game);
		return Err(());
	}
	Ok(())
}
