// https://leetcode.com/problems/number-of-arithmetic-triplets/

pub struct Solution;

// Naive brute force
// impl Solution {
//     pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
//         let mut count = 0;
//         for i in 0..nums.len() {
//             for j in i + 1..nums.len() {
//                 for k in j + 1..nums.len() {
//                     if nums[j] - nums[i] == diff && nums[k] - nums[j] == diff {
//                         count += 1;
//                     }
//                 }
//             }
//         }
//         count
//     }
// }

// Hash set sol'n
// impl Solution {
//     pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
//         let diff = diff as u8;
//         let mut set = std::collections::HashSet::new();
//         for &num in &nums {
//             set.insert(num as u8);
//         }
//         let mut count = 0;
//         for num in nums {
//             if set.contains(&(num as u8 + diff)) && set.contains(&(num as u8 + 2 * diff)) {
//                 count += 1;
//             }
//         }
//         count
//     }
// }

// Binary array hash set sol'n
impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let diff = diff as u8;
        let mut set = [false; 201];
        for &num in &nums {
            set[num as usize] = true;
        }
        let mut count = 0;
        for num in nums {
            if num as u8 + 2 * diff > 200 {
                break;
            }
            if set[(num as u8 + diff) as usize] && set[(num as u8 + 2 * diff) as usize] {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], diff: i32, expected: i32) {
        assert!(nums.len() >= 3);
        assert!(nums.len() <= 200);
        assert!(diff >= 1);
        assert!(diff <= 50);
        for i in 0..nums.len() {
            assert!(nums[i] >= 0);
            assert!(nums[i] <= 200);
            if i > 1 {
                assert!(nums[i] > nums[i - 1]);
            }
        }
        assert_eq!(Solution::arithmetic_triplets(nums.to_vec(), diff), expected);
    }

    #[test]
    fn ex1() {
        test(&[0, 1, 4, 6, 7, 10], 3, 2)
    }

    #[test]
    fn ex2() {
        test(&[4, 5, 6, 7, 8, 9], 2, 2)
    }

    #[test]
    fn my_extreme_ex1() {
        // All ascending
        let mut input = vec![0; 200];
        for i in 0..200 {
            input[i] = i as i32;
        }
        test(&input, 1, 198);
        test(&input, 2, 196);
        test(&input, 3, 194);
        test(&input, 4, 192);
    }
}
