// https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections/

pub struct Solution;

// Painfully naive sol'n
// impl Solution {
//     pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
//         fn check_valid_cuts_1d(n: i32, segments: &[[i32; 2]]) -> bool {
//             if segments.len() < 3 {
//                 return false;
//             }
//             let mut first_end = n;
//             let mut last_start = 0;
//             for &[start, end] in segments {
//                 if start > last_start {
//                     last_start = start;
//                 }
//                 if end < first_end {
//                     first_end = end;
//                 }
//             }
//             // Find valid split locations
//             let mut splits = (first_end..=last_start).collect::<Vec<i32>>();
//             for &[start, end] in segments {
//                 splits.retain(|&x| x <= start || x >= end);
//                 if splits.len() < 2 {
//                     return false;
//                 }
//             }
//             // Finally, check any segment falls
//             // between the min and max splits.
//             let min_split = splits[0];
//             let max_split = splits[splits.len() - 1];
//             for &[start, _] in segments {
//                 if start >= min_split && start < max_split {
//                     return true;
//                 }
//             }
//             false
//         }
//         let vertical = rectangles.iter().map(|r| [r[0], r[2]]).collect::<Vec<_>>();
//         let horizontal = rectangles.iter().map(|r| [r[1], r[3]]).collect::<Vec<_>>();
//         check_valid_cuts_1d(n, &vertical) || check_valid_cuts_1d(n, &horizontal)
//     }
// }

// Merge segments sol'n (Solved)
// impl Solution {
//     pub fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
//         fn check_valid_cuts_1d(segments: &mut [[i32; 2]]) -> bool {
//             if segments.len() < 3 {
//                 return false;
//             }
//             segments.sort();
//             let mut merged_segments = vec![segments[0]];
//             for &[start, end] in &segments[1..] {
//                 if start >= merged_segments.last().unwrap()[1] {
//                     merged_segments.push([start, end]);
//                 } else {
//                     let last_end = &mut merged_segments.last_mut().unwrap()[1];
//                     if end > *last_end {
//                         *last_end = end;
//                     }
//                 }
//             }
//             merged_segments.len() >= 3
//         }
//         if rectangles.len() < 3 {
//             return false;
//         }
//         check_valid_cuts_1d(&mut rectangles.iter().map(|r| [r[0], r[2]]).collect::<Vec<_>>())
//             || check_valid_cuts_1d(&mut rectangles.iter().map(|r| [r[1], r[3]]).collect::<Vec<_>>())
//     }
// }

// In-place merge segments sol'n
impl Solution {
    pub fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        fn check_valid_cuts_1d(segments: &mut [[i32; 2]]) -> bool {
            if segments.len() < 3 {
                return false;
            }
            segments.sort();
            let mut write = 0u32;
            for read in 1..segments.len() as u32 {
                let start = segments[read as usize][0];
                let end = segments[read as usize][1];
                let last_end = &mut segments[write as usize][1];
                if start >= *last_end {
                    write += 1;
                    if write >= 2 {
                        return true;
                    }
                    segments[write as usize] = [start, end];
                } else if *last_end < end {
                    *last_end = end;
                }
            }
            false
        }
        if rectangles.len() < 3 {
            return false;
        }
        check_valid_cuts_1d(&mut rectangles.iter().map(|r| [r[0], r[2]]).collect::<Vec<_>>())
            || check_valid_cuts_1d(&mut rectangles.iter().map(|r| [r[1], r[3]]).collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, rectangles: &[[i32; 4]], expected: bool) {
        assert!(n >= 3);
        assert!(n <= 1_000_000_000);
        assert!(rectangles.len() >= 3);
        assert!(rectangles.len() <= 100_000);
        for &rectangle in rectangles {
            assert!(rectangle[0] >= 0);
            assert!(rectangle[1] >= 0);
            assert!(rectangle[2] > rectangle[0]);
            assert!(rectangle[3] > rectangle[1]);
            assert!(rectangle[2] <= n);
            assert!(rectangle[3] <= n);
        }
        assert_eq!(
            Solution::check_valid_cuts(n as i32, rectangles.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            5,
            &[[1, 0, 5, 2], [0, 2, 2, 4], [3, 2, 5, 3], [0, 4, 4, 5]],
            true,
        )
    }

    #[test]
    fn ex2() {
        test(
            4,
            &[[0, 0, 1, 1], [2, 0, 3, 4], [0, 2, 2, 3], [3, 0, 4, 3]],
            true,
        );
    }

    #[test]
    fn ex3() {
        test(
            4,
            &[
                [0, 2, 2, 4],
                [1, 0, 3, 2],
                [2, 2, 3, 4],
                [3, 0, 4, 2],
                [3, 2, 4, 4],
            ],
            false,
        );
    }

    #[test]
    fn discussion_case1() {
        test(
            5,
            &[
                [0, 2, 2, 4],
                [1, 0, 3, 2],
                [2, 2, 3, 4],
                [3, 0, 4, 2],
                [3, 2, 4, 4],
            ],
            false,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            6,
            &[
                [0, 0, 6, 1],
                [0, 1, 3, 5],
                [3, 1, 4, 5],
                [4, 1, 5, 5],
                [5, 1, 6, 5],
                [0, 5, 6, 6],
            ],
            true,
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            7,
            &[
                [0, 0, 3, 5],
                [3, 0, 4, 5],
                [4, 0, 5, 5],
                [5, 0, 6, 5],
                [6, 0, 7, 5],
                [0, 5, 7, 6],
                [0, 6, 7, 7],
            ],
            true,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            9,
            &[
                [0, 0, 1, 7],
                [1, 0, 5, 6],
                [5, 0, 7, 6],
                [7, 0, 8, 6],
                [8, 0, 9, 6],
                [1, 6, 9, 7],
                [0, 7, 9, 8],
                [0, 8, 9, 9],
            ],
            true,
        )
    }
}
