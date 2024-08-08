// https://leetcode.com/problems/spiral-matrix-iii/

pub struct Solution;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        assert!(rows >= 1);
        assert!(cols >= 1);
        assert!(rows <= 100);
        assert!(cols <= 100);
        assert!(r_start >= 0);
        assert!(c_start >= 0);
        assert!(r_start < rows);
        assert!(c_start < cols);
        let rows = rows as i8;
        let cols = cols as i8;
        let r_start = r_start as i16;
        let c_start = c_start as i16;
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut top = r_start;
        let mut bottom = r_start;
        let mut left = c_start;
        let mut right = c_start;
        while result.len() < rows as usize * cols as usize {
            if top >= 0 {
                for i in std::cmp::max(left, 0)..=std::cmp::min(right, cols as i16- 1) {
                    result.push(vec![top as i32, i as i32]);
                }
            }
            right += 1;
            if right < cols as i16{
                for i in std::cmp::max(top, 0)..=std::cmp::min(bottom, rows as i16- 1) {
                    result.push(vec![i as i32, right as i32]);
                }
            }
            bottom += 1;
            if bottom < rows as i16{
                for i in (std::cmp::max(left, 0)..=std::cmp::min(right, cols as i16- 1)).rev() {
                    result.push(vec![bottom as i32, i as i32]);
                }
            }
            left -= 1;
            if left >= 0 {
                for i in (std::cmp::max(top, 0)..=std::cmp::min(bottom, rows as i16- 1)).rev() {
                    result.push(vec![i as i32, left as i32]);
                }
            }
            top -= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(rows: u8, cols: u8, r_start: u8, c_start: u8, expected: &[[i32; 2]]) {
        assert!(rows >= 1);
        assert!(cols >= 1);
        assert!(rows <= 100);
        assert!(cols <= 100);
        assert!(r_start < rows);
        assert!(c_start < cols);
        assert_eq!(
            Solution::spiral_matrix_iii(rows as i32, cols as i32, r_start as i32, c_start as i32),
            expected
        );
    }

    fn load_result(data: &str) -> Vec<[i32; 2]> {
        // data format in the file:
        // [[1,2],[30,40],[9,7]]
        let mut result = Vec::new();
        let data = data.trim();
        let data = &data[2..data.len() - 2];
        for pair in data.split("],[") {
            let mut pair = pair.split(",");
            let x = pair.next().unwrap().parse().unwrap();
            let y = pair.next().unwrap().parse().unwrap();
            result.push([x, y]);
        }
        result
    }

    #[test]
    fn ex1() {
        test(1, 4, 0, 0, &[[0, 0], [0, 1], [0, 2], [0, 3]])
    }

    #[test]
    fn ex2() {
        test(
            5,
            6,
            1,
            4,
            &[
                [1, 4],
                [1, 5],
                [2, 5],
                [2, 4],
                [2, 3],
                [1, 3],
                [0, 3],
                [0, 4],
                [0, 5],
                [3, 5],
                [3, 4],
                [3, 3],
                [3, 2],
                [2, 2],
                [1, 2],
                [0, 2],
                [4, 5],
                [4, 4],
                [4, 3],
                [4, 2],
                [4, 1],
                [3, 1],
                [2, 1],
                [1, 1],
                [0, 1],
                [4, 0],
                [3, 0],
                [2, 0],
                [1, 0],
                [0, 0],
            ],
        )
    }

    #[test]
    fn myex0() {
        test(1, 1, 0, 0, &[[0, 0]]);
    }

    #[test]
    fn myex1_1() {
        test(1, 4, 0, 1, &[[0, 1], [0, 2], [0, 0], [0, 3]]);
    }

    #[test]
    fn myex1_2() {
        test(1, 4, 0, 2, &[[0, 2], [0, 3], [0, 1], [0, 0]]);
    }

    #[test]
    fn myex1_3() {
        test(1, 4, 0, 3, &[[0, 3], [0, 2], [0, 1], [0, 0]]);
    }

    #[test]
    fn discussion_case1() {
        let expected = load_result(include_str!("discussion_case1.txt"));
        test(100, 100, 1, 1, &expected)
    }

    #[test]
    fn discussion_case2() {
        test(1, 5, 0, 0, &[[0, 0], [0, 1], [0, 2], [0, 3], [0, 4]])
    }

    #[test]
    fn discussion_case3() {
        let expected = load_result(include_str!("discussion_case3.txt"));
        test(100, 100, 50, 50, &expected)
    }

    #[test]
    fn discussion_case4() {
        test(
            4,
            4,
            1,
            1,
            &[
                [1, 1],
                [1, 2],
                [2, 2],
                [2, 1],
                [2, 0],
                [1, 0],
                [0, 0],
                [0, 1],
                [0, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [3, 3],
                [3, 2],
                [3, 1],
                [3, 0],
            ],
        )
    }

    #[test]
    fn my_extreme_ex1() {
        let expected = load_result(include_str!("my_extreme_ex1.txt"));
        test(100, 100, 99, 99, &expected)
    }
}
