// An interface to an actual human.
pub struct Human {}

impl Human {
    pub fn new() -> Human {
        Human {}
    }

    pub fn display(&self, _world: &::world::World) {
        println!("hello world");
    }

    pub fn input(&self) -> ::input::Input {
        let key = 'q';
        match key {
            'd' => ::input::Input::Move(::direction::Direction::North),
            'a' => ::input::Input::Move(::direction::Direction::South),
            'f' => ::input::Input::Move(::direction::Direction::East),
            's' => ::input::Input::Move(::direction::Direction::West),
            'j' => ::input::Input::Move(::direction::Direction::Up),
            'k' => ::input::Input::Move(::direction::Direction::Down),
            'q' => ::input::Input::Quit,
            _ => ::input::Input::None,
        }
    }
}
