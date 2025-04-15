// https://leetcode.com/problems/count-good-triplets-in-an-array/

pub struct Solution;

// Naive compute all triples (O(n^3), too slow)
// impl Solution {
//     pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
//         let nums2_positions = nums2
//             .into_iter()
//             .enumerate()
//             .map(|(i, x)| (x, i as u32))
//             .collect::<std::collections::HashMap<_, _>>();
//         let mut count = 0;
//         for i in 0..nums1.len() {
//             for j in i + 1..nums1.len() {
//                 let p2j = nums2_positions[&nums1[j]];
//                 if nums2_positions[&nums1[i]] >= p2j {
//                     continue;
//                 }
//                 for k in j + 1..nums1.len() {
//                     if p2j >= nums2_positions[&nums1[k]] {
//                         continue;
//                     }
//                     count += 1;
//                 }
//             }
//         }
//         count
//     }
// }

// Double scans (O(n^2), too slow)
// impl Solution {
//     pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
//         let nums2_positions = nums2
//             .into_iter()
//             .enumerate()
//             .map(|(i, x)| (x, i as u32))
//             .collect::<std::collections::HashMap<_, _>>();
//         let mut count = 0;
//         for i in (1..nums1.len() - 1).rev() {
//             let p2i = nums2_positions[&nums1[i]];
//             let mut bigger = 0;
//             for j in i + 1..nums1.len() {
//                 if p2i < nums2_positions[&nums1[j]] {
//                     bigger += 1;
//                 }
//             }
//             if bigger <= 0 {
//                 continue;
//             }
//             for j in 0..i {
//                 if p2i > nums2_positions[&nums1[j]] {
//                     count += bigger as i64;
//                 }
//             }
//         }
//         count
//     }
// }

// Double scans with lookup optimization (O(n^2), too slow... but 10x faster)
// impl Solution {
//     pub fn good_triplets(mut nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
//         let nums2_positions = nums2
//             .into_iter()
//             .enumerate()
//             .map(|(i, x)| (x, i as i32))
//             .collect::<std::collections::HashMap<_, _>>();
//         nums1.iter_mut().for_each(|x| *x = nums2_positions[&x]);
//         std::mem::drop(nums2_positions);
//         let mut count = 0;
//         for i in (1..nums1.len() - 1).rev() {
//             let p2i = nums1[i];
//             let mut bigger = 0;
//             for j in i + 1..nums1.len() {
//                 if p2i < nums1[j] {
//                     bigger += 1;
//                 }
//             }
//             if bigger <= 0 {
//                 continue;
//             }
//             for j in 0..i {
//                 if p2i > nums1[j] {
//                     count += bigger as i64;
//                 }
//             }
//         }
//         count
//     }
// }

// Sorted list binsearch v1
// impl Solution {
//     pub fn good_triplets(mut nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
//         let nums2_positions = nums2
//             .into_iter()
//             .enumerate()
//             .map(|(i, x)| (x, i as i32))
//             .collect::<std::collections::HashMap<_, _>>();
//         nums1.iter_mut().for_each(|x| *x = nums2_positions[&x]);
//         std::mem::drop(nums2_positions);
//         let mut count = 0;
//         let mut seen = std::vec::Vec::with_capacity(nums1.len());
//         for i in 0..nums1.len() {
//             let p2i = nums1[i];
//             let smaller = seen
//                 .binary_search(&p2i)
//                 .expect_err("non-permutation detected");
//             seen.insert(smaller, p2i);
//             if smaller <= 0 {
//                 continue;
//             }
//             let smaller = smaller as i64;
//             for j in i + 1..nums1.len() {
//                 if p2i < nums1[j] {
//                     count += smaller;
//                 }
//             }
//         }
//         count
//     }
// }

// Sorted list binsearch v2 (nlogn if you don't count the insert op)
// impl Solution {
//     pub fn good_triplets(mut nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
//         let nums2_positions = nums2
//             .into_iter()
//             .enumerate()
//             .map(|(i, x)| (x, i as i32))
//             .collect::<std::collections::HashMap<_, _>>();
//         nums1.iter_mut().for_each(|x| *x = nums2_positions[&x]);
//         std::mem::drop(nums2_positions);
//         let mut smaller_than_pos = vec![0; nums1.len()];
//         let mut seen = std::vec::Vec::with_capacity(nums1.len());
//         for i in 0..nums1.len() {
//             let p2i = nums1[i];
//             let smaller = seen
//                 .binary_search(&p2i)
//                 .expect_err("non-permutation detected");
//             seen.insert(smaller, p2i);
//             smaller_than_pos[i] = smaller as u32;
//         }
//         seen.clear();
//         let mut count = 0;
//         for i in (0..nums1.len()).rev() {
//             let p2i = nums1[i];
//             let smaller = seen
//                 .binary_search(&p2i)
//                 .expect_err("non-permutation detected");
//             let larger = seen.len() - smaller;
//             seen.insert(smaller, p2i);
//             count += smaller_than_pos[i] as i64 * larger as i64;
//         }
//         count
//     }
// }

// Fenwick Tree sol'n
impl Solution {
    pub fn good_triplets(mut nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        struct FenwickTree {
            tree: Vec<u32>,
        }
        impl FenwickTree {
            fn with_capacity(n: usize) -> Self {
                FenwickTree {
                    tree: vec![0; n + 1],
                }
            }
            fn add_one(&mut self, mut index: i32) {
                index += 1;
                while index < self.tree.len() as i32 {
                    self.tree[index as usize] += 1;
                    index += index & (-index);
                }
            }
            fn prefix_sum(&self, mut index: i32) -> u32 {
                index += 1;
                let mut result = 0;
                while index > 0 {
                    result += self.tree[index as usize];
                    index -= index & (-index);
                }
                result
            }
            fn clear(&mut self) {
                self.tree.fill(0);
            }
        }
        let nums2_positions = nums2
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i as i32))
            .collect::<std::collections::HashMap<_, _>>();
        nums1.iter_mut().for_each(|x| *x = nums2_positions[&x]);
        std::mem::drop(nums2_positions);
        let mut seen = FenwickTree::with_capacity(nums1.len());
        let smaller_than_pos: Vec<_> = (0..nums1.len())
            .map(|i| {
                let p2i = nums1[i];
                let smaller = seen.prefix_sum(p2i);
                seen.add_one(p2i);
                smaller
            })
            .collect();
        seen.clear();
        let mut count = 0;
        for i in (0..nums1.len() as u32).rev() {
            let p2i = nums1[i as usize];
            seen.add_one(p2i);
            let smaller = seen.prefix_sum(p2i);
            let larger = (nums1.len() as u32 - i) - smaller;
            count += smaller_than_pos[i as usize] as i64 * larger as i64;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums1: &[i32], nums2: &[i32], expected: u64) {
        assert_eq!(nums1.len(), nums2.len());
        assert!(nums1.len() >= 3);
        assert!(nums1.len() <= 100_000);
        let mut seen = std::collections::HashSet::new();
        for &num in nums1 {
            assert!(num >= 0);
            assert!((num as usize) < nums1.len());
            assert!(seen.insert(num));
        }
        seen.clear();
        for &num in nums2 {
            assert!(num >= 0);
            assert!((num as usize) < nums1.len());
            assert!(seen.insert(num));
        }
        assert_eq!(
            Solution::good_triplets(
                nums1.iter().map(|&x| x as i32).collect(),
                nums2.iter().map(|&x| x as i32).collect(),
            ),
            expected as i64
        );
        assert_eq!(
            Solution::good_triplets(
                nums2.iter().map(|&x| x as i32).collect(),
                nums1.iter().map(|&x| x as i32).collect(),
            ),
            expected as i64
        );
    }

    #[test]
    fn ex1() {
        test(&[2, 0, 1, 3], &[0, 1, 2, 3], 1)
    }

    #[test]
    fn ex2() {
        test(&[4, 0, 1, 3, 2], &[4, 1, 0, 2, 3], 4)
    }

    #[test]
    fn myex1() {
        test(&[0, 1, 3, 2], &[1, 0, 2, 3], 0)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[2, 3, 9, 8, 4, 7, 0, 6, 5, 1],
            &[8, 7, 9, 5, 6, 2, 4, 3, 1, 0],
            18,
        )
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 4, 2, 0, 6, 5, 3, 7], &[1, 4, 3, 7, 0, 2, 6, 5], 20)
    }

    #[test]
    fn discussion_case3() {
        test(
            &[13, 14, 10, 2, 12, 3, 9, 11, 15, 8, 4, 7, 0, 6, 5, 1],
            &[8, 7, 9, 5, 6, 14, 15, 10, 2, 11, 4, 13, 3, 12, 1, 0],
            77,
        )
    }

    #[test]
    fn my_extreme_ex100() {
        let inputs = (0..100).collect::<Vec<_>>();
        test(&inputs, &inputs, 161700)
    }

    #[test]
    fn my_extreme_ex1000() {
        let inputs = (0..1_000).collect::<Vec<_>>();
        test(&inputs, &inputs, 166167000)
    }

    #[test]
    fn my_extreme_ex10000() {
        let inputs = (0..10_000).collect::<Vec<_>>();
        test(&inputs, &inputs, 166616670000)
    }

    #[test]
    fn my_extreme_ex100000() {
        let inputs = (0..100_000).collect::<Vec<_>>();
        test(&inputs, &inputs, 166661666700000)
    }

    #[test]
    fn my_extreme_ex100000_rev() {
        let inputs = (0..100_000).rev().collect::<Vec<_>>();
        test(&inputs, &inputs, 166661666700000)
    }
}
