// An interface to an actual human.

extern crate three;
use human::three::Object;
use std::collections::VecDeque;

pub struct Human {
    window: three::Window,
    camera: three::camera::Camera,
    cubes: Vec<three::Mesh>,
    input: VecDeque<three::controls::Key>,
}

impl Human {
    pub fn new() -> Human {
        let mut window = three::Window::new("space game bimensal");
        window.scene.background = three::Background::Color(0x204060);

        let camera = window.factory.perspective_camera(60.0, 1.0..100.0);
        // camera.look_at([-1.8, -8.0, 7.0], [0.0, 0.0, 3.5], None);

        let light = window.factory.point_light(0xffffff, 1.0);
        light.set_position([-10.0, -10.0, 32.0]);
        window.scene.add(&light);

        let material = three::material::Lambert {
            color: 0x8080ff,
            flat: false,
        };

        let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0);

        let cubes: Vec<three::Mesh> = (0..(16 * 16 * 16))
            .map(|i: i32| (i / 16 / 16, i / 16 % 16, i % 16))
            .map(|(x, y, z)| {
                let cube = window.factory.mesh(geometry.clone(), material.clone());
                cube.set_position([x as f32, y as f32, z as f32]);
                cube
            })
            .collect();

        for cube in cubes.iter() {
            window.scene.add(cube);
        }

        Human {
            window,
            camera,
            cubes,
            input: VecDeque::new(),
        }
    }

    pub fn display(&mut self, world: &::world::World) {
        for (index, cube) in self.cubes.iter_mut().enumerate() {
            cube.set_material(material(world.space.get_voxel(::position::Position {
                x: index as i32 / 16 / 16,
                y: index as i32 / 16 % 16,
                z: index as i32 % 16,
            })));
        }
        self.camera.set_position([
            world.viewport.position.x as f32,
            world.viewport.position.y as f32,
            world.viewport.position.z as f32,
        ]);
        self.camera.set_orientation(world.viewport.orientation);
        self.window.render(&self.camera);
        self.window.update();
    }

    pub fn input(&mut self) -> ::input::Input {
        for key in self.window.input.keys_hit().iter() {
            self.input.push_back(*key)
        }

        let inp = self.input.pop_front();

        match inp {
            Some(three::controls::Key::D) => ::input::Input::Move(::direction::Direction::North),
            Some(three::controls::Key::S) => ::input::Input::Move(::direction::Direction::South),
            Some(three::controls::Key::F) => ::input::Input::Move(::direction::Direction::East),
            Some(three::controls::Key::A) => ::input::Input::Move(::direction::Direction::West),
            Some(three::controls::Key::C) => ::input::Input::Move(::direction::Direction::Up),
            Some(three::controls::Key::X) => ::input::Input::Move(::direction::Direction::Down),
            Some(three::controls::Key::L) => ::input::Input::Turn(::direction::Direction::North),
            Some(three::controls::Key::K) => ::input::Input::Turn(::direction::Direction::South),
            Some(three::controls::Key::Semicolon) => {
                ::input::Input::Turn(::direction::Direction::East)
            }
            Some(three::controls::Key::J) => ::input::Input::Turn(::direction::Direction::West),
            Some(three::controls::Key::Comma) => ::input::Input::Turn(::direction::Direction::Up),
            Some(three::controls::Key::M) => ::input::Input::Turn(::direction::Direction::Down),
            Some(three::controls::Key::Q) => ::input::Input::Quit,
            _ => ::input::Input::None,
        }
    }
}

fn material(voxel: ::voxel::Voxel) -> three::Material {
    match voxel {
        ::voxel::Voxel::Asteroid => three::material::Lambert {
            color: 0x8080ff,
            flat: false,
        }.into(),
        ::voxel::Voxel::Vacuum => three::material::Wireframe { color: 0x408055 }.into(),
    }
}
