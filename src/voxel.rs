#[derive(PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
pub enum Voxel {
    Asteroid,
    Vacuum,
}

impl Voxel {
    pub fn impose(&self, other: Voxel, from: Displacement) -> Option<Voxel> {
        match (*self, other, from) {
            (s, _, Displacement::Center) => Some(s),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Displacement {
    Center,
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Into<::position::Position> for Displacement {
    fn into(self) -> ::position::Position {
        match self {
            Displacement::Center => ::position::Position { x: 0, y: 0, z: 0 },
            Displacement::North => ::position::Position { x: 0, y: 0, z: -1 },
            Displacement::South => ::position::Position { x: 0, y: 0, z: 1 },
            Displacement::East => ::position::Position { x: 1, y: 0, z: 0 },
            Displacement::West => ::position::Position { x: -1, y: 0, z: 0 },
            Displacement::Up => ::position::Position { x: 0, y: 1, z: 0 },
            Displacement::Down => ::position::Position { x: 0, y: -1, z: 0 },
        }
    }
}
