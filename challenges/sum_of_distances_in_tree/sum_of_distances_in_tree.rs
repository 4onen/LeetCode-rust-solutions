// https://leetcode.com/problems/sum-of-distances-in-tree/

pub struct Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        assert!(n >= 1);
        assert!(n <= 3 * 10i32.pow(4));
        let n = n as u16;
        assert!(edges.len() <= 3 * 10usize.pow(4));
        assert_eq!(edges.len() as u16, n - 1);
        let graph = {
            let mut graph = vec![vec![]; n as usize];
            for edge in edges {
                let u = edge[0] as u16;
                let v = edge[1] as u16;
                graph[u as usize].push(v);
                graph[v as usize].push(u);
            }
            graph
        };
        let mut count = vec![1; n as usize];
        let mut ans = vec![0; n as usize];
        fn prefix_sum_dfs(
            node: u16,
            prev: u16,
            graph: &[Vec<u16>],
            count: &mut [i32],
            ans: &mut [i32],
        ) {
            for &v in &graph[node as usize] {
                if v == prev {
                    continue;
                }
                prefix_sum_dfs(v, node, graph, count, ans);
                count[node as usize] += count[v as usize];
                ans[node as usize] += ans[v as usize] + count[v as usize] as i32;
            }
        }
        fn answer_dfs(
            node: u16,
            prev: u16,
            graph: &[Vec<u16>],
            count: &[i32],
            ans: &mut [i32],
            n: u16,
        ) {
            for &v in &graph[node as usize] {
                if v == prev {
                    continue;
                }
                ans[v as usize] =
                    ans[node as usize] - count[v as usize] + (n as i32 - count[v as usize]) as i32;
                answer_dfs(v, node, graph, count, ans, n);
            }
        }
        prefix_sum_dfs(0, 0, &graph, &mut count, &mut ans);
        answer_dfs(0, 0, &graph, &count, &mut ans, n);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(edges: &[[u16; 2]], expected: &[u32]) {
        let n = (edges.len() + 1) as u16;
        assert!(edges.iter().all(|&[e1, e2]| e1 < n && e2 < n));
        let edges: Vec<Vec<i32>> = edges
            .into_iter()
            .map(|&[e0, e1]| vec![e0 as i32, e1 as i32])
            .collect();
        let result = Solution::sum_of_distances_in_tree(n as i32, edges);
        assert!(result.iter().all(|&x| x >= 0));
        let result: Vec<u32> = result.into_iter().map(|x| x as u32).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(
            &[[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]],
            &[8, 12, 6, 10, 10, 10],
        )
    }

    #[test]
    fn ex2() {
        test(&[], &[0])
    }

    #[test]
    fn ex3() {
        test(&[[1, 0]], &[1, 1])
    }
}
