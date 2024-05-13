// https://leetcode.com/problems/score-after-flipping-matrix/

pub struct Solution;

impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let num_count = grid.len();
        assert!(num_count > 0);
        assert!(num_count <= 20);
        let num_count = num_count as u8;
        let max_bit = grid[0].len();
        assert!(max_bit > 0);
        assert!(max_bit <= 20);
        let max_bit = max_bit as u8;
        let max_bit_mask = 1i32 << (max_bit - 1);
        let negate_mask = !(-1 << max_bit);
        // Convert the grid to u32 numbers first
        // because it's faster to work within that
        // tiny memory footprint.
        let mut nums: Vec<i32> = grid
            .into_iter()
            .map(|row| {
                let mut num: i32 = 0;
                for (idx, bit) in row.into_iter().rev().enumerate() {
                    if bit == 1 {
                        num |= 1 << idx;
                    }
                }
                num
            })
            .collect();
        // All rows should start with 1s to maximize
        // their unsigned values
        for num in nums.iter_mut() {
            if *num & max_bit_mask == 0 {
                *num ^= negate_mask;
            }
        }
        // For each column, if the column contains
        // more 0s than 1s, flip the column.
        let mut to_flip = 0i32;
        for col in 0..max_bit {
            let mut zero_count = 0u8;
            for num in nums.iter() {
                zero_count += if (num >> col) & 1 == 0 { 1 } else { 0 };
            }
            if zero_count > num_count / 2 {
                to_flip |= 1 << col;
            }
        }
        for num in nums.iter_mut() {
            *num ^= to_flip;
        }
        // Output our result
        nums.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[u8]], expected: u32) {
        assert!(grid.len() > 0);
        assert!(grid.len() <= 20);
        assert!(grid[0].len() > 0);
        assert!(grid[0].len() <= 20);
        for &row in grid {
            assert_eq!(row.len(), grid[0].len());
            for &cell in row {
                assert!(cell == 0 || cell == 1);
            }
        }
        assert!(expected > 0);
        assert!(expected < i32::MAX as u32);
        let grid: Vec<Vec<i32>> = grid
            .into_iter()
            .map(|r| r.into_iter().map(|&x| x as i32).collect())
            .collect();
        assert_eq!(Solution::matrix_score(grid), expected as i32);
    }

    #[test]
    fn ex0() {
        test(&[&[0]], 1)
    }

    #[test]
    fn ex1() {
        test(&[&[0, 0, 1, 1], &[1, 0, 1, 0], &[1, 1, 0, 0]], 39)
    }

    #[test]
    fn discussion_case1() {
        test(&[&[0, 1, 1], &[1, 1, 1], &[1, 1, 1]], 4 + 7 + 7)
    }

    #[test]
    fn discussion_case2() {
        test(&[&[1, 1, 1], &[1, 1, 1], &[1, 1, 1]], 7 + 7 + 7)
    }

    #[test]
    fn discussion_case3() {
        test(&[&[0, 0, 0], &[0, 0, 0], &[0, 0, 0]], 7 + 7 + 7)
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                &[1, 0, 0, 0, 0],
                &[1, 1, 1, 0, 1],
                &[1, 0, 1, 1, 0],
                &[1, 1, 1, 1, 0],
                &[0, 1, 0, 1, 0],
            ],
            129,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                &[0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0],
                &[0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1],
                &[0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1],
                &[1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0],
                &[1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1],
                &[0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0],
                &[0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1],
                &[1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1],
                &[1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0],
                &[0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
                &[1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0],
                &[1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1],
                &[1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1],
                &[1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0],
                &[1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1],
                &[1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0],
                &[1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0],
                &[1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0],
                &[1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1],
                &[1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1],
            ],
            16509997,
        )
    }
}
