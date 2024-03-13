// https://leetcode.com/problems/is-graph-bipartite/

pub struct Solution;

// Initial sol'n (0ms)
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        assert!(graph.len() >= 1);
        assert!(graph.len() <= 100);
        // node_color is a map of node to color. We need a 2-color graph, so
        // the color is a boolean.
        // If a node isn't present, it has yet to be visited.
        let mut node_color = std::collections::HashMap::<u8, bool>::with_capacity(graph.len());
        // to_visit is the set of nodes to visit.
        // If a node is inserted twice with different colors, we already know
        // we have a contradiction.
        // Should only contain nodes not in node_color.
        let mut to_visit = std::collections::BTreeMap::<u8, bool>::new();
        to_visit.insert(0, true);
        'seeder: loop {
            while let Some((node, color)) = to_visit.pop_last() {
                if let Some(&c) = node_color.get(&node) {
                    if c != color {
                        break 'seeder false;
                    }
                    continue;
                }
                node_color.insert(node, color);
                let new_color = !color;
                for &neighbor in &graph[node as usize] {
                    assert!(neighbor >= 0);
                    assert!(neighbor <= 100);
                    let neighbor = neighbor as u8;
                    if let Some(&c) = node_color.get(&neighbor) {
                        if c == color {
                            break 'seeder false;
                        }
                    } else {
                        to_visit.insert(neighbor, new_color);
                    }
                }
            }
            if node_color.len() == graph.len() {
                break 'seeder true;
            }
            let seed = (0..graph.len() as u8)
                .find(|&n| !node_color.contains_key(&n))
                .unwrap();
            to_visit.insert(seed, true);
        }
    }
}

// Vector sol'n (Slow, 3ms)
// impl Solution {
//     pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
//         assert!(graph.len() >= 1);
//         assert!(graph.len() <= 100);
//         type MaybeColor = Option<bool>;
//         let mut node_color: Vec<MaybeColor> = vec![None; graph.len()];
//         let mut colored = 0;
//         let mut to_visit = std::collections::BTreeMap::<usize, bool>::new();
//         to_visit.insert(0, true);
//         'seeder: loop {
//             while let Some((node, color)) = to_visit.pop_last() {
//                 if let Some(c) = node_color[node] {
//                     if c != color {
//                         break 'seeder false;
//                     }
//                     continue;
//                 }
//                 node_color[node] = Some(color);
//                 colored += 1;
//                 let new_color = !color;
//                 for &neighbor in &graph[node] {
//                     let neighbor = neighbor as usize;
//                     if let Some(c) = node_color[neighbor] {
//                         if c == color {
//                             break 'seeder false;
//                         }
//                     } else {
//                         to_visit.insert(neighbor, new_color);
//                     }
//                 }
//             }
//             if colored == graph.len() {
//                 break 'seeder true;
//             }
//             let seed = node_color.iter().position(|mc| mc.is_none()).unwrap();
//             to_visit.insert(seed, true);
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        assert_eq!(Solution::is_bipartite(graph), false);
    }

    #[test]
    fn ex2() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert_eq!(Solution::is_bipartite(graph), true);
    }

    #[test]
    fn discussion_case1() {
        let graph = vec![vec![4], vec![], vec![4], vec![4], vec![0, 2, 3]];
        assert_eq!(Solution::is_bipartite(graph), true);
    }

    #[test]
    fn discussion_case2() {
        let graph = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        assert_eq!(Solution::is_bipartite(graph), false);
    }

    #[test]
    fn my_extreme_disconnected_graph() {
        let graph = std::iter::repeat(vec![]).take(100).collect();
        assert_eq!(Solution::is_bipartite(graph), true);
    }
}
