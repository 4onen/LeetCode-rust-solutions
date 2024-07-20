// https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
//         let mut result = vec![vec![0; col_sum.len()]; row_sum.len()];
//         for i in 0..row_sum.len() {
//             for j in 0..col_sum.len() {
//                 let x = std::cmp::min(row_sum[i], col_sum[j]);
//                 result[i][j] = x;
//                 row_sum[i] -= x;
//                 col_sum[j] -= x;
//             }
//         }
//         result
//     }
// }

// Improved sol'n
impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = std::vec::Vec::with_capacity(row_sum.len());
        let result_rows = result.spare_capacity_mut();
        for i in 0..row_sum.len() as u16 {
            let mut row = std::vec::Vec::with_capacity(col_sum.len());
            let row_cells = row.spare_capacity_mut();
            for j in 0..col_sum.len() as u16 {
                let x = std::cmp::min(row_sum[i as usize], col_sum[j as usize]);
                row_cells[j as usize].write(x);
                row_sum[i as usize] -= x;
                col_sum[j as usize] -= x;
            }
            unsafe { row.set_len(col_sum.len()) };
            result_rows[i as usize].write(row);
        }
        unsafe { result.set_len(row_sum.len()) };
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(row_sum: &[i32], col_sum: &[i32]) {
        assert!(row_sum.len() >= 1);
        assert!(col_sum.len() >= 1);
        assert!(row_sum.len() <= 500);
        assert!(col_sum.len() <= 500);
        for &x in row_sum {
            assert!(x >= 1);
            assert!(x <= 100_000_000);
        }
        for &x in col_sum {
            assert!(x >= 1);
            assert!(x <= 100_000_000);
        }
        let row_sum_sum = row_sum.iter().map(|&x| x as i64).sum::<i64>();
        let col_sum_sum = col_sum.iter().map(|&x| x as i64).sum::<i64>();
        assert_eq!(row_sum_sum, col_sum_sum);
        let result = Solution::restore_matrix(row_sum.to_vec(), col_sum.to_vec());
        let mut col_sum = col_sum.to_vec();
        for (row, expected) in result.iter().zip(row_sum) {
            assert_eq!(row.iter().sum::<i32>(), *expected);
            for (i, &x) in row.iter().enumerate() {
                assert!(x >= 0);
                assert!(x <= col_sum[i]);
                col_sum[i] -= x;
            }
        }
    }

    #[test]
    fn ex1() {
        test(&[3, 8], &[4, 7])
    }

    #[test]
    fn ex2() {
        test(&[5, 7, 10], &[8, 6, 8])
    }

    #[test]
    fn discussion_case1() {
        const BIG: [i32; 500] = [100_000_000; 500];
        test(&BIG, &BIG)
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 1, 6], &[5, 3])
    }
}
