use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub mod prelude {
    pub(crate) use super::Pipe;
    pub use super::{
        Axis, Dimension, Dimension::Flex, Dimension::Pixel, Direction, Pipeline, Point,
        ProposedSize, Size, Vec2,
    };
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
pub enum Dimension {
    Pixel(isize),
    Flex(isize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ProposedSize {
    pub x: Option<isize>,
    pub y: Option<isize>,
}

pub trait Realize {
    fn realize(self, axis_constraint: Option<Dimension>) -> Option<Dimension>;
}

// impl<'a, T: IntoIterator<Item = &'a ProposedSize>> Realize for T {
//     fn realize(self, axis_constraint: Option<Dimension>) -> Option<Dimension> {
//         let pixels_sum = self.into_iter().map(|)
//     }
// }

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
                (None, y2) => y2,
            },
        }
    }
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: match (self.x, rhs.x) {
                (Some(x1), Some(x2)) => Some(x1.max(x2)),
                _ => None,
            },
            y: match (self.y, rhs.y) {
                (Some(y1), Some(y2)) => Some(y1.max(y2)),
                _ => None,
            },
        }
    }
}

impl Add for ProposedSize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: match (self.x, rhs.x) {
                (Some(x1), Some(x2)) => Some(x1 + x2),
                _ => None,
            },
            y: match (self.y, rhs.y) {
                (Some(y1), Some(y2)) => Some(y1 + y2),
                _ => None,
            },
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
                (Some(x1), Some(x2)) => Some(x1 - x2),
                _ => None,
            },
            y: match (self.y, rhs.y) {
                (Some(y1), Some(y2)) => Some(y1 - y2),
                _ => None,
            },
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
            x: if let Some(x) = self.x { Some(-x) } else { None },
            y: if let Some(y) = self.y { Some(-y) } else { None },
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

pub trait Pipeline<Arg, Ret> {
    fn apply(self, arg: Arg) -> Ret;
}

impl<Arg, Ret, F> Pipeline<Arg, Ret> for F
where
    F: FnOnce(Arg) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self(arg)
    }
}

impl<Arg, Ret, A, F1, F2> Pipeline<Arg, Ret> for (F1, F2)
where
    F1: FnOnce(Arg) -> A,
    F2: FnOnce(A) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self.1(self.0.apply(arg))
    }
}

impl<Arg, Ret, A, B, F1, F2, F3> Pipeline<Arg, Ret> for (F1, F2, F3)
where
    F1: FnOnce(Arg) -> A,
    F2: FnOnce(A) -> B,
    F3: FnOnce(B) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self.2((self.0, self.1).apply(arg))
    }
}

impl<Arg, Ret, A, B, C, F1, F2, F3, F4> Pipeline<Arg, Ret> for (F1, F2, F3, F4)
where
    F1: FnOnce(Arg) -> A,
    F2: FnOnce(A) -> B,
    F3: FnOnce(B) -> C,
    F4: FnOnce(C) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self.3((self.0, self.1, self.2).apply(arg))
    }
}

impl<Arg, Ret, A, B, C, D, F1, F2, F3, F4, F5> Pipeline<Arg, Ret> for (F1, F2, F3, F4, F5)
where
    F1: FnOnce(Arg) -> A,
    F2: FnOnce(A) -> B,
    F3: FnOnce(B) -> C,
    F4: FnOnce(C) -> D,
    F5: FnOnce(D) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self.4((self.0, self.1, self.2, self.3).apply(arg))
    }
}

impl<Arg, Ret, A, B, C, D, E, F1, F2, F3, F4, F5, F6> Pipeline<Arg, Ret>
    for (F1, F2, F3, F4, F5, F6)
where
    F1: FnOnce(Arg) -> A,
    F2: FnOnce(A) -> B,
    F3: FnOnce(B) -> C,
    F4: FnOnce(C) -> D,
    F5: FnOnce(D) -> E,
    F6: FnOnce(E) -> Ret,
{
    #[inline]
    fn apply(self, arg: Arg) -> Ret {
        self.5((self.0, self.1, self.2, self.3, self.4).apply(arg))
    }
}

pub(crate) trait Pipe<Arg, Ret> {
    fn pipe(self, pipeline: impl Pipeline<Arg, Ret>) -> Ret;
}

impl<T, Ret> Pipe<T, Ret> for T {
    fn pipe(self, pipeline: impl Pipeline<T, Ret>) -> Ret {
        pipeline.apply(self)
    }
}
