// https://leetcode.com/problems/sqrtx/

pub struct Solution;

// Original
// impl Solution {
//     pub fn my_sqrt(x: i32) -> i32 {
//         if x == 0 {
//             return 0;
//         }
//         let mut low = 1;
//         let mut high = std::cmp::min(
//             46340, // sqrt(i32::MAX)
//             x / 2 + 1,
//         );
//         while low <= high {
//             let mid = low + (high - low) / 2;
//             let mid_squared = mid * mid;
//             if mid_squared == x {
//                 return mid;
//             } else if mid_squared < x {
//                 low = mid + 1;
//             } else {
//                 high = mid - 1;
//             }
//         }
//         high
//     }
// }

impl Solution {
    pub const fn my_sqrt(x: i32) -> i32 {
        let mut low = 1;
        let mut high = x / 2 + 1;
        if high > 46340 {
            high = 46340; // sqrt(i32::MAX)
        }
        while low <= high {
            let mid = low + (high - low) / 2;
            let mid_squared = mid * mid;
            if mid_squared == x {
                return mid;
            } else if mid_squared < x {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        high
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::my_sqrt(0), 0);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::my_sqrt(1), 1);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::my_sqrt(2), 1);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::my_sqrt(3), 1);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::my_sqrt(9), 3);
    }

    #[test]
    fn myex6() {
        assert_eq!(Solution::my_sqrt(10), 3);
    }

    #[test]
    fn myex7() {
        assert_eq!(Solution::my_sqrt(15), 3);
    }

    #[test]
    fn myex8() {
        assert_eq!(Solution::my_sqrt(16), 4);
    }

    #[test]
    fn myex9() {
        assert_eq!(Solution::my_sqrt(17), 4);
    }

    #[test]
    fn myex10() {
        assert_eq!(Solution::my_sqrt(24), 4);
    }

    #[test]
    fn myex11() {
        assert_eq!(Solution::my_sqrt(25), 5);
    }

    #[test]
    fn myex12() {
        assert_eq!(Solution::my_sqrt(26), 5);
    }

    #[test]
    fn failing_case() {
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }

    #[test]
    fn failing_case2() {
        assert_eq!(Solution::my_sqrt(2147483647), 46340);
    }

    #[test]
    fn myex_max() {
        assert_eq!(Solution::my_sqrt(i32::MAX), 46340);
    }
}
