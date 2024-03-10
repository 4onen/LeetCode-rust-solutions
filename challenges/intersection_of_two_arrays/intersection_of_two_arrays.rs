// https://leetcode.com/problems/intersection-of-two-arrays/

pub struct Solution;

// Correct (but slow?) sol'n
// impl Solution {
//     pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
//         nums1.sort_unstable();
//         nums2.sort_unstable();
//         nums1.dedup();
//         let mut write_ptr = 0u16;
//         let mut read_ptr = 0u16;
//         let mut nums2_ptr = 0u16;
//         let mut result = nums1;
//         while read_ptr < result.len() as u16 && nums2_ptr < nums2.len() as u16 {
//             if result[read_ptr as usize] == nums2[nums2_ptr as usize] {
//                 result[write_ptr as usize] = result[read_ptr as usize];
//                 write_ptr += 1;
//                 read_ptr += 1;
//                 nums2_ptr += 1;
//             } else if result[read_ptr as usize] < nums2[nums2_ptr as usize] {
//                 read_ptr += 1;
//             } else {
//                 nums2_ptr += 1;
//             }
//         }
//         if read_ptr < result.len() as u16 && write_ptr < read_ptr {
//             // No need to copy if we're already done or if we're copying to the same place
//             result.copy_within(read_ptr as usize.., write_ptr as usize);
//         }
//         result.truncate(write_ptr as usize);
//         result
//     }
// }

// Braindead (but also slow?) sol'n
// impl Solution {
//     pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
//         // type T = std::collections::HashSet<i32>;
//         type T = std::collections::BTreeSet<i32>;
//         let set1: T = nums1.into_iter().collect();
//         let set2: T = nums2.into_iter().collect();
//         T::intersection(&set1, &set2).cloned().collect()
//     }
// }

// I'm an idiot. Only one needs to be a set.

// Another attempt
// impl Solution {
//     pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
//         type T = std::collections::BTreeSet<i32>;
//         let (smaller, larger) = if nums1.len() < nums2.len() {
//             (nums1, nums2)
//         } else {
//             (nums2, nums1)
//         };
//         let mut set = T::new();
//         for n in smaller {
//             set.insert(n);
//         }
//         larger.into_iter().filter(|n| set.remove(n)).collect()
//     }
// }

// Finally, fastest solution
impl Solution {
    pub fn intersection(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut set = std::collections::BTreeSet::new();
        for n in nums1 {
            set.insert(n);
        }
        let mut i = 0u16;
        while i < nums2.len() as u16 {
            if !set.remove(&nums2[i as usize]) {
                nums2.swap_remove(i as usize);
            } else {
                i += 1;
            }
        }
        nums2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut result = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
        result.sort_unstable();
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn ex2() {
        let mut result = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort_unstable();
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn myex1() {
        let mut result = Solution::intersection(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]);
        result.sort_unstable();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn myex2() {
        let mut result = Solution::intersection(vec![1, 2, 3, 4, 5], vec![5, 6, 7, 8, 9, 10]);
        result.sort_unstable();
        assert_eq!(result, vec![5]);
    }
}
