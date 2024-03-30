pub trait Game<'a> {
	fn run(&self);
}

pub type GameInstance<'a> = (Box<dyn Game<'a>>, &'a str);