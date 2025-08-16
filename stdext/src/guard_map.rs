pub mod prelude {
    pub use super::{Location, Map, map1};
}

pub const fn map1() -> Map<'static> {
    Map {
        dimensions: (5, 5),
        guard: Guard {
            location: Location { x: 4, y: 1 },
            direction: Direction::S,
        },
        walls: &[Location { x: 4, y: 2 }, Location { x: 1, y: 1 }],
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    const fn turn_right(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    x: i32,
    y: i32,
}

impl Location {
    const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    const fn move_in(&self, direction: Direction) -> Location {
        match direction {
            Direction::N => Location {
                x: self.x,
                y: self.y - 1,
            },
            Direction::E => Location {
                x: self.x + 1,
                y: self.y,
            },
            Direction::S => Location {
                x: self.x,
                y: self.y + 1,
            },
            Direction::W => Location {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

struct Guard {
    location: Location,
    direction: Direction,
}

pub struct Map<'a> {
    dimensions: (i32, i32),
    guard: Guard,
    walls: &'a [Location],
}

impl<'a> Map<'a> {
    fn is_wall(&self, location: &Location) -> bool {
        self.walls.contains(location)
    }
    const fn in_bounds(&self, location: &Location) -> bool {
        location.x >= 0
            && location.x < self.dimensions.0
            && location.y >= 0
            && location.y < self.dimensions.1
    }
    const fn guard_in_bounds(&self) -> bool {
        self.in_bounds(&self.guard.location)
    }
    fn walk(&mut self) -> Location {
        let old_direction = self.guard.direction;
        let old_pos = self.guard.location;
        let new_pos = old_pos.move_in(old_direction);
        if !self.is_wall(&new_pos) {
            self.guard.location = new_pos;
        } else {
            self.guard.direction = old_direction.turn_right();
        }
        old_pos
    }
    pub fn walk_till_end(&mut self) -> Location {
        let mut old_pos = self.guard.location;
        while self.guard_in_bounds() {
            old_pos = self.walk();
        }
        old_pos
    }
}

pub mod test {
    use std::time::Instant;

    use super::prelude::*;

    pub fn test() {
        let start = Instant::now();
        let location = map1().walk_till_end();
        let finish = Instant::now();
        println!(
            "Found answer {:?} in {}ns",
            location,
            (finish - start).as_nanos()
        );
    }
}
