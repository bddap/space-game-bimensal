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

impl Into<[f32; 3]> for Position {
    fn into(self) -> [f32; 3] {
        let Position { x, y, z } = self;
        [x as f32, y as f32, z as f32]
    }
}

impl Position {
    // we are sorta defining transfomation matrices here, maybe that will be useful later
    pub fn turn(&self, orientation: ::orientation::Orientation) -> Position {
        let ::orientation::Orientation {
            vertical,
            horizontal,
        } = orientation;
        self.vertical_turn(vertical).horizonal_turn(horizontal)
        // [0, 0, 1] == Forward
        // [0, 1, 0] == Down
        // [1, 0, 0] == Right
    }

    fn horizonal_turn(&self, orientation: ::orientation::HorizontalOrientation) -> Position {
        use orientation::HorizontalOrientation as H;
        let Position { x, y, z } = *self;
        match orientation {
            H::North => [x, y, z],
            H::South => [-x, y, -z],
            H::East => [z, y, -x],
            H::West => [-z, y, x],
        }.into()
    }

    fn vertical_turn(&self, orientation: ::orientation::VerticalOrientation) -> Position {
        use orientation::VerticalOrientation as V;
        let Position { x, y, z } = *self;
        match orientation {
            V::Center => [x, y, z],
            V::Up => [x, -z, y],
            V::Down => [x, z, -y],
        }.into()
    }

    // pub fn turn(&self, direction: ::direction::Direction) -> Position {
    //     use self::Position as P;
    //     use direction::Direction as D;

    //     let P { x, y, z } = *self;

    //     match direction {
    //         D::North => [x, y, z],
    //         D::South => [-x, y, -z],
    //         D::East => [z, y, -x],
    //         D::West => [-z, y, x],
    //         D::Up => [x, -z, y],
    //         D::Down => [x, z, -y],
    //     }.into()

    //     // [0, 0, 1] == Forward
    //     // [0, 1, 0] == Down
    //     // [1, 0, 0] == Right
    // }
}
