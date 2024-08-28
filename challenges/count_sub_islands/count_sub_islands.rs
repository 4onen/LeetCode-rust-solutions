// https://leetcode.com/problems/count-sub-islands/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        assert!(grid1.len() >= 1);
        assert!(grid1.len() <= 500);
        assert_eq!(grid1.len(), grid2.len());
        let mut sub_islands = 0;
        for i in 0..grid1.len() {
            assert_eq!(grid1[i].len(), grid2[i].len());
            for j in 0..grid1[i].len() {
                if grid2[i][j] == 1 {
                    let mut stack = vec![(i as i32, j as i32)];
                    let mut is_sub_island = true;
                    while !stack.is_empty() {
                        let (x, y) = stack.pop().unwrap();
                        if x < 0 || y < 0 || x >= grid2.len() as i32 || y >= grid2[0].len() as i32 {
                            continue;
                        }
                        if grid2[x as usize][y as usize] == 0 {
                            continue;
                        }
                        grid2[x as usize][y as usize] = 0;
                        if grid1[x as usize][y as usize] == 0 {
                            is_sub_island = false;
                        }
                        stack.push((x + 1, y));
                        stack.push((x - 1, y));
                        stack.push((x, y + 1));
                        stack.push((x, y - 1));
                    }
                    if is_sub_island {
                        sub_islands += 1;
                    }
                }
            }
        }
        sub_islands
    }
}


// Reduced size index types (8ms slower)
// impl Solution {
//     pub fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
//         assert!(grid1.len() >= 1);
//         assert!(grid1.len() <= 500);
//         assert_eq!(grid1.len(), grid2.len());
//         let mut sub_islands = 0;
//         for i in 0..grid1.len() as u16 {
//             assert_eq!(grid1[i as usize].len(), grid2[i as usize].len());
//             for j in 0..grid1[i as usize].len() as u16 {
//                 if grid2[i as usize][j as usize] == 1 {
//                     let mut stack = vec![(i as i16, j as i16)];
//                     let mut is_sub_island = true;
//                     while !stack.is_empty() {
//                         let (x, y) = stack.pop().unwrap();
//                         if x < 0 || y < 0 || x >= grid2.len() as i16 || y >= grid2[0].len() as i16 {
//                             continue;
//                         }
//                         if grid2[x as usize][y as usize] == 0 {
//                             continue;
//                         }
//                         grid2[x as usize][y as usize] = 0;
//                         if grid1[x as usize][y as usize] == 0 {
//                             is_sub_island = false;
//                         }
//                         stack.push((x + 1, y));
//                         stack.push((x - 1, y));
//                         stack.push((x, y + 1));
//                         stack.push((x, y - 1));
//                     }
//                     if is_sub_island {
//                         sub_islands += 1;
//                     }
//                 }
//             }
//         }
//         sub_islands
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid1: &[&[i32]], grid2: &[&[i32]], expected: u32) {
        assert_eq!(grid1.len(), grid2.len());
        for row in grid1.iter() {
            assert_eq!(row.len(), grid2[0].len());
            for cell in row.iter() {
                assert!(*cell == 0 || *cell == 1);
            }
        }
        for row in grid2.iter() {
            assert_eq!(row.len(), grid2[0].len());
            for cell in row.iter() {
                assert!(*cell == 0 || *cell == 1);
            }
        }

        let grid1 = grid1.iter().map(|r| r.to_vec()).collect();
        let grid2 = grid2.iter().map(|r| r.to_vec()).collect();
        assert_eq!(Solution::count_sub_islands(grid1, grid2), expected as i32);
    }

    #[test]
    fn ex1() {
        test(
            &[
                &[1, 1, 1, 0, 0],
                &[0, 1, 1, 1, 1],
                &[0, 0, 0, 0, 0],
                &[1, 0, 0, 0, 0],
                &[1, 1, 0, 1, 1],
            ],
            &[
                &[1, 1, 1, 0, 0],
                &[0, 0, 1, 1, 1],
                &[0, 1, 0, 0, 0],
                &[1, 0, 1, 1, 0],
                &[0, 1, 0, 1, 0],
            ],
            3,
        );
    }

    #[test]
    fn ex2() {
        test(
            &[
                &[1, 0, 1, 0, 1],
                &[1, 1, 1, 1, 1],
                &[0, 0, 0, 0, 0],
                &[1, 1, 1, 1, 1],
                &[1, 0, 1, 0, 1],
            ],
            &[
                &[0, 0, 0, 0, 0],
                &[1, 1, 1, 1, 1],
                &[0, 1, 0, 1, 0],
                &[0, 1, 0, 1, 0],
                &[1, 0, 0, 0, 1],
            ],
            2,
        );
    }

    #[test]
    fn myex1() {
        test(&[&[1]], &[&[1]], 1);
    }

    #[test]
    fn discussion_case0() {
        test(&[&[1]], &[&[0]], 0);
    }

    #[test]
    fn my_extreme_ex1() {
        let vals = [[1i32;500];500];
        let refs = vals.iter().map(|r| &r[..]).collect::<Vec<_>>();
        test(&refs, &refs, 1);
    }

    #[test]
    fn my_extreme_ex2() {
        let mut vals = [[1i32;500];500];
        vals[0][0] = 0;
        let refs = vals.iter().map(|r| &r[..]).collect::<Vec<_>>();
        test(&refs, &refs, 1);
    }

    #[test]
    fn my_extreme_ex3() {
        let mut vals = [[1i32;500];500];
        for i in 0..vals.len() {
            if i % 2 == 1 {
                let l = vals[i].len();
                if i % 4 == 1 {
                    vals[i][0..l-1].fill(0);
                } else {
                    vals[i][1..l].fill(0);
                }
            }
        }
        let refs = vals.iter().map(|r| &r[..]).collect::<Vec<_>>();
        test(&refs, &refs, 1);
    }
}
