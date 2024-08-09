// https://leetcode.com/problems/magic-squares-in-grid/

pub struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        fn is_magic_square(grid: &[Vec<i32>], i: u8, j: u8) -> bool {
            const ALL: u16 = 0b1111111110;
            let mut nums = 0u16;
            let mut ix = i;
            let mut jy = j;
            if ix + 2 >= grid.len() as u8 || jy + 2 >= grid[0].len() as u8 {
                return false;
            }
            while ix < i + 3 {
                while jy < j + 3 {
                    let num = grid[ix as usize][jy as usize];
                    if nums & (1 << num) != 0 {
                        return false;
                    }
                    nums |= 1 << num;
                    jy += 1;
                }
                jy = j;
                ix += 1;
            }
            if nums != ALL {
                return false;
            }
            let sum = grid[i as usize][j as usize]
                + grid[i as usize][j as usize + 1]
                + grid[i as usize][j as usize + 2];
            for ix in i..i + 3 {
                if grid[ix as usize][j as usize]
                    + grid[ix as usize][j as usize + 1]
                    + grid[ix as usize][j as usize + 2]
                    != sum
                {
                    return false;
                }
            }
            for jy in j..j + 3 {
                if grid[i as usize][jy as usize]
                    + grid[i as usize + 1][jy as usize]
                    + grid[i as usize + 2][jy as usize]
                    != sum
                {
                    return false;
                }
            }
            if grid[i as usize][j as usize]
                + grid[i as usize + 1][j as usize + 1]
                + grid[i as usize + 2][j as usize + 2]
                != sum
            {
                return false;
            }
            if grid[i as usize][j as usize + 2]
                + grid[i as usize + 1][j as usize + 1]
                + grid[i as usize + 2][j as usize]
                != sum
            {
                return false;
            }
            true
        }
        let mut count = 0;
        if grid.len() < 3 || grid[0].len() < 3 {
            return 0;
        }
        for i in 0..grid.len() - 2 {
            for j in 0..grid[0].len() - 2 {
                if is_magic_square(&grid, i as u8, j as u8) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], expected: i32) {
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 10);
        assert!(grid[0].len() >= 1);
        assert!(grid[0].len() <= 10);
        for row in grid {
            assert_eq!(row.len(), grid[0].len());
            for &num in *row {
                assert!(num >= 0);
                assert!(num <= 15);
            }
        }
        let grid = grid.iter().map(|row| row.to_vec()).collect();
        assert_eq!(Solution::num_magic_squares_inside(grid), expected)
    }

    #[test]
    fn ex1() {
        test(&[&[4, 3, 8, 4], &[9, 5, 1, 9], &[2, 7, 6, 2]], 1)
    }

    #[test]
    fn ex2() {
        test(&[&[8]], 0)
    }

    #[test]
    fn discussion_all_possible_magic_squares() {
        let squares = [
            [[4, 9, 2], [3, 5, 7], [8, 1, 6]],
            [[2, 7, 6], [9, 5, 1], [4, 3, 8]],
            [[6, 1, 8], [7, 5, 3], [2, 9, 4]],
            [[8, 3, 4], [1, 5, 9], [6, 7, 2]],
            [[4, 3, 8], [9, 5, 1], [2, 7, 6]],
            [[2, 9, 4], [7, 5, 3], [6, 1, 8]],
            [[6, 7, 2], [1, 5, 9], [8, 3, 4]],
            [[8, 1, 6], [3, 5, 7], [4, 9, 2]],
        ];
        for square in squares {
            let square_row_refs = square.iter().map(|row| row.as_ref()).collect::<Vec<_>>();
            test(&square_row_refs, 1)
        }
    }

    #[test]
    fn discussion_case1() {
        test(&[&[1, 1, 1], &[1, 1, 1], &[1, 1, 1]], 0)
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                &[9, 9, 5, 1, 9, 5, 5, 7, 2, 5],
                &[9, 1, 8, 3, 4, 6, 7, 2, 8, 9],
                &[4, 1, 1, 5, 9, 1, 5, 9, 6, 4],
                &[5, 5, 6, 7, 2, 8, 3, 4, 0, 6],
                &[1, 9, 1, 8, 3, 1, 4, 2, 9, 4],
                &[2, 8, 6, 4, 2, 7, 3, 2, 7, 6],
                &[9, 2, 5, 0, 7, 8, 2, 9, 5, 1],
                &[2, 1, 4, 4, 7, 6, 2, 4, 3, 8],
                &[1, 2, 5, 3, 0, 5, 10, 8, 5, 2],
                &[6, 9, 6, 8, 8, 4, 3, 6, 0, 9],
            ],
            3,
        )
    }

    #[test]
    fn discussion_case3() {
        test(&[&[7, 0, 5], &[2, 4, 6], &[3, 8, 1]], 0)
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                &[4, 3, 8, 4],
                &[9, 5, 1, 9],
                &[2, 7, 6, 2],
                &[4, 3, 8, 1],
                &[1, 6, 7, 5],
            ],
            1,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                &[4, 3, 8, 1, 6, 7],
                &[9, 5, 1, 7, 8, 9],
                &[2, 7, 6, 4, 9, 1],
                &[8, 1, 6, 4, 3, 8],
                &[3, 7, 9, 2, 8, 1],
                &[4, 3, 8, 1, 6, 7],
                &[9, 5, 1, 7, 8, 9],
                &[2, 7, 6, 4, 9, 1],
                &[8, 1, 6, 4, 3, 8],
                &[1, 6, 7, 8, 9, 2],
            ],
            2,
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            &[
                &[4, 3, 8, 4, 3, 8],
                &[9, 5, 1, 9, 5, 1],
                &[2, 7, 6, 2, 7, 6],
                &[4, 3, 8, 4, 3, 8],
                &[9, 5, 1, 9, 5, 1],
                &[2, 7, 6, 2, 7, 6],
                &[4, 3, 8, 4, 3, 8],
                &[9, 5, 1, 9, 5, 1],
                &[2, 7, 6, 2, 7, 6],
            ],
            6,
        )
    }
}
