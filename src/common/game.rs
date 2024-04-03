pub trait Game<'a> {
	fn run(&self) -> Option<i32>;
}

pub type GameInstance<'a> = (Box<dyn Game<'a>>, &'a str);