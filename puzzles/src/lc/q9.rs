// https://leetcode.com/problems/palindrome-number/description/
use stdext::prelude::SubStr;

fn is_palindrome(s: &String) -> bool {
    match s.len() {
        0 | 1 => true,
        _ => s.chars().nth(0) == s.chars().nth(s.len() - 1)
            && is_palindrome(&s.substr(1, s.len() - 1))
    }
}

pub fn answer(x: i32) -> bool {
    is_palindrome(&x.to_string())
}