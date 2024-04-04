use common::{game::{Game, GameInstance}, screen::menu::*};
use clap::Parser;
use games::*;

mod games;
mod common;

#[derive(Parser, Debug)]
#[command(version, about = "Run games within the terminal", long_about = None)]
struct Args {
	game: Option<String>
}

fn select_game(games: &[GameInstance]) -> String {
	let menu_items = games.iter().map(|(_, label)| {
		MenuItem::new(&label, &label)
	}).collect::<Vec<MenuItem>>();
	let selected_name = draw_menu(&menu_items, "Select a game");
	selected_name
}

fn main() {
	let cli = Args::parse();
	let games = [
		snake::get_game_instance(),
		pong::get_game_instance(),
		quit::get_game_instance()
	];

	'outer: loop {
		let selected_game_name: String;
	
		if let Some(game_name) = &cli.game {
			selected_game_name = game_name.to_owned();
		} else {
			selected_game_name = select_game(&games);
		}

		let mut found_game: Option<&Box<dyn Game>> = None;
		for game_instance in &games {
			let game = &game_instance.0;
			let name = game_instance.1;
			if name == selected_game_name {
				found_game = Some(&game);
			}
		}
		
		if let Some(game) = found_game {
			let menu_items: Vec<MenuItem> = vec![
				MenuItem::new("Play again", "again"),
				MenuItem::new("Play a different game", "different_game"),
				MenuItem::new("Quit", "quit")
			];
			'inner: loop {
				let game_return = game.run();
				let response = draw_menu(&menu_items, &game_return.get_end_text());
				match response.as_str() {
					"again" => {},
					"different_game" => break 'inner,
					_ => break 'outer,
				}
			}
		} else {
			println!("Couldn't find game");
			std::process::exit(1);
		}
	}
}
