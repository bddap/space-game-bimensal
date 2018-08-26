mod camera;
mod direction;
mod human;
mod input;
mod position;
mod space;
mod voxel;
mod world;

fn main() {
    let mut world = world::World {
        space: space::generate(),
        viewport: camera::Camera {
            position: position::Position { x: 7, y: 7, z: 1 },
            direction: direction::Direction::North,
        },
    };
    let human = human::Human::new();
    loop {
        match human.input() {
            input::Input::Quit => return,
            input::Input::Move(direction) => world.move_viewport(direction),
            input::Input::None => {}
        }
        human.display(&world);
    }
}
