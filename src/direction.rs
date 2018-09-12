extern crate mint;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Into<::position::Position> for Direction {
    fn into(self) -> ::position::Position {
        match self {
            Direction::North => ::position::Position { x: 0, y: 0, z: 1 },
            Direction::South => ::position::Position { x: 0, y: 0, z: -1 },
            Direction::East => ::position::Position { x: 1, y: 0, z: 0 },
            Direction::West => ::position::Position { x: -1, y: 0, z: 0 },
            Direction::Up => ::position::Position { x: 0, y: -1, z: 0 },
            Direction::Down => ::position::Position { x: 0, y: 1, z: 0 },
        }
    }
}

impl Into<mint::Quaternion<f32>> for Direction {
    fn into(self) -> mint::Quaternion<f32> {
        match self {
            Direction::North => [1.0, 0.0, 0.0, 0.0].into(),
            Direction::South => [0.0, 0.0, 1.0, 0.0].into(),
            Direction::East => [0.707, 0.0, -0.707, 0.0].into(),
            Direction::West => [0.707, 0.0, 0.707, 0.0].into(),
            Direction::Up => [0.707, 0.0, 0.0, -0.707].into(),
            Direction::Down => [0.707, 0.0, 0.0, 0.707].into(),
        }
    }
}
