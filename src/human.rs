// An interface to an actual human.

extern crate three;
use human::three::Object;
use std::collections::VecDeque;

pub struct Human {
    window: three::Window,
    camera: three::camera::Camera,
    cubes: Vec<three::Mesh>,
    input: VecDeque<three::controls::Key>,
    sprites: Sprites,
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
            camera,
            cubes,
            input: VecDeque::new(),
            sprites: Sprites::load(&mut window),
            window,
        }
    }

    pub fn display(&mut self, world: &::world::World) {
        for (index, cube) in self.cubes.iter_mut().enumerate() {
            cube.set_material(material(
                world.space.get_voxel(::position::Position {
                    x: index as i32 / 16 / 16,
                    y: index as i32 / 16 % 16,
                    z: index as i32 % 16,
                }),
                &self.sprites,
            ));
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
        use self::three::controls;
        use direction::Direction;
        use input::Input::{Move, Turn};

        for key in self.window.input.keys_hit().iter() {
            self.input.push_back(*key)
        }

        let inp = self.input.pop_front();

        match inp {
            Some(controls::Key::D) => Move(Direction::North),
            Some(controls::Key::S) => Move(Direction::South),
            Some(controls::Key::F) => Move(Direction::East),
            Some(controls::Key::A) => Move(Direction::West),
            Some(controls::Key::C) => Move(Direction::Up),
            Some(controls::Key::X) => Move(Direction::Down),
            Some(controls::Key::L) => Turn(Direction::Up),
            Some(controls::Key::K) => Turn(Direction::Down),
            Some(controls::Key::Semicolon) => Turn(Direction::East),
            Some(controls::Key::J) => Turn(Direction::West),
            Some(controls::Key::Q) => ::input::Input::Quit,
            _ => ::input::Input::None,
        }
    }
}

struct Sprites {
    asteroid: three::material::Material, // three::Texture<[f32; 4]>,
}

impl Sprites {
    fn load(window: &mut three::Window) -> Sprites {
        use std::path::Path;
        let assets = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        Sprites {
            asteroid: three::material::Basic {
                color: 0x555555,
                map: Some(window.factory.load_texture(assets.join("asteroid.png"))),
            }.into(),
        }
    }
}

fn material(voxel: ::voxel::Voxel, sprites: &Sprites) -> three::Material {
    match voxel {
        ::voxel::Voxel::Asteroid => sprites.asteroid.clone(),
        ::voxel::Voxel::Vacuum => three::material::Wireframe { color: 0x408055 }.into(),
    }
}
