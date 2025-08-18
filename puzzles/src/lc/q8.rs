/// atoi

fn solution(s: String) -> i32 {
    let (sign, digits) = parse(s);
    number(sign, digits)
}

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

enum Sign { Pos, Neg }

impl Sign {
    fn value(&self) -> i32 {
        match self {
            Sign::Pos => 1,
            Sign::Neg => -1
        }
    }
}

fn parse(s: String) -> (Sign, Vec<i32>) {
    let mut ret = vec![];
    let mut sign = Sign::Pos;
    let mut sig_fig_begin = false;
    for c in s.chars() {
        if DIGITS.contains(&c) {
            sig_fig_begin = true;
            ret.push(digit(c));
        } else {
            if c == ' ' && !sig_fig_begin {
                continue;
            }
            if c == '+' && !sig_fig_begin {
                sig_fig_begin = true;
                continue;
            }
            if c == '-' && !sig_fig_begin {
                sign = Sign::Neg;
                sig_fig_begin = true;
                continue;
            }
            break;
        }
    }
    (sign, ret)
}

fn digit(c: char) -> i32 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => unreachable!()
    }
}

fn number(sign: Sign, digits: Vec<i32>) -> i32 {
    let mut num: i32 = 0;
    for (i, digit) in digits.iter().rev().enumerate() {
        match sign {
            Sign::Pos => num = num.saturating_add(digit.saturating_mul(10i32.pow(i as u32))),
            Sign::Neg => num = num.saturating_sub(digit.saturating_mul(10i32.pow(i as u32)))
        }
    }
    num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        println!("{}", solution("+-12".to_string()));
        assert_eq!(solution("+-12".to_string()), 0);
    }
}