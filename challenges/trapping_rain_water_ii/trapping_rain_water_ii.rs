// https://leetcode.com/problems/trapping-rain-water-ii/

pub struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        let mut q = std::collections::BinaryHeap::new();
        let m = height_map.len() as u8;
        let n = height_map[0].len() as u8;
        let mut water_map = vec![vec![u16::MAX; n as usize]; m as usize];
        for i in 0..height_map.len() as u8 {
            {
                let height = height_map[i as usize][0] as u16;
                q.push(Reverse((height, i, 0)));
                water_map[i as usize][0] = height;
            }
            {
                let height = height_map[i as usize][n as usize - 1] as u16;
                q.push(Reverse((height, i, n - 1)));
                water_map[i as usize][n as usize - 1] = height;
            }
        }
        for j in 0..height_map[0].len() as u8 {
            {
                let height = height_map[0][j as usize] as u16;
                q.push(Reverse((height, 0, j)));
                water_map[0][j as usize] = height;
            }
            {
                let height = height_map[m as usize - 1][j as usize] as u16;
                q.push(Reverse((height, m - 1, j)));
                water_map[m as usize - 1][j as usize] = height;
            }
        }
        while let Some(Reverse((my_water, i, j))) = q.pop() {
            let my_water = std::cmp::max(my_water, height_map[i as usize][j as usize] as u16);
            if i > 0 {
                let (x, y) = (i - 1, j);
                let water = water_map[x as usize][y as usize];
                if my_water < water {
                    water_map[x as usize][y as usize] = my_water;
                    q.push(Reverse((my_water, x, y)));
                }
            }
            if j > 0 {
                let (x, y) = (i, j - 1);
                let water = water_map[x as usize][y as usize];
                if my_water < water {
                    water_map[x as usize][y as usize] = my_water;
                    q.push(Reverse((my_water, x, y)));
                }
            }
            if i < m - 1 {
                let (x, y) = (i + 1, j);
                let water = water_map[x as usize][y as usize];
                if my_water < water {
                    water_map[x as usize][y as usize] = my_water;
                    q.push(Reverse((my_water, x, y)));
                }
            }
            if j < n - 1 {
                let (x, y) = (i, j + 1);
                let water = water_map[x as usize][y as usize];
                if my_water < water {
                    water_map[x as usize][y as usize] = my_water;
                    q.push(Reverse((my_water, x, y)));
                }
            }
        }
        let mut trapped = 0;
        for (height_row, water_row) in std::iter::zip(height_map.into_iter(), water_map.into_iter())
        {
            for (height_el, water_el) in
                std::iter::zip(height_row.into_iter(), water_row.into_iter())
            {
                if water_el as i32 > height_el {
                    trapped += water_el as i32 - height_el;
                }
            }
        }
        trapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(height_map: &[&[i32]], expected: i32) {
        assert!(height_map.len() >= 1);
        assert!(height_map.len() <= 200);
        assert!(height_map[0].len() >= 1);
        assert!(height_map[0].len() <= 200);
        for &row in height_map {
            assert_eq!(row.len(), height_map[0].len());
            for &el in row {
                assert!(el >= 0);
                assert!(el <= 20_000);
            }
        }
        assert_eq!(
            Solution::trap_rain_water(height_map.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &[
                &[1, 4, 3, 1, 3, 2],
                &[3, 2, 1, 3, 2, 4],
                &[2, 3, 3, 2, 3, 1],
            ],
            4,
        )
    }

    #[test]
    fn ex2() {
        test(
            &[
                &[3, 3, 3, 3, 3],
                &[3, 2, 2, 2, 3],
                &[3, 2, 1, 2, 3],
                &[3, 2, 2, 2, 3],
                &[3, 3, 3, 3, 3],
            ],
            10,
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                &[12, 13, 1, 12],
                &[13, 4, 13, 12],
                &[13, 8, 10, 12],
                &[12, 13, 12, 12],
                &[13, 13, 13, 13],
            ],
            14,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                &[9, 9, 9, 9, 9, 9, 8, 9, 9, 9, 9],
                &[9, 0, 0, 0, 0, 0, 1, 0, 0, 0, 9],
                &[9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9],
                &[9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9],
                &[9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            ],
            9 * 3 * 8 - 1,
        )
    }

    #[test]
    fn discussion_case3() {
        test(&[&[2, 3, 7], &[5, 1, 6], &[2, 4, 9]], 2)
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                &[5, 8, 7, 7],
                &[5, 2, 1, 5],
                &[7, 1, 7, 1],
                &[8, 9, 6, 9],
                &[9, 8, 9, 9],
            ],
            3 + 4 + 4 + 1,
        )
    }

    #[test]
    fn discussion_case4_1() {
        test(
            &[&[5, 8, 7, 7], &[5, 2, 1, 5], &[7, 1, 7, 1], &[8, 9, 6, 9]],
            3 + 4 + 4,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                &[18, 13, 13, 17, 12, 11],
                &[17, 2, 6, 10, 5, 10],
                &[11, 10, 2, 8, 8, 2],
                &[12, 6, 10, 8, 8, 7],
                &[18, 4, 7, 6, 7, 4],
                &[20, 5, 9, 2, 3, 10],
            ],
            6 + 2 + 6 + 3 + 1, // ?
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            &[
                &[2, 3, 4],
                &[5, 6, 7],
                &[8, 9, 10],
                &[11, 12, 13],
                &[14, 15, 16],
            ],
            0,
        )
    }

    #[test]
    fn discussion_case7() {
        test(
            &[
                &[
                    10795, 10570, 11434, 10378, 17467, 16601, 10097, 12902, 13317, 10492,
                ],
                &[16652, 756, 7301, 280, 4286, 9441, 3865, 9689, 8444, 6619],
                &[18440, 4729, 8031, 8117, 8097, 5771, 4481, 675, 709, 8927],
                &[14567, 7856, 9497, 2353, 4586, 6965, 5306, 4683, 6219, 8624],
                &[11528, 2871, 5732, 8829, 9503, 19, 8270, 3368, 9708, 6715],
                &[16340, 8149, 7796, 723, 2618, 2245, 2846, 3451, 2921, 3555],
                &[12379, 7488, 7764, 8228, 9841, 2350, 5193, 1500, 7034, 7764],
                &[10124, 4914, 6987, 5856, 3743, 6491, 2227, 8365, 9859, 1936],
                &[11432, 2551, 6437, 9228, 3275, 5407, 1474, 6121, 8858, 4395],
                &[16029, 1237, 8235, 3793, 5818, 4428, 6143, 1011, 5928, 9529],
            ],
            68900,
        )
    }

    #[test]
    fn discussion_case8() {
        test(
            &[
                &[
                    19383, 10886, 12777, 16915, 17793, 18335, 15386, 10492, 16649, 11421,
                ],
                &[12362, 27, 8690, 59, 7763, 3926, 540, 3426, 9172, 5736],
                &[15211, 5368, 2567, 6429, 5782, 1530, 2862, 5123, 4067, 3135],
                &[13929, 9802, 4022, 3058, 3069, 8167, 1393, 8456, 5011, 8042],
                &[16229, 7373, 4421, 4919, 3784, 8537, 5198, 4324, 8315, 4370],
                &[16413, 3526, 6091, 8980, 9956, 1873, 6862, 9170, 6996, 7281],
                &[12305, 925, 7084, 6327, 336, 6505, 846, 1729, 1313, 5857],
                &[16124, 3895, 9582, 545, 8814, 3367, 5434, 364, 4043, 3750],
                &[11087, 6808, 7276, 7178, 5788, 3584, 5403, 2651, 2754, 2399],
                &[19932, 5060, 9676, 3368, 7739, 12, 6226, 8586, 8094, 7539],
            ],
            79058,
        )
    }
}
