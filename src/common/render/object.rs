use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use nalgebra::{Quaternion, Vector3};

pub struct HalfEdge {
	pub from_vertex_index: usize,
	pub to_vertex_index: usize,
	pub twin_edge_index: usize
}

pub struct HalfEdgeVertex {
	pub position: Vector3<f32>,
	pub incident_half_edge_index: usize
}

pub struct HalfEdgeMesh {
	pub vertices: Vec<Rc<RefCell<HalfEdgeVertex>>>,
	pub half_edges: Vec<Rc<RefCell<HalfEdge>>>
}

pub struct Face {
	pub vertices: Vec<Vector3<f32>>
}

impl HalfEdgeMesh {
	pub fn get_faces(self: &Self) -> Vec<Face> {
		let faces: Vec<Face> = vec![];
		let edge = self.half_edges[0].as_ref().borrow();
		let visited_edges: Vec<&HalfEdge> = vec![];
		
		while visited_edges.len() != self.half_edges.len() {

		}

		faces
	}
}

pub trait SceneObject {
	fn get_half_edge_mesh(self: &Self) -> HalfEdgeMesh;
}

pub struct Cube {
	pub position: Vector3<f32>,
	pub size: Vector3<f32>,
	pub orientation: Quaternion<f32>
}
impl SceneObject for Cube {
	fn get_half_edge_mesh(self: &Self) -> HalfEdgeMesh {
		let mut vertices: Vec<Rc<RefCell<HalfEdgeVertex>>> = vec![];
		let mut half_edges: Vec<Rc<RefCell<HalfEdge>>> = vec![];
		let vertex_positions = [
			[0.0, 0.0, 0.0],
			[1.0, 0.0, 0.0],
			[1.0, 1.0, 0.0],
			[1.0, 1.0, 1.0],
			[0.0, 1.0, 1.0],
			[0.0, 0.0, 1.0],
			[1.0, 0.0, 1.0],
			[0.0, 1.0, 0.0],
		];
		let half_edge_vertices = [
			(0, 1), (1, 2), (2, 3), (3, 4),
			(4, 5), (5, 0), (0, 6), (6, 3),
			(3, 7), (7, 2), (2, 1), (1, 6),
			(6, 5), (5, 4), (5, 7), (7, 0)
		];
		for vert_pos in vertex_positions {
			let vert: Rc<RefCell<HalfEdgeVertex>> = Rc::new(RefCell::new(HalfEdgeVertex {
				incident_half_edge_index: 0,
				position: Vector3::from_column_slice(&vert_pos)
			}));
			vertices.push(vert);
		}

		for (from_i, to_i) in half_edge_vertices {
			let half_edge = Rc::new(RefCell::new(HalfEdge {
				from_vertex_index: from_i,
				to_vertex_index: to_i,
				twin_edge_index: 0
			}));
			half_edges.push(half_edge);
			vertices[from_i].borrow_mut().incident_half_edge_index = half_edges.len() - 1;
		}

		for half_edge in half_edges.iter() {
			let mut other_i = 0;
			for other_half_edge in half_edges.iter() {
				let mut half_edge_one = half_edge.borrow_mut();
				let half_edge_two = other_half_edge.as_ref().borrow();
				if 
					half_edge_one.to_vertex_index == half_edge_two.from_vertex_index &&
					half_edge_one.from_vertex_index == half_edge_two.to_vertex_index
				{
					half_edge_one.twin_edge_index = other_i;
					other_i += 1;
					continue;
				}
				other_i += 1;
			}
		}

		HalfEdgeMesh {
			half_edges,
			vertices
		}
	}
}