// https://leetcode.com/problems/special-array-ii/

pub struct Solution;

// Naive sol'n
// impl Solution {
//     pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
//         queries
//             .into_iter()
//             .map(|q| {
//                 let (l, r) = (q[0] as usize, q[1] as usize);
//                 nums[l..=r].windows(2).all(|w| w[0] % 2 != w[1] % 2)
//             })
//             .collect()
//     }
// }

// Precompute parity-change points sol'n
// impl Solution {
//     pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
//         let parity_swap_idxs = nums
//             .windows(2)
//             .enumerate()
//             .filter(|&(_, w)| w[0] % 2 == w[1] % 2)
//             .map(|(i, _)| i)
//             .collect::<Vec<_>>();
//         queries
//             .into_iter()
//             .map(|q| {
//                 let (l, r) = (q[0] as usize, q[1] as usize);
//                 if l == r {
//                     return true;
//                 }
//                 let start_idx = parity_swap_idxs.binary_search(&l);
//                 let end_idx = parity_swap_idxs.binary_search(&r);
//                 let res = match (start_idx, end_idx) {
//                     (Ok(_), _) => false,
//                     (Err(a), Ok(b)) => a==b,
//                     (Err(a), Err(b)) => a==b,
//                 };
//                 res
//             })
//             .collect()
//     }
// }

// Prefix sum parity-unchange sol'n
// impl Solution {
//     pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
//         let first_val_swapped_parity = nums[0] ^ 1;
//         let parity_unswaps_since_start: Vec<u32> = nums
//             .into_iter()
//             .scan(
//                 (0, first_val_swapped_parity),
//                 |(unswaps_since_start, last), n| {
//                     if *last % 2 == n % 2 {
//                         *unswaps_since_start += 1;
//                     }
//                     *last = n;
//                     Some(*unswaps_since_start)
//                 },
//             )
//             .collect();
//         queries
//             .into_iter()
//             .map(|q| {
//                 let (l, r) = (q[0] as usize, q[1] as usize);
//                 if l == r {
//                     return true;
//                 }
//                 parity_unswaps_since_start[l] == parity_unswaps_since_start[r]
//             })
//             .collect()
//     }
// }

// Prefix sum parity-unchange inplace sol'n
impl Solution {
    pub fn is_array_special(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut last = nums[0] ^ 1;
        let mut unswaps_since_start = 0;
        for i in 0..nums.len() {
            if last % 2 == nums[i] % 2 {
                unswaps_since_start += 1;
            }
            last = std::mem::replace(&mut nums[i], unswaps_since_start);
        }
        queries
            .into_iter()
            .map(|q| {
                let (l, r) = (q[0] as usize, q[1] as usize);
                if l == r {
                    return true;
                }
                nums[l] == nums[r]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], queries: &[[i32; 2]], expected: &[bool]) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        for &n in nums {
            assert!(n >= 1);
            assert!(n <= 100_000);
        }
        assert!(queries.len() >= 1);
        assert!(queries.len() <= 100_000);
        assert_eq!(expected.len(), queries.len());
        for &q in queries {
            assert!(q[0] >= 0);
            assert!(q[0] <= q[1]);
            assert!(q[1] < nums.len() as i32);
        }
        assert_eq!(
            Solution::is_array_special(
                nums.to_vec(),
                queries.iter().map(|&x| x.to_vec()).collect()
            ),
            expected
        );
    }

    fn test_alltrue(nums: &[i32]) {
        let queries = (0..nums.len() as i32)
            .map(|i| (i..nums.len() as i32).map(move |j: i32| -> [i32; 2] { [i, j] }))
            .flatten()
            .collect::<Vec<_>>();
        let expected = vec![true; queries.len()];
        test(nums, &queries, &expected);
    }

    #[test]
    fn ex1() {
        test(&[3, 4, 1, 2, 6], &[[0, 4]], &[false])
    }

    #[test]
    fn ex2() {
        test(&[4, 3, 1, 6], &[[0, 2], [2, 3]], &[false, true])
    }

    #[test]
    fn ex2_1() {
        test(&[4, 3, 1, 6], &[[0, 3], [1, 3], [0, 2]], &[false; 3])
    }

    #[test]
    fn discussion_case1() {
        test_alltrue(&[1])
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 1], &[[0, 1]], &[false])
    }

    #[test]
    fn discussion_case2_1() {
        test(&[1, 1], &[[0, 0], [1, 1]], &[true, true])
    }

    #[test]
    fn discussion_case3() {
        test_alltrue(&[4, 5, 2, 7])
    }

    #[test]
    fn discussion_case4() {
        test(
            &[3, 7, 8],
            &[[0, 0], [0, 1], [1, 1], [1, 2], [2, 2], [0, 2]],
            &[true, false, true, true, true, false],
        )
    }

    #[test]
    fn discussion_case5() {
        let nums = [586, 1823];
        let input = nums.iter().cycle().take(300).copied().collect::<Vec<_>>();
        test_alltrue(&input);
    }
}
