// https://leetcode.com/problems/maximal-score-after-applying-k-operations/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut priority_queue: std::collections::BinaryHeap<i32> = nums.into();
        let mut score = 0;
        for _ in 0..k {
            let mut max = priority_queue.peek_mut().unwrap();
            score += *max as i64;
            *max = (*max + 2) / 3;
        }
        score
    }
}

// Optimized sol'n
// Improves if numbers are the same often
// (Too costly for all other cases)
// impl Solution {
//     pub fn max_kelements(nums: Vec<i32>, mut k: i32) -> i64 {
//         let mut counts = std::collections::HashMap::new();
//         for num in nums {
//             *counts.entry(num).or_insert(0) += 1;
//         }
//         let mut priority_queue: std::collections::BinaryHeap<(i32, i32)> =
//             counts.into_iter().collect();
//         let mut score = 0;
//         while k > 0 {
//             let (num, count) = priority_queue.pop().unwrap();
//             let times = std::cmp::min(k, count);
//             score += num as i64 * times as i64;
//             let new_num = (num + 2) / 3;
//             if new_num > 0 {
//                 priority_queue.push((new_num, count));
//             }
//             k -= times;
//         }
//         score
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: i32, expected: i64) {
        assert_eq!(Solution::max_kelements(nums.to_vec(), k), expected);
    }

    #[test]
    fn ex1() {
        test(&[10, 10, 10, 10, 10], 5, 50)
    }

    #[test]
    fn ex1_1() {
        test(&[10, 10, 10, 10, 10, 10], 5, 50)
    }

    #[test]
    fn ex1_2() {
        test(&[10, 10, 10, 10], 5, 44)
    }

    #[test]
    fn ex2() {
        test(&[1, 10, 3, 3, 3], 3, 17)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[1_000_000_000; 100_000], 100_000, 1_000_000_000 * 100_000)
    }
}
