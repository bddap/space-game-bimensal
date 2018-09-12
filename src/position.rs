use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl From<[i32; 3]> for Position {
    fn from(other: [i32; 3]) -> Position {
        let [x, y, z] = other;
        Position { x, y, z }
    }
}

impl Position {
    // we are sorta defining transfomation matrices here, maybe that will be useful later
    pub fn turn(&self, direction: ::direction::Direction) -> Position {
        use self::Position as P;
        use direction::Direction as D;

        let P { x, y, z } = *self;

        match direction {
            D::North => [x, y, z],
            D::South => [-x, y, -z],
            D::East => [z, y, -x],
            D::West => [-z, y, x],
            D::Up => [x, -z, y],
            D::Down => [x, z, -y],
        }.into()

        // [0, 0, 1] == Forward
        // [0, 1, 0] == Down
        // [1, 0, 0] == Right
    }
}
