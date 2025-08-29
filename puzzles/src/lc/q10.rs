// https://leetcode.com/problems/regular-expression-matching/
use stdext::prelude::*;

pub fn answer(s: String, p: String) -> bool {
    if !p.contains("*") {
        if !p.contains(".") {
            return s == p;
        }
        if s.len() != p.len() {
            return false;
        }
    }
    todo!()
}

pub fn test() {
    answer("aa".to_string(), "a".to_string()).must_be(false);
}