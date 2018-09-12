extern crate cgmath;
extern crate mint;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub enum HorizontalOrientation {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone)]
pub enum VerticalOrientation {
    Up,
    Center,
    Down,
}

#[derive(Debug, Copy, Clone)]
pub struct Orientation {
    pub vertical: VerticalOrientation,
    pub horizontal: HorizontalOrientation,
}

impl Into<::direction::Direction> for Orientation {
    fn into(self) -> ::direction::Direction {
        match self {
            Orientation {
                vertical: VerticalOrientation::Up,
                ..
            } => ::direction::Direction::Up,
            Orientation {
                vertical: VerticalOrientation::Down,
                ..
            } => ::direction::Direction::Down,
            Orientation {
                vertical: VerticalOrientation::Center,
                horizontal,
            } => match horizontal {
                HorizontalOrientation::North => ::direction::Direction::North,
                HorizontalOrientation::South => ::direction::Direction::South,
                HorizontalOrientation::East => ::direction::Direction::East,
                HorizontalOrientation::West => ::direction::Direction::West,
            },
        }
    }
}

impl Into<::position::Position> for Orientation {
    fn into(self) -> ::position::Position {
        let d: ::direction::Direction = self.into();
        d.into()
    }
}

impl Into<mint::Quaternion<f32>> for Orientation {
    fn into(self) -> mint::Quaternion<f32> {
        let vert: mint::Quaternion<f32> = match self.vertical {
            VerticalOrientation::Up => ::direction::Direction::Up,
            VerticalOrientation::Center => ::direction::Direction::North,
            VerticalOrientation::Down => ::direction::Direction::Down,
        }.into();
        let hori: mint::Quaternion<f32> = match self.horizontal {
            HorizontalOrientation::North => ::direction::Direction::North,
            HorizontalOrientation::South => ::direction::Direction::South,
            HorizontalOrientation::East => ::direction::Direction::East,
            HorizontalOrientation::West => ::direction::Direction::West,
        }.into();
        to_mint(to_cgmath(vert) * to_cgmath(hori))
    }
}

fn to_mint(a: cgmath::Quaternion<f32>) -> mint::Quaternion<f32> {
    let a: [f32; 4] = a.into();
    a.into()
}

fn to_cgmath(a: mint::Quaternion<f32>) -> cgmath::Quaternion<f32> {
    let a: [f32; 4] = a.into();
    a.into()
}

impl Orientation {
    pub fn turn(&self, direction: ::direction::Direction) -> Self {
        Self {
            vertical: self.vertical + direction.into(),
            horizontal: self.horizontal + direction.into(),
        }
    }
}

impl From<::direction::Direction> for VerticalOrientation {
    fn from(other: ::direction::Direction) -> VerticalOrientation {
        use self::VerticalOrientation as V;
        use direction::Direction as D;
        match other {
            D::Down => V::Down,
            D::Up => V::Up,
            D::North => V::Center,
            D::West => V::Center,
            D::East => V::Center,
            D::South => V::Center,
        }
    }
}

impl From<::direction::Direction> for HorizontalOrientation {
    fn from(other: ::direction::Direction) -> HorizontalOrientation {
        use self::HorizontalOrientation as H;
        use direction::Direction as D;
        match other {
            D::Down => H::North,
            D::Up => H::North,
            D::North => H::North,
            D::West => H::West,
            D::East => H::East,
            D::South => H::South,
        }
    }
}

impl Add for VerticalOrientation {
    type Output = Self;
    fn add(self, other: VerticalOrientation) -> Self::Output {
        use self::VerticalOrientation as V;
        match (self, other) {
            (V::Up, V::Down) => V::Center,
            (V::Down, V::Up) => V::Center,
            (V::Up, _) => V::Up,
            (_, V::Up) => V::Up,
            (V::Center, a) => a,
            (a, V::Center) => a,
            (V::Down, V::Down) => V::Down,
        }
    }
}

impl Add for HorizontalOrientation {
    type Output = Self;
    fn add(self, other: HorizontalOrientation) -> Self::Output {
        use self::HorizontalOrientation as H;
        match (self, other) {
            (H::North, other) => other,
            (slef, H::North) => slef,
            (H::East, H::East) => H::South,
            (H::East, H::West) => H::North,
            (H::East, H::South) => H::West,
            (H::West, H::West) => H::South,
            (H::West, H::South) => H::East,
            (H::South, H::South) => H::North,
            (H::West, H::East) => H::North,
            (H::South, H::East) => H::West,
            (H::South, H::West) => H::East,
        }
    }
}

// impl Orientation {
//     fn turn(&self, other: ::turn::Turn) -> Orientation {
//         use turn::Turn;
//         let vertical = match (self.vertical, other) {};
//         // let horizontal = ;
//     }
// }
