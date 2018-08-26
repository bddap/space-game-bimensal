pub struct Space {
    pub chunk: [[[::voxel::Voxel; 16]; 16]; 16],
}

pub fn generate() -> Space {
    use {space::Space, voxel::Voxel::*};
    let mut chunk = [[[Vacuum; 16]; 16]; 16];
    chunk[7][7][7] = Asteroid;
    chunk[7][7][8] = Asteroid;
    Space { chunk }
}
