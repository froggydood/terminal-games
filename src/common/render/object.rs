use std::{borrow::Borrow, cell::{Ref, RefCell}, rc::Rc};

use nalgebra::{Quaternion, Vector3};

pub struct HalfEdge {
	pub from_vertex_index: usize,
	pub to_vertex_index: usize,
	pub twin_edge_index: usize,
	pub next_index: usize,
	pub prev_index: usize,
}

pub struct HalfEdgeVertex {
	pub position: Vector3<f32>,
	pub incident_half_edge_indices: Vec<usize>
}

pub struct HalfEdgeMesh {
	pub vertices: Vec<Rc<RefCell<HalfEdgeVertex>>>,
	pub half_edges: Vec<Rc<RefCell<HalfEdge>>>
}

#[derive(Debug)]
pub struct Face {
	pub vertices: Vec<usize>
}

#[derive(Debug)]
pub struct TriangleFace {
	pub vertices: [usize; 3]
}

impl TriangleFace {
	pub fn get_absolute(mesh: &HalfEdgeMesh) -> [Rc<Vector3<f32>>; 3] {
		let test: Ref<HalfEdgeVertex> = mesh.vertices[0].borrow();
		return [
		]
	}
}

impl HalfEdgeMesh {
	pub fn get_faces(self: &Self) -> Vec<Face> {
		println!("1");
		let mut faces: Vec<Face> = vec![Face{vertices: vec![]}];
		let mut current_half_edge_index = 0;
		let mut visited_edges: Vec<usize> = vec![];
		let mut current_face_index = 0;
		
		println!("2");
		loop {
			println!("Current half edge index {}", current_half_edge_index);
			println!("Visited edges {:?}", visited_edges);
			let current_half_edge = self.half_edges[current_half_edge_index].as_ref().borrow();
			let to_vertex = self.vertices[current_half_edge.to_vertex_index].as_ref().borrow();
			println!("To vertex: {:?} - {:?}", current_half_edge.to_vertex_index, to_vertex.position);
			println!("To vertex incident: {:?}", to_vertex.incident_half_edge_indices);

			let existing_entry = visited_edges.iter().find(|edge_i| {**edge_i == current_half_edge_index});
			println!("Existing {}",match existing_entry {Some(e) => format!("{}", e), None => "None".to_owned()});
			println!("3");
			if let Some(_) = existing_entry {
				println!("4");
				faces.push(Face {
					vertices: vec![]
				});
				current_face_index += 1;
				
				let new_index = self.half_edges.iter().enumerate().find(|(i, _)| {
					let existing_edge = visited_edges.iter().find(|visited_edge| {*visited_edge == i});
					match existing_edge {
						Some(_) => false,
						None => true,
					}
				});

				println!("5");
				if let Some((i, _)) = new_index {
					println!("6");
					println!("New half edge index {}", i);
					current_half_edge_index = i;
					continue;
				}
				println!("7");
				break;
			};
			println!("8");

			faces[current_face_index].vertices.push(current_half_edge.to_vertex_index);
			println!("9");

			visited_edges.push(current_half_edge_index);
			current_half_edge_index = current_half_edge.next_index;
		}
		println!("10");

		faces
	}

	pub fn get_triangles_from_face(self: &Self, face: Face) -> Vec<TriangleFace> {
		let mut triangles: Vec<TriangleFace> = vec![];
		let start_vert_index = 0;
		let face_vert_len = face.vertices.len();
		for (vert_face_index, vert_i_ref) in face.vertices.iter().enumerate() {
			if vert_face_index == 0 || vert_face_index == face_vert_len - 1 {continue;};
			let vert_i = *vert_i_ref;
			if vert_i == start_vert_index {continue;};
			let next_vert_face_index = if vert_face_index == face_vert_len - 1 {0} else {vert_face_index + 1};
			let next_vert_index = face.vertices[next_vert_face_index];
			triangles.push(TriangleFace {
				vertices: [start_vert_index, vert_i, next_vert_index]
			})
		};
		triangles
	}

	pub fn get_triangles(self: &Self) -> Vec<TriangleFace> {
		let faces = self.get_faces();
		faces
			.into_iter()
			.map(|face| {
				let triangles = self.get_triangles_from_face(face);
				triangles
			})
			.flatten()
			.collect::<Vec<TriangleFace>>()
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

impl Cube {
	pub fn new(position: Vector3<f32>, size: Vector3<f32>) -> Cube {
		Cube {
			position,
			size,
			orientation: Quaternion::identity()
		}
	}
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
			(0, 1), (1, 2), (2, 7), (7, 0),
			(5, 0), (0, 7), (7, 4), (4, 5),
			(0, 5), (5, 6), (6, 1), (1, 0),
			(1, 6), (6, 3), (3, 2), (2, 1),
			(5, 4), (4, 3), (3, 6), (6, 4),
			(2, 3), (3, 4), (4, 7), (7, 2)
		];
		let faces_edges = [
			[0 , 1 , 2 , 3 ],
			[4 , 5 , 6 , 7 ],
			[8 , 9 , 10, 11],
			[12, 13, 14, 15],
			[16, 17, 18, 19],
			[20, 21, 22, 23]
		];
		for vert_pos in vertex_positions {
			let vert: Rc<RefCell<HalfEdgeVertex>> = Rc::new(RefCell::new(HalfEdgeVertex {
				incident_half_edge_indices: vec![],
				position: Vector3::from_column_slice(&vert_pos)
			}));
			vertices.push(vert);
		}

		let mut edge_i = 0;
		for (from_i, to_i) in half_edge_vertices {
			let mut face_edge_indices = (0, 0);
			faces_edges.iter().enumerate().for_each(|(face_i, face_edge)| {
				face_edge.iter().enumerate().for_each(|(face_edge_i, curr_edge_i)| {
					if *curr_edge_i == edge_i {
						face_edge_indices = (face_i, face_edge_i)
					}
				});
			});
			let face_edge_arr = faces_edges[face_edge_indices.0];
			let prev_face_edge_index = if face_edge_indices.1 == 0 {face_edge_arr.len() - 1} else {face_edge_indices.1 - 1};
			let prev_face_edge = face_edge_arr[prev_face_edge_index];
			let next_face_edge_index = if face_edge_indices.1 == face_edge_arr.len() - 1 {0} else {face_edge_indices.1 + 1};
			let next_face_edge = face_edge_arr[next_face_edge_index];
			
			let half_edge = Rc::new(RefCell::new(HalfEdge {
				from_vertex_index: from_i,
				to_vertex_index: to_i,
				twin_edge_index: 0,
				next_index: next_face_edge,
				prev_index: prev_face_edge
			}));
			half_edges.push(half_edge);
			vertices[from_i].borrow_mut().incident_half_edge_indices.push(half_edges.len() - 1);
			edge_i += 1;
		}

		for (i, half_edge) in half_edges.iter().enumerate() {
			let mut other_i = 0;
			for other_half_edge in half_edges.iter() {
				if i == other_i {
					other_i += 1;
					continue;
				};
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
		println!("GOT MESH");
		HalfEdgeMesh {
			half_edges,
			vertices
		}
	}
}