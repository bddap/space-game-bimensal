pub struct World {
    pub space: ::space::Space,
    pub viewport: ::camera::Camera,
}

impl World {
    pub fn move_viewport(&mut self, direction: ::direction::Direction) {
        let proposed_position = self.viewport.position + direction.into();

        if self.space.get_voxel(proposed_position) == ::voxel::Voxel::Vacuum {
            self.viewport.position = proposed_position
        }
        println!("{:?}", self.viewport);
    }

    pub fn turn_viewport(&mut self, direction: ::direction::Direction) {
        self.viewport.orientation = self.viewport.orientation.turn(direction);
        println!("{:?}", self.viewport.orientation);
    }

    pub fn update(&mut self) {
        self.space.update()
    }
}
