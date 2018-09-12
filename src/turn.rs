// extern crate mint;

// #[derive(Debug, Copy, Clone)]
// pub enum Turn {
//     Right,
//     Left,
//     Up,
//     Down,
// }

// impl Into<::position::Position> for Turn {
//     fn into(self) -> ::position::Position {
//         match self {
//             Turn::Right => ::position::Position { x: 1, y: 0, z: 0 },
//             Turn::Left => ::position::Position { x: -1, y: 0, z: 0 },
//             Turn::Up => ::position::Position { x: 0, y: -1, z: 0 },
//             Turn::Down => ::position::Position { x: 0, y: 1, z: 0 },
//         }
//     }
// }

// impl Into<mint::Quaternion<f32>> for Turn {
//     fn into(self) -> mint::Quaternion<f32> {
//         match self {
//             Turn::Right => [0.707, 0.0, -0.707, 0.0].into(),
//             Turn::Left => [0.707, 0.0, 0.707, 0.0].into(),
//             Turn::Up => [0.707, 0.0, 0.0, 0.707].into(),
//             Turn::Down => [0.707, 0.0, 0.0, -0.707].into(),
//         }
//     }
// }

// impl Into<::orientation::VerticalOrientation> for Turn {
//     fn into(self) -> ::orientation::VerticalOrientation {
//         use orientation::VerticalOrientation::*;
//         match self {
//             Turn::Right => Center,
//             Turn::Left => Center,
//             Turn::Up => Up,
//             Turn::Down => Down,
//         }
//     }
// }

// impl Into<::orientation::HorizontalOrientation> for Turn {
//     fn into(self) -> ::orientation::HorizontalOrientation {
//         use orientation::HorizontalOrientation::*;
//         match self {
//             Turn::Right => Right,
//             Turn::Left => Left,
//             Turn::Up => North,
//             Turn::Down => North,
//         }
//     }
// }
