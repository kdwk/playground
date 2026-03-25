use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub mod prelude {
    pub use super::{Axis, Vec2, Point, Size, Direction, ProposedSize};
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}
#[derive(Debug, Clone, Copy, Hash, Default, PartialEq, Eq)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    pub fn adjacent(self, direction: Direction) -> Option<Self> {
        Some(Self {
            x: match direction {
                Direction::Start => self.x - 1,
                Direction::End => self.x + 1,
                _ => self.x,
            },
            y: match direction {
                Direction::Up => self.y - 1,
                Direction::Down => self.y + 1,
                _ => self.y,
            },
        })
    }

    pub fn within_constraint(&self, constraint: &Self) -> bool {
        self.x < constraint.x && self.y < constraint.y
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub type Point = Vec2;
pub type Size = Vec2;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ProposedSize {
    pub x: Option<isize>,
    pub y: Option<isize>
}

impl ProposedSize {
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: match (self.x, rhs.x) {
                (Some(x1), Some(x2)) => Some(x1.min(x2)),
                (x1, None) => x1,
                (None, x2) => x2,
            },
            y: match (self.y, rhs.y) {
                (Some(y1), Some(y2)) => Some(y1.min(y2)),
                (y1, None) => y1,
                (None, y2) => y2
            }
        }
    }
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: match (self.x, rhs.x) {
                (Some(x1), Some(x2)) => Some(x1.max(x2)),
                _ => None
            },
            y: match (self.y, rhs.y) {
                (Some(y1), Some(y2)) => Some(y1.max(y2)),
                _ => None
            }
        }
    }
}

impl Add for ProposedSize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: match (self.x, rhs.x) {
                (Some(x1), Some(x2)) => Some(x1+x2),
                _ => None
            },
            y: match (self.y, rhs.y) {
                (Some(y1), Some(y2)) => Some(y1+y2),
                _ => None
            }
        }
    }
}

impl AddAssign for ProposedSize {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for ProposedSize {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: match (self.x, rhs.x) {
                (Some(x1), Some(x2)) => Some(x-x2),
                _ => None
            },
            y: match (self.y, rhs.y) {
                (Some(y1), Some(y2)) => Some(y1-y2),
                _ => None
            }
        }
    }
}

impl SubAssign for ProposedSize {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for ProposedSize {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: if let Some(x) = self.x {Some(-x)} else {None},
            y: if let Some(y) = self.y {Some(-y)} else {None}
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Start,
    End,
    Up,
    Down,
}
