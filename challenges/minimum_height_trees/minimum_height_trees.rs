// https://leetcode.com/problems/minimum-height-trees/

pub struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n < 2 {
            return vec![0];
        }
        if n == 2 {
            return vec![0, 1];
        }
        assert!(n <= 20_000);
        let n = n as u16;
        assert!(edges.len() < n as usize);
        // Algorithm:
        // 1. Figure out the leaves of the tree (degree = 1)
        // 2. Trim the leaves of the tree.
        // 3. Repeat until there are 2 or less nodes left.
        let mut adjacency_list = vec![vec![]; n as usize];
        let mut degrees: Vec<u16> = vec![0; n as usize];
        for edge in edges {
            let u = edge[0] as u16;
            let v = edge[1] as u16;
            adjacency_list[u as usize].push(v);
            adjacency_list[v as usize].push(u);
            degrees[u as usize] += 1;
            degrees[v as usize] += 1;
        }
        let mut leaves: Vec<u16> = Vec::new();
        for i in 0..n {
            if degrees[i as usize] == 1 {
                leaves.push(i);
            }
        }
        let mut remaining = n as usize;
        while remaining > 2 {
            remaining -= leaves.len();
            let mut new_leaves = Vec::new();
            for leaf in leaves.iter() {
                let leaf = *leaf;
                for &neighbor in adjacency_list[leaf as usize].iter() {
                    degrees[neighbor as usize] -= 1;
                    if degrees[neighbor as usize] == 1 {
                        new_leaves.push(neighbor);
                    }
                }
            }
            leaves = new_leaves;
        }
        leaves.iter().map(|&x| x as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u16, edges: &[[i32; 2]], expected: &[i32]) {
        assert!(n > edges.len().checked_sub(1).unwrap_or_default() as u16);
        let edges = edges.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::find_min_height_trees(n as i32, edges), expected);
    }

    #[test]
    fn ex1() {
        test(4, &[[1, 0], [1, 2], [1, 3]], &[1]);
    }

    #[test]
    fn ex2() {
        test(6, &[[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]], &[3, 4]);
    }

    #[test]
    fn discussion_case0() {
        test(1, &[], &[0]);
    }

    #[test]
    fn discussion_case1() {
        // Leaves: 0, 6, 5
        // Trim [0,1], [4,6], [3,5]
        // Remaining: [1,2], [1,3], [2,4]
        // Leaves: 3, 4
        // Trim [1,3], [2,4]
        // Remaining: [1,2]
        // Return [1,2]
        test(
            7,
            &[[0, 1], [1, 2], [1, 3], [2, 4], [3, 5], [4, 6]],
            &[1, 2],
        );
    }
}
