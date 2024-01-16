// https://leetcode.com/problems/special-positions-in-a-binary-matrix/

pub struct Solution;

// Failed first guess
// impl Solution {
//     pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
//         let special_rows = mat
//             .iter()
//             .filter(|row| row.iter().sum::<i32>() == 1)
//             .count();
//         let special_cols = (0..mat[0].len())
//             .into_iter()
//             .filter(|col| {
//                 let mut acc = 0;
//                 for row in 0..mat.len() {
//                     acc += mat[row][*col];
//                     if acc > 1 {
//                         break;
//                     };
//                 }
//                 acc == 1
//             })
//             .count();

//         usize::min(special_cols, special_rows) as i32
//     }
// }

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        (0..mat.len())
            .into_iter()
            .filter_map(|row| {
                let row_slice = &mat[row][..];
                let special_col = row_slice.iter().position(|&v| v == 1);
                special_col.and_then(|col| {
                    if ((col + 1)..row_slice.len()).all(|c| mat[row][c] == 0) {
                        Some(col)
                    } else {
                        None
                    }
                })
            })
            .filter(|&special_col| {
                ((0..mat.len()).map(|row| mat[row][special_col]).sum::<i32>()) == 1
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            1
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        )
    }

    #[test]
    fn loooong1() {
        assert_eq!(
            Solution::num_special(vec![vec![0, 0, 0, 0, 1, 0, 0, 0, 0]]),
            1
        );
    }

    #[test]
    fn loooong2() {
        assert_eq!(
            Solution::num_special(vec![vec![1, 1, 1, 0, 0, 0, 0, 1, 1, 1]]),
            0
        )
    }

    #[test]
    fn empty() {
        assert_eq!(Solution::num_special(vec![vec![0]]), 0)
    }

    #[test]
    fn full() {
        assert_eq!(Solution::num_special(vec![vec![1, 1], vec![1, 1]]), 0)
    }

    #[test]
    fn one() {
        assert_eq!(Solution::num_special(vec![vec![1]]), 1)
    }

    #[test]
    fn failed_case() {
        assert_eq!(
            Solution::num_special(vec![
                vec![0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 1],
                vec![0, 0, 0, 0, 1, 0, 0, 0],
                vec![1, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            1
        )
    }

    #[test]
    fn smaller_fail() {
        assert_eq!(
            Solution::num_special(vec![
                vec![0, 0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 0, 0]
            ]),
            1
        )
    }
}
