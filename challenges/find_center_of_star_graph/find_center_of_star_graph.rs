// https://leetcode.com/problems/find-center-of-star-graph/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
//         let mut counts: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
//         for edge in edges {
//             for node in edge {
//                 let count = counts.entry(node).or_insert(false);
//                 if *count {
//                     return node;
//                 }
//                 *count = true;
//             }
//         }
//         return -1;
//     }
// }

// Two-edge sol'n
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let first = edges[0][0];
        let second = edges[0][1];
        if first == edges[1][0] || first == edges[1][1] {
            return first;
        } else {
            return second;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(edges: &[[i32; 2]], expected: i32) {
        let edges: Vec<Vec<i32>> = edges.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::find_center(edges), expected);
    }

    #[test]
    fn ex1() {
        test(&[[1, 2], [2, 3], [4, 2]], 2);
    }

    #[test]
    fn ex2() {
        test(&[[1, 2], [5, 1], [1, 3], [1, 4]], 1);
    }
}
