// https://leetcode.com/problems/making-a-large-island/

pub struct Solution;

impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        // Connected component indices start from 2.
        // We can't have more than 500*500/2 so this handily
        // fits into the i32 of the original grid. So let's
        // fill every cell of the original grid with a cc id
        // for its connected landmass.
        let n = grid.len() as u16;
        let mut cc_areas = vec![0, 0];
        let mut max_cc_area = 0;
        let mut cc_dfs_stack = vec![];
        let mut had_empty = false;
        for i in 0..n {
            for j in 0..n {
                had_empty |= grid[i as usize][j as usize] < 1;
                if grid[i as usize][j as usize] == 1 {
                    let mut cc_size = 0;
                    cc_dfs_stack.push((i, j));
                    while let Some((i, j)) = cc_dfs_stack.pop() {
                        if grid[i as usize][j as usize] == 1 {
                            grid[i as usize][j as usize] = cc_areas.len() as i32;
                            cc_size += 1;
                            if i > 0 {
                                cc_dfs_stack.push((i - 1, j));
                            }
                            if j > 0 {
                                cc_dfs_stack.push((i, j - 1));
                            }
                            if i + 1 < n {
                                cc_dfs_stack.push((i + 1, j));
                            }
                            if j + 1 < n {
                                cc_dfs_stack.push((i, j + 1));
                            }
                        }
                    }
                    cc_areas.push(cc_size);
                    if cc_size > max_cc_area {
                        max_cc_area = cc_size;
                    }
                }
            }
        }
        if max_cc_area == 0 {
            return 1;
        }
        if cc_areas.len() < 4 {
            return max_cc_area as i32 + had_empty as i32;
        }
        let mut max_merge = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i as usize][j as usize] == 0 {
                    let mut cc_size = 1;
                    let mut ccs_seen = [0;4];
                    if i > 0 {
                        let cc = grid[i as usize - 1][j as usize];
                        ccs_seen[0] = cc;
                        if cc > 1 {
                            cc_size += cc_areas[cc as usize];
                        }
                    }
                    if j > 0 {
                        let cc = grid[i as usize][j as usize - 1];
                        ccs_seen[1] = cc;
                        if ccs_seen[0] != cc && cc > 1 {
                            cc_size += cc_areas[cc as usize];
                        }
                    }
                    if i + 1 < n {
                        let cc = grid[1 + i as usize][j as usize];
                        ccs_seen[2] = cc;
                        if ccs_seen[0] != cc && ccs_seen[1] != cc && cc > 1 {
                            cc_size += cc_areas[cc as usize];
                        }
                    }
                    if j + 1 < n {
                        let cc = grid[i as usize][1 + j as usize];
                        ccs_seen[3] = cc;
                        if ccs_seen[0] != cc && ccs_seen[1] != cc && ccs_seen[2] != cc && cc > 1 {
                            cc_size += cc_areas[cc as usize];
                        }
                    }
                    if cc_size > max_merge {
                        max_merge = cc_size;
                    }
                }
            }
        }
        max_merge
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], expected: i32) {
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 500);
        for &row in grid {
            assert_eq!(row.len(), grid.len());
            for &el in row {
                assert!(el == 0 || el == 1);
            }
        }
        assert_eq!(
            Solution::largest_island(grid.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[&[1, 0], &[0, 1]], 3)
    }

    #[test]
    fn ex2() {
        test(&[&[1, 1], &[1, 0]], 4)
    }

    #[test]
    fn ex3() {
        test(&[&[1, 1], &[1, 1]], 4)
    }

    #[test]
    fn myex0() {
        test(&[&[0]], 1);
    }

    #[test]
    fn myex1() {
        test(&[&[1]], 1);
    }

    #[test]
    fn my_extreme_ex1() {
        let in_row: &[i32] = &[1; 500];
        test(&[in_row; 500], 500 * 500)
    }

    #[test]
    fn my_extreme_ex2() {
        let mut in_row: [i32; 500] = [1; 500];
        in_row[499] = 0;
        let input: &[i32] = &in_row;
        test(&[input; 500], 499 * 500 + 1)
    }

    #[test]
    fn my_extreme_ex3() {
        let mut in_row: [i32; 500] = [1; 500];
        in_row[0] = 0;
        let input: &[i32] = &in_row;
        test(&[input; 500], 499 * 500 + 1)
    }

    #[test]
    fn discussion_case1() {
        test(&[&[0, 0], &[0, 0]], 1)
    }

    #[test]
    fn discussion_case2() {
        test(
            &[&[1, 1, 0, 1], &[1, 0, 0, 1], &[1, 0, 0, 1], &[1, 0, 0, 1]],
            10,
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[&[1, 0, 1, 0], &[0, 0, 0, 1], &[1, 0, 1, 0], &[1, 0, 1, 0]],
            5,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                &[0, 0, 0, 1, 1],
                &[1, 0, 0, 1, 0],
                &[1, 1, 1, 1, 1],
                &[1, 1, 1, 0, 1],
                &[0, 0, 0, 1, 0],
            ],
            15,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                &[0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0],
                &[0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0],
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            9,
        )
    }
}
