// https://leetcode.com/problems/build-a-matrix-with-conditions/

pub struct Solution;

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        assert!(k >= 2);
        assert!(k <= 400);
        let k = k as u16;
        fn topo_sort(k: u16, conditions: Vec<Vec<i32>>) -> Option<Vec<i32>> {
            // k is the number of nodes in the graph
            // conditions contains the directed edges in the graph we need
            //  to topologically sort.
            let mut graph = vec![vec![]; k as usize];
            let mut in_degree = vec![0; k as usize];
            for condition in conditions {
                let from = condition[0] as usize - 1;
                let to = condition[1] as usize - 1;
                graph[from].push(to);
                in_degree[to] += 1;
            }
            let mut queue = std::collections::VecDeque::new();
            for i in 0..k as usize {
                if in_degree[i] == 0 {
                    queue.push_back(i);
                }
            }
            let mut sorted = Vec::new();
            while let Some(node) = queue.pop_front() {
                sorted.push(node as i32);
                for &neighbour in &graph[node] {
                    in_degree[neighbour] -= 1;
                    if in_degree[neighbour] == 0 {
                        queue.push_back(neighbour);
                    }
                }
            }
            if sorted.len() == k as usize {
                Some(sorted)
            } else {
                None
            }
        }
        let Some(row_order) = topo_sort(k, row_conditions) else {
            return vec![];
        };
        let Some(col_order) = topo_sort(k, col_conditions) else {
            return vec![];
        };
        let mut matrix = vec![vec![0; k as usize]; k as usize];
        for (&row_val, row) in std::iter::zip(row_order.iter(), matrix.iter_mut()) {
            for (&col_val, cell) in std::iter::zip(col_order.iter(), row.iter_mut()) {
                if row_val == col_val {
                    *cell = row_val + 1;
                    break;
                }
            }
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(
        k: u16,
        row_conditions: &[[i32; 2]],
        col_conditions: &[[i32; 2]],
        solution_expected: bool,
    ) {
        fn mat_to_row_col_order(mat: &[&[i32]]) -> (Vec<i32>, Vec<i32>) {
            // Each row of the mat should have only one value that is non-zero
            // Assert on that.
            let k = mat.len();
            let mut row_order = vec![0; k];
            let mut col_order = vec![0; k];
            for (i, row) in mat.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    if cell != 0 {
                        assert_eq!(row_order[i], 0);
                        assert_eq!(col_order[j], 0);
                        row_order[i] = cell;
                        col_order[j] = cell;
                    }
                }
            }
            (row_order, col_order)
        }
        fn assert_order_matches_conditions(order: &[i32], conditions: &[[i32; 2]]) {
            for &[from, to] in conditions {
                let from_index = order.iter().position(|&x| x == from).unwrap();
                let to_index = order.iter().position(|&x| x == to).unwrap();
                assert!(from_index < to_index);
            }
        }
        assert!(k >= 2);
        assert!(k <= 400);
        assert!(row_conditions.len() >= 1);
        assert!(row_conditions.len() <= 10_000);
        assert!(col_conditions.len() >= 1);
        assert!(col_conditions.len() <= 10_000);
        for &[above, below] in row_conditions.iter() {
            assert!(above >= 1);
            assert!(above <= k as i32);
            assert!(below >= 1);
            assert!(below <= k as i32);
            assert_ne!(above, below);
        }
        for &[left, right] in col_conditions.iter() {
            assert!(left >= 1);
            assert!(left <= k as i32);
            assert!(right >= 1);
            assert!(right <= k as i32);
            assert_ne!(left, right);
        }
        let row_conditions_vec = row_conditions.iter().map(|x| x.to_vec()).collect();
        let col_conditions_vec = col_conditions.iter().map(|x| x.to_vec()).collect();
        let result = Solution::build_matrix(k as i32, row_conditions_vec, col_conditions_vec);
        if solution_expected {
            let result_ref = result.iter().map(|x| x.as_slice()).collect::<Vec<_>>();
            let (row_order, col_order) = mat_to_row_col_order(&result_ref);
            assert_eq!(row_order.len(), k as usize);
            assert_eq!(col_order.len(), k as usize);
            assert_order_matches_conditions(&row_order, row_conditions);
            assert_order_matches_conditions(&col_order, col_conditions);
        } else {
            let empty_soln: &[Vec<i32>] = &[];
            assert_eq!(result, empty_soln);
        }
    }

    #[test]
    fn ex1() {
        test(3, &[[1, 2], [3, 2]], &[[2, 1], [3, 2]], true)
    }

    #[test]
    fn ex2() {
        test(3, &[[1, 2], [2, 3], [3, 1], [2, 3]], &[[2, 1]], false)
    }

    #[test]
    fn discussion_case1() {
        test(3, &[[1, 2], [3, 2]], &[[2, 1], [3, 2]], true)
    }

    #[test]
    fn discussion_case2() {
        test(10, &[[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1]], &[[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1],[1,2],[2,3],[3,4],[4,5],[5,6],[6,7],[7,8],[8,9],[9,10],[10,1]], false)
    }

    #[test]
    fn discussion_case3() {
        test(4, &[[1,2],[2,3],[3,4]], &[[1,2],[2,3],[3,4]], true)
    }

    #[test]
    fn discussion_case4() {
        test(8, &[[1,2]], &[[5,7],[2,7],[4,3],[6,7],[4,3],[2,3],[6,2]], true)
    }

    #[test]
    fn discussion_case5() {
        // Diagonal
        let conditions = [[1,2],[2,3],[3,4],[4,5],[5,6]];
        test(6, &conditions, &conditions, true)
    }

    #[test]
    fn discussion_case6() {
        test(
            50,
            &[[1, 2], [3, 4], [5, 6], [7, 8], [9, 10], [11, 12], [13, 14], [15, 16], [17, 18], [19, 20], [21, 22], [23, 24], [25, 26], [27, 28], [29, 30], [31, 32], [33, 34], [35, 36], [37, 38], [39, 40], [41, 42], [43, 44], [45, 46], [47, 48], [49, 50], [2, 5], [4, 7], [6, 9], [8, 11], [10, 13], [12, 15], [14, 17], [16, 19], [18, 21], [20, 23], [22, 25], [24, 27], [26, 29], [28, 31], [30, 33], [32, 35], [34, 37], [36, 39], [38, 41], [40, 43], [42, 45], [44, 47], [46, 49]],
            &[[1, 3], [2, 4], [5, 7], [6, 8], [9, 11], [10, 12], [13, 15], [14, 16], [17, 19], [18, 20], [21, 23], [22, 24], [25, 27], [26, 28], [29, 31], [30, 32], [33, 35], [34, 36], [37, 39], [38, 40], [41, 43], [42, 44], [45, 47], [46, 48], [49, 50], [3, 6], [4, 8], [7, 10], [9, 12], [11, 14], [13, 16], [15, 18], [17, 20], [19, 22], [21, 24], [23, 26], [25, 28], [27, 30], [29, 32], [31, 34], [33, 36], [35, 38], [37, 40], [39, 42], [41, 44], [43, 46], [45, 48]],
            true
        )
    }
}
