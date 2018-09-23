extern crate dubble;
use self::dubble::DoubleBuffered;

pub struct Space {
    chunk: DoubleBuffered<[[[::voxel::Voxel; 16]; 16]; 16]>,
}

impl Space {
    pub fn get_voxel(&self, position: ::position::Position) -> ::voxel::Voxel {
        if self.exists(position) {
            self.chunk[position.x as usize][position.y as usize][position.z as usize]
        } else {
            ::voxel::Voxel::Vacuum
        }
    }

    fn set_voxel(&mut self, position: ::position::Position, voxel: ::voxel::Voxel) {
        if self.exists(position) {
            let ::position::Position { x, y, z } = position;
            self.chunk[x as usize][y as usize][z as usize] = voxel;
        }
    }

    fn exists(&self, position: ::position::Position) -> bool {
        let within = |x| x >= 0 && x < 16;
        within(position.x) && within(position.y) && within(position.z)
    }

    pub fn update(&mut self) {
        let neighbors = [
            ::voxel::Displacement::Center,
            ::voxel::Displacement::North,
            ::voxel::Displacement::South,
            ::voxel::Displacement::East,
            ::voxel::Displacement::West,
            ::voxel::Displacement::Up,
            ::voxel::Displacement::Down,
        ];

        for x in 0..16i32 {
            for y in 0..16i32 {
                for z in 0..16i32 {
                    let here = ::position::Position {
                        x: x as i32,
                        y: y as i32,
                        z: z as i32,
                    };
                    let me = self.get_voxel(here);
                    let next_me = neighbors
                        .iter()
                        .filter_map(|&displacement| {
                            let neighbor_position = here + displacement.into();
                            let neighbor = self.get_voxel(neighbor_position);
                            neighbor.impose(me, displacement)
                        }).fold(me, ::voxel::Voxel::combine);
                    self.set_voxel(here, next_me);
                }
            }
        }
        self.chunk.update();
    }
}

pub fn generate() -> Space {
    use {space::Space, voxel::Voxel::*};
    Space {
        chunk: DoubleBuffered::new({
            let mut chunk = [[[Vacuum; 16]; 16]; 16];
            chunk[7][7][7] = Asteroid;
            chunk[7][7][8] = Asteroid;
            chunk
        }),
    }
}
