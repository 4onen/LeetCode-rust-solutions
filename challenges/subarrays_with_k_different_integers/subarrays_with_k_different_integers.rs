// https://leetcode.com/problems/subarrays-with-k-different-integers/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn subarrays_with_k_or_fewer_distinct(nums: &[i32], k: i32) -> i32 {
//         if k < 1 {
//             0
//         } else {
//             let mut map = std::collections::HashMap::<i32, u16>::new();
//             let mut left = 0u16;
//             let mut count = 0 as i32;
//             for right in 0..nums.len() as u16 {
//                 map.entry(nums[right as usize])
//                     .and_modify(|x| *x += 1)
//                     .or_insert(0);
//                 while map.len() > k as usize {
//                     let num_slot = map.get_mut(&nums[left as usize]).unwrap();
//                     if let Some(n) = num_slot.checked_sub(1) {
//                         *num_slot = n;
//                     } else {
//                         map.remove(&nums[left as usize]);
//                     }
//                     left += 1;
//                 }
//                 count += (right - left + 1) as i32;
//             }
//             count
//         }
//     }
//     pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
//         Self::subarrays_with_k_or_fewer_distinct(&nums, k)
//             - Self::subarrays_with_k_or_fewer_distinct(&nums, k - 1)
//     }
// }

// Vec instead of hashmap
impl Solution {
    pub fn subarrays_with_k_or_fewer_distinct(nums: &[i32], k: i32) -> i32 {
        if k < 1 {
            0
        } else {
            let k = k as u16;
            let mut map = vec![0; nums.len() + 1];
            let mut left = 0u16;
            let mut count = 0 as i32;
            let mut distinct = 0u16;
            for right in 0..nums.len() as u16 {
                if map[nums[right as usize] as usize] == 0 {
                    distinct += 1;
                }
                map[nums[right as usize] as usize] += 1;
                while distinct > k {
                    map[nums[left as usize] as usize] -= 1;
                    if map[nums[left as usize] as usize] == 0 {
                        distinct -= 1;
                    }
                    left += 1;
                }
                count += (right - left + 1) as i32;
            }
            count
        }
    }
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarrays_with_k_or_fewer_distinct(&nums, k)
            - Self::subarrays_with_k_or_fewer_distinct(&nums, k - 1)
    }
}

// Vec instead of hashmap, and merge passes (Slower than just 2 passes)
// impl Solution {
//     pub fn subarrays_with_1_distinct(nums: &[i32]) -> i32 {
//         let mut last = -1;
//         let mut left = 0u16;
//         let mut count = 0 as i32;
//         for right in 0..nums.len() as u16 {
//             if last != nums[right as usize] {
//                 last = nums[right as usize];
//                 left = right;
//             }
//             count += (right - left + 1) as i32;
//         }
//         count
//     }
//     pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
//         if k < 1 {
//             0
//         } else if k == 1 {
//             Self::subarrays_with_1_distinct(&nums)
//         } else {
//             let k = k as u16;
//             let mut map = vec![0; nums.len() + 1];
//             let mut map_km1 = vec![0; nums.len() + 1];
//             let mut left = 0u16;
//             let mut left_km1 = 0u16;
//             let mut distinct = 0u16;
//             let mut distinct_km1 = 0u16;
//             let mut count = 0i32;
//             for right in 0..nums.len() as u16 {
//                 let right_num = nums[right as usize] as usize;
//                 if map[right_num] == 0 {
//                     distinct += 1;
//                 }
//                 map[right_num] += 1;
//                 if map_km1[right_num] == 0 {
//                     distinct_km1 += 1;
//                 }
//                 map_km1[right_num] += 1;
//                 while distinct > k {
//                     map[nums[left as usize] as usize] -= 1;
//                     if map[nums[left as usize] as usize] == 0 {
//                         distinct -= 1;
//                     }
//                     left += 1;
//                 }
//                 while distinct_km1 > k - 1 {
//                     map_km1[nums[left_km1 as usize] as usize] -= 1;
//                     if map_km1[nums[left_km1 as usize] as usize] == 0 {
//                         distinct_km1 -= 1;
//                     }
//                     left_km1 += 1;
//                 }
//                 count += (left_km1 - left) as i32;
//             }
//             count
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2),
            7
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3),
            3
        );
    }

    #[test]
    fn my_extreme_ex1() {
        let nums = vec![1; 2 * 10000];
        assert_eq!(
            Solution::subarrays_with_k_distinct(nums, 1),
            (20000 * 20001) / 2
        );
    }

    #[test]
    fn my_extreme_ex2() {
        let nums = vec![1; 2 * 10000];
        assert_eq!(Solution::subarrays_with_k_distinct(nums, 2), 0);
    }

    #[test]
    fn my_extreme_ex3() {
        let nums = vec![1; 2 * 10000];
        assert_eq!(Solution::subarrays_with_k_distinct(nums, 3), 0);
    }

    #[test]
    fn my_extreme_ex4() {
        let nums = (1..=20000).collect();
        assert_eq!(Solution::subarrays_with_k_distinct(nums, 1), 20000);
    }

    #[test]
    fn my_extreme_ex5() {
        let nums = (1..=20000).collect();
        assert_eq!(Solution::subarrays_with_k_distinct(nums, 2), 19999);
    }

    #[test]
    fn my_extreme_ex6() {
        let nums = (1..=20000).collect();
        // 1, 2, 3, 4, 5, 6, 7, 8, ...
        // _  _  _
        //    _  _  _
        //       _  _  _
        //          _  _  _
        //             _  _  _
        //                _  _  _
        assert_eq!(Solution::subarrays_with_k_distinct(nums, 3), 19998);
    }

    #[test]
    fn my_extreme_ex7() {
        let mut nums: Vec<i32> = (1..=20000).collect();
        nums[2] = 1;
        nums[4] = 1;
        // 1, 2, 1, 4, 1, 6, 7, 8, ...
        // _  _  _  _  _
        // _  _  _  _
        //    _  _  _
        //    _  _  _  _
        //       _  _  _  _
        //          _  _  _
        //             _  _  _
        //                _  _  _
        assert_eq!(Solution::subarrays_with_k_distinct(nums, 3), 20000);
    }

    #[test]
    fn myex1() {
        let nums = vec![1, 2, 1, 4, 1, 6, 7, 8];
        assert_eq!(Solution::subarrays_with_k_distinct(nums, 3), 8);
    }
}
