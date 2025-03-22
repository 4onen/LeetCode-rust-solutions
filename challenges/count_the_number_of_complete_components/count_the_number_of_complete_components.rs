// https://leetcode.com/problems/count-the-number-of-complete-components/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
//         let n = n as u8;
//         let mut graph = vec![vec![]; n as usize];
//         for edge in edges {
//             graph[edge[0] as usize].push(edge[1] as usize);
//             graph[edge[1] as usize].push(edge[0] as usize);
//         }
//         let mut visited = vec![false; n as usize];
//         let mut stack = vec![];
//         let mut count = 0;
//         for i in 0..n {
//             if !visited[i as usize] {
//                 let expected_degree = graph[i as usize].len() as u16;
//                 let mut is_complete = true;
//                 let mut component_count = 0u8;
//                 stack.push(i);
//                 while let Some(u) = stack.pop() {
//                     if visited[u as usize] {
//                         continue;
//                     }
//                     visited[u as usize] = true;
//                     component_count += 1;
//                     if graph[u as usize].len() != expected_degree as usize {
//                         is_complete = false;
//                     }
//                     for &v in &graph[u as usize] {
//                         stack.push(v as u8);
//                     }
//                 }
//                 if is_complete && expected_degree + 1 == component_count as u16 {
//                     count += 1;
//                 }
//             }
//         }
//         count
//     }
// }

// Bitset scans sol'n
impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as u8;
        let mut connections = vec![0u64; n as usize];
        for edge in edges {
            connections[edge[0] as usize] |= 1 << edge[1];
            connections[edge[1] as usize] |= 1 << edge[0];
        }
        let mut buckets = std::collections::HashMap::new();
        for node in 0..n {
            buckets
                .entry(connections[node as usize] | 1 << node)
                .or_insert(vec![])
                .push(node);
        }
        let mut count = 0;
        for (expected, actual) in buckets {
            if expected.count_ones() == actual.len() as u32 {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u8, edges: &[[u8; 2]], expected: i32) {
        assert!(n >= 1);
        assert!(n <= 50);
        assert!(edges.len() <= (n as u16 * (n as u16 - 1) / 2) as usize);
        let mut seen = std::collections::HashSet::new();
        for &[a, b] in edges {
            assert!(a < n);
            assert!(b < n);
            assert_ne!(a, b);
            assert!(seen.insert([a, b]));
        }
        assert_eq!(
            Solution::count_complete_components(
                n as i32,
                edges
                    .iter()
                    .map(|&[a, b]| vec![a as i32, b as i32])
                    .collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(6, &[[0, 1], [0, 2], [1, 2], [3, 4]], 3)
    }

    #[test]
    fn ex2() {
        test(6, &[[0, 1], [0, 2], [1, 2], [3, 4], [3, 5]], 1)
    }

    #[test]
    fn discussion_case1() {
        test(4, &[[1, 0], [2, 0], [3, 1], [3, 2]], 0)
    }

    #[test]
    fn discussion_case2() {
        test(2, &[[1, 0]], 1)
    }
}
