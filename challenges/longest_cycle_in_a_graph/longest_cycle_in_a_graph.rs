// https://leetcode.com/problems/longest-cycle-in-a-graph/

pub struct Solution;

// Initial sol'n (naive, n^2)
// impl Solution {
//     pub fn longest_cycle(edges: Vec<i32>) -> i32 {
//         let mut max_cycle = -1;
//         for i in 0..edges.len() as i32 {
//             let mut cycle = 0;
//             let mut j = i as i32;
//             let mut visited = vec![false; edges.len()];
//             while j >= 0 && !visited[j as usize] {
//                 visited[j as usize] = true;
//                 j = edges[j as usize];
//                 cycle += 1;
//             }
//             if j == i as i32 {
//                 dbg!(i, cycle);
//                 max_cycle = std::cmp::max(max_cycle, cycle);
//             }
//         }
//         max_cycle
//     }
// }

// Hashset sol'n (MUCH faster)
impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        // Once we land on the longest cycle, we don't care where on it we are
        // because all nodes have at most 1 outgoing edge.
        // Thus, we can skip checking any node we've checked before,
        // so long as that node was in any cycle.
        let mut checked = std::collections::HashSet::with_capacity(edges.len());
        let mut max_cycle = -1;
        for i in 0..edges.len() as i32 {
            if checked.contains(&i) {
                continue;
            }
            let mut cycle = 0;
            let mut j = i as i32;
            // This hashset will figure itself out, since we don't know
            // we need every node every time.
            let mut visited = std::collections::HashSet::new();
            while j >= 0 && !visited.contains(&j) {
                if checked.contains(&j) {
                    break;
                }
                visited.insert(j);
                j = edges[j as usize];
                cycle += 1;
            }
            if j == i as i32 {
                max_cycle = std::cmp::max(max_cycle, cycle);
                checked.extend(visited);
            }
        }
        max_cycle
    }
}

// Optimized inner loop (or not. It's slower)
// impl Solution {
//     pub fn longest_cycle(edges: Vec<i32>) -> i32 {
//         // Once we land on the longest cycle, we don't care where on it we are
//         // because all nodes have at most 1 outgoing edge.
//         // Thus, we can skip checking any node we've checked before,
//         // so long as that node was in any cycle.
//         let mut checked = std::collections::HashSet::with_capacity(edges.len());
//         let mut max_cycle = -1;
//         for i in 0..edges.len() as i32 {
//             if checked.contains(&i) {
//                 continue;
//             }
//             let mut cycle = 0;
//             let mut j = i as i32;
//             // This hashset will figure itself out, since we don't know
//             // we need every node every time.
//             let mut visited = std::collections::HashSet::new();
//             if {
//                 loop {
//                     if j < 0 {
//                         break false;
//                     }
//                     if visited.contains(&j) {
//                         break j == i;
//                     }
//                     visited.insert(j);
//                     j = edges[j as usize];
//                     cycle += 1;
//                     if checked.contains(&j) {
//                         break false;
//                     }
//                 }
//             } {
//                 max_cycle = std::cmp::max(max_cycle, cycle);
//                 checked.extend(visited);
//             }
//         }
//         max_cycle
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let edges = vec![3, 3, 4, 2, 3];
        assert_eq!(Solution::longest_cycle(edges), 3);
    }

    #[test]
    fn ex2() {
        let edges = vec![2, -1, 3, 1];
        assert_eq!(Solution::longest_cycle(edges), -1);
    }

    #[test]
    fn discussion_case1() {
        let edges = vec![
            1, 19, 30, 87, 53, 91, 36, 6, 95, 14, 73, 2, 59, 76, 49, 41, 29, 28, 8, 9, 96, 80, 68,
            10, 31, 24, 0, 42, 39, 4, 51, 64, 25, 90, 35, 71, 97, 32, 16, 18, 62, 22, 40, 78, 55,
            13, 99, 93, 66, 26, 98, 5, 88, 74, 89, 81, 43, 12, 44, 57, 75, 47, 34, 72, 85, 77, 3,
            65, 46, 20, 60, 33, 48, 94, 84, 21, 69, 54, 56, 11, 70, 83, 86, 79, 61, 37, 67, 15, 7,
            38, 23, 52, 58, 27, 50, 63, 92, 45, 17, 82,
        ];
        assert_eq!(Solution::longest_cycle(edges), 50);
    }

    #[test]
    fn discussion_case2() {
        // 0 is a leaf
        // 1 goes to 4
        // 2 is a leaf
        // 3 goes to 2
        // 4 goes to 0
        // 5 goes to 4
        let edges = vec![-1, 4, -1, 2, 0, 4];
        assert_eq!(Solution::longest_cycle(edges), -1);
    }

    #[test]
    fn discussion_case3() {
        // 0 goes to 1
        // 1 goes to 2
        let edges = vec![1, 0];
        assert_eq!(Solution::longest_cycle(edges), 2);
    }

    #[test]
    fn myex1() {
        // 0 goes to 1
        // 1 goes to 2
        // 2 goes to 0
        let edges = vec![1, 2, 0];
        assert_eq!(Solution::longest_cycle(edges), 3);
    }

    #[test]
    fn time_limit_exceeded_case1() {
        let input = include_str!("tle_case_1.txt");
        let edges: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(Solution::longest_cycle(edges), 43242);
    }

    #[test]
    fn my_extreme_example1() {
        let mut edges: Vec<i32> = (1..=100_000).collect();
        *edges.last_mut().unwrap() = 0;
        assert_eq!(Solution::longest_cycle(edges), 100_000);
    }
}
