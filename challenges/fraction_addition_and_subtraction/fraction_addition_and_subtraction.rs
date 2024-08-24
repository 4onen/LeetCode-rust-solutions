// https://leetcode.com/problems/fraction-addition-and-subtraction/

pub struct Solution;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        const fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                (b, a) = (a % b, b);
            }
            a
        }
        const fn two_divisible(n: i32) -> bool {
            n & 1 == 0
        }
        const fn three_divisible(n: i32) -> bool {
            n % 3 == 0
        }
        let mut numerator: i32 = 0;
        let mut denominator: i32 = 1;
        let mut iter = expression.bytes().peekable();
        while let Some(&b) = iter.peek() {
            let sign = match b {
                b'-' => {iter.next();-1},
                b'+' => {iter.next();1},
                _ => 1,
            };
            let mut num = 0;
            while let Some(&b) = iter.peek() {
                if b == b'/' {
                    iter.next();
                    break;
                }
                num = num * 10 + (b-b'0') as i32;
                iter.next();
            }
            let mut this_denominator = 0;
            while let Some(&b) = iter.peek() {
                if b == b'+' || b == b'-' {
                    break;
                }
                this_denominator = this_denominator * 10 + (b-b'0') as i32;
                iter.next();
            }
            numerator = numerator * this_denominator + sign * num * denominator;
            denominator *= this_denominator;
            // We only need to reduce the fraction's powers of 2 & 3
            // because we have at most 10 fractions, so we have to worry about
            // ten 1/10s or ten 1/9s, etc, both of which contain
            // powers of 2 and 3.
            if two_divisible(numerator) && two_divisible(denominator) {
                numerator >>= 1;
                denominator >>= 1;
            }
            if three_divisible(numerator) && three_divisible(denominator) {
                numerator /= 3;
                denominator /= 3;
            }
        }
        let gcd = gcd(numerator.abs(), denominator);
        format!("{}/{}", numerator / gcd, denominator / gcd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(expression: &str, expected: &str) {
        for b in expression.bytes() {
            assert!(b == b'-' || b == b'+' || b == b'/' || b >= b'0' && b <= b'9');
        }
        for b in expected.bytes() {
            assert!(b == b'-' || b == b'+' || b == b'/' || b >= b'0' && b <= b'9');
        }
        assert_eq!(Solution::fraction_addition(expression.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test("-1/2+1/2", "0/1")
    }

    #[test]
    fn ex2() {
        test("-1/2+1/2+1/3", "1/3")
    }

    #[test]
    fn ex3() {
        test("1/3-1/2", "-1/6")
    }

    #[test]
    fn myex1() {
        test("1/2+1/3", "5/6")
    }

    #[test]
    fn myex2() {
        test("1/2-1/3", "1/6")
    }

    #[test]
    fn myex3() {
        test("1/2+1/3-1/4", "7/12")
    }

    #[test]
    fn myex4() {
        test("7/12+1/5", "47/60");
        test("1/2+1/3-1/4+1/5", "47/60")
    }

    #[test]
    fn myex5() {
        test("1/2+1/3-1/4+1/5-1/6", "37/60")
    }

    #[test]
    fn myex6() {
        test("1/2+1/3-1/4+1/5-1/6+1/7", "319/420")
    }

    #[test]
    fn myex7() {
        test("1/2+1/3-1/4+1/5-1/6+1/7-1/8", "533/840")
    }

    #[test]
    fn myex8() {
        test("1/2+1/3-1/4+1/5-1/6+1/7-1/8+1/9", "1879/2520")
    }

    #[test]
    fn myex9() {
        test("1/2+1/3-1/4+1/5-1/6+1/7-1/8+1/9-1/10", "1627/2520")
    }

    #[test]
    fn my_extreme_ex1() {
        test("1/10+1/10+1/10+1/10+1/10+1/10+1/10+1/10+1/10+1/10", "1/1")
    }

    #[test]
    fn my_extreme_ex2() {
        test("1/9+1/9+1/9+1/9+1/9+1/9+1/9+1/9+1/9+1/9", "10/9")
    }

    #[test]
    fn my_extreme_ex3() {
        test("10/10+10/10+10/10+10/10+10/10+10/10+10/10+10/10+10/10+10/10", "10/1")
    }

    #[test]
    fn my_extreme_ex4() {
        test("10/9+10/9+10/9+10/9+10/9+10/9+10/9+10/9+10/9+10/9", "100/9")
    }
}
