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
            Direction::Up => ::position::Position { x: 0, y: 1, z: 0 },
            Direction::Down => ::position::Position { x: 0, y: -1, z: 0 },
        }
    }
}
