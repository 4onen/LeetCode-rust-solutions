// https://leetcode.com/problems/trapping-rain-water/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn trap(height: Vec<i32>) -> i32 {
//         assert!(height.len() > 0);
//         assert!(height.len() <= 20_000);
//         let right_max = {
//             let mut max = 0;
//             let mut right_max = std::vec::Vec::with_capacity(height.len());
//             let right_maxes = right_max.spare_capacity_mut();
//             for i in (0..height.len() as u16).rev() {
//                 max = std::cmp::max(max, height[i as usize]);
//                 right_maxes[i as usize].write(max);
//             }
//             unsafe { right_max.set_len(height.len()) }
//             right_max
//         };
//         let mut trapped = 0;
//         let mut left_max = 0;
//         for i in 0..height.len() {
//             left_max = std::cmp::max(left_max, height[i]);
//             trapped += std::cmp::min(left_max, right_max[i]) - height[i];
//         }
//         trapped
//     }
// }

// Best sol'n optimized
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        assert!(height.len() > 0);
        assert!(height.len() <= 20_000);
        let mut peak_i = 0u16;
        for i in 1..height.len() as u16 {
            if height[i as usize] >= height[peak_i as usize] {
                peak_i = i;
            }
        }
        let mut water = 0;
        let mut left_max = height[0];
        for i in 0..peak_i {
            left_max = std::cmp::max(left_max, height[i as usize]);
            water += left_max - height[i as usize];
        }
        let mut right_max = height[height.len() - 1];
        for i in (peak_i..height.len() as u16).rev() {
            right_max = std::cmp::max(right_max, height[i as usize]);
            water += right_max - height[i as usize];
        }
        water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(heights: &[i32], expected: i32) {
        assert_eq!(Solution::trap(heights.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6);
    }

    #[test]
    fn ex2() {
        test(&[4, 2, 0, 3, 2, 5], 9);
    }

    #[test]
    fn myex3() {
        test(&[1, 2, 3, 4, 5], 0);
    }

    #[test]
    fn discussion_case1() {
        test(&[4, 2, 3], 1);
    }

    #[test]
    fn discussion_case2() {
        test(&[5, 4, 1, 2], 1);
    }

    #[test]
    fn discussion_case3() {
        test(
            &[0, 1, 2, 0, 3, 0, 1, 2, 0, 0, 4, 2, 1, 2, 5, 0, 1, 2, 0, 2],
            26,
        );
    }

    #[test]
    fn discussion_case4() {
        test(&[9, 6, 8, 8, 5, 6, 3], 3);
    }

    #[test]
    fn my_extreme_ex0() {
        test(&[0], 0);
    }

    #[test]
    fn my_extreme_ex1() {
        let heights = [1, 2].repeat(5000);
        test(&heights, 4999);
    }
}
