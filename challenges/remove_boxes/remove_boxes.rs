// https://leetcode.com/problems/remove-boxes/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
//         fn recurse(boxes: &[i32], mut i: u8, j: u8, mut k: u8, dp: &mut [Vec<Vec<i32>>]) -> i32 {
//             if i > j {
//                 return 0;
//             }
//             if dp[i as usize][j as usize][k as usize] != 0 {
//                 return dp[i as usize][j as usize][k as usize];
//             }
//             while i < j && boxes[i as usize] == boxes[i as usize+1] {
//                 i += 1;
//                 k += 1;
//             }
//             let mut res = (k as i32+1)*(k as i32+1) + recurse(boxes, i+1, j, 0, dp);
//             for m in i+1..=j {
//                 if boxes[i as usize] == boxes[m as usize] {
//                     let new_res = recurse(boxes, i+1, m-1, 0, dp) + recurse(boxes, m, j, k+1, dp);
//                     if new_res > res {
//                         res = new_res;
//                     }
//                 }
//             }
//             dp[i as usize][j as usize][k as usize] = res;
//             res
//         }
//         assert!(boxes.len() >= 1);
//         assert!(boxes.len() <= 100);
//         let n = boxes.len() as u8;
//         let mut dp = vec![vec![vec![0; n as usize]; n as usize]; n as usize];
//         recurse(&boxes, 0, n as u8-1, 0, &mut dp)
//     }
// }

// Initial sol'n despite optimizations
impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        type Idx = u8;
        fn recurse(boxes: &[i32], i: Idx, j: Idx, k: Idx, dp: &mut [Vec<Vec<i32>>]) -> i32 {
            if i > j {
                return 0;
            }
            if dp[i as usize][j as usize][k as usize] != 0 {
                return dp[i as usize][j as usize][k as usize];
            }
            let mut res = (k as i32 + 1) * (k as i32 + 1) + recurse(boxes, i + 1, j, 0, dp);
            for m in i + 1..=j {
                if boxes[i as usize] == boxes[m as usize] {
                    let new_res =
                        recurse(boxes, i + 1, m - 1, 0, dp) + recurse(boxes, m, j, k + 1, dp);
                    if new_res > res {
                        res = new_res;
                    }
                }
            }
            dp[i as usize][j as usize][k as usize] = res;
            res
        }
        assert!(boxes.len() >= 1);
        assert!(boxes.len() <= 100);
        let n = boxes.len() as Idx;
        let mut dp = vec![vec![vec![0; n as usize]; n as usize]; n as usize];
        recurse(&boxes, 0, n - 1, 0, &mut dp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(boxes: &[u8], expected: u16) {
        assert!(boxes.len() >= 1);
        assert!(boxes.len() <= 100);
        for &b in boxes {
            assert!(b >= 1);
            assert!(b <= 100);
        }
        assert!(expected >= 1);
        assert!(expected <= 100 * 100);
        assert_eq!(
            Solution::remove_boxes(boxes.iter().map(|&x| x as i32).collect()),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 3, 2, 2, 2, 3, 4, 3, 1], 23)
    }

    #[test]
    fn ex2() {
        test(&[1, 1, 1], 9)
    }

    #[test]
    fn ex3() {
        test(&[1], 1)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 2, 1, 2, 1], 11)
    }

    #[test]
    fn tle_case1() {
        test(
            &[
                1, 2, 2, 1, 1, 1, 2, 1, 1, 2, 1, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 1, 2, 2, 2, 2, 1,
                2, 1, 1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 1, 2, 2, 1, 1, 1, 2, 2, 1, 2, 1, 2, 2,
                1, 2, 1, 1, 1, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1,
                1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1,
            ],
            2758,
        )
    }

    #[test]
    fn myex1() {
        test(
            &[
                1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
                1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
                1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
                1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
            ],
            2550,
        )
    }

    #[test]
    fn myex2() {
        test(
            &[
                1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2,
                1, 2, 2, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2,
                2, 2, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 2,
                2, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 2, 2, 1, 2,
            ],
            4260,
        )
    }

    #[test]
    fn myex3() {
        test(
            &[
                1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1,
            ],
            7758,
        )
    }
}
