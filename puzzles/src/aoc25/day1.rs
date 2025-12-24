use documents::prelude::*;
use std::{fmt::Display, ops::Range};
use stdext::prelude::*;

use crate::aoc25::inputs::input_file::input_file;

#[derive(Debug)]
enum Action {
    L(i32),
    R(i32),
}

fn parse(file: impl Display) -> Vec<Action> {
    let mut actions = vec![];
    with(&[input_file(file)], |d| {
        for line in d["input"].lines()? {
            let line = line?;
            if line.starts_with("L") {
                actions.push(Action::L(line[1..].parse().unwrap()));
            } else {
                actions.push(Action::R(line[1..].parse().unwrap()));
            }
        }
        Ok(())
    });
    actions
}

fn wrap_times(target: i32, range: Range<i32>) -> i32 {
    if range.contains(&target) {
        0
    } else if target < range.start {
        let res = range.start.abs_diff(target) as i32;
        let diff = range.end - range.start;
        res / diff + 1
    } else {
        let res = range.end.abs_diff(target) as i32;
        let diff = range.end - range.start;
        res / diff + 1
    }
}

fn password(start: i32, actions: &[Action], acc: i32) -> i32 {
    match actions {
        [] => acc,
        [action, actions @ ..] => match action {
            Action::L(clicks) => {
                let result = wrap(start - clicks, 0..100);
                password(result, actions, if result == 0 { acc + 1 } else { acc })
            }
            Action::R(clicks) => {
                let result = wrap(start + clicks, 0..100);
                password(result, actions, if result == 0 { acc + 1 } else { acc })
            }
        },
    }
}

fn password2(start: i32, actions: &[Action], acc: i32) -> i32 {
    match actions {
        [] => acc,
        [action, actions @ ..] => match action {
            Action::L(clicks) => {
                assert!((0..100).contains(&wrap(start - clicks, 0..100)));
                password2(
                    wrap(start - clicks, 0..100),
                    actions,
                    acc + wrap_times(start - clicks, 0..100),
                )
            }
            Action::R(clicks) => {
                assert!((0..100).contains(&wrap(start - clicks, 0..100)));
                password2(
                    wrap(start + clicks, 0..100),
                    actions,
                    acc + wrap_times(start + clicks, 0..100),
                )
            }
        },
    }
}

pub fn part1() -> impl Display {
    password(50, &parse("day1"), 0)
}

pub fn part2() -> impl Display {
    password2(50, &parse("day1"), 0)
}

#[cfg(test)]
mod test {
    use stdext::prelude::*;

    use crate::aoc25::day1::wrap_times;

    #[test]
    fn test1() {
        wrap_times(50 - 68, 0..100).must_be(1);
    }

    #[test]
    fn test2() {
        wrap_times(50 + 1000, 0..100).must_be(10);
    }
}
