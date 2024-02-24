// https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/

pub struct Solution;

//// Failed scan state machine sol'n
// Reframe the problem:
//  This problem wants us to break the array into chunks of
//  size 2 arrays of equal elements
//  size 3 arrays of equal elements
//  size 3 arrays of contiguously increasing elements
// Instead of thinking about the problem on an array of numbers,
// frame it as on the array of relationships between adjacent numbers.
// Consider an array of three kinds of element: E, I, and N.
// E is for equal, I is for increasing, and N is for none.
// Then the problem wants us to break the array into chunks of
//  size 1 arrays of E
//  size 2 arrays of I
// Separated by at most one N or one I.
// enum Relationship {
//     Equal,
//     Increasing,
//     None,
// }
// enum State {
//     Reset,
//     Equal,
//     I1,
//     I2,
// }
// impl Solution {
//     pub fn valid_partition(nums: Vec<i32>) -> bool {
//         if nums.len() < 2 {
//             return false;
//         }
//         let mut state = State::Reset;
//         for window in nums.windows(2) {
//             let relationship = match *window {
//                 [a, b] if a == b => Relationship::Equal,
//                 [a, b] if a + 1 == b => Relationship::Increasing,
//                 _ => Relationship::None,
//             };
//             state = match (state, relationship) {
//                 (State::Reset, Relationship::Equal) => State::Equal,
//                 (State::Reset, Relationship::Increasing) => State::I1,
//                 (State::Reset, Relationship::None) => return false,
//                 (State::Equal, Relationship::Equal) => State::Equal,
//                 (State::Equal, Relationship::Increasing) => State::Reset,
//                 (State::Equal, Relationship::None) => State::Reset,
//                 (State::I1, Relationship::Increasing) => State::I2,
//                 (State::I1, _) => return false,
//                 (State::I2, Relationship::Equal) => State::Reset,
//                 (State::I2, Relationship::Increasing) => State::Reset,
//                 (State::I2, Relationship::None) => State::Reset,
//             };
//         }
//         match state {
//             State::Reset => false,
//             State::Equal => true,
//             State::I1 => false,
//             State::I2 => true,
//         }
//     }
// }

//// DP sol'n
// impl Solution {
//     pub fn valid_partition(nums: Vec<i32>) -> bool {
//         if nums.len() < 2 {
//             return false;
//         }
//         let mut dp = vec![false; nums.len()];
//         for i in 1..nums.len() {
//             if nums[i] == nums[i - 1] {
//                 dp[i] = (i < 2) || dp[i - 2];
//                 if i > 1 && (nums[i - 1] == nums[i - 2]) {
//                     dp[i] |= (i < 3) || dp[i - 3];
//                 }
//             }
//             if i > 1 && (nums[i] == nums[i - 1] + 1) && (nums[i - 1] == nums[i - 2] + 1) {
//                 dp[i] |= (i < 3) || dp[i - 3];
//             }
//         }
//         dp[nums.len() - 1]
//     }
// }

//// Constant space DP sol'n
// impl Solution {
//     pub fn valid_partition(nums: Vec<i32>) -> bool {
//         if nums.len() < 2 {
//             return false;
//         }
//         let mut history = [false, true, true];
//         for i in 1..nums.len() {
//             let mut is_partitionable_here = false;
//             if nums[i] == nums[i - 1] {
//                 is_partitionable_here |= history[1];
//                 if i > 1 && (nums[i - 1] == nums[i - 2]) {
//                     is_partitionable_here |= history[2];
//                 }
//             }
//             if i > 1 && (nums[i] == nums[i - 1] + 1) && (nums[i - 1] == nums[i - 2] + 1) {
//                 is_partitionable_here |= history[2];
//             }
//             history = [is_partitionable_here, history[0], history[1]];
//         }
//         history[0]
//     }
// }

//// Bitfield DP sol'n
// impl Solution {
//     pub fn valid_partition(nums: Vec<i32>) -> bool {
//         if nums.len() < 2 {
//             return false;
//         }
//         let mut history: u8 = 0b1100;
//         if nums[1] == nums[0] {
//             history |= 0b0001;
//         }
//         for i in 2..nums.len() {
//             history <<= 1;
//             if nums[i] == nums[i - 1] {
//                 history |= (history & 0b0100) >> 2;
//                 if nums[i - 1] == nums[i - 2] {
//                     history |= (history & 0b1000) >> 3;
//                 }
//             }
//             if (nums[i] == nums[i - 1] + 1) && (nums[i - 1] == nums[i - 2] + 1) {
//                 history |= (history & 0b1000) >> 3;
//             }
//         }
//         history & 0b0001 > 0
//     }
// }

//// Bitfield DP sol'n with alternate matching
impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let mut history: u8 = 0b1100;
        if nums[1] == nums[0] {
            history |= 0b0001;
        }
        for i in 2..nums.len() {
            history <<= 1;
            match (nums[i] - nums[i - 1], nums[i - 1] - nums[i - 2]) {
                (0, 0) => {
                    history |= (history & 0b0100) >> 2;
                    history |= (history & 0b1000) >> 3;
                }
                (0, _) => history |= (history & 0b0100) >> 2,
                (1, 1) => {
                    history |= (history & 0b1000) >> 3;
                }
                _ => (),
            }
        }
        history & 0b0001 > 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn short_tests() {
        assert!(!Solution::valid_partition(vec![1]));
        assert!(Solution::valid_partition(vec![1, 1]));
        assert!(!Solution::valid_partition(vec![1, 2]));
        assert!(!Solution::valid_partition(vec![1, 3]));
        assert!(Solution::valid_partition(vec![1, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 1, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 2]));
        assert!(Solution::valid_partition(vec![1, 2, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 4]));
        assert!(!Solution::valid_partition(vec![1, 3, 1]));
        assert!(!Solution::valid_partition(vec![1, 3, 3]));
        assert!(!Solution::valid_partition(vec![1, 3, 4]));
        assert!(Solution::valid_partition(vec![1, 1, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 3]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 1]));
        assert!(Solution::valid_partition(vec![1, 1, 2, 2]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 3]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 1]));
        assert!(Solution::valid_partition(vec![1, 1, 3, 3]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 4]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 2, 2, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 2, 2]));
        assert!(!Solution::valid_partition(vec![1, 2, 2, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 3, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 3, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 3, 4]));
        assert!(!Solution::valid_partition(vec![1, 2, 3, 5]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 4]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 5]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 6]));
        assert!(!Solution::valid_partition(vec![1, 3, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 3, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 3, 1, 3]));
        assert!(!Solution::valid_partition(vec![1, 3, 2, 1]));
        assert!(!Solution::valid_partition(vec![1, 3, 2, 2]));
        assert!(!Solution::valid_partition(vec![1, 3, 2, 3]));
        assert!(!Solution::valid_partition(vec![1, 3, 2, 4]));
        assert!(!Solution::valid_partition(vec![1, 3, 3, 1]));
        assert!(!Solution::valid_partition(vec![1, 3, 3, 3]));
        assert!(!Solution::valid_partition(vec![1, 3, 3, 4]));
        assert!(!Solution::valid_partition(vec![1, 3, 4, 1]));
        assert!(!Solution::valid_partition(vec![1, 3, 4, 4]));
        assert!(!Solution::valid_partition(vec![1, 3, 4, 5]));
        assert!(!Solution::valid_partition(vec![1, 3, 4, 6]));
        assert!(!Solution::valid_partition(vec![1, 3, 5, 1]));
        assert!(!Solution::valid_partition(vec![1, 3, 5, 5]));
        assert!(!Solution::valid_partition(vec![1, 3, 5, 6]));
        assert!(!Solution::valid_partition(vec![1, 3, 5, 7]));
        assert!(Solution::valid_partition(vec![1, 1, 1, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 1, 3]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 2, 1]));
        assert!(Solution::valid_partition(vec![1, 1, 1, 2, 2]));
        assert!(Solution::valid_partition(vec![1, 1, 1, 2, 3]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 2, 4]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 3, 1]));
        assert!(Solution::valid_partition(vec![1, 1, 1, 3, 3]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 3, 4]));
        assert!(!Solution::valid_partition(vec![1, 1, 1, 3, 5]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 2, 1]));
        assert!(Solution::valid_partition(vec![1, 1, 2, 2, 2]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 2, 3]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 3, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 3, 3]));
        assert!(Solution::valid_partition(vec![1, 1, 2, 3, 4]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 3, 5]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 4, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 4, 4]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 4, 5]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 4, 6]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 1, 3]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 3, 1]));
        assert!(Solution::valid_partition(vec![1, 1, 3, 3, 3]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 3, 4]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 3, 5]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 4, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 4, 4]));
        assert!(Solution::valid_partition(vec![1, 1, 3, 4, 5]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 4, 6]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 5, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 5, 5]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 5, 6]));
        assert!(!Solution::valid_partition(vec![1, 1, 3, 5, 7]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 1, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 2, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 2, 2]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 2, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 2, 4]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 3, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 3, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 3, 4]));
        assert!(!Solution::valid_partition(vec![1, 2, 1, 3, 5]));
        assert!(Solution::valid_partition(vec![1, 2, 3, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 3, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 2, 3, 1, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 3, 3, 1]));
        assert!(Solution::valid_partition(vec![1, 2, 3, 3, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 3, 3, 4]));
        assert!(!Solution::valid_partition(vec![1, 2, 3, 3, 5]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 1, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 4, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 4, 4]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 4, 5]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 4, 6]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 5, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 5, 5]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 5, 6]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 5, 7]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 6, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 6, 6]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 6, 7]));
        assert!(!Solution::valid_partition(vec![1, 2, 4, 6, 8]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 1, 2]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 1, 3]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 5, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 5, 5]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 5, 6]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 5, 7]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 6, 1]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 6, 6]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 6, 7]));
        assert!(!Solution::valid_partition(vec![1, 2, 5, 6, 8]));
    }

    #[test]
    fn test_inc_partition() {
        assert!(Solution::valid_partition(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));
        assert!(!Solution::valid_partition(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10
        ]));
        assert!(!Solution::valid_partition(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9
        ]));
        assert!(!Solution::valid_partition(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 8
        ]));
        assert!(!Solution::valid_partition(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 8, 9
        ]));
        assert!(!Solution::valid_partition(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 8, 9, 10
        ]));
    }

    #[test]
    fn test_eq_partition() {
        assert!(Solution::valid_partition(vec![
            1, 1, 2, 2, 1, 1, 2, 2, 1, 1
        ]));
        assert!(Solution::valid_partition(vec![1, 1, 1, 1, 1, 1, 1, 1, 1]));
        assert!(Solution::valid_partition(vec![1, 1, 1, 1, 1, 1, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 2, 1, 2, 2, 1, 1]));
        assert!(!Solution::valid_partition(vec![1, 1, 2, 2, 1, 1, 2, 2, 1,]));
    }

    #[test]
    fn test_random_shtuff() {
        assert!(!Solution::valid_partition(vec![
            1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 3, 4, 5, 6, 7, 8, 9
        ]));
        assert!(!Solution::valid_partition(vec![
            1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 3, 4, 5, 6, 7, 8, 9, 10
        ]));
        assert!(Solution::valid_partition(vec![
            1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11
        ]));
        assert!(Solution::valid_partition(vec![1, 2, 3, 3, 3, 4, 5, 6]));
        assert!(!Solution::valid_partition(vec![1, 2, 3, 3, 3, 4, 5, 6, 7]));
    }

    #[test]
    fn discussion_case1() {
        assert!(!Solution::valid_partition(vec![5, 6, 7, 8, 9, 10, 11]));
        assert!(!Solution::valid_partition(vec![
            993335, 993336, 993337, 993338, 993339, 993340, 993341
        ]))
    }

    #[test]
    fn discussion_case2() {
        assert!(Solution::valid_partition(vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 1, 1, 1, 5, 5, 5, 8, 8, 8, 8, 8, 8, 8,
            8, 8, 8
        ]))
    }

    #[test]
    fn discussion_case3() {
        assert!(!Solution::valid_partition(vec![10, 20, 30]))
    }

    #[test]
    fn discussion_case4() {
        assert!(Solution::valid_partition(vec![1, 2, 3, 3, 4, 5, 7, 7, 7]))
    }

    #[test]
    fn discussion_case5() {
        assert!(!Solution::valid_partition(vec![1, 2, 3, 3, 4, 5, 7, 7, 8]))
    }
}
