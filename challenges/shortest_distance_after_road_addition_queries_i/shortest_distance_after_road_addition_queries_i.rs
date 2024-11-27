// https://leetcode.com/problems/shortest-distance-after-road-addition-queries-i/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
//         assert!(n >= 3);
//         let n = n as u16;
//         let mut dist_to_end: Vec<u16> = (0..n).rev().collect();
//         let mut starts_per_end: std::collections::HashMap<u16, Vec<u16>> =
//             std::collections::HashMap::new();
//         let mut results = std::vec::Vec::with_capacity(queries.len());
//         for query in queries {
//             let start = query[0] as u16;
//             let end = query[1] as u16;
//             starts_per_end.entry(end).or_insert(vec![]).push(start);
//             for i in (1..=end).rev() {
//                 dist_to_end[i as usize - 1] =
//                     std::cmp::min(dist_to_end[i as usize - 1], dist_to_end[i as usize] + 1);
//                 let Some(starts) = starts_per_end.get(&i) else {
//                     continue;
//                 };
//                 for &start in starts {
//                     dist_to_end[start as usize] =
//                         std::cmp::min(dist_to_end[start as usize], dist_to_end[i as usize] + 1);
//                 }
//             }
//             results.push(dist_to_end[0] as i32)
//         }
//         results
//     }
// }

// Optimized sol'n (limited fixing points)
// impl Solution {
//     pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
//         assert!(n >= 3);
//         let n = n as u16;
//         let mut dist_to_end: Vec<u16> = (0..n).rev().collect();
//         let mut starts_per_end: std::collections::HashMap<u16, Vec<u16>> =
//             std::collections::HashMap::new();
//         let mut results = std::vec::Vec::with_capacity(queries.len());
//         let mut correction_stack: Vec<(u16, u16)> = vec![];
//         for query in queries {
//             let start = query[0] as u16;
//             let end = query[1] as u16;
//             starts_per_end.entry(end).or_insert(vec![]).push(start);
//             correction_stack.push((start, end));
//             while let Some((start, end)) = correction_stack.pop() {
//                 if dist_to_end[start as usize] > dist_to_end[end as usize] + 1 {
//                     dist_to_end[start as usize] = dist_to_end[end as usize] + 1;
//                     if start > 0 {
//                         correction_stack.push((start - 1, start));
//                     }
//                     correction_stack.extend(
//                         starts_per_end
//                             .get(&start)
//                             .unwrap_or(&vec![])
//                             .iter()
//                             .map(|&x| (x, start)),
//                     );
//                 }
//             }
//             results.push(dist_to_end[0] as i32)
//         }
//         results
//     }
// }

// Further optimized sol'n (no hashmap)
impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        assert!(n >= 3);
        let n = n as u16;
        let mut dist_to_end: Vec<u16> = (0..n).rev().collect();
        let mut starts_per_end: Vec<Vec<u16>> = (0..n)
            .map(|i| if i > 0 { vec![i - 1] } else { vec![] })
            .collect();
        let mut results = std::vec::Vec::with_capacity(queries.len());
        let mut correction_stack: Vec<(u16, u16)> = vec![];
        for query in queries {
            let start = query[0] as u16;
            let end = query[1] as u16;
            starts_per_end[end as usize].push(start);
            correction_stack.push((start, end));
            while let Some((start, end)) = correction_stack.pop() {
                if dist_to_end[start as usize] > dist_to_end[end as usize] + 1 {
                    dist_to_end[start as usize] = dist_to_end[end as usize] + 1;
                    if start > 0 {
                        correction_stack.push((start - 1, start));
                    }
                    correction_stack
                        .extend(starts_per_end[start as usize].iter().map(|&x| (x, start)));
                }
            }
            results.push(dist_to_end[0] as i32)
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, queries: &[[i32; 2]], expected: &[i32]) {
        assert!(n >= 3);
        assert!(n <= 500);
        assert!(queries.len() >= 1);
        assert!(queries.len() <= 500);
        let mut seen = std::collections::HashSet::new();
        for query in queries {
            assert!(query[0] >= 0);
            assert!(query[0] < query[1]);
            assert!(query[1] < n);
            assert!(1 < query[1] - query[0]);
            assert!(seen.insert(query));
        }
        assert_eq!(
            Solution::shortest_distance_after_queries(
                n,
                queries.iter().map(|&x| x.to_vec()).collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(5, &[[2, 4], [0, 2], [0, 4]], &[3, 2, 1]);
    }

    #[test]
    fn ex2() {
        test(4, &[[0, 3], [0, 2]], &[1, 1])
    }

    #[test]
    fn myex1() {
        test(6, &[[0, 4], [3, 5]], &[2, 2])
    }

    #[test]
    fn myex2() {
        test(10, &[[2, 7], [0, 4], [4, 9]], &[5, 5, 2])
    }

    #[test]
    fn myex2_1() {
        test(10, &[[0, 4], [2, 7], [4, 9]], &[6, 5, 2])
    }

    #[test]
    fn myex2_2() {
        test(10, &[[0, 4], [4, 9], [2, 7]], &[6, 2, 2])
    }

    #[test]
    fn my_extreme_ex1() {
        let mut queries = vec![];
        let mut ans = vec![];
        for i in 2..500 {
            queries.push([0, i]);
            ans.push(500 - i);
        }
        test(500, &queries, &ans)
    }

    #[test]
    fn my_extreme_ex2() {
        let mut queries = vec![];
        let mut ans = vec![];
        for i in (0..498).rev() {
            queries.push([i, 499]);
            ans.push(i + 1);
        }
        test(500, &queries, &ans)
    }
}
