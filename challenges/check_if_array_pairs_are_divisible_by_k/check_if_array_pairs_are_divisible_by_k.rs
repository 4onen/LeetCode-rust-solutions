// https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
//         assert!(k >= 1);
//         let k = k as usize;
//         let mut divis_by = vec![0; k];
//         for n in arr {
//             divis_by[(n.rem_euclid(k as i32)) as usize] += 1;
//         }
//         for i in 1..k / 2 + 1 {
//             if divis_by[i] != divis_by[k - i] {
//                 return false;
//             }
//         }
//         divis_by[0] % 2 == 0
//     }
// }

// Optimized sol'n (precheck divis_by[0] % 2) (no faster, dang)
// impl Solution {
//     pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
//         assert!(k >= 1);
//         let k = k as usize;
//         let mut divis_by = vec![0; k];
//         for n in arr {
//             divis_by[(n.rem_euclid(k as i32)) as usize] += 1;
//         }
//         if divis_by[0] % 2 != 0 {
//             return false;
//         }
//         for i in 1..k / 2 + 1 {
//             if divis_by[i] != divis_by[k - i] {
//                 return false;
//             }
//         }
//         true
//     }
// }

// Optimized sol'n: Change remainder op to double modulo op
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        assert!(k >= 1);
        let mut divis_by = vec![0; k as usize];
        for n in arr {
            divis_by[(((n % k) + k) % k) as usize] += 1;
        }
        if divis_by[0] % 2 != 0 {
            return false;
        }
        for i in 1..k / 2 + 1 {
            if divis_by[i as usize] != divis_by[(k - i) as usize] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr: &[i32], k: i32, expected: bool) {
        assert!(arr.len() >= 1);
        assert!(arr.len() <= 100_000);
        assert!(arr.len() % 2 == 0);
        for &n in arr {
            assert!(n >= -1_000_000_000);
            assert!(n <= 1_000_000_000);
        }
        assert!(k >= 1);
        assert!(k <= 100_000);
        assert_eq!(Solution::can_arrange(arr.to_vec(), k), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5, true)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4, 5, 6], 7, true)
    }

    #[test]
    fn ex3() {
        test(&[1, 2, 3, 4, 5, 6], 10, false)
    }

    #[test]
    fn myex1() {
        test(&[1, 2], 3, true)
    }

    #[test]
    fn myex2() {
        test(&[1, 2], 5, false)
    }

    #[test]
    fn failing_case1() {
        test(&[-1, 1, -2, 2, -3, 3, -4, 4], 3, true)
        // 1 - 1
        // 2 - 2
        // 3 - 3
        // 4 - 4
    }

    #[test]
    fn failing_case2() {
        test(&[-1, -1, -1, -1, 2, 2, -2, -2], 3, false)
    }
}
