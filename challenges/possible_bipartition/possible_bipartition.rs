// https://leetcode.com/problems/possible-bipartition/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
//         assert!(n >= 1);
//         assert!(n <= 2000);
//         assert!(dislikes.len() <= 10_000);
//         if dislikes.len() as i32 > n * n / 4 + 1 {
//             return false;
//         }
//         let n = n as u16;
//         let mut graph = vec![vec![]; n as usize];
//         for dislike in dislikes {
//             let a = dislike[0] as usize - 1;
//             let b = dislike[1] as usize - 1;
//             graph[a].push(b);
//             graph[b].push(a);
//         }
//         let mut node_color = std::collections::HashMap::<u16, bool>::with_capacity(graph.len());
//         let mut to_visit = std::collections::BTreeMap::<u16, bool>::new();
//         to_visit.insert(0, true);
//         'seeder: loop {
//             while let Some((node, color)) = to_visit.pop_last() {
//                 if let Some(&c) = node_color.get(&node) {
//                     if c != color {
//                         break 'seeder false;
//                     }
//                     continue;
//                 }
//                 node_color.insert(node, color);
//                 let new_color = !color;
//                 for &neighbor in &graph[node as usize] {
//                     let neighbor = neighbor as u16;
//                     if let Some(&c) = node_color.get(&neighbor) {
//                         if c == color {
//                             break 'seeder false;
//                         }
//                     } else {
//                         to_visit.insert(neighbor, new_color);
//                     }
//                 }
//             }
//             if node_color.len() == graph.len() {
//                 break 'seeder true;
//             }
//             let seed = (0..graph.len() as u16)
//                 .find(|&n| !node_color.contains_key(&n))
//                 .unwrap();
//             to_visit.insert(seed, true);
//         }
//     }
// }

// Simpler DFS
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        assert!(n >= 1);
        assert!(n <= 2000);
        assert!(dislikes.len() <= 10_000);
        if dislikes.len() as i32 > n * n / 4 + 1 {
            return false;
        }
        let n = n as u32;
        let mut graph: Vec<Vec<u32>> = vec![vec![]; n as usize + 1];
        for dislike in dislikes {
            let a = dislike[0] as u32;
            let b = dislike[1] as u32;
            graph[a as usize].push(b);
            graph[b as usize].push(a);
        }
        let mut node_color = vec![0u8; n as usize + 1];
        let mut to_visit = std::vec::Vec::<u32>::with_capacity(n as usize + 1);
        let mut seed = 1;
        'seeder: loop {
            if seed > n {
                break 'seeder true;
            }
            if node_color[seed as usize] != 0 {
                seed += 1;
                continue 'seeder;
            }
            to_visit.push(seed);
            node_color[seed as usize] = 1;
            while let Some(node) = to_visit.pop() {
                let color = node_color[node as usize];
                for &neighbor in &graph[node as usize] {
                    if node_color[neighbor as usize] == color {
                        break 'seeder false;
                    } else if node_color[neighbor as usize] == 0 {
                        node_color[neighbor as usize] = 3 ^ color;
                        to_visit.push(neighbor);
                    }
                }
            }
        }
    }
}

// Ripping out all the fancy
// impl Solution {
//     pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
//         assert!(n >= 1);
//         assert!(n <= 2000);
//         assert!(dislikes.len() <= 10_000);
//         if dislikes.len() as i32 > n * n / 4 + 1 {
//             return false;
//         }
//         let mut graph = vec![vec![]; n as usize + 1];
//         for dislike in dislikes {
//             let a = dislike[0] as i32;
//             let b = dislike[1] as i32;
//             graph[a as usize].push(b);
//             graph[b as usize].push(a);
//         }
//         let mut node_color = vec![0u8; n as usize + 1];
//         let mut to_visit = std::vec::Vec::with_capacity(n as usize + 1);
//         for seed in 1..=n {
//             if node_color[seed as usize] != 0 {
//                 continue;
//             }
//             to_visit.push(seed);
//             while let Some(node) = to_visit.pop() {
//                 let color = node_color[node as usize];
//                 let new_color = 3 ^ color;
//                 for &neighbor in &graph[node as usize] {
//                     if node_color[neighbor as usize] == 0 {
//                         node_color[neighbor as usize] = new_color;
//                         to_visit.push(neighbor);
//                     } else if node_color[neighbor as usize] != new_color {
//                         return false;
//                     }
//                 }
//             }
//         }
//         true
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let n = 4;
        let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 4]];
        assert_eq!(Solution::possible_bipartition(n, dislikes), true);
    }

    #[test]
    fn ex2() {
        let n = 3;
        let dislikes = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        assert_eq!(Solution::possible_bipartition(n, dislikes), false);
    }

    #[test]
    fn discussion_case1() {
        let n = 4;
        let dislikes = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![2, 4]];
        assert_eq!(Solution::possible_bipartition(n, dislikes), false);
    }

    #[test]
    fn my_extreme_connected_graph() {
        let n = 2000;
        let dislikes: Vec<Vec<i32>> = (1..=n)
            .map(|i| (i..=n).map(move |j| vec![i, j]))
            .flatten()
            .take(10_000)
            .collect();
        assert_eq!(Solution::possible_bipartition(n, dislikes), false);
    }

    #[test]
    fn failed_case1() {
        let n = 10;
        let dislikes: Vec<Vec<i32>> = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
            vec![6, 7],
            vec![8, 9],
            vec![7, 8],
        ];
        assert_eq!(Solution::possible_bipartition(n, dislikes), true);
    }
}
