pub struct Space {
    pub chunk: [[[::voxel::Voxel; 16]; 16]; 16],
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
}

pub fn generate() -> Space {
    use {space::Space, voxel::Voxel::*};
    let mut chunk = [[[Vacuum; 16]; 16]; 16];
    chunk[7][7][7] = Asteroid;
    chunk[7][7][8] = Asteroid;
    Space { chunk }
}
