// https://leetcode.com/problems/convert-1d-array-into-2d-array/

pub struct Solution;

// Initial iterator copying sol'n
// impl Solution {
//     pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
//         assert!(m > 0 && n > 0);
//         if (m * n) as usize != original.len() {
//             vec![]
//         } else {
//             let mut result = Vec::with_capacity(m as usize);
//             let mut original_iter = original.iter();
//             for _ in 0..m {
//                 let mut row = Vec::with_capacity(n as usize);
//                 row.extend(original_iter.by_ref().take(n as usize));
//                 result.push(row);
//             }
//             result
//         }
//     }
// }

// Alloc from iterator sol'n
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        assert!(m > 0 && n > 0);
        if (m * n) as usize != original.len() {
            vec![]
        } else {
            let mut result = Vec::with_capacity(m as usize);
            let mut original_iter = original.into_iter();
            for _ in 0..m {
                let row = original_iter.by_ref().take(n as usize).collect();
                result.push(row);
            }
            result
        }
    }
}

// "Unsafe" parallelizeable slice copying sol'n
// impl Solution {
//     pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
//         assert!(m > 0 && n > 0);
//         if (m * n) as usize != original.len() {
//             vec![]
//         } else {
//             let mut result = Vec::with_capacity(m as usize);
//             for i in 0..m {
//                 let mut row = Vec::with_capacity(n as usize);
//                 unsafe { row.set_len(n as usize) };
//                 let start = (i * n) as usize;
//                 row.copy_from_slice(&original[start..start + n as usize]);
//                 result.push(row);
//             }
//             result
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(original: &[i32], m: i32, n: i32, expected: &[&[i32]]) {
        assert!(original.len() > 0);
        assert!(original.len() <= 50_000);
        for &val in original {
            assert!(val >= 1);
            assert!(val <= 100_000);
        }
        let mut original_val_iter = original.iter();
        for &row in expected {
            assert!(row.len() > 0);
            assert!(row.len() <= 50_000);
            for &val in row {
                assert!(val >= 1);
                assert!(val <= 100_000);
                assert_eq!(original_val_iter.next(), Some(&val));
            }
        }
        assert!(m >= 1);
        assert!(m <= 40_000);
        assert!(n >= 1);
        assert!(n <= 40_000);
        let result = Solution::construct2_d_array(original.to_vec(), m as i32, n as i32);
        if expected.is_empty() {
            assert!(result.is_empty());
            return;
        }
        assert_eq!(result.len(), m as usize);
        for i in 0..m as usize {
            assert_eq!(result[i as usize].len(), n as usize);
            assert_eq!(result[i as usize].as_slice(), expected[i]);
        }
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3, 4], 2, 2, &[&[1, 2], &[3, 4]])
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3], 1, 3, &[&[1, 2, 3]])
    }

    #[test]
    fn ex3() {
        test(&[1, 2], 1, 1, &[])
    }
}
