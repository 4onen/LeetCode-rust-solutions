// https://leetcode.com/problems/path-with-maximum-probability/

pub struct Solution;

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        const EPS: f64 = 1e-5;
        assert!(n >= 2);
        assert!(n <= 10_000);
        assert!(start_node < n);
        assert!(end_node < n);
        let n = n as u16;
        let start_node = start_node as u16;
        let end_node = end_node as u16;
        let mut targets = vec![vec![]; n as usize];
        for (edge, succ) in std::iter::zip(edges.into_iter(), succ_prob.into_iter()) {
            if succ <= EPS {
                continue;
            }
            targets[edge[0] as usize].push((edge[1] as u16, succ));
            targets[edge[1] as usize].push((edge[0] as u16, succ));
        }
        let mut probs = vec![0.0f64; n as usize];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start_node as u16, 1.0f64));
        while let Some((node, new_prob)) = queue.pop_front() {
            let old_prob = probs[node as usize];
            if new_prob > old_prob {
                probs[node as usize] = new_prob;
                if node == end_node {
                    continue;
                }
                queue.extend(
                    targets[node as usize]
                        .iter()
                        .map(|&(tgt, succ)| (tgt, new_prob * succ)),
                );
            };
        }
        probs[end_node as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_DEVIATION: f64 = 1e-5;

    fn test(
        n: u16,
        edges: &[[u16; 2]],
        succ_prob: &[f64],
        start_node: u16,
        end_node: u16,
        expected: f64,
    ) {
        assert!(n >= 2);
        assert!(n <= 10_000);
        assert_eq!(edges.len(), succ_prob.len());
        assert!(edges.len() <= 20_000);
        assert!(start_node < n);
        assert!(end_node < n);
        for &prob in succ_prob {
            assert!(prob >= 0.0);
            assert!(prob <= 1.0);
        }
        let mut seen_edges = std::collections::HashSet::new();
        for &edge in edges {
            assert!(edge[0] < n);
            assert!(edge[1] < n);
            assert_ne!(edge[0], edge[1]);
            assert!(seen_edges.insert([
                std::cmp::min(edge[0], edge[1]),
                std::cmp::max(edge[0], edge[1])
            ]));
        }
        let result = Solution::max_probability(
            n as i32,
            edges
                .iter()
                .map(|e| e.iter().map(|&x| x as i32).collect())
                .collect(),
            succ_prob.to_vec(),
            start_node as i32,
            end_node as i32,
        );
        assert!(
            (result - expected).abs() < MAX_DEVIATION,
            "result: {}, expected: {}",
            result,
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            3,
            &[[0, 1], [1, 2], [0, 2]],
            &[0.5, 0.5, 0.2],
            0,
            2,
            0.25000,
        )
    }

    #[test]
    fn ex2() {
        test(
            3,
            &[[0, 1], [1, 2], [0, 2]],
            &[0.5, 0.5, 0.3],
            0,
            2,
            0.30000,
        )
    }

    #[test]
    fn ex3() {
        test(3, &[[0, 1]], &[0.5], 0, 2, 0.00000)
    }

    #[test]
    fn discussion_case1() {
        const INPUT: &str = include_str!("discussion_case1.txt");
        let mut lines = INPUT.lines();
        let edges_line = lines.next().unwrap();
        let succ_prob_line = lines.next().unwrap();
        let edges: Vec<_> = edges_line[2..edges_line.len() - 3]
            .split("],[")
            .map(|pair| {
                let mut parts = pair.split(',');
                let a = parts.next().unwrap().parse().unwrap();
                let b = parts.next().unwrap().parse().unwrap();
                [a, b]
            })
            .collect();
        let succ_prob: Vec<_> = succ_prob_line[1..succ_prob_line.len() - 1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        test(10000, &edges, &succ_prob, 0, 999, 0.)
    }
}
