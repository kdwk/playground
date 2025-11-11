// https://leetcode.com/problems/fruit-into-baskets/

use std::collections::HashMap;

use stdext::{prelude::Assertable, recipe::Log};

pub fn answer(fruits: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut end = 1;
    let mut unique_fruits = vec![fruits[0]];
    while end < fruits.len() {
        end += 1;
        let new_fruit = fruits[end - 1];
        if !unique_fruits.contains(&new_fruit) {
            if unique_fruits.len() == 2 {
                unique_fruits.remove(0);
                start += 1;
            }
            unique_fruits.push(new_fruit);
        }
        (start, end).log();
    }
    (end - start) as i32
}

pub fn test() {
    answer(vec![1, 2, 1]).must_be(3);
    answer(vec![0, 1, 2, 2]).must_be(3);
    answer(vec![1, 2, 3, 2, 2]).must_be(4);
    answer(vec![0, 1, 2, 2, 3, 2, 4, 2, 2]).must_be(4);
}
