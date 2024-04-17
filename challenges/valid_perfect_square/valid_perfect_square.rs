// https://leetcode.com/problems/valid-perfect-square/

pub struct Solution;

impl Solution {
    pub const fn is_perfect_square(num: i32) -> bool {
        if num < 0 {
            return false;
        }
        if num == 0 {
            return true;
        }
        let num = num as u32;
        let mut left = 1u32;
        let mut right = 1u32 << (num.ilog2() / 2 + 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            let Some(mid_squared) = mid.checked_mul(mid) else {
                right = mid - 1;
                continue;
            };
            if mid_squared == num {
                return true;
            } else if mid_squared < num {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::is_perfect_square(16), true)
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::is_perfect_square(14), false)
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::is_perfect_square(1), true)
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::is_perfect_square(2), false)
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::is_perfect_square(3), false)
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::is_perfect_square(4), true)
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::is_perfect_square(5), false)
    }

    #[test]
    fn myex6() {
        assert_eq!(Solution::is_perfect_square(6), false)
    }

    #[test]
    fn myex7() {
        assert_eq!(Solution::is_perfect_square(7), false)
    }

    #[test]
    fn myex8() {
        assert_eq!(Solution::is_perfect_square(8), false)
    }

    #[test]
    fn myex9() {
        assert_eq!(Solution::is_perfect_square(9), true)
    }

    #[test]
    fn myex10() {
        assert_eq!(Solution::is_perfect_square(10), false)
    }

    #[test]
    fn myex11() {
        assert_eq!(Solution::is_perfect_square(11), false)
    }

    #[test]
    fn myex12() {
        assert_eq!(Solution::is_perfect_square(12), false)
    }

    #[test]
    fn myex13() {
        assert_eq!(Solution::is_perfect_square(13), false)
    }

    #[test]
    fn myex15() {
        assert_eq!(Solution::is_perfect_square(15), false)
    }

    #[test]
    fn myex17() {
        assert_eq!(Solution::is_perfect_square(17), false)
    }

    #[test]
    fn myex18() {
        assert_eq!(Solution::is_perfect_square(18), false)
    }

    #[test]
    fn myex19() {
        assert_eq!(Solution::is_perfect_square(19), false)
    }

    #[test]
    fn myex20() {
        assert_eq!(Solution::is_perfect_square(20), false)
    }

    #[test]
    fn myex21() {
        assert_eq!(Solution::is_perfect_square(21), false)
    }

    #[test]
    fn myex22() {
        assert_eq!(Solution::is_perfect_square(22), false)
    }

    #[test]
    fn myex23() {
        assert_eq!(Solution::is_perfect_square(23), false)
    }

    #[test]
    fn myex24() {
        assert_eq!(Solution::is_perfect_square(24), false)
    }

    #[test]
    fn myex25() {
        assert_eq!(Solution::is_perfect_square(25), true)
    }

    #[test]
    fn my_extreme_ex1() {
        assert_eq!(Solution::is_perfect_square(i32::MAX), false)
    }
}
