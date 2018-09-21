mod camera;
mod direction;
mod human;
mod input;
mod orientation;
mod position;
mod space;
mod turn;
mod voxel;
mod world;

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
            Some(input::Input::Quit) => return,
            Some(input::Input::Move(direction)) => world.move_viewport(direction),
            Some(input::Input::Turn(direction)) => world.turn_viewport(direction),
            None => {}
        }
        human.display(&world);
        world.update();
    }
}
