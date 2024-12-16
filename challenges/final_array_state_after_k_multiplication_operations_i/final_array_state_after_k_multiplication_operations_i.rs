// https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/

pub struct Solution;

// Heap sol'n
// impl Solution {
//     pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
//         if multiplier == 1 {
//             return nums;
//         }
//         // Form heap
//         type Idx = u8;
//         let mut heap: std::vec::Vec<Idx> = (0..nums.len() as Idx).into_iter().collect();
//         let n = heap.len() as u8;
//         for i in (0..=(n - 1) / 2).rev() {
//             let mut j = i;
//             loop {
//                 let mut smallest = j;
//                 let left = 2 * j + 1;
//                 let right = 2 * j + 2;
//                 if left < n
//                     && i32::cmp(
//                         &nums[heap[left as usize] as usize],
//                         &nums[heap[smallest as usize] as usize],
//                     )
//                     .then_with(|| u8::cmp(&heap[left as usize], &heap[smallest as usize]))
//                     .is_lt()
//                 {
//                     smallest = left;
//                 }
//                 if right < n
//                     && i32::cmp(
//                         &nums[heap[right as usize] as usize],
//                         &nums[heap[smallest as usize] as usize],
//                     )
//                     .then_with(|| u8::cmp(&heap[right as usize], &heap[smallest as usize]))
//                     .is_lt()
//                 {
//                     smallest = right;
//                 }
//                 if smallest == j {
//                     break;
//                 }
//                 heap.swap(smallest as usize, j as usize);
//                 j = smallest;
//             }
//         }
//         // Perform operations
//         for _ in 0..k {
//             nums[heap[0] as usize] *= multiplier;
//             let mut j = 0;
//             loop {
//                 let mut smallest = j;
//                 let left = 2 * j + 1;
//                 let right = 2 * j + 2;
//                 if left < n
//                     && i32::cmp(
//                         &nums[heap[left as usize] as usize],
//                         &nums[heap[smallest as usize] as usize],
//                     )
//                     .then_with(|| u8::cmp(&heap[left as usize], &heap[smallest as usize]))
//                     .is_lt()
//                 {
//                     smallest = left;
//                 }
//                 if right < n
//                     && i32::cmp(
//                         &nums[heap[right as usize] as usize],
//                         &nums[heap[smallest as usize] as usize],
//                     )
//                     .then_with(|| u8::cmp(&heap[right as usize], &heap[smallest as usize]))
//                     .is_lt()
//                 {
//                     smallest = right;
//                 }
//                 if smallest == j {
//                     break;
//                 }
//                 heap.swap(smallest as usize, j as usize);
//                 j = smallest;
//             }
//         }
//         nums
//     }
// }

// N^2 sol'n
impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        for _  in 0..k {
            *nums.iter_mut().min().unwrap() *= multiplier;
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: i32, multiplier: i32, expected: &[i32]) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 100);
        }
        assert!(k >= 1);
        assert!(k <= 100);
        assert!(multiplier >= 1);
        assert!(multiplier <= 5);
        assert_eq!(
            Solution::get_final_state(nums.to_vec(), k, multiplier),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[2, 1, 3, 5, 6], 5, 2, &[8, 4, 6, 5, 6])
    }

    #[test]
    fn ex2() {
        test(&[1, 2], 3, 4, &[16, 8])
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[100; 100], 100, 5, &[500; 100])
    }

    #[test]
    fn my_extreme_ex2() {
        let mut result = [400; 100];
        let ops = 50;
        result[ops as usize..].fill(100);
        test(&[100; 100], ops, 4, &result)
    }

    #[test]
    fn failing_case1() {
        // Forgot to handle very small arrays (n<=2)
        test(&[2, 1], 1, 2, &[2, 2])
    }

    #[test]
    fn failing_case1_1() {
        test(&[2], 1, 2, &[4])
    }

    #[test]
    fn failing_case2() {
        // Heapification didn't reach all children because I
        // didn't start from the very extra last non-leaf node
        // (Classic off-by-one.)
        test(&[2, 5, 5, 1, 3, 1], 2, 5, &[2, 5, 5, 5, 3, 5])
    }
}
