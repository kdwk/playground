// https://leetcode.com/problems/total-distance-traveled/

use std::cmp::min;

use stdext::prelude::*;

fn answer(mut main_tank: i32, mut additional_tank: i32) -> i32 {
    let mut used_gas = 0;
    loop {
        if main_tank <= 0 {
            break;
        }
        used_gas += 1;
        main_tank -= 1;
        if used_gas % 5 == 0 && additional_tank > 0 {
            main_tank += 1;
            additional_tank -= 1;
        }
    }
    used_gas * 10
}

pub fn test() {
    answer(5, 10).must_be(60);
    answer(1, 2).must_be(10);
    answer(9, 2).must_be(110);
}
