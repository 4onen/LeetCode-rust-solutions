// https://leetcode.com/problems/reverse-integer/

pub struct Solution;

// impl Solution {
//     pub fn reverse(x: i32) -> i32 {
//         let is_neg = x.is_negative();
//         let x = x
//             .abs()
//             .to_string()
//             .chars()
//             .rev()
//             .collect::<String>()
//             .parse::<i32>()
//             .unwrap_or(0);
//         if is_neg {
//             -x
//         } else {
//             x
//         }
//     }
// }

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_neg: bool = x.is_negative();
        let mut out: u32 = (x % 10).abs() as u32;
        let mut x: u32 = (x / 10).abs() as u32;
        while x > 0 {
            out = match (match out.checked_mul(10) {
                Some(v) => v,
                None => return 0,
            })
            .checked_add(x % 10)
            {
                Some(v) => v,
                None => return 0,
            };
            x /= 10;
        }
        if is_neg {
            if out > i32::MAX as u32 + 1 {
                0
            } else if out == i32::MAX as u32 + 1 {
                i32::MIN
            } else {
                -(out as i32)
            }
        } else {
            if out > i32::MAX as u32 {
                0
            } else {
                out as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::reverse(0), 0);
    }

    #[test]
    fn my_extrema_ex1() {
        assert_eq!(Solution::reverse(i32::MAX), 0);
    }

    #[test]
    fn my_extrema_ex2() {
        assert_eq!(Solution::reverse(i32::MIN), 0);
    }

    #[test]
    fn my_extrema_ex3() {
        assert_eq!(Solution::reverse(1111111111), 1111111111);
    }

    #[test]
    fn my_extrema_ex4() {
        assert_eq!(Solution::reverse(-1111111111), -1111111111);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(Solution::reverse(1563847412), 0);
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(Solution::reverse(-1563847412), 0);
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}
