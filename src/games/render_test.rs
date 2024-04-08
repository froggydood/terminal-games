use std::time::Duration;

use crate::common::game::{Game, GameInstance, GameReturn};
use crate::common::render::renderer::{Renderer, TerminalRenderer};
use crate::common::render::scene::Scene;

pub struct RenderTest{}
impl<'a> Game<'a> for RenderTest {
	fn run(&self) -> GameReturn {
		let renderer = TerminalRenderer::default();
		let mut image_buffer = Vec::from_iter((0..(renderer.screen_width*renderer.screen_height)).map(|_arg| {0 as u8}));
		let scene = Scene {
			objects: &[]
		};
		let image = renderer.rasterize(& scene, &mut image_buffer);
		renderer.render(image);
		std::thread::sleep(Duration::from_secs(5));
		GameReturn::default()
	}
}
pub fn get_game_instance<'a>() -> GameInstance<'a> {
	(Box::new(RenderTest {}), "render test")
}