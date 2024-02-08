// https://leetcode.com/problems/perfect-squares/

pub struct Solution;

const CAP: u16 = 10000;
const fn count_perfect_squares_to_cap() -> u16 {
    let mut count = 0;
    let mut i = 1;
    loop {
        if i * i <= CAP {
            count += 1;
        } else {
            break;
        }
        i += 1;
    }
    count
}
const PERFECT_SQUARES_UNDER_CAP_COUNT: u16 = count_perfect_squares_to_cap();
const fn make_perfect_squares_to_cap() -> [u16; PERFECT_SQUARES_UNDER_CAP_COUNT as usize] {
    let mut squares = [0; PERFECT_SQUARES_UNDER_CAP_COUNT as usize];
    let mut square_idx = 0;
    let mut i = 1;
    loop {
        if i * i <= CAP {
            squares[square_idx] = i * i;
            square_idx += 1;
        } else {
            break;
        }
        i += 1;
    }
    squares
}
const PERFECT_SQUARES_UNDER_CAP: [u16; PERFECT_SQUARES_UNDER_CAP_COUNT as usize] =
    make_perfect_squares_to_cap();
fn two_sum(nums: &[u16], target: u16) -> bool {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let sum = nums[left] + nums[right];
        match sum.cmp(&target) {
            std::cmp::Ordering::Equal => {
                return true;
            }
            std::cmp::Ordering::Less => {
                left += 1;
            }
            std::cmp::Ordering::Greater => match right.checked_sub(1) {
                Some(new_right) => right = new_right,
                None => break,
            },
        }
    }
    false
}
fn three_sum(nums: &[u16], target: u16) -> bool {
    // (0..nums.len() - 2).any(|i| two_sum(&nums[i..], target - nums[i]))
    let mut i = 0;
    while i < nums.len() - 2 {
        if two_sum(&nums[i..], target - nums[i]) {
            return true;
        }
        i += 1;
    }
    false
}
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as u16;
        // By Lagrange's four-square theorem:
        //  any natural number can be represented
        //  as the sum of four integer squares.
        // So, the answer is either 1, 2, 3, or 4.
        // If n is a perfect square, then the answer is 1.
        // Check if n is in our list of perfect squares.
        match PERFECT_SQUARES_UNDER_CAP.binary_search(&n) {
            Ok(_) => 1, // n is a perfect square
            Err(i) => {
                // n is the sum of 2, 3, or 4 perfect squares.
                // Apply the k_sum algorithm from src/four_sum.rs
                // to find the answer.
                if two_sum(&PERFECT_SQUARES_UNDER_CAP[..=i + 1], n) {
                    2
                } else if three_sum(&PERFECT_SQUARES_UNDER_CAP[..=i + 1], n) {
                    3
                } else {
                    4
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::num_squares(13), 2);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::num_squares(6024), 3);
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(Solution::num_squares(43), 3);
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(Solution::num_squares(22), 3);
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(Solution::num_squares(55), 4);
    }

    #[test]
    fn discussion_case5() {
        assert_eq!(Solution::num_squares(48), 3);
    }

    #[test]
    fn discussion_case6() {
        assert_eq!(Solution::num_squares(10000), 1);
    }

    #[test]
    fn failing_case1() {
        assert_eq!(Solution::num_squares(3), 3);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::num_squares(1), 1);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::num_squares(2), 2);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::num_squares(4), 1);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::num_squares(5), 2);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::num_squares(6), 3);
    }

    #[test]
    fn myex6() {
        assert_eq!(Solution::num_squares(7), 4);
    }

    #[test]
    fn myex7() {
        assert_eq!(Solution::num_squares(8), 2);
    }

    #[test]
    fn myex8() {
        assert_eq!(Solution::num_squares(9), 1);
    }

    #[test]
    fn myex9() {
        assert_eq!(Solution::num_squares(10), 2);
    }

    #[test]
    fn myex10() {
        assert_eq!(Solution::num_squares(11), 3);
    }

    #[test]
    fn myex11() {
        assert_eq!(Solution::num_squares(14), 3);
    }

    #[test]
    fn myex12() {
        assert_eq!(Solution::num_squares(15), 4);
    }

    #[test]
    fn myex13() {
        assert_eq!(Solution::num_squares(16), 1);
    }

    #[test]
    fn myex14() {
        assert_eq!(Solution::num_squares(17), 2);
    }
}
