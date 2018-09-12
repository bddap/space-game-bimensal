pub struct World {
    pub space: ::space::Space,
    pub viewport: ::camera::Camera,
}

impl World {
    pub fn move_viewport(&mut self, direction: ::direction::Direction) {
        let local_translation: ::position::Position = direction.into();
        let global_translation = local_translation.turn(self.viewport.orientation.into());
        let proposed_position = self.viewport.position + global_translation;
        if self.space.get_voxel(proposed_position) == ::voxel::Voxel::Vacuum {
            self.viewport.position = proposed_position
        }
        println!("{:?} {:?}", self.viewport, {
            let direction: ::direction::Direction = self.viewport.orientation.into();
            direction
        });
    }

    pub fn turn_viewport(&mut self, direction: ::direction::Direction) {
        self.viewport.orientation = self.viewport.orientation.turn(direction);
        println!("{:?} {:?}", self.viewport, {
            let direction: ::direction::Direction = self.viewport.orientation.into();
            direction
        });
    }

    pub fn update(&mut self) {
        self.space.update()
    }
}
