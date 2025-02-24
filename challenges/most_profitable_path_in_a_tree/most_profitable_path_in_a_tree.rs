// https://leetcode.com/problems/most-profitable-path-in-a-tree/

pub struct Solution;

// Initial sol'n (Correct but the DFS will stack overflow)
// impl Solution {
//     pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
//         // Step 1: Build an adjacency list representation of the tree
//         let n = edges.len() as u32 + 1;
//         let mut adj_list = vec![vec![]; n as usize];
//         for edge in edges {
//             let ai = edge[0];
//             let bi = edge[1];
//             adj_list[ai as usize].push(bi);
//             adj_list[bi as usize].push(ai);
//         }
//         // Step 2: Find the times Bob visits each node by DFS
//         fn bob_dfs(
//             adj_list: &[Vec<i32>],
//             bob: i32,
//             visited: &mut [bool],
//             times: &mut std::collections::HashMap<i32, u32>,
//             current_time: u32,
//         ) -> bool {
//             if visited[bob as usize] {
//                 return false;
//             }
//             visited[bob as usize] = true;
//             if bob == 0 {
//                 times.insert(0, current_time);
//                 return true;
//             }
//             for &child in &adj_list[bob as usize] {
//                 if bob_dfs(adj_list, child, visited, times, current_time + 1) {
//                     times.insert(bob, current_time);
//                     return true;
//                 }
//             }
//             false
//         }
//         let mut visited = vec![false; n as usize];
//         let mut times = std::collections::HashMap::new();
//         assert!(bob_dfs(&adj_list, bob, &mut visited, &mut times, 0));
//         // Step 3: BFS the tree to find the maximum profit path
//         // based on the times Bob visits each node.
//         // Remember, Alice cannot stop at any node but a leaf,
//         // so find the maximum profit only over leaf nodes.
//         visited.fill(false);
//         let mut queue = std::collections::VecDeque::new();
//         queue.push_back((0, 0, 0)); // (node, current profit, current time)
//         let mut max_profit = i32::MIN;
//         while let Some((node, last_profit, current_time)) = queue.pop_front() {
//             if visited[node as usize] {
//                 continue;
//             }
//             let reward = {
//                 if let Some(&time_bob_visits) = times.get(&node) {
//                     if time_bob_visits < current_time {
//                         0
//                     } else if time_bob_visits == current_time {
//                         amount[node as usize] / 2
//                     } else {
//                         amount[node as usize]
//                     }
//                 } else {
//                     amount[node as usize]
//                 }
//             };
//             let current_profit = last_profit + reward;
//             visited[node as usize] = true;
//             if adj_list[node as usize].len() <= 1 && node != 0 {
//                 if max_profit < current_profit {
//                     max_profit = current_profit;
//                 }
//                 continue;
//             }
//             for &child in &adj_list[node as usize] {
//                 queue.push_back((child, current_profit, current_time + 1));
//             }
//         }
//         max_profit
//     }
// }

// Initial sol'n with heap stack DFS
impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        // Step 1: Build an adjacency list representation of the tree
        let n = edges.len() as u32 + 1;
        let mut adj_list = vec![vec![]; n as usize];
        for edge in edges {
            let ai = edge[0];
            let bi = edge[1];
            adj_list[ai as usize].push(bi);
            adj_list[bi as usize].push(ai);
        }
        // Step 2: Find the times Bob visits each node by DFS
        let mut visited = vec![false; n as usize];
        // Stack contents: (node, current_time, next_child_to_visit)
        let mut stack = vec![(bob, 0, 0)];
        'outer: while let Some((node, current_time, mut next_child_to_visit)) = stack.pop() {
            visited[node as usize] = true;
            if node == 0 {
                break; // Current stack contents is the path from root to bob
            }
            while next_child_to_visit < adj_list[node as usize].len() {
                let child = adj_list[node as usize][next_child_to_visit as usize];
                if !visited[child as usize] {
                    stack.push((node, current_time, next_child_to_visit + 1));
                    stack.push((child, current_time + 1, 0));
                    continue 'outer;
                }
                next_child_to_visit += 1;
            }
        }
        let times = stack
            .into_iter()
            .map(|(node, current_time, _)| (node, current_time))
            .collect::<std::collections::HashMap<i32, u32>>();
        // Step 3: BFS the tree to find the maximum profit path
        // based on the times Bob visits each node.
        // Remember, Alice cannot stop at any node but a leaf,
        // so find the maximum profit only over leaf nodes.
        visited.fill(false);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0, 0)); // (node, current profit, current time)
        let mut max_profit = i32::MIN;
        while let Some((node, last_profit, current_time)) = queue.pop_front() {
            if visited[node as usize] {
                continue;
            }
            let reward = {
                if let Some(&time_bob_visits) = times.get(&node) {
                    if time_bob_visits < current_time {
                        0
                    } else if time_bob_visits == current_time {
                        amount[node as usize] / 2
                    } else {
                        amount[node as usize]
                    }
                } else {
                    amount[node as usize]
                }
            };
            let current_profit = last_profit + reward;
            visited[node as usize] = true;
            if adj_list[node as usize].len() <= 1 && node != 0 {
                if max_profit < current_profit {
                    max_profit = current_profit;
                }
                continue;
            }
            for &child in &adj_list[node as usize] {
                queue.push_back((child, current_profit, current_time + 1));
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(edges: &[[i32; 2]], bob: i32, amount: &[i16], expected: i32) {
        let n = edges.len() as u32 + 1;
        assert!(n >= 2);
        assert!(n <= 100_000);
        assert!(bob >= 0);
        assert!(bob < n as i32);
        assert_eq!(amount.len(), n as usize);
        for &[ai, bi] in edges {
            assert!(ai >= 0);
            assert!(ai < n as i32);
            assert!(bi >= 0);
            assert!(bi < n as i32);
            assert_ne!(ai, bi);
        }
        assert_eq!(
            Solution::most_profitable_path(
                edges.iter().map(|&[ai, bi]| vec![ai, bi]).collect(),
                bob,
                amount.iter().map(|&x| x as i32).collect()
            ),
            expected
        )
    }

    #[test]
    fn ex1() {
        test(&[[0, 1], [1, 2], [1, 3], [3, 4]], 3, &[-2, 4, 2, -4, 6], 6)
    }

    #[test]
    fn ex2() {
        test(&[[0, 1]], 1, &[-7280, 2350], -7280)
    }

    #[test]
    fn myex1() {
        test(&[[0, 1], [1, 2]], 2, &[-1000, -1000, -1000], -1500)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                [0, 21],
                [0, 6],
                [0, 29],
                [1, 3],
                [1, 38],
                [2, 32],
                [2, 34],
                [2, 27],
                [3, 24],
                [3, 8],
                [4, 5],
                [4, 21],
                [5, 11],
                [5, 28],
                [6, 27],
                [7, 23],
                [7, 21],
                [8, 12],
                [8, 22],
                [8, 36],
                [9, 10],
                [10, 17],
                [12, 15],
                [13, 24],
                [14, 29],
                [16, 25],
                [16, 35],
                [16, 39],
                [17, 19],
                [17, 39],
                [18, 37],
                [18, 26],
                [20, 27],
                [26, 38],
                [26, 30],
                [29, 30],
                [29, 39],
                [30, 31],
                [33, 34],
            ],
            31,
            &[
                562, 5200, 8954, -1176, 3208, -140, 940, 9548, -662, -4974, -9054, -5868, -3888,
                404, -5184, 418, 3890, -9434, -8184, 642, -5484, -4542, -372, -7818, -268, 4512,
                -2648, -9016, 8782, 542, -8812, -7262, -9804, 6622, -7030, 8164, 8354, -8176, 5412,
                -5648,
            ],
            15584,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[[0, 1], [1, 2], [2, 3]],
            3,
            &[-5644, -6018, 1188, -8502],
            -11662,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        // Generate a test with an extremely deep tree to make sure
        // we don't have stack overflow issues.
        let mut edges = Vec::new();
        let mut amount = Vec::new();
        amount.push(0);
        for i in 1..100_000 {
            edges.push([(i - 1) as i32, i as i32]);
            amount.push(10_000);
        }
        test(&edges, 99_999, &amount, 10_000 * 49_999);
    }
}
