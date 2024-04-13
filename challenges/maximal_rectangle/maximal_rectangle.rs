// https://leetcode.com/problems/maximal-rectangle/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
//         const fn mapc(c: char) -> u8 {
//             match c {
//                 '0' => 0,
//                 '1' => 1,
//                 _ => unreachable!(),
//             }
//         }
//         fn row_to_bytes(row: Vec<char>) -> Vec<u8> {
//             row.into_iter().map(mapc).collect()
//         }
//         fn max_rectangle_in_histogram(hist: &[u8]) -> u16 {
//             let mut max_area = 0;
//             let mut stack = vec![0];
//             for i in 1..hist.len() as u8 {
//                 let mut rectangle_start = *stack.last().unwrap();
//                 while hist[rectangle_start as usize] > hist[i as usize] {
//                     let rectangle_height = hist[stack.pop().unwrap() as usize];
//                     rectangle_start = *stack.last().unwrap();
//                     let area: u16 = (i - rectangle_start - 1) as u16 * rectangle_height as u16;
//                     max_area = std::cmp::max(max_area, area);
//                 }
//                 stack.push(i);
//             }
//             max_area
//         }
//         let mut histogram = vec![0; matrix[0].len() + 2];
//         let mut max_area = 0;
//         for row in matrix {
//             let row = row_to_bytes(row);
//             for i in 0..row.len() {
//                 histogram[i + 1] = if row[i] == 0 { 0 } else { histogram[i + 1] + 1 };
//             }
//             max_area = std::cmp::max(max_area, max_rectangle_in_histogram(&histogram));
//         }
//         max_area as i32
//     }
// }

// Don't convert to bytes in the middle
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        fn max_rectangle_in_histogram(hist: &[u8]) -> u16 {
            let mut max_area = 0;
            let mut stack = vec![0];
            for i in 1..hist.len() as u8 {
                let mut rectangle_start = *stack.last().unwrap();
                while hist[rectangle_start as usize] > hist[i as usize] {
                    let rectangle_height = hist[stack.pop().unwrap() as usize];
                    rectangle_start = *stack.last().unwrap();
                    let area: u16 = (i - rectangle_start - 1) as u16 * rectangle_height as u16;
                    max_area = std::cmp::max(max_area, area);
                }
                stack.push(i);
            }
            max_area
        }
        let mut histogram = vec![0; matrix[0].len() + 2];
        let mut max_area = 0;
        for row in matrix {
            for i in 0..row.len() {
                histogram[i + 1] = if row[i] == '0' {
                    0
                } else {
                    histogram[i + 1] + 1
                };
            }
            max_area = std::cmp::max(max_area, max_rectangle_in_histogram(&histogram));
        }
        max_area as i32
    }
}

// Add constraints to let the optimizer run wild (Somehow slower)
// impl Solution {
//     pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
//         assert!(matrix.len() >= 1);
//         assert!(matrix.len() <= 200);
//         fn max_rectangle_in_histogram(hist: &[u8]) -> u16 {
//             let mut max_area = 0;
//             let mut stack = vec![0];
//             for i in 1..hist.len() as u8 {
//                 let mut rectangle_start = *stack.last().unwrap();
//                 while hist[rectangle_start as usize] > hist[i as usize] {
//                     let rectangle_height = hist[stack.pop().unwrap() as usize];
//                     rectangle_start = *stack.last().unwrap();
//                     let area: u16 = (i - rectangle_start - 1) as u16 * rectangle_height as u16;
//                     max_area = std::cmp::max(max_area, area);
//                 }
//                 stack.push(i);
//             }
//             max_area
//         }
//         let rowlen = matrix[0].len();
//         assert!(rowlen >= 1);
//         assert!(rowlen <= 200);
//         let mut histogram = vec![0; rowlen + 2];
//         let mut max_area = 0;
//         for row in matrix {
//             debug_assert_eq!(row.len(), rowlen);
//             for i in 0..row.len() {
//                 histogram[i + 1] = match row[i] {
//                     '0' => 0,
//                     '1' => histogram[i + 1] + 1,
//                     _ => unreachable!(),
//                 };
//             }
//             max_area = std::cmp::max(max_area, max_rectangle_in_histogram(&histogram));
//         }
//         max_area as i32
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(matrix: Vec<Vec<&str>>, expected: i32) {
        let matrix = matrix
            .iter()
            .map(|row| row.iter().map(|&s| s.chars().next().unwrap()).collect())
            .collect();
        assert_eq!(Solution::maximal_rectangle(matrix), expected);
    }

    #[test]
    fn ex1() {
        test(
            vec![
                vec!["1", "0", "1", "0", "0"],
                vec!["1", "0", "1", "1", "1"],
                vec!["1", "1", "1", "1", "1"],
                vec!["1", "0", "0", "1", "0"],
            ],
            6,
        )
    }

    #[test]
    fn ex2() {
        test(vec![vec!["0"]], 0)
    }

    #[test]
    fn ex3() {
        test(vec![vec!["1"]], 1)
    }

    #[test]
    fn discussion_case1() {
        test(
            vec![
                vec!["0", "1", "0", "0"],
                vec!["0", "1", "1", "1"],
                vec!["1", "1", "1", "0"],
                vec!["0", "0", "1", "0"],
            ],
            4,
        )
    }

    #[test]
    fn discussion_case1_flip() {
        test(
            vec![
                vec!["0", "0", "1", "0"],
                vec!["1", "1", "1", "0"],
                vec!["0", "1", "1", "1"],
                vec!["0", "1", "0", "0"],
            ],
            4,
        )
    }

    #[test]
    fn discussion_case2() {
        test(vec![vec!["0", "0"], vec!["0", "1"]], 1)
    }

    #[test]
    fn discussion_case2_vflip() {
        test(vec![vec!["0", "1"], vec!["0", "0"]], 1)
    }

    #[test]
    fn discussion_case2_hflip() {
        test(vec![vec!["0", "0"], vec!["1", "0"]], 1)
    }

    #[test]
    fn discussion_case2_180() {
        test(vec![vec!["1", "0"], vec!["0", "0"]], 1)
    }

    #[test]
    fn myex0() {
        test(vec![vec!["0", "0"], vec!["0", "0"]], 0)
    }

    #[test]
    fn myex1() {
        test(
            vec![
                vec!["0", "0", "1", "0"],
                vec!["0", "1", "1", "1"],
                vec!["1", "1", "1", "0"],
                vec!["1", "1", "0", "0"],
            ],
            4,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        test(vec![vec!["1"; 200]; 200], 200 * 200)
    }

    #[test]
    fn my_extreme_ex2() {
        test(vec![["1", "0"].repeat(100); 200], 200)
    }

    #[test]
    fn my_extreme_ex3() {
        test(vec![["1", "0", "1"].repeat(33); 200], 400)
    }
}
