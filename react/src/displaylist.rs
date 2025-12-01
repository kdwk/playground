use std::ops::{Add, Sub};

use crate::element::Frame;

pub mod prelude {}

#[derive(Debug, Clone, Copy, Hash, Default, PartialEq)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    pub fn adjacent(self, direction: Direction) -> Option<Self> {
        Some(Self {
            x: match direction {
                Direction::Start => self.x.checked_sub(1)?,
                Direction::End => self.x.checked_add(1)?,
                _ => self.x,
            },
            y: match direction {
                Direction::Up => self.y.checked_sub(1)?,
                Direction::Down => self.y.checked_add(1)?,
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

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub type Point = Vec2;
pub type Size = Vec2;

#[derive(Debug, Clone, Copy, Hash)]
pub enum Direction {
    Start,
    End,
    Up,
    Down,
}

#[derive(Debug, Clone, Hash)]
pub enum Operation {
    PutChar(char),
    MoveTo(Point),
    Move(Direction),
}

impl Operation {
    pub fn realize(
        self,
        anchor: &Point,
        offset: &mut Point,
        buffer: &mut Frame,
        constraint: &Size,
    ) {
        match self {
            Operation::PutChar(c) => {
                if offset.within_constraint(constraint) {
                    let target = *anchor + *offset;
                    if let Some(row) = buffer.get_mut(target.y) {
                        if let Some(col) = row.get_mut(target.x) {
                            *col = c;
                        }
                    }
                }
            }
            Operation::MoveTo(Point { x, y }) => {
                *offset = Point {
                    x: std::cmp::min(x, constraint.x - 1),
                    y: std::cmp::min(y, constraint.y - 1),
                }
            }
            Operation::Move(direction) => {
                if let Some(new_offset) = offset.adjacent(direction) {
                    *offset = new_offset;
                }
            }
        }
    }
}

pub struct DisplayList<Operations: IntoIterator<Item = Operation>>(Operations);

impl<Operations: IntoIterator<Item = Operation>> DisplayList<Operations> {
    fn draw_on(self, buffer: &mut Frame, constraint: Size, anchor: Point) {
        let mut offset = Point::default();
        self.0
            .into_iter()
            .for_each(|op| op.realize(&anchor, &mut offset, buffer, &constraint));
    }
    fn merge_with(&mut self, other: DisplayList<impl IntoIterator<Item = Operation>>,)
}

#[cfg(test)]
mod test {
    use super::*;
    use stdext::prelude::*;

    #[test]
    fn test6() {
        let mut buffer = vec![vec![' '; 5]; 5];
        DisplayList([]).draw_on(&mut buffer, Size { x: 3, y: 3 }, Point { x: 2, y: 1 });
        buffer.must_be(vec![
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
        ]);
    }
    #[test]
    fn test7() {
        let mut buffer = vec![vec![' '; 5]; 5];
        DisplayList([Operation::PutChar('a')]).draw_on(
            &mut buffer,
            Size { x: 3, y: 3 },
            Point { x: 2, y: 1 },
        );
        buffer.must_be(vec![
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', 'a', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
        ]);
    }
    #[test]
    fn test8() {
        let mut buffer = vec![vec![' '; 5]; 5];
        DisplayList([
            Operation::PutChar('a'),
            Operation::Move(Direction::End),
            Operation::PutChar('b'),
            Operation::Move(Direction::End),
            Operation::PutChar('c'),
        ])
        .draw_on(&mut buffer, Size { x: 2, y: 3 }, Point { x: 2, y: 1 });
        buffer.must_be(vec![
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', 'a', 'b', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
        ]);
    }
}
