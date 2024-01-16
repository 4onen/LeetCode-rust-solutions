// https://leetcode.com/problems/calculate-money-in-leetcode-bank/

pub struct Solution;

// impl Solution {
//     // Loop solution
//     pub fn total_money(n: i32) -> i32 {
//         let mut total = 0;
//         let mut days = 0;
//         let mut week = 0;
//         let mut day_of_week = 1;
//         while days < n {
//             while days < n && day_of_week <= 7 {
//                 total += day_of_week + week;
//                 day_of_week += 1;
//                 days += 1;
//             }
//             week += 1;
//             day_of_week = 1;
//         }
//         total
//     }
// }

impl Solution {
    // Math sol'n
    pub const fn total_money(n: i32) -> i32 {
        // https://www.desmos.com/calculator/xyoskuljuy
        let x = n - 1;
        let week_num = x / 7;
        let day_num = x % 7 + 1;
        let up_to_current_week = (7 * week_num * (week_num + 7)) >> 1;
        let this_week = (day_num * week_num) + ((day_num * (day_num + 1)) >> 1);
        up_to_current_week + this_week
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::total_money(4), 10);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::total_money(10), 37);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::total_money(20), 96);
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::total_money(1), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::total_money(2), 1 + 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::total_money(3), 1 + 2 + 3);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::total_money(5), 1 + 2 + 3 + 4 + 5);
    }

    #[test]
    fn test6() {
        assert_eq!(Solution::total_money(6), 1 + 2 + 3 + 4 + 5 + 6)
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::total_money(7), 1 + 2 + 3 + 4 + 5 + 6 + 7)
    }

    #[test]
    fn test8() {
        assert_eq!(Solution::total_money(8), 1 + 2 + 3 + 4 + 5 + 6 + 7 + 2)
    }

    #[test]
    fn test9() {
        assert_eq!(Solution::total_money(9), 1 + 2 + 3 + 4 + 5 + 6 + 7 + 2 + 3)
    }

    #[test]
    fn test11() {
        assert_eq!(
            Solution::total_money(11),
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 2 + 3 + 4 + 5
        )
    }

    #[test]
    fn test12() {
        assert_eq!(
            Solution::total_money(12),
            1 + 2 + 3 + 4 + 5 + 6 + 7 + 2 + 3 + 4 + 5 + 6
        )
    }
}
