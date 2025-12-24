use std::{fmt::Display, ops::RangeInclusive};

use documents::prelude::*;
use stdext::recipe::Log;

use crate::aoc25::inputs::input_file::input_file;

fn parse(file: impl Display) -> Vec<RangeInclusive<u64>> {
    let mut ret = vec![];
    with(&[input_file(file)], |d| {
        let content = d["input"].content()?;
        for range in content.split(",") {
            let (start, end) = range.trim().split_once("-").unwrap();
            ret.push(start.parse().unwrap()..=end.parse().unwrap());
        }
        Ok(())
    });
    ret
}

fn is_one_repeat(i: u64) -> bool {
    let i = i.to_string();
    i.len() % 2 == 0 && {
        let (left, right) = i.split_at(i.len() / 2);
        left == right
    }
}

fn is_any_repeat(i: u64) -> bool {
    let _is_any_repeat = |s: &str, n: usize| {
        if n > s.len() {}
    };
    false
}

pub fn part1() -> impl Display {
    parse("day2")
        .into_iter()
        .map(|range| range.filter(|i| is_one_repeat(*i)).sum::<u64>())
        .sum::<u64>()
}
