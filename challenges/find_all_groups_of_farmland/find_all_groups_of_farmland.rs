// https://leetcode.com/problems/find-all-groups-of-farmland/

pub struct Solution;

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        type IdxT = u16;
        let mut result = Vec::new();
        #[derive(Debug)]
        struct ActiveLand {
            start_row: IdxT,
            start_col: IdxT,
            end_col: IdxT,
        }
        impl PartialEq for ActiveLand {
            fn eq(&self, other: &Self) -> bool {
                self.start_col == other.start_col
            }
        }
        impl PartialOrd for ActiveLand {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.start_col.partial_cmp(&other.start_col)
            }
        }
        let mut active_groups: Vec<ActiveLand> = Vec::new();
        for rownum in 0..land.len() as IdxT {
            let mut colnum = 0;
            let mut groupnum = 0;
            loop {
                if colnum >= land[rownum as usize].len() as IdxT {
                    break;
                }
                while groupnum < active_groups.len() && active_groups[groupnum].start_col < colnum {
                    // Group is still here, but we're past it.
                    groupnum += 1;
                }
                if groupnum < active_groups.len() && active_groups[groupnum].start_col == colnum {
                    if land[rownum as usize][colnum as usize] == 1 {
                        // Group is still here. Skip to its end plus two.
                        colnum = active_groups[groupnum].end_col + 2;
                    } else {
                        // Group ended last row.
                        let group = active_groups.remove(groupnum);
                        result.push(vec![
                            group.start_row as i32,
                            group.start_col as i32,
                            (rownum - 1) as i32,
                            group.end_col as i32,
                        ]);
                    }
                    continue;
                }
                if land[rownum as usize][colnum as usize] == 1 {
                    // New group begins here.
                    let start_col = colnum;
                    loop {
                        colnum += 1;
                        if colnum == land[rownum as usize].len() as IdxT
                            || land[rownum as usize][colnum as usize] == 0
                        {
                            break;
                        }
                    }
                    let new_land = ActiveLand {
                        start_row: rownum,
                        start_col,
                        end_col: colnum - 1,
                    };
                    // Insert sorted.
                    let mut insert_at = active_groups.len();
                    for (i, group) in active_groups.iter().enumerate() {
                        if group.start_col > new_land.start_col {
                            insert_at = i;
                            break;
                        }
                    }
                    active_groups.insert(insert_at, new_land);
                    continue;
                }
                colnum += 1;
            }
        }
        // Pop any remaining active groups.
        for group in active_groups {
            result.push(vec![
                group.start_row as i32,
                group.start_col as i32,
                (land.len() as IdxT - 1) as i32,
                group.end_col as i32,
            ]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(land: &[&[i32]], expected: &[[i32; 4]]) {
        let land = land.iter().map(|r| r.to_vec()).collect();
        let mut result = Solution::find_farmland(land);
        result.sort_unstable();
        let mut expected = expected.to_vec();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test(
            &[&[1, 0, 0], &[0, 1, 1], &[0, 1, 1]],
            &[[0, 0, 0, 0], [1, 1, 2, 2]],
        );
    }

    #[test]
    fn ex2() {
        test(&[&[1, 1], &[1, 1]], &[[0, 0, 1, 1]]);
    }

    #[test]
    fn ex3() {
        test(&[&[0]], &[]);
    }

    #[test]
    fn myex1() {
        test(
            &[&[1, 0, 1], &[1, 0, 0], &[1, 0, 1], &[1, 0, 0], &[1, 0, 1]],
            &[[0, 0, 4, 0], [0, 2, 0, 2], [2, 2, 2, 2], [4, 2, 4, 2]],
        );
    }

    #[test]
    fn myex2() {
        test(
            &[&[1, 0, 1], &[0, 0, 1], &[1, 0, 1], &[0, 0, 1], &[1, 0, 1]],
            &[[0, 2, 4, 2], [0, 0, 0, 0], [2, 0, 2, 0], [4, 0, 4, 0]],
        );
    }

    #[test]
    fn myex3() {
        test(
            &[&[1, 0, 1, 0, 1, 0, 1], &[0; 7], &[1, 1, 1, 1, 1, 1, 1]],
            &[
                [0, 0, 0, 0],
                [0, 2, 0, 2],
                [0, 4, 0, 4],
                [0, 6, 0, 6],
                [2, 0, 2, 6],
            ],
        )
    }

    #[test]
    fn myex4() {
        test(
            &[&[1, 1, 1, 1, 1, 1, 1], &[0; 7], &[1, 0, 1, 0, 1, 0, 1]],
            &[
                [0, 0, 0, 6],
                [2, 0, 2, 0],
                [2, 2, 2, 2],
                [2, 4, 2, 4],
                [2, 6, 2, 6],
            ],
        )
    }

    #[test]
    fn failing_case1() {
        test(&[&[0, 1], &[1, 0]], &[[0, 1, 0, 1], [1, 0, 1, 0]])
    }

    #[test]
    fn myex5() {
        test(&[&[1, 0], &[0, 1]], &[[0, 0, 0, 0], [1, 1, 1, 1]])
    }
}
