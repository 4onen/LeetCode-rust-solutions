// https://leetcode.com/problems/find-eventual-safe-states/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        type Idx = u16;
        fn dfs(graph: &Vec<Vec<i32>>, state: &mut [u8], u: Idx) -> bool {
            if state[u as usize] > 0 {
                return state[u as usize] == 2;
            }
            state[u as usize] = 1;
            for &v in &graph[u as usize] {
                if state[v as usize] == 2 {
                    continue;
                }
                if state[v as usize] == 1 {
                    return false;
                }
                if !dfs(graph, state, v as Idx) {
                    return false;
                }
            }
            state[u as usize] = 2;
            true
        }
        let n = graph.len() as Idx;
        let mut state = vec![0; n as usize];
        let mut res = Vec::new();
        for u in 0..n {
            if dfs(&graph, &mut state, u) {
                res.push(u as i32);
            }
        }
        res
    }
}

// Saved one branch
// impl Solution {
//     pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
//         type Idx = u16;
//         fn dfs(graph: &Vec<Vec<i32>>, state: &mut [u8], u: Idx) -> bool {
//             if state[u as usize] > 0 {
//                 return state[u as usize] == 2;
//             }
//             state[u as usize] = 1;
//             for &v in &graph[u as usize] {
//                 if state[v as usize] == 2 {
//                     continue;
//                 }
//                 if !dfs(graph, state, v as Idx) {
//                     return false;
//                 }
//             }
//             state[u as usize] = 2;
//             true
//         }
//         let n = graph.len() as Idx;
//         let mut state = vec![0; n as usize];
//         let mut res = Vec::new();
//         for u in 0..n {
//             if dfs(&graph, &mut state, u) {
//                 res.push(u as i32);
//             }
//         }
//         res
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(graph: &[&[i32]], expected: &[i32]) {
        assert!(graph.len() >= 1);
        assert!(graph.len() <= 10_000);
        let mut edges = 0;
        for &neighbors in graph {
            assert!(neighbors.len() <= graph.len());
            assert!(neighbors.is_sorted());
            edges += neighbors.len();
            for &v in neighbors {
                assert!(v >= 0);
                assert!(v < graph.len() as i32);
            }
        }
        assert!(edges >= 1);
        assert!(edges <= 40_000);
        assert_eq!(
            Solution::eventual_safe_nodes(graph.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &[&[1, 2], &[2, 3], &[5], &[0], &[5], &[], &[]],
            &[2, 4, 5, 6],
        )
    }

    #[test]
    fn ex2() {
        test(&[&[1, 2, 3, 4], &[1, 2], &[3, 4], &[0, 4], &[]], &[4])
    }

    #[test]
    fn my_extreme_ex1() {
        let mut graph = vec![vec![]; 10_000];
        for i in 0..10_000 {
            graph[i as usize].push((i + 1) % 10_000);
        }
        test(
            &graph.iter().map(|x| x.as_slice()).collect::<Vec<&[i32]>>(),
            &[],
        )
    }

    #[test]
    fn my_extreme_ex2() {
        let mut graph = vec![vec![]; 10_000];
        for i in 0..10_000 {
            graph[i as usize].push(i);
        }
        test(
            &graph.iter().map(|x| x.as_slice()).collect::<Vec<&[i32]>>(),
            &[],
        )
    }

    #[test]
    fn my_extreme_ex3() {
        let mut graph = vec![vec![]; 10_000];
        for i in 0..10_000 {
            graph[i as usize].push(10_000 - i - 1);
        }
        test(
            &graph.iter().map(|x| x.as_slice()).collect::<Vec<&[i32]>>(),
            &[],
        )
    }

    #[test]
    fn my_extreme_ex4() {
        let mut graph = vec![vec![]; 10_000];
        for i in 0..9_999 {
            graph[i as usize].push((i + 1) % 10_000);
        }
        let mut ans = vec![0; 10_000];
        for i in 0..10_000 {
            ans[i] = i as i32;
        }
        test(
            &graph.iter().map(|x| x.as_slice()).collect::<Vec<&[i32]>>(),
            &ans,
        )
    }

    #[test]
    fn discussion_case1() {
        test(&[&[], &[0, 2, 3, 4], &[3], &[4], &[]], &[0, 1, 2, 3, 4])
    }

    #[test]
    fn discussion_case2() {
        test(&[&[], &[0, 1, 2, 3, 4], &[2, 3], &[4], &[]], &[0, 3, 4])
    }

    #[test]
    fn discussion_case3() {
        test(&[&[4], &[0, 1, 2, 3, 4], &[2, 3], &[2, 4], &[1]], &[])
    }

    #[test]
    fn discussion_case4() {
        test(&[&[0], &[]], &[1])
    }
}
