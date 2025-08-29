// https://leetcode.com/problems/reverse-integer/
use stdext::prelude::*;

pub fn answer(x: i32) -> i32 {
    let mut s = x.to_string();
    let neg = s.contains("-");
    if neg {
        s.remove(0);
    }
    let new = (if neg { "-" } else { "" }).chars().chain(s.chars().rev()).collect::<String>();
    new.parse().unwrap_or(0)
}

pub fn test() {
    answer(-123).must_be(-321);
}