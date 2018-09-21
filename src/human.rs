// An interface to an actual human.

extern crate kiss3d;
extern crate na;
extern crate three;

use self::kiss3d::event::WindowEvent;
use self::kiss3d::event::{Action, Key};
use self::kiss3d::light::Light;
use self::kiss3d::resource::ShaderUniform;
use self::kiss3d::scene::SceneNode;
use self::kiss3d::window::Canvas;
use self::kiss3d::window::Window;
use self::na::{Isometry3, Matrix4, Perspective3, Point3, Translation3, UnitQuaternion, Vector3};
use std::collections::VecDeque;
use std::f32;
// use std::path::Path;

pub struct Human {
    window: Window,
    // camera: three::camera::Camera,
    cubes: Vec<SceneNode>,
    input: VecDeque<::input::Input>,
    events: kiss3d::event::EventManager,
}

impl Human {
    pub fn new() -> Human {
        let mut window = Window::new("space game bimensal");
        // c.set_color(1.0, 0.0, 0.0);
        // c.set_texture_from_file(&Path::new("media/kitten.png"), "kitten");
        window.set_light(Light::StickToCamera);

        // let mut window = three::Window::new("space game bimensal");
        // window.scene.background = three::Background::Color(0x204060);

        // let camera = window.factory.perspective_camera(60.0, 1.0..100.0);
        // camera.look_at([-1.8, -8.0, 7.0], [0.0, 0.0, 3.5], None);

        // let light = window.factory.point_light(0xffffff, 1.0);
        // light.set_position([-10.0, -10.0, 32.0]);
        // window.scene.add(&light);

        // let material = three::material::Lambert {
        //     color: 0x8080ff,
        //     flat: false,
        // };

        // let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0);

        let cubes: Vec<SceneNode> = (0..(16 * 16 * 16))
            .map(|i: i32| (i / 16 / 16, i / 16 % 16, i % 16))
            .map(|(x, y, z)| window.add_cube(x as f32, y as f32, z as f32))
            .collect();

        let events = window.events();

        Human {
            cubes,
            input: VecDeque::new(),
            // sprites: Sprites::load(&mut window),
            window,
            events,
        }
    }

    pub fn display(&mut self, world: &::world::World) {
        let mut camera = Camera(world.viewport);
        self.window.render_with_camera(&mut camera);

        // for (index, cube) in self.cubes.iter_mut().enumerate() {
        //     cube.set_material(material(
        //         world.space.get_voxel(::position::Position {
        //             x: index as i32 / 16 / 16,
        //             y: index as i32 / 16 % 16,
        //             z: index as i32 % 16,
        //         }),
        //         &self.sprites,
        //     ));
        // }
        // self.camera.set_position([
        //     world.viewport.position.x as f32,
        //     world.viewport.position.y as f32,
        //     world.viewport.position.z as f32,
        // ]);
        // self.camera.set_orientation(world.viewport.orientation);
        // self.window.render(&self.camera);
        // self.window.update();
    }

    pub fn input(&mut self) -> Option<::input::Input> {
        use direction::Direction;
        use input::Input::{Move, Quit, Turn};

        if self.window.should_close() {
            return Some(Quit);
        }

        for event in self.events.iter() {
            match event.value {
                WindowEvent::Key(key, Action::Press, _) => {
                    let input = match key {
                        Key::D => Some(Move(Direction::North)),
                        Key::S => Some(Move(Direction::South)),
                        Key::F => Some(Move(Direction::East)),
                        Key::A => Some(Move(Direction::West)),
                        Key::C => Some(Move(Direction::Up)),
                        Key::X => Some(Move(Direction::Down)),
                        Key::L => Some(Turn(Direction::Up)),
                        Key::K => Some(Turn(Direction::Down)),
                        Key::Semicolon => Some(Turn(Direction::East)),
                        Key::J => Some(Turn(Direction::West)),
                        Key::Q => Some(Quit),
                        _ => None,
                    };
                    if let Some(input) = input {
                        self.input.push_back(input)
                    }
                }
                _ => {}
            }
        }

        self.input.pop_front()
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

struct Camera(::camera::Camera);

impl Camera {
    fn projection() -> Matrix4<f32> {
        *Perspective3::new(800.0 / 600.0, f32::consts::PI / 4.0, 0.1, 1024.0).as_matrix()
    }
}

impl kiss3d::camera::Camera for Camera {
    fn clip_planes(&self) -> (f32, f32) {
        (0.1, 1024.0)
    }

    /// The camera view transformation (i-e transformation without projection).
    fn view_transform(&self) -> Isometry3<f32> {
        let [x, y, z]: [f32; 3] = self.0.position.into();
        let rot: UnitQuaternion<f32> = self.0.orientation.into();
        Isometry3::from_parts(Translation3::from_vector(Vector3::new(x, y, z)), rot)
    }

    fn handle_event(&mut self, _: &Canvas, _: &WindowEvent) {}

    fn eye(&self) -> Point3<f32> {
        let p: [f32; 3] = self.0.position.into();
        p.into()
    }

    fn transformation(&self) -> Matrix4<f32> {
        println!("here");
        self.view_transform().to_homogeneous() * Camera::projection()
    }

    fn inverse_transformation(&self) -> Matrix4<f32> {
        self.transformation().try_inverse().unwrap_or(na::zero())
    }

    #[inline]
    fn upload(
        &self,
        _: usize,
        proj: &mut ShaderUniform<Matrix4<f32>>,
        view: &mut ShaderUniform<Matrix4<f32>>,
    ) {
        proj.upload(&Camera::projection());
        view.upload(&self.view_transform().to_homogeneous());
    }

    fn update(&mut self, _: &Canvas) {}
}
