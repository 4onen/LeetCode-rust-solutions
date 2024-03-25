// https://leetcode.com/problems/find-all-duplicates-in-an-array/

pub struct Solution;

// GitHub Copilot Sol'n
// impl Solution {
//     pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
//         let mut nums = nums;
//         let mut res = Vec::new();
//         for i in 0..nums.len() {
//             let index = (nums[i].abs() - 1) as usize;
//             if nums[index] < 0 {
//                 res.push(nums[i].abs());
//             } else {
//                 nums[index] = -nums[index];
//             }
//         }
//         res
//     }
// }

// My Sol'n (faster)
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut seen = vec![false; nums.len() + 1];
        for num in nums {
            if seen[num as usize] {
                res.push(num);
            } else {
                seen[num as usize] = true;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let res = vec![2, 3];
        assert_eq!(Solution::find_duplicates(nums), res);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 1, 2];
        let res = vec![1];
        assert_eq!(Solution::find_duplicates(nums), res);
    }

    #[test]
    fn ex3() {
        let nums = vec![1];
        let res = vec![];
        assert_eq!(Solution::find_duplicates(nums), res);
    }
}
