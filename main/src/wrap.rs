use std::ops::{Add, Bound, Range, RangeBounds};

pub mod prelude {
    pub use super::wrap;
}

trait ToRange: RangeBounds<i32> {
    fn to_bounds(&self) -> Range<i32> {
        let Bound::Included(&start) = self.start_bound() else {
            panic!()
        };
        let end = match self.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            _ => panic!(),
        };
        start..end
    }
}

/// Wrap an i32 in a range such that it wraps around to the other end of range as it leaves the range
///
/// range: inclusive..exclusive. End must be greater than (>) start.
///
/// Examples:
/// ```
/// assert_eq!(wrap(2, -7..-5), -6);
/// assert_eq!(wrap(5, -2..2), 1);
/// assert_eq!(wrap(-10, -7..-5), -6);
/// assert_eq!(wrap(21, 10..20), 11);
/// assert_eq!(wrap(-1, 10..20), 19);
/// ```
#[inline]
pub fn wrap(i: i32, range: Range<i32>) -> i32 {
    let end = range.end;
    let start = range.start;
    let diff = end - start;
    assert!(diff > 0);
    if i >= end {
        let offset = (i - end) % diff;
        start + offset
    } else if i < start {
        let offset = (start - i) % diff;
        if offset == 0 { start } else { end - offset }
    } else {
        i
    }
}

#[cfg(test)]
mod test {
    use super::prelude::*;

    #[test]
    fn wrap_test1() {
        assert_eq!(wrap(2, -7..-5), -6);
    }

    #[test]
    fn wrap_test2() {
        assert_eq!(wrap(5, -2..2), 1);
    }

    #[test]
    fn wrap_test3() {
        assert_eq!(wrap(-10, -7..-5), -6);
    }

    #[test]
    fn wrap_test4() {
        assert_eq!(wrap(21, 10..20), 11);
    }

    #[test]
    fn wrap_test5() {
        assert_eq!(wrap(30, 10..20), 10);
    }

    #[test]
    fn wrap_test6() {
        assert_eq!(wrap(6 - 300, 0..7), 0);
    }
}
