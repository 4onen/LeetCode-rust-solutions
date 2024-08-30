// https://leetcode.com/problems/modify-graph-edge-weights/

pub struct Solution;

impl Solution {
    pub fn modified_graph_edges(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        const MAX_EDGE: i32 = 1_000_000_000;
        const MAX_MODIFIED_EDGE: i32 = 2 * MAX_EDGE;
        fn shortest_path(
            graph: &[Vec<u8>],
            weights: &[i32],
            dist: &mut [i32],
            parent: &mut [u8],
            heap: &mut std::collections::BinaryHeap<(std::cmp::Reverse<i32>, u8)>,
            source: u8,
            missing_weight: i32,
        ) {
            assert_eq!(graph.len(), dist.len());
            heap.push((std::cmp::Reverse(0), source));
            while let Some((std::cmp::Reverse(d), u)) = heap.pop() {
                if dist[u as usize] < d {
                    continue;
                }
                for &v in &graph[u as usize] {
                    let w = weights[u as usize * graph.len() + v as usize];
                    let w = if w < 0 { missing_weight } else { w };
                    if dist[v as usize] > d + w {
                        dist[v as usize] = d + w;
                        parent[v as usize] = u;
                        heap.push((std::cmp::Reverse(d + w), v));
                    }
                }
            }
        }
        assert!(n >= 1);
        assert!(n <= 100);
        let n = n as u8;
        assert!(edges.len() >= 1);
        assert!(source >= 0);
        assert!(source < n as i32);
        let source = source as u8;
        assert!(destination >= 0);
        assert!(destination < n as i32);
        let destination = destination as u8;
        let mut graph = vec![vec![]; n as usize];
        let mut weights = vec![0; n as usize * n as usize];
        for edge in edges.iter() {
            assert_eq!(edge.len(), 3);
            let u = edge[0] as u8;
            let v = edge[1] as u8;
            let w = edge[2];
            graph[u as usize].push(v);
            graph[v as usize].push(u);
            weights[u as usize * n as usize + v as usize] = w;
            weights[v as usize * n as usize + u as usize] = w;
        }
        let mut dist_dest = vec![i32::MAX; n as usize];
        dist_dest[destination as usize] = 0;
        let mut parent_dest = vec![0; n as usize];
        let mut heap = std::collections::BinaryHeap::new();
        shortest_path(
            &graph,
            &weights,
            &mut dist_dest,
            &mut parent_dest,
            &mut heap,
            destination as u8,
            target,
        );
        if dist_dest[source as usize] < target {
            return vec![];
        }
        if dist_dest[source as usize] == target {
            // Set every negative edge weight to target.
            // since that just worked.
            for edge in edges.iter_mut() {
                if edge[2] < 0 {
                    edge[2] = target;
                }
            }
            return edges;
        }
        // Max all missing edges didn't work. Try setting them to 1.
        // We can do this more efficiently by incrementing the previous calc
        // distance of each node by 1.
        let mut dist_src = vec![i32::MAX; n as usize];
        dist_src[source as usize] = 0;
        let mut parent_src = parent_dest;
        //parent_src.fill(u8::MAX);
        heap.clear();
        shortest_path(
            &graph,
            &weights,
            &mut dist_src,
            &mut parent_src,
            &mut heap,
            source as u8,
            1,
        );
        if dist_src[destination as usize] > target {
            return vec![];
        }
        if dist_src[destination as usize] == target {
            // Set every negative edge weight to 1.
            for edge in edges.iter_mut() {
                if edge[2] < 0 {
                    edge[2] = 1;
                }
            }
            return edges;
        }
        // We need to edit the shortest path through the graph.
        // To do this, let's list out the nodes on the shortest path
        // between the source and destination from dist_src
        let shortest_path_nodes = {
            let mut shortest_path_nodes = vec![destination];
            let mut u = destination;
            while u != source {
                assert!(
                    shortest_path_nodes.len() < n as usize,
                    "Graph is disconnected"
                );
                let next = parent_src[u as usize];
                shortest_path_nodes.push(next);
                u = next;
            }
            shortest_path_nodes.reverse();
            shortest_path_nodes
        };
        for i in 0..shortest_path_nodes.len() - 1 {
            let u = shortest_path_nodes[i];
            let v = shortest_path_nodes[i + 1];
            let w = weights[u as usize * n as usize + v as usize];
            if w > 0 {
                // We can't change this edge weight.
                continue;
            }
            // Since we have the distances both directions, with one direction
            // being disconnected at every negative edge such as this one, we
            // can find the shortest path between u and v and ensure we don't
            // mess things up with our modification.
            let dist_without_edge_weight = dist_src[u as usize] + dist_dest[v as usize];
            if dist_without_edge_weight < target {
                let w = target - dist_without_edge_weight;
                weights[u as usize * n as usize + v as usize] = w;
                weights[v as usize * n as usize + u as usize] = w;
                break;
            } else {
                weights[u as usize * n as usize + v as usize] = 1;
                weights[v as usize * n as usize + u as usize] = 1;
            }
        }
        for edge in edges.iter_mut() {
            if edge[2] < 0 {
                let w = weights[edge[0] as usize * n as usize + edge[1] as usize];
                if w < 0 {
                    edge[2] = MAX_MODIFIED_EDGE;
                } else {
                    edge[2] = w;
                }
            }
        }
        edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slice_edges_to_vec(edges: &[[i32; 3]]) -> Vec<Vec<i32>> {
        edges.iter().map(|e| e.to_vec()).collect()
    }

    fn shortest_path_within(
        n: u8,
        edges: Vec<Vec<i32>>,
        source: u8,
        destination: u8,
        target: i32,
    ) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[0] as usize].push((edge[1] as u8, edge[2]));
            graph[edge[1] as usize].push((edge[0] as u8, edge[2]));
        }
        let mut dist = vec![std::i32::MAX; n as usize];
        dist[source as usize] = 0;
        let mut heap = std::collections::BinaryHeap::new();
        heap.push((std::cmp::Reverse(0), source));
        while let Some((std::cmp::Reverse(d), u)) = heap.pop() {
            if u == destination || d > target {
                break;
            }
            if dist[u as usize] < d {
                continue;
            }
            for &(v, w) in &graph[u as usize] {
                if dist[v as usize] > d + w {
                    dist[v as usize] = d + w;
                    heap.push((std::cmp::Reverse(d + w), v));
                }
            }
        }
        // The graph is connected, so the destination is reachable.
        dist[destination as usize]
    }

    fn test_arg_checks(n: u8, edges: &[[i32; 3]], source: u8, destination: u8, target: u32) {
        assert!(n >= 1);
        assert!(n <= 100);
        assert!(edges.len() >= 1);
        assert!(edges.len() <= n as usize * (n as usize - 1) / 2);
        let mut seen_nodes = edges
            .iter()
            .flat_map(|e| e.iter().take(2))
            .collect::<Vec<_>>();
        seen_nodes.sort_unstable();
        seen_nodes.dedup();
        assert_eq!(seen_nodes.len(), n as usize);
        let mut seen_edges = std::collections::HashSet::new();
        for edge in edges {
            assert!(edge[0] >= 0);
            assert!(edge[0] < n as i32);
            assert!(edge[1] >= 0);
            assert!(edge[1] < n as i32);
            assert_ne!(edge[0], edge[1]);
            assert!(seen_edges.insert((edge[0], edge[1])));
            assert!(seen_edges.insert((edge[1], edge[0])));
            assert!(edge[2] != 0);
            assert!(edge[2] >= -1);
            assert!(edge[2] <= 10_000_000);
        }
        assert!(source < n as u8);
        assert!(destination < n as u8);
        assert_ne!(source, destination);
        assert!(target >= 1);
        assert!(target <= 1_000_000_000);
    }

    fn test(
        n: u8,
        edges: &[[i32; 3]],
        source: u8,
        destination: u8,
        target: u32,
        expect_solution: bool,
    ) {
        test_arg_checks(n, edges, source, destination, target);
        let res = Solution::modified_graph_edges(
            n as i32,
            slice_edges_to_vec(edges),
            source as i32,
            destination as i32,
            target as i32,
        );
        assert!(res.len() == 0 || res.len() == edges.len());
        for edge in &res {
            assert_eq!(edge.len(), 3);
            assert!(edge[0] >= 0);
            assert!(edge[0] < n as i32);
            assert!(edge[1] >= 0);
            assert!(edge[1] < n as i32);
            assert_ne!(edge[0], edge[1]);
            assert!(edge[2] >= 1);
            assert!(edge[2] <= 2_000_000_000);
        }
        if expect_solution {
            assert!(res.len() > 0);
            assert_eq!(res.len(), edges.len());
            let actual = shortest_path_within(n, res, source, destination, target as i32);
            assert_eq!(actual, target as i32);
        } else {
            assert_eq!(res.len(), 0);
        }
    }

    #[test]
    fn ex1() {
        test(
            5,
            &[[4, 1, -1], [2, 0, -1], [0, 3, -1], [4, 3, -1]],
            0,
            1,
            5,
            true,
        )
    }

    #[test]
    fn ex2() {
        test(3, &[[0, 1, -1], [0, 2, 5]], 0, 2, 6, false)
    }

    #[test]
    fn ex3() {
        test(
            4,
            &[[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, -1]],
            0,
            2,
            6,
            true,
        )
    }

    #[test]
    fn helpful_discussion_case1() {
        test(3, &[[0, 1, -1], [0, 2, 5]], 0, 2, 6, false)
    }

    #[test]
    fn helpful_discussion_case2() {
        test(3, &[[0, 1, -1], [0, 2, 6]], 0, 2, 6, true)
    }

    #[test]
    fn helpful_discussion_case3() {
        test(3, &[[0, 1, -1], [0, 2, -1]], 0, 2, 5, true)
    }

    #[test]
    fn discussion_case1() {
        test(
            4,
            &[[0, 1, -1], [0, 2, -1], [2, 3, -1], [3, 1, -1]],
            0,
            1,
            3,
            true,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            4,
            &[[0, 1, -1], [0, 2, -1], [2, 3, -1], [3, 1, -1]],
            3,
            1,
            3,
            true,
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            4,
            &[[0, 1, 5], [0, 2, -1], [2, 3, -1], [3, 1, 4]],
            0,
            1,
            6,
            false,
        )
    }

    #[test]
    fn discussion_case4() {
        // You start on node 1 and go to node 3. Your options for shortest paths are:
        // ```
        // 1 -> 2 -> 3 (cost 10+6 = 16)
        // 1 -> 0 -> 2 -> 3 (cost X+2+6 = 8+X)
        // 1 -> 0 -> 3 (cost X + Y)
        // ```
        test(
            4,
            &[[0, 1, -1], [2, 0, 2], [3, 2, 6], [2, 1, 10], [3, 0, -1]],
            1,
            3,
            12,
            true,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            4,
            &[[0, 1, -1], [1, 2, -1], [3, 1, -1], [3, 0, 2], [0, 2, 5]],
            2,
            3,
            8,
            false,
        )
    }

    #[test]
    fn discussion_case6_1() {
        test(
            5,
            &[
                [1, 4, 1],
                [2, 4, -1],
                [3, 0, 2],
                [0, 4, -1],
                [1, 3, 10],
                [1, 0, 10],
            ],
            0,
            2,
            15,
            true,
        )
    }

    #[test]
    fn discussion_case6_2() {
        test(
            5,
            &[
                [1, 4, 1],
                [2, 4, -1],
                [3, 0, 2],
                [0, 4, -1],
                [1, 3, 10],
                [1, 0, 10],
            ],
            3,
            2,
            15,
            true,
        )
    }

    #[test]
    fn discussion_case100() {
        test(
            100,
            &[
                [0, 1, -1],
                [1, 2, -1],
                [2, 3, -1],
                [3, 4, -1],
                [4, 5, -1],
                [5, 6, -1],
                [6, 7, -1],
                [7, 8, -1],
                [8, 9, -1],
                [9, 10, -1],
                [10, 11, -1],
                [11, 12, -1],
                [12, 13, -1],
                [13, 14, -1],
                [14, 15, -1],
                [15, 16, -1],
                [16, 17, -1],
                [17, 18, -1],
                [18, 19, -1],
                [19, 20, -1],
                [20, 21, -1],
                [21, 22, -1],
                [22, 23, -1],
                [23, 24, -1],
                [24, 25, -1],
                [25, 26, -1],
                [26, 27, -1],
                [27, 28, -1],
                [28, 29, -1],
                [29, 30, -1],
                [30, 31, -1],
                [31, 32, -1],
                [32, 33, -1],
                [33, 34, -1],
                [34, 35, -1],
                [35, 36, -1],
                [36, 37, -1],
                [37, 38, -1],
                [38, 39, -1],
                [39, 40, -1],
                [40, 41, -1],
                [41, 42, -1],
                [42, 43, -1],
                [43, 44, -1],
                [44, 45, -1],
                [45, 46, -1],
                [46, 47, -1],
                [47, 48, -1],
                [48, 49, -1],
                [49, 50, -1],
                [50, 51, -1],
                [51, 52, -1],
                [52, 53, -1],
                [53, 54, -1],
                [54, 55, -1],
                [55, 56, -1],
                [56, 57, -1],
                [57, 58, -1],
                [58, 59, -1],
                [59, 60, -1],
                [60, 61, -1],
                [61, 62, -1],
                [62, 63, -1],
                [63, 64, -1],
                [64, 65, -1],
                [65, 66, -1],
                [66, 67, -1],
                [67, 68, -1],
                [68, 69, -1],
                [69, 70, -1],
                [70, 71, -1],
                [71, 72, -1],
                [72, 73, -1],
                [73, 74, -1],
                [74, 75, -1],
                [75, 76, -1],
                [76, 77, -1],
                [77, 78, -1],
                [78, 79, -1],
                [79, 80, -1],
                [80, 81, -1],
                [81, 82, -1],
                [82, 83, -1],
                [83, 84, -1],
                [84, 85, -1],
                [85, 86, -1],
                [86, 87, -1],
                [87, 88, -1],
                [88, 89, -1],
                [89, 90, -1],
                [90, 91, -1],
                [91, 92, -1],
                [92, 93, -1],
                [93, 94, -1],
                [94, 95, -1],
                [95, 96, -1],
                [96, 97, -1],
                [97, 98, -1],
                [98, 99, -1],
                [0, 99, -1],
            ],
            0,
            99,
            200,
            true,
        )
    }

    #[test]
    fn failing_case1() {
        test(
            100,
            &[
                [0, 1, 10000000],
                [1, 2, 10000000],
                [2, 3, 10000000],
                [3, 4, 10000000],
                [4, 5, 10000000],
                [5, 6, 10000000],
                [6, 7, 10000000],
                [7, 8, 10000000],
                [8, 9, 10000000],
                [9, 10, 10000000],
                [10, 11, 10000000],
                [11, 12, 10000000],
                [12, 13, 10000000],
                [13, 14, 10000000],
                [14, 15, 10000000],
                [15, 16, 10000000],
                [16, 17, 10000000],
                [17, 18, 10000000],
                [18, 19, 10000000],
                [19, 20, 10000000],
                [20, 21, 10000000],
                [21, 22, 10000000],
                [22, 23, 10000000],
                [23, 24, 10000000],
                [24, 25, 10000000],
                [25, 26, 10000000],
                [26, 27, 10000000],
                [27, 28, 10000000],
                [28, 29, 10000000],
                [29, 30, 10000000],
                [30, 31, 10000000],
                [31, 32, 10000000],
                [32, 33, 10000000],
                [33, 34, 10000000],
                [34, 35, 10000000],
                [35, 36, 10000000],
                [36, 37, 10000000],
                [37, 38, 10000000],
                [38, 39, 10000000],
                [39, 40, 10000000],
                [40, 41, 10000000],
                [41, 42, 10000000],
                [42, 43, 10000000],
                [43, 44, 10000000],
                [44, 45, 10000000],
                [45, 46, 10000000],
                [46, 47, 10000000],
                [47, 48, 10000000],
                [48, 49, 10000000],
                [49, 50, 10000000],
                [50, 51, 10000000],
                [51, 52, 10000000],
                [52, 53, 10000000],
                [53, 54, 10000000],
                [54, 55, 10000000],
                [55, 56, 10000000],
                [56, 57, 10000000],
                [57, 58, 10000000],
                [58, 59, 10000000],
                [59, 60, 10000000],
                [60, 61, 10000000],
                [61, 62, 10000000],
                [62, 63, 10000000],
                [63, 64, 10000000],
                [64, 65, 10000000],
                [65, 66, 10000000],
                [66, 67, 10000000],
                [67, 68, 10000000],
                [68, 69, 10000000],
                [69, 70, 10000000],
                [70, 71, 10000000],
                [71, 72, 10000000],
                [72, 73, 10000000],
                [73, 74, 10000000],
                [74, 75, 10000000],
                [75, 76, 10000000],
                [76, 77, 10000000],
                [77, 78, 10000000],
                [78, 79, 10000000],
                [79, 80, 10000000],
                [80, 81, 10000000],
                [81, 82, 10000000],
                [82, 83, 10000000],
                [83, 84, 10000000],
                [84, 85, 10000000],
                [85, 86, 10000000],
                [86, 87, 10000000],
                [87, 88, 10000000],
                [88, 89, 10000000],
                [89, 90, 10000000],
                [90, 91, 10000000],
                [91, 92, 10000000],
                [92, 93, 10000000],
                [93, 94, 10000000],
                [94, 95, 10000000],
                [95, 96, 10000000],
                [96, 97, 10000000],
                [97, 98, 10000000],
                [98, 99, 10000000],
                [0, 99, -1],
            ],
            0,
            99,
            990000000,
            true,
        )
    }

    #[test]
    fn failing_case2() {
        test(
            5,
            &[[0,1,-1],[3,1,-1],[4,0,2697905],[4,1,1914390],[2,4,-1],[3,2,6790233]],
            0,
            2,
            744504190,
            true,
        )
    }

    #[test]
    fn failing_case2_1() {
        test(
            5,
            &[[0,1,-1],[3,1,-1],[4,0,26979],[4,1,19143],[2,4,-1],[3,2,67902]],
            0,
            2,
            744504190,
            true,
        )
    }
}
