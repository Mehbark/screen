use std::ops;

use crate::components::Direction;

/// Represents any generic position
///
/// `x` is from left to right,
/// `y` is from top to bottom
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl ops::Add<Direction> for Pos {
    type Output = Self;

    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => self - Self { x: 0, y: 1 },
            Direction::Down => self + Self { x: 0, y: 1 },
            Direction::Left => self - Self { x: 1, y: 0 },
            Direction::Right => self + Self { x: 1, y: 0 },
        }
    }
}

impl ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
