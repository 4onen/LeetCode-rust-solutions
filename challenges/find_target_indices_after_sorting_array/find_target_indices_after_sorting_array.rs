// https://leetcode.com/problems/find-target-indices-after-sorting-array/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
//         let mut end = nums.len();
//         let mut pp = 0;
//         while pp < end {
//             if nums[pp] >= target {
//                 nums.swap(pp, end - 1);
//                 end -= 1;
//                 continue;
//             }
//             pp += 1;
//         }
//         (pp as i32..pp as i32 + nums[pp..].iter().filter(|&&x| x == target).count() as i32)
//             .into_iter()
//             .collect()
//     }
// }

// Memory reuse sol'n
impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut end = nums.len() as u8;
        let mut pp = 0u8;
        while pp < end {
            if nums[pp as usize] >= target {
                nums.swap(pp as usize, (end - 1) as usize);
                end -= 1;
                continue;
            }
            pp += 1;
        }
        let target_count = nums[pp as usize..].iter().filter(|&&x| x == target).count();
        for i in 0..target_count as u8 {
            nums[i as usize] = (pp + i) as i32;
        }
        unsafe { nums.set_len(target_count as usize) };
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], target: i32, expected: &[i32]) {
        assert_eq!(Solution::target_indices(nums.to_owned(), target), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 5, 2, 3], 2, &[1, 2])
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 5, 2, 3], 3, &[3])
    }

    #[test]
    fn ex3() {
        test(&[1, 2, 5, 2, 3], 5, &[4])
    }
}
