// https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/

pub struct Solution;

// Naive sol'n (No submission attempted, obviously too slow)
// impl Solution {
//     pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
//         // Alice and Bob can only move to the right,
//         // so they can't meet any more left than the
//         // right of the two.
//         let n = heights.len() as u16;
//         queries
//             .into_iter()
//             .map(|query| {
//                 let (a, b) = (query[0] as u16, query[1] as u16);
//                 if a == b {
//                     return a as i32;
//                 }
//                 let (l, r) = (std::cmp::min(a, b), std::cmp::max(a, b));
//                 if heights[l as usize] < heights[r as usize] {
//                     return r as i32;
//                 }
//                 for i in r..n {
//                     if heights[l as usize] < heights[i as usize] {
//                         // We don't need to check heights[r] < heights[i]
//                         // because we know heights[l] >= heights[r].
//                         return i as i32;
//                     }
//                 }
//                 -1
//             })
//             .collect()
//     }
// }

// Naive sol'n + caching (Barely passes at 1823ms)
// impl Solution {
//     pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
//         // Alice and Bob can only move to the right,
//         // so they can't meet any more left than the
//         // right of the two.
//         let n = heights.len() as u16;
//         let mut cache = std::collections::HashMap::new();
//         queries
//             .into_iter()
//             .map(|query| {
//                 let (a, b) = (query[0] as u16, query[1] as u16);
//                 *cache.entry((a, b)).or_insert_with(|| {
//                     if a == b {
//                         return a as i32;
//                     }
//                     let (l, r) = (std::cmp::min(a, b), std::cmp::max(a, b));
//                     if heights[l as usize] < heights[r as usize] {
//                         return r as i32;
//                     }
//                     for i in r..n {
//                         if heights[l as usize] < heights[i as usize] {
//                             // We don't need to check heights[r] < heights[i]
//                             // because we know heights[l] >= heights[r].
//                             return i as i32;
//                         }
//                     }
//                     -1
//                 })
//             })
//             .collect()
//     }
// }

// My monotonic stack sol'n (Adapted from best)
impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        type Idx = u16;
        let mut ans = vec![-1; queries.len()];
        let mut height_table = vec![vec![]; heights.len()];
        for i in 0..queries.len() as u16 {
            let query = &queries[i as usize];
            let (a, b) = (query[0] as Idx, query[1] as Idx);
            if a == b {
                ans[i as usize] = a as i32;
                continue;
            }
            let (l, r) = (std::cmp::min(a, b), std::cmp::max(a, b));
            if heights[l as usize] < heights[r as usize] {
                ans[i as usize] = r as i32;
                continue;
            }
            height_table[r as usize].push((heights[l as usize], i as Idx));
        }
        let mut monotonic_stack: Vec<(i32, Idx)> = Vec::new();
        for (height_index, entries) in height_table.into_iter().enumerate().rev() {
            for (height, query_index) in entries {
                let pos = monotonic_stack.partition_point(|&(h, _)| h > height);
                if pos > 0 {
                    ans[query_index as usize] = monotonic_stack[pos - 1].1 as i32;
                }
            }
            loop {
                let Some(&(h, _)) = monotonic_stack.last() else {
                    break;
                };
                if h > heights[height_index] {
                    break;
                }
                monotonic_stack.pop();
            }
            monotonic_stack.push((heights[height_index], height_index as u16))
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(heights: &[i32], queries: &[[u16; 2]], expected: &[i32]) {
        assert!(heights.len() >= 1);
        assert!(heights.len() <= 50_000);
        for &height in heights {
            assert!(height >= 1);
            assert!(height <= 1_000_000_000);
        }
        assert!(queries.len() >= 1);
        assert!(queries.len() <= 50_000);
        for &[a, b] in queries {
            assert!(a < heights.len() as u16);
            assert!(b < heights.len() as u16);
        }
        assert_eq!(
            Solution::leftmost_building_queries(
                heights.to_vec(),
                queries
                    .iter()
                    .map(|&[a, b]| vec![a as i32, b as i32])
                    .collect(),
            ),
            expected
        );
    }

    fn test_allqueries(heights: &[i32], expected: &[i32]) {
        assert!(heights.len() >= 1);
        assert!(heights.len() <= 223); // floor(sqrt(50_000))
        let queries: Vec<_> = (0..heights.len() as u16)
            .into_iter()
            .map(|i| (0..heights.len() as u16).into_iter().map(move |j| [i, j]))
            .flatten()
            .collect();
        test(heights, &queries, expected);
    }

    #[test]
    fn ex1() {
        test(
            &[6, 4, 8, 5, 2, 7],
            &[[0, 1], [0, 3], [2, 4], [3, 4], [2, 2]],
            &[2, 5, -1, 5, 2],
        )
    }

    #[test]
    fn ex1_1() {
        test(&[6, 4, 8], &[[0, 1]], &[2])
    }

    #[test]
    fn ex1_2() {
        test(&[6, 4, 8, 5, 2, 7], &[[0, 1], [0, 3]], &[2, 5])
    }

    #[test]
    fn ex2() {
        test(
            &[5, 3, 8, 2, 6, 1, 4, 6],
            &[[0, 7], [3, 5], [5, 2], [3, 0], [1, 6]],
            &[7, 6, -1, 4, 6],
        )
    }

    #[test]
    fn discussion_case1() {
        test_allqueries(
            &[1, 2, 1, 2, 1, 2],
            &[
                0, 1, 3, 3, 5, 5, // [0,x]
                1, 1, -1, -1, -1, -1, // [1,x]
                3, -1, 2, 3, 5, 5, // [2,x]
                3, -1, 3, 3, -1, -1, // ? [3,x]
                5, -1, 5, -1, 4, 5, // ? [4,x]
                5, -1, 5, -1, 5, 5, // [5,x]
            ],
        )
    }

    #[test]
    fn discussion_case2() {
        test_allqueries(
            &[3, 4, 1, 2],
            &[
                0, 1, -1, -1, // [0,x]
                1, 1, -1, -1, // [1,x]
                -1, -1, 2, 3, // [2,x]
                -1, -1, 3, 3, // [3,x]
            ],
        )
    }

    #[test]
    fn discussion_case3() {
        test_allqueries(
            &[3, 1, 2, 4],
            &[
                0, 3, 3, 3, // [0,x]
                3, 1, 2, 3, // [1,x]
                3, 2, 2, 3, // [2,x]
                3, 3, 3, 3, // [3,x]
            ],
        )
    }

    #[test]
    fn my_extreme_ex1() {
        const EXTREMA: i32 = 223; // floor(sqrt(50_000))
        let heights = (1..=EXTREMA).into_iter().collect::<Vec<i32>>();
        let answers = (0..EXTREMA as u16)
            .into_iter()
            .map(|i| {
                (0..EXTREMA as u16)
                    .into_iter()
                    .map(move |j| std::cmp::max(i, j) as i32)
            })
            .flatten()
            .collect::<Vec<i32>>();
        test_allqueries(&heights, &answers)
    }

    #[test]
    fn my_extreme_ex2() {
        const EXTREMA: u16 = 50_000;
        let heights = (1..=EXTREMA as i32).into_iter().collect::<Vec<_>>();
        let queries = &[[0, EXTREMA - 1]; EXTREMA as usize];
        let answers = &[EXTREMA as i32 - 1; EXTREMA as usize];
        test(&heights, queries, answers)
    }

    #[test]
    fn extreme_discussion_case1() {
        let heights_str = &include_str!("extreme_discussion_case1_heights.txt");
        let heights = (&heights_str[1..heights_str.len() - 2])
            .split(',')
            .map(|s| -> i32 { s.parse().unwrap() })
            .collect::<Vec<i32>>();
        let queries_str = &include_str!("extreme_discussion_case1_queries.txt");
        let queries = (&queries_str[2..queries_str.len() - 3])
            .split("],[")
            .map(|s| {
                let mut nums = s.split(',').map(|s| s.parse());
                let a: i32 = nums.next().unwrap().unwrap();
                let b: i32 = nums.next().unwrap().unwrap();
                vec![a, b]
            })
            .collect::<Vec<Vec<i32>>>();
        Solution::leftmost_building_queries(heights, queries);
    }
}
