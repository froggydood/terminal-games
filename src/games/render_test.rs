use std::time::Duration;

use nalgebra::Vector3;

use crate::common::game::{Game, GameInstance, GameReturn};
use crate::common::render::object::{Cube, SceneObject};
use crate::common::render::renderer::{Renderer, TerminalRenderer};
use crate::common::render::scene::Scene;

pub struct RenderTest{}
impl<'a> Game<'a> for RenderTest {
	fn run(&self) -> GameReturn {
		let renderer = TerminalRenderer::default();
		let mut image_buffer = Vec::from_iter((0..(renderer.screen_width*renderer.screen_height)).map(|_arg| {0 as u8}));
		let cube = Cube::new(
			Vector3::from_column_slice(&[0.0, 0.0, 0.0]),
			Vector3::from_column_slice(&[1.0, 1.0, 1.0])
		);
		let mesh = cube.get_half_edge_mesh();
		println!("Mesh Vertices: {:?}", mesh.vertices.iter().map(|v| {v.borrow().position}).collect::<Vec<Vector3<f32>>>());
		println!("Mesh Edges: {:?}", mesh.half_edges.iter().map(|v| {(v.borrow().from_vertex_index, v.borrow().to_vertex_index)}).collect::<Vec<(usize, usize)>>());
		println!("Mesh Edge Pairs: {:?}", mesh.half_edges.iter().enumerate().map(|(i1, edge)| {
			(i1, edge.as_ref().borrow().twin_edge_index)
		}).collect::<Vec<(usize, usize)>>());
		let triangles = mesh.get_triangles();
		println!("Triangles: {:?}", triangles);
		let scene = Scene {
			objects: &[&cube]
		};
		let image = renderer.rasterize(&scene, &mut image_buffer);
		renderer.render(image);
		let term = console::Term::stdout();
		term.read_char();
		GameReturn::default()
	}
}
pub fn get_game_instance<'a>() -> GameInstance<'a> {
	(Box::new(RenderTest {}), "render test")
}