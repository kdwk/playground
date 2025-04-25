use extend::ext;
use itertools::{Itertools, repeat_n};

pub mod prelude {
    pub use super::{Operator, find_combination};
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Operator {
    const fn values() -> &'static [Self] {
        &[Self::Plus, Self::Minus, Self::Mul, Self::Div]
    }
}

fn calc(nums: &[i32], ops: &[&Operator]) -> i32 {
    match nums {
        [] => 0,
        [num, nums @ ..] => match ops {
            [] => *num,
            [op, ops @ ..] => match op {
                Operator::Plus => num + calc(nums, ops),
                Operator::Minus => num - calc(nums, ops),
                Operator::Mul => num * calc(nums, ops),
                Operator::Div => {
                    let ans = calc(nums, ops);
                    if ans != 0 { num / ans } else { 0 }
                }
            },
        },
    }
}

pub fn find_combination(nums: &[i32], goal: i32) -> Vec<&Operator> {
    let mut combinations =
        repeat_n(Operator::values().into_iter(), nums.len() - 1).multi_cartesian_product();
    combinations
        .find(|ops| {
            let ans = calc(nums, ops);
            println!("{ops:?} = {ans}");
            ans == goal
        })
        .unwrap_or(vec![])
}

#[cfg(test)]
pub mod test {
    use super::prelude::*;

    #[test]
    pub fn test1() {
        assert_ne!(
            find_combination(&[1, 3, 7, 10, 25, 50], 765),
            Vec::<&Operator>::new()
        );
    }
}
