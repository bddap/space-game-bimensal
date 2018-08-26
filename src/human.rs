// An interface to an actual human.
pub struct Human {}

impl Human {
    pub fn new() -> Human {
        Human {}
    }
    pub fn display(&self, world: &::world::World) {
        println!("hello world");
    }
    pub fn input(&self) -> ::input::Input {
        ::input::Input::Quit
    }
}
