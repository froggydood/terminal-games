use super::object::*;

pub struct Scene<'a> {
	pub objects: &'a[&'a dyn SceneObject]
}
