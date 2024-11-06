// https://leetcode.com/problems/find-if-array-can-be-sorted/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn can_sort_array(nums: Vec<i32>) -> bool {
//         let mut range_min_maxs = vec![];
//         for i in 0..nums.len() {
//             if range_min_maxs.is_empty() {
//                 range_min_maxs.push((nums[i], nums[i]));
//             } else {
//                 let (min, max) = range_min_maxs.last_mut().unwrap();
//                 let new_val = nums[i];
//                 if min.count_ones() == new_val.count_ones() {
//                     if new_val < *min {
//                         *min = new_val;
//                     } else if new_val > *max {
//                         *max = new_val;
//                     }
//                 } else {
//                     range_min_maxs.push((new_val, new_val));
//                 }
//             }
//         }
//         // Now, we have ranges of each popcnt in the array. If those ranges
//         // aren't already sorted, we can't sort the array.
//         dbg!(&range_min_maxs);
//         for i in 0..range_min_maxs.len() - 1 {
//             let (_, max) = range_min_maxs[i];
//             let (next_min, _) = range_min_maxs[i + 1];
//             if max > next_min {
//                 return false;
//             }
//         }
//         true
//     }
// }

// One-pass sol'n
impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut last_max = 0;
        let mut cur_min = nums[0];
        let mut cur_max = nums[0];
        for i in 1..nums.len() {
            let new_val = nums[i];
            if new_val.count_ones() == cur_min.count_ones() {
                if new_val < cur_min {
                    cur_min = new_val;
                } else if new_val > cur_max {
                    cur_max = new_val;
                }
            } else {
                if cur_min < last_max {
                    return false;
                }
                last_max = cur_max;
                cur_min = new_val;
                cur_max = new_val;
            }
        }
        cur_min >= last_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: bool) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 256);
        }
        assert_eq!(Solution::can_sort_array(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[8, 4, 2, 30, 15], true)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4, 5], true)
    }

    #[test]
    fn ex3() {
        test(&[3, 16, 8, 4, 2], false)
    }

    #[test]
    fn ex3_1() {
        test(&[2, 3, 16, 8, 4], true)
    }

    #[test]
    fn ex3_2() {
        test(&[2, 3, 16, 8, 4, 1], false)
    }

    #[test]
    fn ex3_2_1() {
        test(&[1, 2, 3, 16, 8, 4], true)
    }

    #[test]
    fn ex3_3() {
        test(&[16, 8, 4, 2], true)
    }

    #[test]
    fn discussion_case() {
        test(&[75, 34, 30], false)
    }
}
