extern crate dubble;
use self::dubble::DoubleBuffered;

pub struct Space {
    pub chunk: DoubleBuffered<[[[::voxel::Voxel; 16]; 16]; 16]>,
}

impl Space {
    pub fn get_voxel(&self, position: ::position::Position) -> ::voxel::Voxel {
        let within = |x| x >= 0 && x < 16;
        if within(position.x) && within(position.y) && within(position.z) {
            self.chunk[position.x as usize][position.y as usize][position.z as usize]
        } else {
            ::voxel::Voxel::Vacuum
        }
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

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    self.chunk[x][y][z] = neighbors
                        .iter()
                        .filter_map(|&displacement| {
                            let neighbor = self.get_voxel(displacement.into());
                            neighbor.impose(self.chunk[x][y][z], displacement)
                        })
                        .max()
                        .unwrap_or(self.chunk[x][y][z])
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
