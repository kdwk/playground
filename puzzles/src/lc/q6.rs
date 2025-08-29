// https://leetcode.com/problems/zigzag-conversion/
use std::cmp::{max, min};
use stdext::prelude::*;

pub fn answer(s: String, num_rows: i32) -> String {
    let mut lines: Vec<Vec<char>> = vec![];
    for _ in 0..num_rows {
        lines.push(vec![]);
    }
    let mut rowidx = 0i32;
    let mut step = 1;
    for c in s.chars() {
        lines[rowidx as usize].push(c);
        if rowidx + step == num_rows {
            step = -1;
        } else if rowidx + step == -1 {
            step = 1;
        }
        rowidx = max(0, min(rowidx + step, num_rows - 1));
    }
    lines
        .into_iter()
        .map(|line| line
            .into_iter()
            .collect::<String>())
        .collect::<String>()
}

pub fn test() {
    answer("PAYPALISHIRING".to_string(), 3).must_be("PAHNAPLSIIGYIR");
    answer("PAYPALISHIRING".to_string(), 1).must_be("PAYPALISHIRING");
}