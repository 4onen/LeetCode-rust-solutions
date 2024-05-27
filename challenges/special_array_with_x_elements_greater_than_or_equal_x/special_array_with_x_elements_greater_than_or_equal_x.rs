// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn special_array(mut nums: Vec<i32>) -> i32 {
//         assert!(nums.len() >= 1);
//         assert!(nums.len() <= 100);
//         nums.sort_unstable();
//         let n = nums.len() as i32;
//         let mut partition_point = 0;
//         for x in 0..=n {
//             while partition_point < n && nums[partition_point as usize] < x {
//                 partition_point += 1;
//             }
//             if n - partition_point == x {
//                 return x;
//             }
//         }
//         -1
//     }
// }

// Braindead sol'n
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100);
        let n = nums.len() as i32;
        for x in 0..=n {
            let mut count = 0;
            for &num in &nums {
                if num >= x {
                    count += 1;
                }
            }
            if count == x {
                return x;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100);
        for &num in nums {
            assert!(num >= 0);
            assert!(num <= 1000);
        }
        assert_eq!(Solution::special_array(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[3, 5], 2)
    }

    #[test]
    fn ex2() {
        test(&[0, 0], -1)
    }

    #[test]
    fn ex3() {
        test(&[0, 4, 3, 0, 4], 3)
    }

    #[test]
    fn myex1() {
        test(&[0], -1)
    }

    #[test]
    fn myex2() {
        test(&[1], 1)
    }

    #[test]
    fn myex3() {
        test(&[2], 1)
    }

    #[test]
    fn myex4() {
        test(&[1, 2], -1)
    }
}
