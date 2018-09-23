#[derive(PartialEq, Copy, Clone, Ord, PartialOrd, Eq, Debug)]
pub enum Voxel {
    Asteroid,
    Vacuum,
}

use self::Voxel as V;

impl Voxel {
    pub fn impose(&self, other: Voxel, from: Displacement) -> Option<Voxel> {
        match (*self, other, from) {
            _ => None,
        }
    }

    pub fn combine(self, other: Voxel) -> Voxel {
        match (self, other) {
            (V::Asteroid, V::Vacuum) | (V::Vacuum, V::Asteroid) => V::Asteroid,
            (V::Asteroid, V::Asteroid) => V::Asteroid,
            (V::Vacuum, V::Vacuum) => V::Vacuum,
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
            Displacement::North => ::position::Position { x: 0, y: 0, z: 1 },
            Displacement::South => ::position::Position { x: 0, y: 0, z: -1 },
            Displacement::East => ::position::Position { x: 1, y: 0, z: 0 },
            Displacement::West => ::position::Position { x: -1, y: 0, z: 0 },
            Displacement::Up => ::position::Position { x: 0, y: -1, z: 0 },
            Displacement::Down => ::position::Position { x: 0, y: 1, z: 0 },
        }
    }
}
