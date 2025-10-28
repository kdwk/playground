// https://leetcode.com/problems/divide-two-integers/

use stdext::prelude::Assertable;

type Digit = u8;

trait IntExt {
    fn to_digits(&self) -> Vec<Digit>;
}

impl IntExt for i32 {
    fn to_digits(&self) -> Vec<Digit> {
        self.to_string()
            .chars()
            .map(|i| i.to_digit(10).unwrap() as Digit)
            .collect()
    }
}

impl IntExt for i64 {
    fn to_digits(&self) -> Vec<Digit> {
        self.to_string()
            .chars()
            .map(|i| i.to_digit(10).unwrap() as Digit)
            .collect()
    }
}

trait IterDigitExt {
    fn combine(self) -> i32;
    fn combine_signed(self, sign: i8) -> i32;
}

impl<T: IntoIterator<Item = Digit>> IterDigitExt for T {
    fn combine(self) -> i32 {
        self.into_iter()
            .map(|n| n.to_string())
            .reduce(|result, next| result + &next)
            .unwrap()
            .parse()
            .unwrap_or(i32::MAX)
    }
    fn combine_signed(self, sign: i8) -> i32 {
        let s = self
            .into_iter()
            .map(|n| n.to_string())
            .reduce(|result, next| result + &next)
            .unwrap();
        if sign < 0 {
            ("-".to_string() + &s).parse().unwrap_or(i32::MIN)
        } else {
            s.parse().unwrap_or(i32::MAX)
        }
    }
}

fn divide(dividend: impl IntoIterator<Item = Digit>, divisor: i64) -> (Digit, Vec<Digit>) {
    let dividend = dividend.combine() as i64;
    let divisor = divisor.abs();
    let mut sum = 0;
    let mut quotient: i64 = 0;
    let mut remainder = 0;
    while sum < dividend {
        if sum + divisor <= dividend {
            sum += divisor;
            quotient += 1;
        } else {
            remainder = dividend - sum;
            break;
        }
    }
    (quotient as Digit, (remainder as i32).to_digits())
}

fn long_division(dividend: i32, divisor: i32) -> i32 {
    if dividend == divisor {
        return 1;
    }
    let neg = (dividend < 0) ^ (divisor < 0);
    let dividend = (dividend as i64).abs();
    let divisor = (divisor as i64).abs();
    let dividend_digits = dividend.to_digits();
    let mut carry_over: Vec<Digit> = vec![0];
    let mut result = vec![];
    for digit in dividend_digits {
        let (quotient, remainder) = divide([carry_over, vec![digit]].concat(), divisor);
        carry_over = remainder;
        result.push(quotient);
    }
    if neg {
        result.combine_signed(-1)
    } else {
        result.combine()
    }
}

pub fn test() {
    // long_division(10, 3).must_be(3);
    // long_division(7, -3).must_be(-2);
    // long_division(1, 1).must_be(1);
    // long_division(100, 3).must_be(33);
    // long_division(1000, -22).must_be(-45);
    // long_division(-2147483648, -1).must_be(2147483647);
    // long_division(2147483647, 2).must_be(1073741823);
    long_division(-2147483648, -2147483648).must_be(1);
}
