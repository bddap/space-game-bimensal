// An interface to an actual human.

extern crate mint;
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

        let camera = window.factory.perspective_camera(60.0, 0.4..100.0);

        let light = window.factory.point_light(0xffffff, 1.0);
        light.set_position([8.0, 8.0, 11.0]);
        window.scene.add(&light);

        let material = three::material::Lambert {
            color: 0x8080ff,
            flat: false,
        };

        let geometry = cube_geometry();

        let cubes: Vec<three::Mesh> = (0..(16 * 16 * 16))
            .map(|i: i32| (i / 16 / 16, i / 16 % 16, i % 16))
            .map(|(x, y, z)| {
                let cube = window.factory.mesh(geometry.clone(), material.clone());
                cube.set_position([x as f32, y as f32, z as f32]);
                cube
            }).collect();

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
            let material = material(
                world.space.get_voxel(::position::Position {
                    x: index as i32 / 16 / 16,
                    y: index as i32 / 16 % 16,
                    z: index as i32 % 16,
                }),
                &self.sprites,
            );
            match material {
                Some(material) => {
                    cube.set_visible(true);
                    cube.set_material(material);
                }
                None => cube.set_visible(false),
            }
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
    asteroid: three::material::Material,
}

impl Sprites {
    fn load(window: &mut three::Window) -> Sprites {
        use std::path::Path;
        let assets = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        let sampler: three::Sampler = window.factory.sampler(
            three::FilterMethod::Scale,
            three::WrapMode::Clamp,
            three::WrapMode::Clamp,
        );
        Sprites {
            asteroid: three::material::Basic {
                color: 0xffffff,
                map: Some(
                    window
                        .factory
                        .load_texture_with_sampler(assets.join("asteroid.png"), sampler),
                ),
            }.into(),
        }
    }
}

fn material(voxel: ::voxel::Voxel, sprites: &Sprites) -> Option<three::Material> {
    match voxel {
        ::voxel::Voxel::Asteroid => Some(sprites.asteroid.clone()),
        ::voxel::Voxel::Vacuum => None,
    }
}

fn cube_geometry() -> three::Geometry {
    use self::mint::{Point2, Point3, Vector3};
    use self::three::{Geometry, Joints, Shape};

    let blue = Point2 { x: 0.0, y: 0.0 };
    let red = Point2 { x: 1.0, y: 1.0 };
    let purple = Point2 { x: 0.0, y: 1.0 };
    let green = Point2 { x: 1.0, y: 0.0 };

    Geometry {
        base: Shape {
            vertices: vec![
                Point3 {
                    x: 0.5,
                    y: 0.5,
                    z: -0.5,
                },
                Point3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5,
                },
                Point3 {
                    x: 0.5,
                    y: -0.5,
                    z: 0.5,
                },
                Point3 {
                    x: 0.5,
                    y: -0.5,
                    z: -0.5,
                },
                Point3 {
                    x: -0.5,
                    y: -0.5,
                    z: -0.5,
                },
                Point3 {
                    x: -0.5,
                    y: -0.5,
                    z: 0.5,
                },
                Point3 {
                    x: -0.5,
                    y: 0.5,
                    z: 0.5,
                },
                Point3 {
                    x: -0.5,
                    y: 0.5,
                    z: -0.5,
                },
                Point3 {
                    x: -0.5,
                    y: 0.5,
                    z: 0.5,
                },
                Point3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5,
                },
                Point3 {
                    x: 0.5,
                    y: 0.5,
                    z: -0.5,
                },
                Point3 {
                    x: -0.5,
                    y: 0.5,
                    z: -0.5,
                },
                Point3 {
                    x: 0.5,
                    y: -0.5,
                    z: -0.5,
                },
                Point3 {
                    x: 0.5,
                    y: -0.5,
                    z: 0.5,
                },
                Point3 {
                    x: -0.5,
                    y: -0.5,
                    z: 0.5,
                },
                Point3 {
                    x: -0.5,
                    y: -0.5,
                    z: -0.5,
                },
                Point3 {
                    x: 0.5,
                    y: -0.5,
                    z: 0.5,
                },
                Point3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5,
                },
                Point3 {
                    x: -0.5,
                    y: 0.5,
                    z: 0.5,
                },
                Point3 {
                    x: -0.5,
                    y: -0.5,
                    z: 0.5,
                },
                Point3 {
                    x: -0.5,
                    y: -0.5,
                    z: -0.5,
                },
                Point3 {
                    x: -0.5,
                    y: 0.5,
                    z: -0.5,
                },
                Point3 {
                    x: 0.5,
                    y: 0.5,
                    z: -0.5,
                },
                Point3 {
                    x: 0.5,
                    y: -0.5,
                    z: -0.5,
                },
            ],
            normals: vec![
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
            ],
            tangents: vec![],
        },
        tex_coords: vec![
            blue, green, red, purple, red, purple, blue, green, red, green, blue, purple, red,
            green, blue, purple, red, green, blue, purple, red, green, blue, purple,
        ],
        faces: vec![
            [0, 1, 2],
            [2, 3, 0],
            [4, 5, 6],
            [6, 7, 4],
            [8, 9, 10],
            [10, 11, 8],
            [12, 13, 14],
            [14, 15, 12],
            [16, 17, 18],
            [18, 19, 16],
            [20, 21, 22],
            [22, 23, 20],
        ],
        joints: Joints {
            indices: vec![],
            weights: vec![],
        },
        shapes: vec![],
    }
}
