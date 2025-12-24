use std::ops::Range;

pub fn wrap(target: i32, range: Range<i32>) -> i32 {
    let range_size = range.end - range.start;

    let wrapped_value = (target - range.start) % range_size + range.start;

    if wrapped_value < range.start {
        wrapped_value + range_size
    } else {
        wrapped_value
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::Assertable;

    use super::wrap;

    #[test]
    fn test1() {
        assert_eq!(wrap(-1, 0..50), 49);
    }

    #[test]
    fn test2() {
        assert_eq!(wrap(0, 0..50), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(wrap(50, 0..50), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(wrap(51, 0..50), 1);
    }

    #[test]
    fn test5() {
        assert_eq!(wrap(100, 0..50), 0);
    }

    #[test]
    fn test6() {
        assert_eq!(wrap(-11, -10..-1), -2);
    }

    #[test]
    fn test7() {
        assert_eq!(wrap(0 - 1, 0..100), 99);
    }

    #[test]
    fn test8() {
        assert_eq!(wrap(60 - 860, 0..100), 0);
    }

    #[test]
    fn test9() {
        assert_eq!(wrap(-10, -10..-1), -10);
    }

    #[test]
    fn test10() {
        wrap(-10, 20..30).must_be(20);
    }

    #[test]
    fn test11() {
        wrap(100, 20..30).must_be(20);
    }

    #[test]
    fn test12() {
        wrap(-50, -30..-20).must_be(-30);
    }

    #[test]
    fn test13() {
        wrap(0, -30..-20).must_be(-30);
    }
}
