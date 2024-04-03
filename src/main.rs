use common::{game::Game, screen::menu::*};
use clap::Parser;
use games::*;

mod games;
mod common;

#[derive(Parser, Debug)]
#[command(version, about = "Run games within the terminal", long_about = None)]
struct Args {
	game: Option<String>
}

fn main() {
	let mut found_game: Option<Box<dyn Game>> = None;
	let cli = Args::parse();
	let games = [
		snake::get_game_instance(),
		pong::get_game_instance(),
	];
	let selected_game_name: String;

	if let Some(game_name) = cli.game {
		selected_game_name = game_name.to_owned();
	} else {
		let menu_items = games.iter().map(|(_, label)| {
			MenuItem::new(&label, &label)
		}).collect::<Vec<MenuItem>>();
		selected_game_name = draw_menu(&menu_items, &"Pick a game");
	}

	for game_instance in games {
		let game = game_instance.0;
		let name = game_instance.1;
		if name == selected_game_name {
			found_game = Some(game);
		}
	}
	
	if let Some(game) = found_game {
		game.run();
	} else {
		println!("Couldn't find game");
		std::process::exit(1);
	}
}
