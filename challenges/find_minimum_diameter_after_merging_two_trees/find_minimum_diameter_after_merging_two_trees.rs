// https://leetcode.com/problems/find-minimum-diameter-after-merging-two-trees/

pub struct Solution;

// Initial sol'n (BFS)
impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        type AdjTable = Vec<Vec<u32>>;
        fn adj_table(edges: Vec<Vec<i32>>) -> AdjTable {
            let mut adj_table = vec![vec![]; edges.len() + 1];
            for edge in edges {
                let (a, b) = (edge[0] as u32, edge[1] as u32);
                adj_table[a as usize].push(b);
                adj_table[b as usize].push(a);
            }
            adj_table
        }
        fn max_dist_node(tree: &AdjTable, starting_node: u32) -> (u32, u32) {
            let mut nodes = vec![starting_node];
            let mut next_nodes = std::vec::Vec::new();
            let mut seen = vec![false; tree.len()];
            let mut depth = 0;
            loop {
                let last = nodes[0];
                for node in nodes.drain(..) {
                    seen[node as usize] = true;
                    next_nodes.extend(
                        tree[node as usize]
                            .iter()
                            .copied()
                            .filter(|&x| !seen[x as usize]),
                    );
                }
                std::mem::swap(&mut nodes, &mut next_nodes);
                if nodes.len() <= 0 {
                    break (last, depth);
                }
                depth += 1;
            }
        }
        fn tree_diameter(tree: &AdjTable) -> u32 {
            // Heuristic: Start from the node with the widest reach.
            let starting_node = tree
                .iter()
                .enumerate()
                .max_by_key(|(_, x)| x.len())
                .unwrap()
                .0 as u32;
            let (furthest_from_start, _) = max_dist_node(&tree, starting_node);
            let (_, distance) = max_dist_node(&tree, furthest_from_start);
            distance
        }
        let diameter1 = tree_diameter(&adj_table(edges1));
        let diameter2 = tree_diameter(&adj_table(edges2));
        std::cmp::max(
            std::cmp::max(diameter1, diameter2),
            (diameter1 + 1) / 2 + (diameter2 + 1) / 2 + 1,
        ) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e2in(edges: &[[u32; 2]]) -> Vec<Vec<i32>> {
        edges
            .iter()
            .map(|&[a, b]| vec![a as i32, b as i32])
            .collect()
    }

    fn test(edges1: &[[u32; 2]], edges2: &[[u32; 2]], expected: i32) {
        assert!(edges1.len() < 100_000);
        assert!(edges2.len() < 100_000);
        for &[a, b] in edges1 {
            assert!(a <= edges1.len() as u32);
            assert!(b <= edges1.len() as u32);
        }
        for &[a, b] in edges2 {
            assert!(a <= edges2.len() as u32);
            assert!(b <= edges2.len() as u32);
        }
        // Solution should work forward
        assert_eq!(
            Solution::minimum_diameter_after_merge(e2in(edges1), e2in(edges2)),
            expected
        );
        // Solution should work with all edge directions swapped
        assert_eq!(
            Solution::minimum_diameter_after_merge(
                edges1
                    .iter()
                    .map(|&[b, a]| vec![a as i32, b as i32])
                    .collect(),
                edges2
                    .iter()
                    .map(|&[b, a]| vec![a as i32, b as i32])
                    .collect(),
            ),
            expected
        );
        // Solution should work after edge sorting (by first el)
        let mut input1 = e2in(edges1);
        let mut input2 = e2in(edges2);
        input1.sort_unstable();
        input2.sort_unstable();
        assert_eq!(
            Solution::minimum_diameter_after_merge(input1.clone(), input2.clone()),
            expected
        );
        // Solution should work after edge sorting (by second el)
        input1.sort_unstable_by_key(|x| x[1]);
        input2.sort_unstable_by_key(|x| x[1]);
        assert_eq!(
            Solution::minimum_diameter_after_merge(input1.clone(), input2.clone()),
            expected
        );
        // Solution should work after edge sorting (by sum of edge nodes)
        input1.sort_unstable_by_key(|x| x[0] + x[1]);
        input2.sort_unstable_by_key(|x| x[0] + x[1]);
        assert_eq!(
            Solution::minimum_diameter_after_merge(input1.clone(), input2.clone()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[[0, 1], [0, 2], [0, 3]], &[[0, 1]], 3)
    }

    #[test]
    fn ex2() {
        test(
            &[[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]],
            &[[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]],
            5,
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                [0, 1],
                [2, 0],
                [3, 2],
                [3, 6],
                [8, 7],
                [4, 8],
                [5, 4],
                [3, 5],
                [3, 9],
            ],
            &[[0, 1], [0, 2], [0, 3]],
            7,
        )
    }

    #[test]
    fn discussion_case2() {
        test(&[[0, 1]], &[], 2)
    }

    #[test]
    fn discussion_case3() {
        test(&[[0, 1], [1, 2]], &[], 2)
    }

    #[test]
    fn discussion_case4() {
        test(&[], &[], 1)
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                [0, 1],
                [2, 0],
                [3, 2],
                [3, 6],
                [8, 7],
                [4, 8],
                [5, 4],
                [3, 5],
                [3, 9],
            ],
            &[[0, 1], [0, 2], [0, 3]],
            7,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        let mut edges = vec![[0, 0]; 99_999];
        for i in 0..edges.len() as u32 {
            edges[i as usize] = [i, i + 1];
        }
        test(&edges, &edges, 100_001)
    }
}
