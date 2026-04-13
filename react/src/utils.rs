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
use Dimension::{Flex, Pixel};

impl Dimension {
    fn is_pixel(self) -> bool {
        if let Pixel(_) = self { true } else { false }
    }
    fn is_flex(self) -> bool {
        if let Flex(_) = self { true } else { false }
    }
    fn to_pixel(self) -> isize {
        match self {
            Pixel(pix) => pix,
            Flex(_) => 0,
        }
    }
    fn to_flex(self) -> isize {
        match self {
            Pixel(_) => 0,
            Flex(flex) => flex,
        }
    }
    fn add_pixels(self, rhs: Self) -> isize {
        self.to_pixel() + rhs.to_pixel()
    }
    fn add_flex(self, rhs: Self) -> isize {
        self.to_flex() + rhs.to_flex()
    }
}

pub trait OptionDimensionExt {
    fn add_pixels(self, rhs: Option<Dimension>) -> Option<Dimension>;
    fn add_flex(self, rhs: Option<Dimension>) -> Option<Dimension>;
}

impl OptionDimensionExt for Option<Dimension> {
    fn add_pixels(self, rhs: Option<Dimension>) -> Option<Dimension> {
        match (self, rhs) {
            (Some(dim1), Some(dim2)) => Some(Pixel(dim1.add_pixels(dim2))),
            _ => None,
        }
    }
    fn add_flex(self, rhs: Option<Dimension>) -> Option<Dimension> {
        match (self, rhs) {
            (Some(dim1), Some(dim2)) => Some(Flex(dim1.add_flex(dim2))),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ProposedSize {
    pub x: Option<Dimension>,
    pub y: Option<Dimension>,
}

pub trait Realize {
    fn realize(
        self,
        axis_constraint: Option<Dimension>,
    ) -> impl IntoIterator<Item = Option<Dimension>>;
}

struct PixelFlexSum {
    pixels: Option<Dimension>,
    flex: Option<Dimension>,
}

impl<T: IntoIterator<Item = Option<Dimension>>> Realize for T
where
    T::IntoIter: Clone,
{
    fn realize(
        self,
        axis_constraint: Option<Dimension>,
    ) -> impl IntoIterator<Item = Option<Dimension>> {
        let dims = self.into_iter();
        let PixelFlexSum { pixels, flex } = dims.clone().fold(
            PixelFlexSum {
                pixels: Some(Pixel(0)),
                flex: Some(Flex(0)),
            },
            |acc, e| PixelFlexSum {
                pixels: acc.pixels.add_pixels(e),
                flex: acc.flex.add_flex(e),
            },
        );
        assert!(pixels.is_none_or(|dim| dim.is_pixel()));
        assert!(flex.is_none_or(|dim| dim.is_flex()));
        let pixels_for_flex = if let Some(Pixel(constraint)) = axis_constraint
            && let Some(Pixel(pixels_sum)) = pixels
        {
            Some(constraint - pixels_sum)
        } else {
            None
        };
        let flex_sum = flex;
        dims.map(move |dim| match dim {
            Some(Pixel(pix)) => Some(Pixel(pix)),
            Some(Flex(flex)) => if let Some(Flex(flex_sum)) = flex_sum && let Some(pixels_for_flex) = pixels_for_flex {
                Some(Pixel(flex * (pixels_for_flex / flex_sum)))
            } else {
                Some(Flex(flex))
            },
            None => None
        })
    }
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
