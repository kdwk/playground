// https://leetcode.com/problems/single-number/
use stdext::prelude::*;

pub fn answer(nums: Vec<i32>) -> i32 {
    nums.into_iter().reduce(|num, acc| acc ^ num).unwrap_or(0)
}

pub fn test() {
    answer(vec![2, 2, 1]).log(); // 1
    answer(vec![4, 1, 2, 1, 2]).log(); // 4
    answer(vec![1]).log(); // 1
}
