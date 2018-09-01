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
            } => ::direction::Direction::Up,
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
        use direction::Direction;
        let vertical = match (self.vertical, direction) {
            (_, ::direction::Direction::North) => VerticalOrientation::Center,
            (VerticalOrientation::Up, Direction::Up) => VerticalOrientation::Up,
            (VerticalOrientation::Up, Direction::Down) => VerticalOrientation::Center,
            (VerticalOrientation::Center, Direction::Up) => VerticalOrientation::Up,
            (VerticalOrientation::Center, Direction::Down) => VerticalOrientation::Down,
            (VerticalOrientation::Down, Direction::Up) => VerticalOrientation::Center,
            (VerticalOrientation::Down, Direction::Down) => VerticalOrientation::Down,
            (s, _) => s,
        };
        let hdirection = match direction {
            Direction::North | Direction::Up | Direction::Down => HorizontalOrientation::North,
            Direction::South => HorizontalOrientation::South,
            Direction::East => HorizontalOrientation::East,
            Direction::West => HorizontalOrientation::West,
        };

        Self {
            vertical,
            horizontal: hdirection + self.horizontal,
        }
    }
}

impl Add for HorizontalOrientation {
    type Output = Self;
    fn add(self, other: HorizontalOrientation) -> Self::Output {
        match (self, other) {
            (HorizontalOrientation::North, other) => other,
            (slef, HorizontalOrientation::North) => slef,
            (HorizontalOrientation::East, HorizontalOrientation::East) => {
                HorizontalOrientation::South
            }
            (HorizontalOrientation::East, HorizontalOrientation::West) => {
                HorizontalOrientation::North
            }
            (HorizontalOrientation::East, HorizontalOrientation::South) => {
                HorizontalOrientation::West
            }
            (HorizontalOrientation::West, HorizontalOrientation::West) => {
                HorizontalOrientation::South
            }
            (HorizontalOrientation::West, HorizontalOrientation::South) => {
                HorizontalOrientation::East
            }
            (HorizontalOrientation::South, HorizontalOrientation::South) => {
                HorizontalOrientation::North
            }
            (HorizontalOrientation::West, HorizontalOrientation::East) => {
                HorizontalOrientation::North
            }
            (HorizontalOrientation::South, HorizontalOrientation::East) => {
                HorizontalOrientation::West
            }
            (HorizontalOrientation::South, HorizontalOrientation::West) => {
                HorizontalOrientation::East
            }
        }
    }
}
