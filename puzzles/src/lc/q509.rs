// https://leetcode.com/problems/fibonacci-number/
use stdext::prelude::*;

pub fn answer(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        n => answer(n - 1) + answer(n - 2),
    }
}

#[test]
pub fn test() {
    answer(4).log();
}
