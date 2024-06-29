// https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/

pub struct Solution;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        assert!(n >= 1);
        assert!(n <= 1000);
        assert!(edges.len() <= std::cmp::min(2000, (n as usize) * (n as usize - 1) / 2));
        fn dfs<'a>(node: u16, graph: &Vec<Vec<i32>>, ancestors: &'a mut Vec<Vec<i32>>) -> &'a [i32] {
            if ancestors[node as usize].is_empty() {
                let mut node_ancestors: Vec<i32> = vec![];
                for &parent in graph[node as usize].iter() {
                    let slice = dfs(parent as u16, graph, ancestors);
                    node_ancestors.extend_from_slice(slice);
                    node_ancestors.push(parent);
                }
                node_ancestors.sort_unstable();
                node_ancestors.dedup();
                ancestors[node as usize] = node_ancestors;
            }
            &ancestors[node as usize]
        }
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[1] as usize].push(edge[0]);
        }
        let mut ancestors = vec![vec![]; n as usize];
        for i in 0..n as u16 {
            dfs(i, &graph, &mut ancestors);
        }
        ancestors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, edges: &[[i32; 2]], expected: &[&[i32]]) {
        let edges = edges.iter().map(|e| e.to_vec()).collect();
        assert_eq!(Solution::get_ancestors(n, edges), expected);
    }

    #[test]
    fn ex1() {
        test(
            8,
            &[
                [0, 3],
                [0, 4],
                [1, 3],
                [2, 4],
                [2, 7],
                [3, 5],
                [3, 6],
                [3, 7],
                [4, 6],
            ],
            &[
                &[],
                &[],
                &[],
                &[0, 1],
                &[0, 2],
                &[0, 1, 3],
                &[0, 1, 2, 3, 4],
                &[0, 1, 2, 3],
            ],
        )
    }

    #[test]
    fn ex2() {
        test(
            5,
            &[
                [0, 1],
                [0, 2],
                [0, 3],
                [0, 4],
                [1, 2],
                [1, 3],
                [1, 4],
                [2, 3],
                [2, 4],
                [3, 4],
            ],
            &[&[], &[0], &[0, 1], &[0, 1, 2], &[0, 1, 2, 3]],
        )
    }
}
