// https://leetcode.com/problems/valid-arrangement-of-pairs/

pub struct Solution;

// Hierholzer’s algorithm on a directed graph
// impl Solution {
//     pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         // Given that no two pairs are the same, so no two (directed)
//         // edges in the graph are the same.
//         // We're also guaranteed that a valid arrangement exists, so we know
//         // an eulerian path exists.
//         let mut incomers = std::collections::HashMap::new();
//         let mut outgoers = std::collections::HashMap::new();
//         for pair in &pairs {
//             incomers.entry(pair[1]).or_insert(vec![]).push(pair[0]);
//             outgoers.entry(pair[0]).or_insert(vec![]).push(pair[1]);
//         }
//         // Now we have a directed graph with in-degree and out-degree for each node.
//         // We need to find a path that visits each edge exactly once.
//         // We can start from any node with odd out-degree (unless one exists
//         // with 1 greater out-degree than in-degree, in which case we must
//         // start from it).
//         let start_node = {
//             let mut start_node = None;
//             for (node, outgoers) in &outgoers {
//                 if outgoers.len() == incomers.get(node).map(std::vec::Vec::len).unwrap_or(0) + 1 {
//                     start_node = Some(*node);
//                     break;
//                 }
//                 start_node = Some(*node);
//             }
//             start_node.unwrap()
//         };
//         let mut path = vec![];
//         let mut stack = vec![start_node];
//         loop {
//             let Some(node) = stack.pop() else { break };
//             if let Some(outgoer) = outgoers.get_mut(&node).and_then(|x| x.pop()) {
//                 stack.push(node);
//                 stack.push(outgoer);
//             } else {
//                 path.push(node);
//             }
//         }
//         path.reverse();
//         path.windows(2).map(|x| x.to_vec()).collect()
//     }
// }

// Indegree count only
impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Given that no two pairs are the same, so no two (directed)
        // edges in the graph are the same.
        // We're also guaranteed that a valid arrangement exists, so we know
        // an eulerian path exists.
        let mut incomers = std::collections::HashMap::new();
        let mut outgoers = std::collections::HashMap::new();
        for pair in &pairs {
            *incomers.entry(pair[1]).or_insert(0) += 1;
            outgoers.entry(pair[0]).or_insert(vec![]).push(pair[1]);
        }
        // Now we have a directed graph with in-degree and out-degree for each node.
        // We need to find a path that visits each edge exactly once.
        // We can start from any node with odd out-degree (unless one exists
        // with 1 greater out-degree than in-degree, in which case we must
        // start from it).
        let start_node = {
            let mut start_node = None;
            for (node, outgoers) in &outgoers {
                if outgoers.len() == incomers.get(node).copied().unwrap_or(0) + 1 {
                    start_node = Some(*node);
                    break;
                }
                start_node = Some(*node);
            }
            start_node.unwrap()
        };
        let mut path = vec![];
        let mut stack = vec![start_node];
        loop {
            let Some(node) = stack.pop() else { break };
            if let Some(outgoer) = outgoers.get_mut(&node).and_then(|x| x.pop()) {
                stack.push(node);
                stack.push(outgoer);
            } else {
                path.push(node);
            }
        }
        path.reverse();
        path.windows(2).map(|x| x.to_vec()).collect()
    }
}

// Adjust incomers counts to be u32 not usize
// impl Solution {
//     pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         // Given that no two pairs are the same, so no two (directed)
//         // edges in the graph are the same.
//         // We're also guaranteed that a valid arrangement exists, so we know
//         // an eulerian path exists.
//         let mut incomers = std::collections::HashMap::new();
//         let mut outgoers = std::collections::HashMap::new();
//         for pair in &pairs {
//             *incomers.entry(pair[1]).or_insert(0) += 1u32;
//             outgoers.entry(pair[0]).or_insert(vec![]).push(pair[1]);
//         }
//         // Now we have a directed graph with in-degree and out-degree for each node.
//         // We need to find a path that visits each edge exactly once.
//         // We can start from any node with odd out-degree (unless one exists
//         // with 1 greater out-degree than in-degree, in which case we must
//         // start from it).
//         let start_node = {
//             let mut start_node = None;
//             for (node, outgoers) in &outgoers {
//                 if outgoers.len() as u32 == incomers.get(node).copied().unwrap_or(0) + 1 {
//                     start_node = Some(*node);
//                     break;
//                 }
//                 start_node = Some(*node);
//             }
//             start_node.unwrap()
//         };
//         let mut path = vec![];
//         let mut stack = vec![start_node];
//         loop {
//             let Some(node) = stack.pop() else { break };
//             if let Some(outgoer) = outgoers.get_mut(&node).and_then(|x| x.pop()) {
//                 stack.push(node);
//                 stack.push(outgoer);
//             } else {
//                 path.push(node);
//             }
//         }
//         path.reverse();
//         path.windows(2).map(|x| x.to_vec()).collect()
//     }
// }

// Remove vector reversal by swapping incomers and outgoers
// impl Solution {
//     pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         // Given that no two pairs are the same, so no two (directed)
//         // edges in the graph are the same.
//         // We're also guaranteed that a valid arrangement exists, so we know
//         // an eulerian path exists.
//         let mut incomers = std::collections::HashMap::new();
//         let mut outgoers = std::collections::HashMap::new();
//         for pair in &pairs {
//             *incomers.entry(pair[0]).or_insert(0) += 1u32;
//             outgoers.entry(pair[1]).or_insert(vec![]).push(pair[0]);
//         }
//         // Now we have a directed graph with in-degree and out-degree for each node.
//         // We need to find a path that visits each edge exactly once.
//         // We can start from any node with odd out-degree (unless one exists
//         // with 1 greater out-degree than in-degree, in which case we must
//         // start from it).
//         let start_node = {
//             let mut start_node = None;
//             for (node, outgoers) in &outgoers {
//                 if outgoers.len() as u32 == incomers.get(node).copied().unwrap_or(0) + 1 {
//                     start_node = Some(*node);
//                     break;
//                 }
//                 start_node = Some(*node);
//             }
//             start_node.unwrap()
//         };
//         let mut path = vec![];
//         let mut stack = vec![start_node];
//         loop {
//             let Some(node) = stack.pop() else { break };
//             if let Some(outgoer) = outgoers.get_mut(&node).and_then(|x| x.pop()) {
//                 stack.push(node);
//                 stack.push(outgoer);
//             } else {
//                 path.push(node);
//             }
//         }
//         path.windows(2).map(|x| x.to_vec()).collect()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn check_input(pairs: &[[i32; 2]]) {
        assert!(pairs.len() >= 1);
        assert!(pairs.len() <= 100_000);
        let mut seen = std::collections::HashSet::new();
        for &pair in pairs {
            for num in pair {
                assert!(num >= 0);
                assert!(num <= 1_000_000_000);
            }
            assert_ne!(pair[0], pair[1]);
            assert!(seen.insert(pair));
        }
    }

    fn test(pairs: &[[i32; 2]], expected: &[[i32; 2]]) {
        assert_eq!(pairs.len(), expected.len());
        check_input(pairs);
        assert_eq!(
            Solution::valid_arrangement(pairs.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    fn property_test(pairs: &[[i32; 2]]) {
        check_input(pairs);
        let result = Solution::valid_arrangement(pairs.iter().map(|&x| x.to_vec()).collect());
        let mut result_sorted = result.clone();
        let mut pairs_sorted = pairs.to_vec();
        result_sorted.sort_unstable();
        pairs_sorted.sort_unstable();
        assert_eq!(result_sorted, pairs_sorted);
        for i in 1..pairs.len() {
            assert_eq!(result[i - 1][1], result[i][0]);
        }
    }

    #[test]
    fn ex1() {
        test(
            &[[5, 1], [4, 5], [11, 9], [9, 4]],
            &[[11, 9], [9, 4], [4, 5], [5, 1]],
        )
    }

    #[test]
    fn ex2() {
        property_test(&[[1, 3], [3, 2], [2, 1]])
    }

    #[test]
    fn ex3() {
        test(&[[1, 2], [1, 3], [2, 1]], &[[1, 2], [2, 1], [1, 3]])
    }

    #[test]
    fn discussion_case1() {
        property_test(&[[1, 2], [1, 3], [2, 1]])
    }

    #[test]
    fn discussion_case2() {
        property_test(&[[299, 1], [1, 2], [1, 3], [2, 1], [3, 1]])
    }

    #[test]
    fn discussion_case3() {
        const IN: &[[i32; 2]] = &[
            [1, 3],
            [3, 9],
            [9, 4],
            [4, 1],
            [1, 4],
            [4, 6],
            [6, 3],
            [3, 1],
            [1, 456],
        ];
        property_test(IN)
    }

    #[test]
    fn discussion_case4() {
        property_test(&[
            [299, 1],
            [1, 2],
            [1, 3],
            [3, 5],
            [5, 8],
            [8, 3],
            [3, 7],
            [7, 6],
            [6, 9],
            [9, 3],
            [3, 6],
            [6, 7],
            [7, 3],
            [3, 8],
            [8, 5],
            [5, 3],
            [3, 1],
            [2, 1],
        ])
    }
}
