// https://leetcode.com/problems/distribute-money-to-maximum-children/

pub struct Solution;

impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money < children {
            return -1;
        }
        let money = money - children;
        // Now everyone has 1 dollar. Remaining rules are -1'd.
        let satisfied = money / 7;
        let rem = money % 7;
        if satisfied > children {
            children - 1
        } else if satisfied == children && rem > 0 {
            children - 1
        } else if satisfied == children {
            children
        } else if satisfied == children - 1 && rem != 3 {
            children - 1
        } else if satisfied == children - 1 {
            // && rem == 3
            children - 2
        } else {
            // satisfied < children - 1
            satisfied
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(money: u8, children: i8, expected: i8) {
        assert_eq!(
            Solution::dist_money(money as i32, children as i32),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test(20, 3, 1)
    }

    #[test]
    fn ex2() {
        test(16, 2, 2)
    }

    #[test]
    fn discussion_case1() {
        test(1, 2, -1)
    }

    #[test]
    fn discussion_case2() {
        test(17, 2, 1)
    }

    #[test]
    fn myex1() {
        test(12, 2, 0)
    }

    #[test]
    fn myex1_1() {
        test(12, 3, 1)
    }
}
