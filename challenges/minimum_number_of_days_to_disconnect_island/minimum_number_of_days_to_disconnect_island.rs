// https://leetcode.com/problems/minimum-number-of-days-to-disconnect-island/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
//         fn bfs(grid: &Vec<Vec<i32>>, start: (u8, u8)) -> u16 {
//             let mut queue = std::collections::VecDeque::new();
//             queue.push_back((start.0 as i8, start.1 as i8));
//             let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
//             visited[start.0 as usize][start.1 as usize] = true;
//             let mut island_size = 0u16;
//             while let Some((i, j)) = queue.pop_front() {
//                 island_size += 1;
//                 for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
//                     let ni = i + di;
//                     let nj = j + dj;
//                     if ni >= 0
//                         && ni < grid.len() as i8
//                         && nj >= 0
//                         && nj < grid[0].len() as i8
//                         && grid[ni as usize][nj as usize] == 1
//                         && !visited[ni as usize][nj as usize]
//                     {
//                         visited[ni as usize][nj as usize] = true;
//                         queue.push_back((ni, nj));
//                     }
//                 }
//             }
//             island_size
//         }
//         assert!(grid.len() >= 1);
//         assert!(grid.len() <= 30);
//         // Find the first island size and count all land cells
//         let mut land_count = 0u16;
//         let mut island_count = 0u16;
//         for i in 0..grid.len() as u8 {
//             assert!(grid[i as usize].len() >= 1);
//             assert!(grid[i as usize].len() <= 30);
//             for j in 0..grid[i as usize].len() as u8 {
//                 if grid[i as usize][j as usize] == 1 {
//                     land_count += 1;
//                     if island_count == 0 {
//                         island_count = bfs(&mut grid, (i, j));
//                     }
//                 }
//             }
//         }
//         assert!(island_count <= land_count); // Make sure our BFS is okay.
//         match land_count {
//             0 => 0,
//             1 => 1,
//             2 if island_count == 2 => 2,
//             2 => 0,
//             3 if island_count == 3 => 1,
//             3 => 0,
//             _ if island_count < land_count => 0,
//             _ => {
//                 // Now we need to count the neighbors of every tile.
//                 // If any tile has only 1 neighbor, the answer is 1.
//                 // Otherwise, need to check if we can remove any single tile
//                 // to disconnect the island. If we can, the answer is 1.
//                 // Otherwise, the answer is 2.
//                 let mut neighbor_counts = vec![vec![0u8; grid[0].len()]; grid.len()];
//                 for i in 0..grid.len() {
//                     for j in 0..grid[i].len() {
//                         if grid[i][j] == 1 {
//                             for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
//                                 let ni = i as i8 + di;
//                                 let nj = j as i8 + dj;
//                                 if ni >= 0
//                                     && ni < grid.len() as i8
//                                     && nj >= 0
//                                     && nj < grid[0].len() as i8
//                                     && grid[ni as usize][nj as usize] == 1
//                                 {
//                                     neighbor_counts[i][j] += 1;
//                                 }
//                             }
//                             if neighbor_counts[i][j] == 1 {
//                                 return 1;
//                             }
//                         }
//                     }
//                 }
//                 // We now know all tiles have 2 or more neighbors.
//                 // Go to every tile, try removing it, then BFS to check connectivity.
//                 for i in 0..grid.len() {
//                     for j in 0..grid[i].len() {
//                         if grid[i][j] == 1 {
//                             grid[i][j] = 0;
//                             // Find any land neighbor and use bfs from before
//                             for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
//                                 let ni = i as i8 + di;
//                                 let nj = j as i8 + dj;
//                                 if ni >= 0
//                                     && ni < grid.len() as i8
//                                     && nj >= 0
//                                     && nj < grid[0].len() as i8
//                                     && grid[ni as usize][nj as usize] == 1
//                                 {
//                                     if bfs(&grid, (ni as u8, nj as u8)) < island_count - 1 {
//                                         return 1;
//                                     } else {
//                                         break;
//                                     }
//                                 }
//                             }
//                             grid[i][j] = 1;
//                         }
//                     }
//                 }
//                 2
//             }
//         }
//     }
// }

// Removing neighbor memoization
impl Solution {
    pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
        fn bfs(grid: &Vec<Vec<i32>>, start: (u8, u8)) -> u16 {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((start.0 as i8, start.1 as i8));
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
            visited[start.0 as usize][start.1 as usize] = true;
            let mut island_size = 0u16;
            while let Some((i, j)) = queue.pop_front() {
                island_size += 1;
                for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let ni = i + di;
                    let nj = j + dj;
                    if ni >= 0
                        && ni < grid.len() as i8
                        && nj >= 0
                        && nj < grid[0].len() as i8
                        && grid[ni as usize][nj as usize] == 1
                        && !visited[ni as usize][nj as usize]
                    {
                        visited[ni as usize][nj as usize] = true;
                        queue.push_back((ni, nj));
                    }
                }
            }
            island_size
        }
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 30);
        // Find the first island size and count all land cells
        let mut land_count = 0u16;
        let mut island_count = 0u16;
        for i in 0..grid.len() as u8 {
            assert!(grid[i as usize].len() >= 1);
            assert!(grid[i as usize].len() <= 30);
            for j in 0..grid[i as usize].len() as u8 {
                if grid[i as usize][j as usize] == 1 {
                    land_count += 1;
                    if island_count == 0 {
                        island_count = bfs(&mut grid, (i, j));
                    }
                }
            }
        }
        assert!(island_count <= land_count); // Make sure our BFS is okay.
        match land_count {
            0 => 0,
            1 => 1,
            2 if island_count == 2 => 2,
            2 => 0,
            3 if island_count == 3 => 1,
            3 => 0,
            _ if island_count < land_count => 0,
            _ => {
                // Now we need to count the neighbors of every tile.
                // If any tile has only 1 neighbor, the answer is 1.
                // Otherwise, need to check if we can remove any single tile
                // to disconnect the island. If we can, the answer is 1.
                // Otherwise, the answer is 2.
                for i in 0..grid.len() {
                    for j in 0..grid[i].len() {
                        if grid[i][j] == 1 {
                            let mut neighbor_count = 0;
                            for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                                let ni = i as i8 + di;
                                let nj = j as i8 + dj;
                                if ni >= 0
                                    && ni < grid.len() as i8
                                    && nj >= 0
                                    && nj < grid[0].len() as i8
                                    && grid[ni as usize][nj as usize] == 1
                                {
                                    neighbor_count += 1;
                                }
                            }
                            if neighbor_count == 1 {
                                return 1;
                            }
                        }
                    }
                }
                // We now know all tiles have 2 or more neighbors.
                // Go to every tile, try removing it, then BFS to check connectivity.
                for i in 0..grid.len() {
                    for j in 0..grid[i].len() {
                        if grid[i][j] == 1 {
                            grid[i][j] = 0;
                            // Find any land neighbor and use bfs from before
                            for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                                let ni = i as i8 + di;
                                let nj = j as i8 + dj;
                                if ni >= 0
                                    && ni < grid.len() as i8
                                    && nj >= 0
                                    && nj < grid[0].len() as i8
                                    && grid[ni as usize][nj as usize] == 1
                                {
                                    if bfs(&grid, (ni as u8, nj as u8)) < island_count - 1 {
                                        return 1;
                                    } else {
                                        break;
                                    }
                                }
                            }
                            grid[i][j] = 1;
                        }
                    }
                }
                2
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], output: i32) {
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 30);
        for &row in grid {
            assert!(row.len() >= 1);
            assert!(row.len() <= 30);
            assert!(row.len() == grid[0].len());
            for &cell in row {
                assert!(cell == 0 || cell == 1);
            }
        }
        assert_eq!(
            Solution::min_days(grid.iter().map(|v| v.to_vec()).collect()),
            output
        )
    }

    #[test]
    fn ex1() {
        test(&[&[0, 1, 1, 0], &[0, 1, 1, 0], &[0, 0, 0, 0]], 2)
    }

    #[test]
    fn ex2() {
        test(&[&[1, 1]], 2)
    }

    #[test]
    fn discussion_case4by4() {
        const N: usize = 30;
        test(&[&[1; N][..]; N], 2)
    }

    #[test]
    fn discussion_case30by30() {
        const N: usize = 30;
        test(&[&[1; N][..]; N], 2)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                &[0, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 0],
            ],
            1,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                &[1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 0, 1, 1],
                &[1, 1, 0, 1, 1],
            ],
            1,
        )
    }

    #[test]
    fn discussion_case3() {
        test(&[&[1, 1, 0], &[1, 1, 1], &[0, 1, 0]], 1)
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                &[1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1],
                &[1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1],
                &[0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0],
                &[1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
                &[1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0],
                &[0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1],
                &[1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
                &[1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1],
                &[0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
            ],
            0,
        )
    }

    #[test]
    fn discussion_case5() {
        test(&[&[1, 0], &[0, 1]], 0)
    }

    #[test]
    fn discussion_case6() {
        test(
            &[
                &[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 1,
                ],
                &[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 1,
                ],
                &[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 1,
                ],
                &[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 1,
                ],
                &[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 1,
                ],
                &[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 1,
                ],
                &[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 1,
                ],
                &[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 1,
                ],
                &[
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 1,
                ],
                &[
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1,
                ],
            ],
            1,
        )
    }

    #[test]
    fn discussion_case7() {
        test(&[&[0, 0]], 0)
    }

    #[test]
    fn discussion_case8() {
        test(&[&[0, 0, 0], &[0, 1, 0], &[0, 0, 0]], 1)
    }

    #[test]
    fn discussion_case9() {
        test(
            &[
                &[1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1],
            ],
            2,
        )
    }

    #[test]
    fn discussion_case10() {
        test(
            &[
                &[1, 1, 0, 1, 1],
                &[1, 1, 1, 1, 1],
                &[1, 1, 0, 1, 1],
                &[1, 1, 0, 1, 1],
            ],
            1,
        )
    }

    #[test]
    fn myex1() {
        test(
            &[
                &[1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
            1,
        )
    }

    #[test]
    fn failing_case1() {
        // Literally the last test case in the problem: 98/99 passed
        test(&[&[0, 1, 1], &[1, 1, 1], &[1, 1, 0]], 1)
    }
}
