mod camera;
mod direction;
mod human;
mod input;
mod orientation;
mod position;
mod space;
mod voxel;
mod world;
mod turn;

fn main() {
    let mut world = world::World {
        space: space::generate(),
        viewport: camera::Camera {
            position: position::Position { x: 7, y: 7, z: 1 },
            orientation: orientation::Orientation {
                vertical: orientation::VerticalOrientation::Center,
                horizontal: orientation::HorizontalOrientation::North,
            },
        },
    };
    let mut human = human::Human::new();
    loop {
        match human.input() {
            input::Input::Quit => return,
            input::Input::Move(direction) => world.move_viewport(direction),
            input::Input::Turn(direction) => world.turn_viewport(direction),
            input::Input::None => {}
        }
        human.display(&world);
        world.update();
    }
}
