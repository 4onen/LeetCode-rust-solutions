// https://leetcode.com/problems/distribute-repeating-integers/

pub struct Solution;

// O(n!) where n is the number of unique values in nums.
// Obviously too slow, but it's correct to test against...
// ... okay, never mind, just needed to flip the quantity sort.
// impl Solution {
//     pub fn can_distribute(nums: Vec<i32>, mut quantity: Vec<i32>) -> bool {
//         assert!(nums.len() >= 1);
//         assert!(nums.len() <= 100_000);
//         assert!(quantity.len() >= 1);
//         assert!(quantity.len() <= 10);
//         // We know that the maximum value of nums[i] is 1000, so we can use
//         // a u16 as the key. Unfortunately the counts may go up to 100_000,
//         // so we need to use a u32 as the value.
//         let mut counts: std::collections::HashMap<u16, u32> =
//             std::collections::HashMap::with_capacity(50);
//         for n in nums {
//             *counts.entry(n as u16).or_insert(0) += 1;
//         }
//         let mut counts = counts.into_values().collect::<Vec<_>>();
//         counts.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
//         quantity.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
//         // Now let's just use recursive backtracking to try all 10*50
//         // assignments of the quantities to the counts.
//         fn backtracking(counts: &mut [u32], quantities_remaining: &[i32]) -> bool {
//             if quantities_remaining.is_empty() {
//                 return true;
//             }
//             (0..counts.len()).any(|i| {
//                 if counts[i] >= quantities_remaining[0] as u32 {
//                     counts[i] -= quantities_remaining[0] as u32;
//                     let result = backtracking(counts, &quantities_remaining[1..]);
//                     counts[i] += quantities_remaining[0] as u32;
//                     result
//                 } else {
//                     false
//                 }
//             })
//         }
//         backtracking(&mut counts, &quantity)
//     }
// }

// Rewrite backtracking fn to for loop instead of iterators
impl Solution {
    pub fn can_distribute(nums: Vec<i32>, mut quantity: Vec<i32>) -> bool {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        assert!(quantity.len() >= 1);
        assert!(quantity.len() <= 10);
        // We know that the maximum value of nums[i] is 1000, so we can use
        // a u16 as the key. Unfortunately the counts may go up to 100_000,
        // so we need to use a u32 as the value.
        let mut counts: std::collections::HashMap<u16, u32> =
            std::collections::HashMap::with_capacity(50);
        for n in nums {
            *counts.entry(n as u16).or_insert(0) += 1;
        }
        let mut counts = counts.into_values().collect::<Vec<_>>();
        counts.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
        quantity.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
        // Now let's just use recursive backtracking to try all 10*50
        // assignments of the quantities to the counts.
        fn backtracking(counts: &mut [u32], quantities: &[i32], quantity_index: u8) -> bool {
            if quantity_index >= quantities.len() as u8 {
                return true;
            }
            let quantity = quantities[quantity_index as usize] as u32;
            for i in 0..counts.len() as u8 {
                if counts[i as usize] >= quantity {
                    counts[i as usize] -= quantity;
                    if backtracking(counts, quantities, quantity_index + 1) {
                        return true;
                    }
                    counts[i as usize] += quantity;
                }
            }
            false
        }
        backtracking(&mut counts, &quantity, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![1, 2, 3, 4];
        let quantity = vec![2];
        assert_eq!(Solution::can_distribute(nums, quantity), false);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 2, 3, 3];
        let quantity = vec![2];
        assert_eq!(Solution::can_distribute(nums, quantity), true);
    }

    #[test]
    fn ex3() {
        let nums = vec![1, 1, 2, 2];
        let quantity = vec![2, 2];
        assert_eq!(Solution::can_distribute(nums, quantity), true);
    }

    #[test]
    fn discussion_case1() {
        let nums = vec![1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2];
        let quantity = vec![3, 3, 3, 4];
        assert_eq!(Solution::can_distribute(nums, quantity), true);
    }

    #[test]
    fn my_extreme_ex1() {
        let nums = (1..=50).into_iter().cycle().take(100_000).collect();
        let quantity = vec![100_000 / 50; 10];
        assert_eq!(Solution::can_distribute(nums, quantity), true);
    }

    #[test]
    fn my_extreme_ex2() {
        let mut nums: Vec<i32> = (1..=50).into_iter().collect();
        nums.extend((1..=1).into_iter().cycle().take(100_000 - 50));
        let quantity = vec![100_000 / 10; 10];
        assert_eq!(Solution::can_distribute(nums, quantity), false);
    }

    #[test]
    fn my_extreme_ex3() {
        let mut nums: Vec<i32> = (1..=50).into_iter().collect();
        nums.extend((1..=1).into_iter().cycle().take(100_000 - 50));
        let quantity = vec![100_000 / 11; 10];
        assert_eq!(Solution::can_distribute(nums, quantity), true);
    }

    #[test]
    fn failing_case1() {
        let nums = vec![
            1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
            14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24,
            25, 25, 26, 26, 27, 27, 28, 28, 29, 29, 30, 30, 31, 31, 32, 32, 33, 33, 34, 34, 35, 35,
            36, 36, 37, 37, 38, 38, 39, 39, 40, 40, 41, 41, 42, 42, 43, 43, 44, 44, 45, 45, 46, 46,
            47, 47, 48, 48, 49, 49, 50, 50,
        ];
        let quantity = vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 3];
        assert_eq!(Solution::can_distribute(nums, quantity), false);
    }

    #[test]
    fn my_extreme_ex4() {
        let nums = (1..=50).into_iter().cycle().take(100).collect();
        let quantity = vec![2; 10];
        assert_eq!(Solution::can_distribute(nums, quantity), true);
    }

    #[test]
    fn my_extreme_ex5() {
        let nums = (1..=50).into_iter().cycle().take(100).collect();
        let quantity = vec![3; 10];
        assert_eq!(Solution::can_distribute(nums, quantity), false);
    }

    #[test]
    fn my_extreme_ex6() {
        let nums = (1..=50).into_iter().cycle().take(100).collect();
        let mut quantity = vec![2; 10];
        quantity[3] = 3;
        assert_eq!(Solution::can_distribute(nums, quantity), false);
    }

    #[test]
    fn my_greedy_counterexample1() {
        let mut nums = Vec::new();
        nums.extend(Some(1).into_iter().cycle().take(11));
        nums.extend(Some(2).into_iter().cycle().take(12));
        let quantity = vec![10, 5, 6, 2];
        assert_eq!(Solution::can_distribute(nums, quantity), true);
    }

    #[test]
    fn my_greedy_counterexample2() {
        let mut nums = Vec::new();
        nums.extend(Some(1).into_iter().cycle().take(11));
        nums.extend(Some(2).into_iter().cycle().take(12));
        let quantity = vec![10, 5, 6, 3];
        assert_eq!(Solution::can_distribute(nums, quantity), false);
    }
}
