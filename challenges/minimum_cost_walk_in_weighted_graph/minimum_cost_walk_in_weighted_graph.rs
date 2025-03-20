// https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph/

pub struct Solution;

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        type Idx = u32;
        let n = n as Idx;
        let mut node_to_group: Vec<_> = (0..n).into_iter().collect();
        let mut group_cost: Vec<_> = vec![i32::MAX; n as usize];
        for edge in edges {
            let u = edge[0] as Idx;
            let v = edge[1] as Idx;
            let mut w = edge[2];
            let mut u_group = node_to_group[u as usize];
            let mut v_group = node_to_group[v as usize];
            while u_group != node_to_group[u_group as usize] {
                let l = group_cost[u_group as usize] & w;
                group_cost[u_group as usize] = l;
                w = l;
                let next = node_to_group[u_group as usize];
                u_group = next;
            }
            while v_group != node_to_group[v_group as usize] {
                let l = group_cost[v_group as usize] & w;
                group_cost[v_group as usize] = l;
                w = l;
                let next = node_to_group[v_group as usize];
                v_group = next;
            }
            let l = w & group_cost[u_group as usize] & group_cost[v_group as usize];
            group_cost[u_group as usize] = l;
            group_cost[v_group as usize] = l;
            if u_group < v_group {
                node_to_group[v_group as usize] = u_group;
                node_to_group[u as usize] = u_group;
                node_to_group[v as usize] = u_group;
            } else if v_group < u_group {
                node_to_group[u_group as usize] = v_group;
                node_to_group[u as usize] = v_group;
                node_to_group[v as usize] = v_group;
            }
        }
        query
            .iter()
            .map(|q| {
                let u = q[0] as Idx;
                let v = q[1] as Idx;
                let mut u_group = node_to_group[u as usize];
                let mut v_group = node_to_group[v as usize];
                while u_group != node_to_group[u_group as usize] {
                    u_group = node_to_group[u_group as usize];
                }
                while v_group != node_to_group[v_group as usize] {
                    v_group = node_to_group[v_group as usize];
                }
                if u_group == v_group {
                    group_cost[u_group as usize]
                } else {
                    -1
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u32, edges: &[[u32; 3]], query: &[[u32; 2]], expected: &[i32]) {
        assert!(n >= 2);
        assert!(n <= 100_000);
        assert!(edges.len() <= 100_000);
        for &[u, v, w] in edges {
            assert!(u <= n - 1);
            assert!(v <= n - 1);
            assert_ne!(u, v);
            assert!(w <= 100_000);
        }
        assert!(query.len() >= 1);
        assert!(query.len() <= 100_000);
        for &[u, v] in query {
            assert!(u <= n - 1);
            assert!(v <= n - 1);
            assert_ne!(u, v);
        }
        assert_eq!(
            Solution::minimum_cost(
                n as i32,
                edges
                    .iter()
                    .map(|&x| vec![x[0] as i32, x[1] as i32, x[2] as i32])
                    .collect(),
                query
                    .iter()
                    .map(|&x| vec![x[0] as i32, x[1] as i32])
                    .collect(),
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            5,
            &[[0, 1, 7], [1, 3, 7], [1, 2, 1]],
            &[[0, 3], [3, 4]],
            &[1, -1],
        )
    }

    #[test]
    fn ex2() {
        test(
            3,
            &[[0, 2, 7], [0, 1, 15], [1, 2, 6], [1, 2, 1]],
            &[[1, 2]],
            &[0],
        )
    }

    #[test]
    fn ex2_1() {
        test(3, &[[1, 2, 6], [1, 2, 1]], &[[1, 2]], &[0])
    }

    #[test]
    fn discussion_case1() {
        test(
            3,
            &[
                [1, 0, 4],
                [0, 2, 5],
                [0, 2, 3],
                [0, 2, 14],
                [0, 2, 12],
                [2, 0, 14],
                [0, 2, 4],
            ],
            &[[2, 1]],
            &[0],
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            9,
            &[[0, 4, 7], [3, 5, 1], [1, 3, 5], [1, 5, 1]],
            &[[0, 4], [1, 5], [3, 0], [3, 2], [2, 0], [7, 0]],
            &[7, 1, -1, -1, -1, -1],
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            8,
            &[
                [2, 0, 0],
                [6, 7, 1],
                [1, 2, 0],
                [4, 6, 1],
                [1, 5, 0],
                [5, 1, 1],
                [4, 3, 1],
                [5, 2, 0],
                [3, 0, 1],
            ],
            &[[6, 7], [6, 2], [3, 4], [3, 2], [0, 3], [7, 3]],
            &[0, 0, 0, 0, 0, 0],
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            6,
            &[
                [2, 5, 3],
                [0, 3, 1],
                [1, 4, 0],
                [0, 3, 0],
                [0, 2, 5],
                [2, 0, 2],
                [5, 1, 2],
                [1, 3, 1],
                [2, 1, 4],
                [3, 2, 3],
            ],
            &[[1, 5], [0, 1], [1, 5], [3, 1], [1, 2]],
            &[0, 0, 0, 0, 0],
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            9,
            &[[1, 7, 7], [5, 6, 2], [3, 8, 5], [3, 6, 3]],
            &[[3, 8], [1, 4], [5, 2], [5, 2], [1, 3]],
            &[0, -1, -1, -1, -1],
        )
    }
}
