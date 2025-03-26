// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/

pub struct Solution;

// Select nth unstable sol'n
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let x = x as u16;
        let mut flattened = grid.into_iter().flatten().collect::<Vec<_>>();
        let len = flattened.len();
        let (_, median, _) = flattened.select_nth_unstable(len / 2);
        let median = *median;
        // TODO: Calculate the true median for even len
        let mut operations = 0u32;
        for el in flattened {
            let diff = i32::abs_diff(median, el);
            let ops = diff / x as u32;
            let rem = diff % x as u32;
            if rem != 0 {
                return -1;
            }
            operations += ops;
        }
        operations as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[u16]], x: u16, expected: i32) {
        assert!(x >= 1);
        assert!(x <= 10_000);
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 100_000);
        for &row in grid {
            assert!(row.len() >= 1);
            assert!(row.len() <= 100_000);
            assert!(grid.len() * row.len() >= 1);
            assert!(grid.len() * row.len() <= 100_000);
            for &el in row {
                assert!(el >= 1);
                assert!(el <= 10_000);
                if expected != -1 {
                    assert!(el % x == grid[0][0] % x);
                }
            }
        }
        assert_eq!(
            Solution::min_operations(
                grid.iter()
                    .map(|&x| x.iter().map(|&x| x as i32).collect())
                    .collect(),
                x as i32
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[&[2, 4], &[6, 8]], 2, 4)
    }

    #[test]
    fn ex2() {
        test(&[&[1, 5], &[2, 3]], 1, 5)
    }

    #[test]
    fn ex3() {
        test(&[&[1, 2], &[3, 4]], 2, -1)
    }

    #[test]
    fn ex3_1() {
        test(&[&[1, 2], &[3, 4]], 1, 4)
    }

    #[test]
    fn myex1() {
        test(&[&[2, 2], &[4, 4]], 2, 2)
    }

    #[test]
    fn myex2() {
        test(&[&[2, 2], &[8, 8]], 2, 6)
    }

    #[test]
    fn myex3() {
        test(&[&[2, 2], &[8, 8]], 3, 4)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                &[980, 476, 644, 56],
                &[644, 140, 812, 308],
                &[812, 812, 896, 560],
                &[728, 476, 56, 812],
            ],
            84,
            45,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                &[596, 904, 960, 232, 120, 932, 176],
                &[372, 792, 288, 848, 960, 960, 764],
                &[652, 92, 904, 120, 680, 904, 120],
                &[372, 960, 92, 680, 876, 624, 904],
                &[176, 652, 64, 344, 316, 764, 316],
                &[820, 624, 848, 596, 960, 960, 372],
                &[708, 120, 456, 92, 484, 932, 540],
            ],
            28,
            473,
        )
    }
}
