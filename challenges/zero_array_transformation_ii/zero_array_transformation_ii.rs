// https://leetcode.com/problems/zero-array-transformation-ii/

pub struct Solution;

// Naive sol'n
// impl Solution {
//     pub fn min_zero_array(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
//         let mut nonzero_nums = nums.iter().filter(|&&x| x != 0).count();
//         if nonzero_nums == 0 {
//             return 0;
//         }
//         for (k, query) in queries.into_iter().enumerate() {
//             let l = query[0] as u32;
//             let r = query[1] as u32;
//             let value = query[2];
//             for i in l..=r {
//                 if nums[i as usize] == 0 {
//                     continue;
//                 }
//                 nums[i as usize] -= std::cmp::min(value, nums[i as usize]);
//                 if nums[i as usize] == 0 {
//                     nonzero_nums -= 1;
//                 }
//             }
//             if nonzero_nums == 0 {
//                 return k as i32 + 1;
//             }
//         }
//         return -1;
//     }
// }

// Optimization: Store the min and max nonzero indices
// impl Solution {
//     pub fn min_zero_array(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
//         let mut nonzero_nums = 0;
//         let mut min_nonzero = u32::MAX;
//         let mut max_nonzero = 0;
//         for (i, &n) in nums.iter().enumerate() {
//             if n != 0 {
//                 nonzero_nums += 1;
//                 if min_nonzero > i as u32 {
//                     min_nonzero = i as u32;
//                 }
//                 if max_nonzero < i as u32 {
//                     max_nonzero = i as u32;
//                 }
//             }
//         }
//         if nonzero_nums == 0 {
//             return 0;
//         }
//         for (k, query) in queries.into_iter().enumerate() {
//             let l = u32::max(query[0] as u32, min_nonzero);
//             let r = u32::min(query[1] as u32, max_nonzero);
//             let value = query[2];
//             for i in l..=r {
//                 if nums[i as usize] == 0 {
//                     continue;
//                 }
//                 nums[i as usize] -= std::cmp::min(value, nums[i as usize]);
//                 if nums[i as usize] == 0 {
//                     nonzero_nums -= 1;
//                     while min_nonzero < nums.len() as u32 && nums[min_nonzero as usize] == 0 {
//                         min_nonzero += 1;
//                     }
//                     while max_nonzero > 0 && nums[max_nonzero as usize] == 0 {
//                         max_nonzero -= 1;
//                     }
//                 }
//             }
//             if nonzero_nums == 0 {
//                 return k as i32 + 1;
//             }
//         }
//         return -1;
//     }
// }

// Diff pass based sol'n
// impl Solution {
//     pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
//         fn diffs_from_queries(queries: &[Vec<i32>], diffs: &mut [i32]) {
//             for q in queries.iter() {
//                 let l = q[0] as usize;
//                 let r = q[1] as usize;
//                 let v = q[2];
//                 diffs[l] += v;
//                 diffs[r + 1] -= v;
//             }
//         }
//         if nums.iter().all(|&x| x == 0) {
//             return 0;
//         }
//         let mut size = queries.len();
//         let mut base = 0;
//         if size == 0 {
//             return -1;
//         }
//         let diffs_prealloc = &mut vec![0; nums.len() + 1];
//         'outer: while size > 1 {
//             let half = size / 2;
//             let mid = base + half;
//             diffs_from_queries(unsafe { queries.get_unchecked(..mid) }, diffs_prealloc);
//             let mut sum = 0;
//             for i in 0..nums.len() {
//                 sum += diffs_prealloc[i as usize];
//                 if sum < nums[i as usize] {
//                     // The current queries are not enough -> binary search right
//                     base = mid;
//                     size -= half;
//                     diffs_prealloc.fill(0);
//                     continue 'outer;
//                 }
//             }
//             // Current queries were enough -> binary search left
//             size -= half;
//             diffs_prealloc.fill(0);
//             continue 'outer;
//         }
//         let final_pass = {
//             diffs_from_queries(unsafe { queries.get_unchecked(..=base) }, diffs_prealloc);
//             let mut final_pass = true;
//             let mut sum = 0;
//             for i in 0..nums.len() {
//                 sum += diffs_prealloc[i as usize];
//                 if sum < nums[i as usize] {
//                     // The current queries are not enough -> binary search right
//                     final_pass = false;
//                     break;
//                 }
//             }
//             final_pass
//         };
//         let res = base + (!final_pass as usize) + 1;
//         if res > queries.len() {
//             -1
//         } else {
//             res as i32
//         }
//     }
// }

// Optimize by saving pre-computed diffs from last step, if possible
impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        fn diffs_from_queries(queries: &[Vec<i32>], diffs: &mut [i32]) {
            for q in queries.iter() {
                let l = q[0] as usize;
                let r = q[1] as usize;
                let v = q[2];
                diffs[l] += v;
                diffs[r + 1] -= v;
            }
        }
        if nums.iter().all(|&x| x == 0) {
            return 0;
        }
        let mut size = queries.len();
        let mut base = 0;
        if size == 0 {
            return -1;
        }
        let diffs_prealloc = &mut vec![0; nums.len() + 1];
        let mut diffs_precomputed_to = 0;
        'outer: while size > 1 {
            let half = size / 2;
            let mid = base + half;
            diffs_from_queries(
                unsafe { queries.get_unchecked(diffs_precomputed_to..mid) },
                diffs_prealloc,
            );
            diffs_precomputed_to = mid;
            let mut sum = 0;
            for i in 0..nums.len() {
                sum += diffs_prealloc[i as usize];
                if sum < nums[i as usize] {
                    // The current queries are not enough -> binary search right
                    base = mid;
                    size -= half;
                    continue 'outer;
                }
            }
            // Current queries were enough -> binary search left
            size -= half;
            diffs_prealloc.fill(0);
            diffs_precomputed_to = 0;
            continue 'outer;
        }
        let final_pass = {
            diffs_from_queries(
                unsafe { queries.get_unchecked(diffs_precomputed_to..=base) },
                diffs_prealloc,
            );
            let mut final_pass = true;
            let mut sum = 0;
            for i in 0..nums.len() {
                sum += diffs_prealloc[i as usize];
                if sum < nums[i as usize] {
                    final_pass = false;
                    break;
                }
            }
            final_pass
        };
        let res = base + (!final_pass as usize) + 1;
        if res > queries.len() {
            -1
        } else {
            res as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u32], queries: &[[u32; 3]], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        assert!(queries.len() >= 1);
        assert!(queries.len() <= 100_000);
        for &n in nums {
            assert!(n <= 500_000);
        }
        for &[l, r, value] in queries {
            assert!(l <= r);
            assert!((r as usize) < nums.len());
            assert!(value >= 1);
            assert!(value <= 5);
        }
        assert_eq!(
            Solution::min_zero_array(
                nums.iter().map(|&x| x as i32).collect(),
                queries
                    .iter()
                    .map(|&[l, r, v]| vec![l as i32, r as i32, v as i32])
                    .collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[2, 0, 2], &[[0, 2, 1], [0, 2, 1], [1, 1, 3]], 2)
    }

    #[test]
    fn ex2() {
        test(&[4, 3, 2, 1], &[[1, 3, 2], [0, 2, 1]], -1)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[0],
            &[[0, 0, 2], [0, 0, 4], [0, 0, 4], [0, 0, 3], [0, 0, 5]],
            0,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[2, 10],
            &[
                [1, 1, 5],
                [0, 1, 2],
                [1, 1, 1],
                [1, 1, 5],
                [0, 1, 1],
                [0, 1, 4],
                [1, 1, 3],
                [1, 1, 3],
                [0, 0, 5],
                [0, 1, 2],
                [1, 1, 3],
                [1, 1, 4],
                [1, 1, 4],
                [0, 1, 5],
                [1, 1, 1],
            ],
            4,
        )
    }

    #[test]
    fn discussion_case3() {
        test(&[5], &[[0, 0, 5], [0, 0, 1], [0, 0, 3], [0, 0, 2]], 1)
    }

    #[test]
    fn discussion_case4() {
        test(&[3, 6, 4], &[[2, 2, 4]], -1)
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                826, 445, 164, 290, 78, 343, 992, 336, 184, 603, 815, 853, 527, 733, 450, 295, 186,
                11, 284, 533, 322,
            ],
            &[
                [5, 13, 5],
                [19, 19, 4],
                [2, 11, 4],
                [2, 20, 2],
                [17, 20, 4],
                [1, 20, 5],
                [14, 19, 4],
                [0, 5, 1],
                [0, 0, 4],
                [0, 9, 3],
                [16, 20, 2],
                [0, 3, 1],
                [2, 19, 3],
                [3, 14, 5],
                [0, 16, 4],
                [19, 20, 1],
                [0, 17, 4],
                [0, 5, 5],
                [11, 16, 2],
                [0, 0, 2],
                [19, 20, 1],
                [6, 8, 4],
                [3, 12, 4],
                [10, 17, 5],
                [0, 19, 1],
                [10, 14, 1],
                [0, 1, 5],
                [6, 7, 4],
                [11, 14, 5],
                [6, 6, 1],
                [6, 14, 5],
                [5, 14, 1],
                [0, 7, 5],
                [11, 20, 1],
                [0, 9, 2],
                [10, 16, 5],
                [17, 20, 4],
                [10, 12, 1],
                [1, 10, 4],
                [6, 18, 2],
                [0, 15, 3],
                [2, 6, 4],
                [4, 13, 1],
                [15, 15, 2],
                [0, 11, 4],
                [0, 6, 5],
                [13, 15, 4],
                [14, 18, 4],
                [1, 16, 2],
                [17, 18, 4],
                [8, 20, 3],
                [6, 20, 2],
                [5, 14, 3],
                [0, 20, 4],
                [0, 6, 1],
                [2, 16, 2],
                [6, 7, 4],
                [6, 12, 4],
                [13, 17, 3],
                [20, 20, 5],
                [0, 16, 4],
                [19, 20, 3],
                [0, 16, 4],
                [10, 16, 5],
                [6, 7, 2],
                [11, 15, 5],
                [1, 19, 5],
                [0, 1, 5],
                [2, 15, 4],
                [1, 13, 2],
                [7, 11, 4],
                [8, 19, 2],
                [12, 20, 5],
                [0, 1, 2],
                [10, 13, 4],
                [16, 17, 1],
                [17, 20, 4],
                [11, 20, 3],
                [15, 19, 3],
                [0, 9, 5],
                [1, 16, 4],
                [2, 10, 5],
                [0, 1, 3],
                [0, 11, 2],
                [6, 6, 2],
                [10, 18, 1],
                [6, 10, 3],
                [8, 10, 1],
                [3, 19, 3],
                [4, 19, 4],
                [19, 20, 1],
                [0, 20, 3],
                [7, 9, 2],
                [6, 20, 2],
                [16, 18, 5],
                [7, 12, 2],
                [16, 20, 1],
                [18, 20, 5],
                [6, 11, 4],
                [4, 9, 5],
                [10, 19, 1],
                [0, 6, 1],
                [0, 14, 2],
                [0, 19, 4],
                [6, 12, 4],
                [0, 20, 5],
                [0, 14, 3],
                [19, 20, 4],
                [0, 7, 4],
                [14, 20, 5],
                [12, 13, 3],
                [0, 20, 2],
                [18, 20, 5],
                [0, 8, 2],
                [1, 8, 3],
                [15, 20, 3],
                [17, 18, 4],
                [10, 18, 5],
                [17, 18, 2],
                [8, 14, 5],
                [12, 20, 4],
                [14, 18, 3],
                [0, 8, 3],
                [20, 20, 5],
                [13, 14, 1],
                [16, 20, 3],
                [0, 7, 4],
                [0, 1, 1],
                [1, 4, 4],
                [6, 20, 5],
                [8, 20, 3],
                [0, 16, 3],
                [2, 16, 2],
                [20, 20, 2],
                [0, 13, 1],
                [3, 20, 4],
                [1, 1, 2],
                [0, 8, 2],
                [0, 0, 1],
                [0, 10, 2],
                [4, 6, 5],
                [18, 19, 5],
                [5, 7, 4],
                [7, 16, 5],
                [3, 20, 5],
                [19, 20, 2],
                [16, 20, 5],
                [2, 18, 4],
                [0, 8, 3],
                [0, 19, 1],
                [5, 10, 3],
                [2, 19, 5],
                [3, 17, 2],
                [9, 19, 1],
                [0, 9, 2],
                [1, 12, 4],
                [1, 18, 1],
                [4, 19, 5],
                [13, 20, 1],
                [6, 12, 5],
                [0, 9, 2],
                [0, 8, 1],
                [0, 19, 3],
                [3, 20, 2],
                [0, 15, 1],
                [0, 11, 2],
                [3, 18, 2],
                [5, 5, 2],
                [1, 7, 4],
                [0, 9, 3],
                [0, 6, 5],
                [0, 5, 3],
                [4, 11, 2],
                [6, 14, 5],
                [6, 9, 4],
                [1, 6, 2],
                [14, 18, 4],
                [15, 19, 5],
                [15, 20, 4],
                [1, 15, 2],
                [10, 20, 2],
                [3, 20, 1],
                [0, 20, 3],
                [0, 1, 1],
                [5, 15, 1],
                [0, 2, 1],
                [7, 12, 4],
                [18, 18, 1],
                [5, 19, 2],
                [0, 12, 3],
                [0, 15, 5],
                [8, 16, 3],
                [8, 10, 4],
                [16, 17, 1],
                [20, 20, 5],
                [0, 8, 2],
                [19, 20, 3],
                [4, 12, 5],
                [14, 20, 1],
                [8, 8, 3],
                [5, 10, 3],
                [4, 20, 4],
                [15, 17, 3],
                [0, 2, 2],
                [5, 7, 2],
                [0, 17, 3],
                [4, 9, 4],
                [12, 16, 2],
                [0, 20, 4],
                [9, 18, 4],
                [7, 19, 5],
                [16, 19, 5],
                [0, 0, 5],
                [3, 11, 5],
                [14, 16, 5],
                [1, 7, 3],
                [1, 14, 1],
                [12, 20, 4],
                [2, 19, 4],
                [3, 13, 2],
                [0, 18, 5],
                [5, 13, 5],
                [15, 17, 3],
                [3, 15, 1],
                [8, 14, 1],
                [8, 13, 3],
                [0, 6, 4],
                [10, 11, 5],
                [16, 19, 4],
                [20, 20, 3],
                [3, 6, 2],
                [5, 20, 5],
                [13, 17, 5],
                [1, 16, 3],
                [19, 20, 4],
                [4, 11, 5],
                [1, 14, 2],
                [0, 19, 1],
                [5, 15, 1],
                [0, 14, 2],
                [0, 2, 3],
                [10, 19, 4],
                [0, 7, 1],
                [2, 19, 2],
                [3, 17, 5],
                [17, 20, 4],
                [3, 20, 1],
                [8, 19, 2],
                [4, 14, 4],
                [0, 8, 5],
                [10, 10, 2],
                [15, 19, 1],
                [0, 3, 5],
                [13, 15, 3],
                [12, 20, 5],
                [18, 20, 5],
                [0, 20, 2],
                [6, 7, 2],
                [1, 19, 2],
                [0, 7, 1],
                [6, 16, 3],
                [0, 12, 4],
                [13, 13, 4],
                [9, 20, 3],
                [8, 16, 5],
                [3, 8, 2],
                [14, 20, 2],
                [16, 20, 3],
                [10, 19, 4],
                [2, 2, 4],
                [12, 20, 4],
                [18, 19, 4],
                [0, 19, 3],
                [7, 20, 2],
                [0, 0, 2],
                [9, 19, 2],
                [3, 4, 1],
                [18, 18, 2],
                [0, 5, 3],
                [8, 11, 2],
                [7, 18, 1],
                [2, 16, 4],
                [9, 11, 2],
                [0, 0, 3],
                [14, 20, 4],
                [7, 8, 5],
                [3, 20, 3],
                [8, 19, 1],
                [17, 19, 4],
                [17, 17, 3],
                [18, 19, 1],
                [0, 10, 2],
                [9, 20, 3],
                [14, 20, 5],
                [1, 19, 1],
                [0, 9, 4],
                [0, 19, 4],
                [7, 19, 3],
                [18, 20, 2],
                [4, 19, 3],
                [0, 14, 2],
                [2, 19, 2],
                [0, 8, 1],
                [17, 19, 5],
                [19, 19, 4],
                [9, 18, 1],
                [6, 18, 1],
                [15, 19, 1],
                [18, 19, 2],
                [16, 17, 2],
                [16, 20, 1],
                [11, 18, 5],
                [0, 20, 3],
                [15, 18, 4],
                [2, 7, 2],
                [18, 18, 3],
                [0, 20, 2],
                [0, 20, 4],
                [14, 18, 2],
                [15, 20, 4],
                [0, 0, 5],
                [1, 20, 2],
                [2, 15, 3],
                [0, 10, 2],
                [18, 20, 4],
                [19, 20, 5],
                [18, 18, 3],
                [0, 20, 1],
                [0, 19, 5],
                [0, 19, 1],
                [9, 9, 1],
                [7, 19, 1],
                [9, 12, 4],
                [0, 17, 2],
                [20, 20, 5],
                [8, 20, 4],
                [12, 14, 3],
                [12, 17, 2],
                [5, 8, 4],
                [8, 11, 3],
                [14, 15, 2],
                [5, 5, 2],
                [14, 18, 4],
                [8, 19, 1],
                [0, 15, 5],
                [1, 3, 2],
                [6, 17, 2],
                [15, 19, 3],
                [10, 12, 4],
                [9, 19, 5],
                [0, 1, 4],
                [9, 11, 1],
                [17, 19, 2],
                [4, 16, 1],
                [8, 20, 4],
                [2, 16, 1],
                [13, 20, 4],
                [0, 9, 4],
                [6, 8, 5],
                [4, 20, 4],
                [4, 17, 3],
                [0, 20, 1],
                [20, 20, 5],
                [0, 7, 1],
                [0, 16, 5],
                [6, 9, 3],
                [6, 11, 1],
                [8, 20, 1],
                [0, 9, 2],
                [0, 2, 2],
                [0, 8, 4],
                [7, 12, 5],
                [6, 20, 3],
                [0, 20, 2],
                [0, 5, 2],
                [2, 19, 2],
                [8, 13, 5],
                [10, 20, 1],
                [18, 20, 3],
                [1, 3, 4],
                [2, 11, 4],
                [5, 15, 1],
                [1, 18, 3],
                [5, 7, 5],
                [0, 2, 4],
                [0, 18, 4],
                [0, 11, 4],
                [2, 9, 1],
                [0, 15, 1],
                [5, 17, 2],
                [10, 14, 1],
                [7, 20, 1],
                [9, 18, 1],
                [6, 16, 3],
                [19, 19, 2],
                [13, 13, 1],
                [0, 20, 4],
                [12, 12, 3],
                [6, 20, 4],
                [3, 6, 1],
                [0, 20, 5],
                [16, 20, 5],
                [10, 11, 4],
                [0, 14, 4],
                [15, 20, 3],
                [0, 20, 1],
                [16, 20, 2],
                [13, 18, 1],
                [0, 7, 3],
                [4, 5, 3],
                [8, 17, 5],
                [18, 19, 5],
                [13, 19, 4],
                [0, 2, 1],
                [9, 12, 2],
                [3, 11, 5],
                [14, 19, 2],
                [0, 4, 3],
                [15, 16, 5],
                [0, 1, 3],
                [0, 12, 5],
                [16, 20, 4],
                [0, 19, 3],
                [6, 6, 4],
                [6, 6, 3],
                [6, 12, 1],
                [0, 5, 5],
                [0, 20, 3],
                [6, 15, 3],
                [17, 20, 1],
                [12, 20, 4],
                [19, 20, 5],
                [8, 13, 5],
                [0, 1, 5],
                [0, 7, 1],
                [7, 9, 4],
                [0, 13, 5],
                [0, 6, 4],
                [12, 13, 5],
                [7, 19, 1],
                [0, 9, 1],
                [0, 10, 5],
                [0, 14, 3],
                [18, 19, 4],
                [0, 8, 2],
                [0, 11, 5],
                [14, 14, 1],
                [5, 7, 3],
                [1, 2, 3],
                [18, 20, 4],
                [10, 20, 2],
                [0, 7, 5],
                [0, 16, 2],
                [6, 7, 4],
                [1, 19, 2],
                [2, 19, 3],
                [14, 15, 4],
                [13, 20, 2],
                [0, 0, 2],
                [0, 20, 5],
                [0, 19, 2],
                [11, 14, 5],
                [2, 11, 2],
                [7, 18, 4],
                [0, 19, 4],
                [0, 2, 3],
                [1, 20, 2],
                [15, 20, 2],
                [6, 18, 2],
                [18, 19, 2],
                [15, 18, 4],
                [5, 12, 2],
                [20, 20, 2],
                [5, 7, 2],
                [9, 10, 2],
                [12, 20, 4],
                [0, 19, 5],
                [0, 9, 1],
                [2, 16, 1],
                [10, 17, 2],
                [0, 1, 2],
                [0, 12, 1],
                [17, 17, 5],
                [1, 19, 1],
                [13, 16, 3],
                [4, 12, 2],
                [6, 13, 1],
                [20, 20, 2],
                [6, 13, 5],
                [0, 1, 4],
                [12, 16, 2],
                [0, 19, 5],
                [0, 4, 4],
                [0, 14, 4],
                [14, 17, 2],
                [12, 19, 1],
                [1, 15, 2],
                [17, 20, 1],
                [0, 13, 1],
                [7, 20, 2],
                [5, 19, 3],
                [0, 18, 3],
                [0, 14, 3],
                [0, 7, 3],
                [6, 20, 2],
                [6, 8, 4],
                [4, 9, 3],
                [12, 20, 5],
                [0, 0, 4],
                [3, 19, 2],
                [0, 7, 5],
                [0, 17, 3],
                [11, 20, 2],
                [11, 19, 3],
                [8, 18, 1],
                [4, 5, 2],
                [6, 19, 4],
                [8, 15, 2],
                [0, 3, 5],
                [5, 9, 1],
                [14, 20, 4],
                [0, 13, 4],
                [4, 19, 1],
                [0, 13, 3],
                [9, 19, 2],
                [2, 20, 4],
                [10, 20, 2],
                [14, 16, 5],
                [2, 10, 3],
                [2, 6, 2],
                [0, 8, 2],
                [10, 17, 2],
                [2, 19, 3],
                [12, 18, 4],
                [10, 20, 3],
                [2, 17, 3],
                [12, 16, 3],
                [19, 20, 4],
                [5, 14, 1],
                [0, 12, 4],
                [7, 12, 4],
                [9, 13, 3],
                [8, 12, 3],
                [1, 18, 2],
                [5, 8, 2],
                [14, 18, 1],
                [4, 18, 5],
                [0, 1, 4],
                [19, 19, 5],
                [3, 17, 1],
                [0, 14, 3],
                [4, 4, 2],
                [0, 2, 5],
                [0, 8, 5],
                [7, 20, 2],
                [12, 19, 4],
                [0, 9, 2],
                [2, 2, 4],
                [0, 18, 2],
                [0, 5, 4],
                [0, 17, 4],
                [0, 15, 5],
                [14, 20, 2],
                [0, 2, 2],
                [0, 17, 3],
                [0, 4, 1],
                [0, 1, 3],
                [6, 19, 2],
                [0, 14, 5],
                [1, 1, 4],
                [9, 12, 1],
                [9, 19, 5],
                [15, 19, 2],
                [19, 19, 4],
                [12, 19, 2],
                [0, 8, 4],
                [17, 19, 1],
                [12, 19, 1],
                [13, 15, 5],
                [20, 20, 5],
                [7, 14, 5],
                [0, 5, 2],
                [7, 15, 5],
                [19, 19, 3],
                [5, 13, 2],
                [13, 20, 3],
                [0, 6, 3],
                [0, 15, 5],
                [3, 18, 4],
                [7, 20, 5],
                [18, 19, 3],
                [2, 4, 3],
                [5, 19, 5],
                [5, 10, 4],
                [7, 12, 1],
                [0, 0, 3],
                [20, 20, 1],
                [4, 19, 5],
                [0, 20, 3],
                [0, 18, 5],
                [11, 20, 2],
                [11, 14, 5],
                [0, 9, 4],
                [11, 11, 1],
                [8, 15, 3],
                [2, 5, 5],
                [8, 19, 4],
                [0, 8, 3],
                [1, 16, 3],
                [1, 15, 4],
                [12, 19, 4],
                [0, 19, 2],
                [9, 12, 2],
                [0, 2, 3],
                [20, 20, 4],
                [0, 7, 2],
                [0, 8, 2],
                [3, 14, 4],
                [0, 17, 5],
                [0, 2, 4],
                [5, 14, 2],
                [20, 20, 1],
                [17, 17, 4],
                [13, 17, 1],
                [16, 18, 3],
                [4, 10, 3],
                [12, 16, 5],
                [10, 10, 1],
                [8, 11, 5],
                [0, 17, 3],
                [18, 20, 2],
                [8, 14, 5],
                [11, 18, 1],
                [14, 16, 3],
                [0, 12, 5],
                [0, 20, 5],
                [0, 8, 4],
                [13, 18, 1],
                [6, 19, 3],
                [6, 11, 2],
                [6, 8, 1],
                [12, 16, 4],
                [13, 19, 3],
                [5, 16, 1],
                [20, 20, 2],
                [20, 20, 3],
                [0, 5, 4],
                [2, 18, 2],
                [6, 14, 5],
                [10, 16, 4],
                [0, 2, 1],
                [0, 6, 3],
                [19, 19, 1],
                [0, 17, 3],
                [19, 20, 5],
                [3, 19, 4],
                [0, 14, 4],
                [0, 0, 4],
                [4, 20, 3],
                [15, 19, 1],
                [1, 14, 1],
                [2, 11, 1],
                [2, 20, 4],
                [0, 12, 3],
                [0, 20, 2],
                [0, 12, 5],
                [11, 15, 2],
                [13, 13, 1],
                [16, 16, 3],
                [14, 15, 5],
                [0, 7, 4],
                [9, 15, 1],
                [13, 18, 4],
                [0, 20, 4],
                [15, 15, 5],
                [3, 10, 5],
                [7, 20, 3],
                [3, 20, 5],
                [0, 0, 5],
                [0, 14, 5],
                [0, 17, 4],
                [19, 20, 4],
                [5, 20, 2],
                [0, 2, 1],
                [19, 20, 4],
                [13, 14, 1],
                [2, 17, 3],
                [16, 20, 3],
                [6, 7, 5],
                [13, 18, 5],
                [13, 15, 2],
                [0, 17, 1],
                [19, 19, 2],
                [12, 20, 2],
                [0, 0, 4],
                [19, 20, 4],
                [0, 11, 3],
                [4, 20, 2],
                [0, 11, 3],
                [3, 11, 5],
                [0, 15, 1],
                [18, 20, 3],
                [13, 18, 1],
                [0, 1, 1],
                [0, 12, 3],
                [13, 20, 1],
                [2, 10, 2],
                [5, 12, 2],
                [0, 4, 1],
                [11, 15, 5],
                [1, 20, 1],
                [5, 16, 1],
                [0, 1, 1],
                [7, 12, 1],
                [10, 20, 4],
                [1, 9, 2],
                [1, 7, 2],
                [0, 1, 3],
                [1, 9, 2],
                [4, 19, 4],
                [0, 0, 4],
                [3, 16, 3],
                [19, 20, 3],
                [7, 18, 4],
                [0, 18, 5],
                [9, 11, 4],
                [0, 15, 4],
                [3, 20, 5],
                [19, 19, 5],
                [0, 11, 3],
                [20, 20, 5],
                [14, 20, 4],
                [4, 18, 4],
                [7, 18, 1],
                [12, 19, 5],
                [18, 18, 1],
                [13, 14, 3],
                [0, 15, 1],
                [4, 11, 2],
                [13, 20, 2],
                [16, 19, 2],
                [0, 11, 5],
                [13, 20, 2],
                [0, 13, 5],
                [0, 18, 5],
                [5, 20, 5],
                [13, 13, 3],
                [7, 20, 3],
                [0, 13, 2],
                [0, 2, 1],
                [7, 19, 2],
                [0, 19, 2],
                [2, 6, 5],
                [18, 20, 1],
                [18, 19, 1],
                [14, 15, 2],
                [14, 15, 1],
                [9, 9, 2],
                [2, 11, 3],
                [4, 18, 5],
                [0, 2, 2],
                [9, 19, 3],
                [14, 19, 2],
                [15, 18, 4],
                [9, 10, 1],
                [13, 13, 3],
                [16, 18, 2],
                [1, 8, 1],
                [3, 6, 2],
                [0, 2, 1],
                [19, 20, 3],
                [11, 17, 1],
                [17, 20, 2],
                [0, 19, 4],
                [0, 3, 3],
                [10, 17, 3],
                [5, 16, 5],
                [10, 16, 4],
                [5, 14, 3],
                [0, 10, 3],
                [7, 12, 1],
                [19, 20, 3],
                [12, 16, 5],
                [0, 2, 5],
                [2, 12, 4],
                [13, 15, 1],
                [13, 13, 2],
                [0, 1, 1],
                [0, 4, 4],
                [6, 7, 5],
                [19, 19, 4],
                [6, 13, 5],
                [0, 0, 5],
                [2, 20, 3],
                [1, 10, 4],
                [10, 10, 4],
                [11, 11, 1],
                [6, 12, 2],
                [20, 20, 1],
                [16, 16, 3],
                [14, 14, 4],
                [7, 12, 1],
                [5, 12, 4],
                [0, 12, 3],
                [0, 5, 3],
                [11, 19, 2],
                [9, 10, 1],
                [5, 19, 5],
                [8, 19, 3],
                [9, 13, 3],
                [0, 0, 1],
                [10, 18, 2],
                [18, 18, 3],
                [4, 9, 5],
                [13, 13, 4],
                [18, 20, 3],
                [0, 7, 1],
                [5, 6, 4],
                [3, 18, 1],
                [5, 19, 5],
                [0, 4, 2],
                [19, 20, 4],
                [3, 8, 4],
                [0, 17, 1],
                [2, 16, 4],
                [0, 14, 5],
                [2, 19, 4],
                [4, 11, 4],
                [3, 19, 2],
                [17, 17, 3],
                [17, 18, 3],
                [2, 14, 3],
                [3, 6, 4],
                [11, 19, 5],
                [0, 17, 5],
                [0, 10, 5],
                [14, 20, 1],
                [3, 15, 5],
                [0, 12, 4],
                [11, 16, 1],
                [18, 19, 4],
                [4, 20, 3],
                [3, 10, 4],
                [9, 11, 5],
                [0, 7, 1],
                [5, 16, 2],
                [14, 14, 1],
                [3, 16, 4],
                [11, 13, 2],
                [5, 16, 3],
                [14, 14, 3],
                [0, 1, 5],
                [11, 13, 5],
                [0, 2, 2],
                [0, 0, 3],
                [18, 20, 4],
                [0, 19, 2],
                [1, 7, 3],
                [11, 13, 2],
                [0, 6, 2],
                [14, 19, 3],
                [1, 12, 5],
                [0, 18, 5],
                [0, 14, 2],
                [6, 19, 4],
                [6, 17, 5],
                [1, 18, 2],
                [16, 20, 1],
                [6, 12, 4],
                [0, 19, 1],
                [10, 14, 3],
                [19, 19, 2],
                [2, 10, 5],
                [0, 16, 2],
                [0, 14, 1],
                [0, 18, 5],
                [5, 6, 3],
                [4, 12, 3],
                [0, 20, 3],
                [0, 7, 2],
                [0, 17, 4],
                [0, 0, 3],
                [1, 5, 1],
                [17, 17, 5],
                [4, 11, 3],
                [16, 16, 1],
                [8, 15, 3],
                [0, 11, 1],
                [17, 20, 3],
                [0, 9, 1],
                [1, 19, 2],
                [0, 0, 4],
                [13, 13, 4],
                [10, 20, 3],
                [2, 15, 3],
                [3, 4, 1],
                [0, 10, 1],
                [8, 20, 2],
                [5, 19, 2],
                [0, 1, 2],
                [3, 20, 5],
                [0, 0, 3],
                [0, 8, 4],
                [0, 17, 3],
                [19, 20, 2],
                [0, 1, 4],
                [3, 8, 2],
                [6, 19, 3],
                [10, 19, 2],
                [14, 19, 4],
                [11, 19, 3],
                [9, 12, 4],
                [5, 13, 1],
                [1, 15, 5],
                [4, 9, 3],
                [18, 19, 3],
                [0, 19, 2],
                [16, 20, 5],
                [1, 20, 4],
                [14, 17, 2],
                [0, 4, 2],
                [0, 9, 2],
                [0, 19, 5],
                [2, 8, 4],
                [0, 14, 1],
                [8, 20, 4],
                [0, 3, 3],
                [16, 20, 3],
                [6, 19, 3],
                [12, 20, 4],
                [0, 4, 4],
                [10, 19, 3],
                [8, 13, 2],
                [0, 1, 5],
                [6, 12, 5],
                [16, 19, 4],
                [0, 19, 3],
                [7, 20, 1],
                [0, 16, 3],
                [5, 14, 3],
                [14, 17, 3],
                [0, 18, 1],
                [18, 19, 2],
                [13, 16, 2],
                [4, 19, 2],
                [15, 19, 1],
                [2, 13, 2],
                [20, 20, 5],
                [0, 9, 1],
                [14, 20, 1],
                [13, 16, 4],
                [0, 3, 3],
                [13, 17, 4],
                [0, 3, 5],
                [4, 18, 2],
                [0, 15, 4],
                [14, 15, 5],
                [16, 20, 3],
                [8, 19, 5],
                [5, 12, 2],
                [1, 10, 5],
                [8, 13, 2],
                [6, 11, 4],
                [9, 19, 4],
                [19, 20, 2],
                [0, 18, 4],
                [19, 19, 4],
                [4, 20, 3],
                [4, 13, 2],
                [19, 19, 3],
                [11, 19, 5],
                [19, 19, 3],
                [5, 19, 2],
                [0, 20, 1],
                [3, 14, 3],
                [15, 19, 5],
                [0, 17, 5],
                [15, 19, 5],
                [17, 18, 5],
                [0, 12, 1],
                [20, 20, 4],
                [0, 7, 2],
                [2, 15, 3],
                [9, 17, 2],
                [5, 8, 2],
                [0, 17, 3],
                [6, 20, 5],
                [5, 12, 5],
                [15, 20, 3],
                [0, 9, 5],
                [3, 5, 5],
                [8, 12, 3],
                [3, 12, 1],
                [1, 20, 5],
                [14, 15, 3],
                [9, 19, 4],
                [7, 14, 2],
                [5, 6, 1],
                [0, 18, 4],
                [9, 18, 2],
            ],
            927,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        let input = vec![1; 100_000];
        let queries = vec![[0, 99_999, 1]; 100_000];
        test(&input, &queries, 1);
    }

    #[test]
    fn my_extreme_ex2() {
        let input = vec![1; 100_000];
        let mut queries = vec![];
        for i in 0..100_000 {
            queries.push([i, i, 1]);
        }
        test(&input, &queries, 100_000);
    }

    #[test]
    fn my_extreme_ex3() {
        let input = vec![1; 100_000];
        let mut queries = vec![];
        for i in 0..100_000 {
            queries.push([0, i, 1]);
        }
        test(&input, &queries, 100_000);
    }

    #[test]
    fn my_extreme_ex4() {
        let input = vec![1; 100_000];
        let mut queries = vec![];
        for i in 0..100_000 {
            queries.push([i, 99_999, 1]);
        }
        test(&input, &queries, 1);
    }

    #[test]
    fn my_extreme_ex5() {
        let input = vec![2; 100_000];
        let mut queries = vec![];
        for i in 0..100_000 {
            queries.push([i, i, 1]);
        }
        test(&input, &queries, -1);
    }

    #[test]
    fn my_extreme_ex6() {
        let input = vec![2; 100_000];
        let mut queries = vec![];
        for i in 0..100_000 {
            queries.push([0, i, 1]);
        }
        test(&input, &queries, -1);
    }

    #[test]
    fn my_extreme_ex7() {
        let input = vec![2; 100_000];
        let mut queries = vec![];
        for i in 0..100_000 {
            queries.push([i, 99_999, 1]);
        }
        test(&input, &queries, -1);
    }
}
