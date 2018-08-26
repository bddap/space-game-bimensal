mod camera;
mod direction;
mod human;
mod space;
mod voxel;
mod world;

fn main() {
    let world = world::World {
        space: space::generate(),
        viewport: camera::Camera {
            x: 7,
            y: 7,
            z: 1,
            direction: direction::Direction::North,
        },
    };
    let human = human::Human::new();
    loop {
        match human.input() {
            human::Input::Quit => return,
            _ => human.display(&world),
        }
    }
}
