// https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
//         let n = mat.len();
//         // Identity
//         if mat == target {
//             return true;
//         }
//         // 90 degree
//         let mut found = true;
//         for i in 0..n {
//             for j in 0..n {
//                 if mat[i][j] != target[j][n - i - 1] {
//                     found = false;
//                     break;
//                 }
//             }
//         }
//         if std::mem::replace(&mut found, true) {
//             return true;
//         }
//         // 180 degree
//         for i in 0..n {
//             if !(mat[i].iter().eq(target[n - i - 1].iter().rev())) {
//                 found = false;
//                 break;
//             }
//         }
//         if std::mem::replace(&mut found, true) {
//             return true;
//         }
//         // 270 degree
//         for i in 0..n {
//             for j in 0..n {
//                 if mat[i][j] != target[n - j - 1][i] {
//                     return false;
//                 }
//             }
//         }
//         true
//     }
// }

// Use the same technique for each rotation except identity
impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();
        if mat == target {
            return true;
        }
        // 90 degree
        let mut found = true;
        for i in 0..n {
            for j in 0..n {
                if mat[i][j] != target[j][n - i - 1] {
                    found = false;
                    break;
                }
            }
        }
        if std::mem::replace(&mut found, true) {
            return true;
        }
        // 180 degree
        for i in 0..n {
            for j in 0..n {
                if mat[i][j] != target[n - i - 1][n - j - 1] {
                    found = false;
                    break;
                }
            }
        }
        if std::mem::replace(&mut found, true) {
            return true;
        }
        // 270 degree
        for i in 0..n {
            for j in 0..n {
                if mat[i][j] != target[n - j - 1][i] {
                    found = false;
                    break;
                }
            }
        }
        if std::mem::replace(&mut found, true) {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(mat: &[&[i32]], target: &[&[i32]], expected: bool) {
        assert_eq!(
            Solution::find_rotation(
                mat.iter().map(|v| v.to_vec()).collect(),
                target.iter().map(|v| v.to_vec()).collect(),
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[&[0, 1], &[1, 0]], &[&[1, 0], &[0, 1]], true);
    }

    #[test]
    fn ex2() {
        test(&[&[0, 1], &[1, 1]], &[&[1, 0], &[0, 1]], false);
    }

    #[test]
    fn ex3() {
        test(
            &[&[0, 0, 0], &[0, 1, 0], &[1, 1, 1]],
            &[&[1, 1, 1], &[0, 1, 0], &[0, 0, 0]],
            true,
        );
    }

    #[test]
    fn failing_case1() {
        test(
            &[&[0, 0, 0], &[0, 0, 1], &[0, 0, 1]],
            &[&[0, 0, 0], &[0, 0, 1], &[0, 0, 1]],
            true,
        );
    }
}
