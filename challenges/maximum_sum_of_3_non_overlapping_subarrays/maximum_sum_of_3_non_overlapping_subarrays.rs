// https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/

pub struct Solution;

// Naive sol'n (No DP, O(n^3) runs too slow)
// impl Solution {
//     pub fn max_sum_of_three_subarrays(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
//         assert!(nums.len() <= 20_000);
//         assert!((k as usize) < nums.len());
//         let n = nums.len() as u16;
//         let k = k as u16;
//         let arr_count = nums.len() as u16 - (k - 1);
//         {
//             // Precompute subarray sums
//             let mut s = 0;
//             for i in 0..n {
//                 s += nums[i as usize];
//                 if i >= k - 1 {
//                     let tmp = nums[(i - (k - 1)) as usize];
//                     nums[(i - (k - 1)) as usize] = s;
//                     s -= tmp;
//                 }
//             }
//             unsafe {
//                 nums.set_len(arr_count as usize);
//             }
//         }
//         // Compute answer
//         let mut best_answer_val = 0;
//         let mut best_answer_indices = [0, 0, 0];
//         for i in 0..arr_count {
//             for j in i + k..arr_count {
//                 let partial_ans_sum = nums[i as usize] + nums[j as usize];
//                 for h in j + k..arr_count {
//                     let answer_val = nums[h as usize] + partial_ans_sum;
//                     let answer_indices = [i, j, h];
//                     if answer_val > best_answer_val {
//                         best_answer_val = answer_val;
//                         best_answer_indices = answer_indices;
//                     }
//                 }
//             }
//         }
//         best_answer_indices.map(|x| x as i32).to_vec()
//     }
// }

// DP sol'n (Has an off-by-one error somewhere causing a failing case)
// impl Solution {
//     pub fn max_sum_of_three_subarrays(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
//         assert!(nums.len() <= 20_000);
//         assert!((k as usize) < nums.len());
//         let n = nums.len() as u16;
//         let k = k as u16;
//         let arr_count = nums.len() as u16 - (k - 1);
//         {
//             // Precompute subarray sums
//             let mut s = 0;
//             for i in 0..n {
//                 s += nums[i as usize];
//                 if i >= k - 1 {
//                     let tmp = nums[(i - (k - 1)) as usize];
//                     nums[(i - (k - 1)) as usize] = s;
//                     s -= tmp;
//                 }
//             }
//             unsafe {
//                 nums.set_len(arr_count as usize);
//             }
//         }
//         // Compute answer (Not pretty, not space-saving, but damn is she fast.)
//         let mut stage_1_indices = vec![0u16; arr_count as usize];
//         let mut stage1_best = 0;
//         let mut stage_2_indices = vec![[0u16; 2]; (arr_count - k) as usize];
//         let mut stage2_best = 0;
//         let mut stage_3_indices = vec![[0u16; 3]; (arr_count - k - k) as usize];
//         let mut stage3_best = 0;
//         *stage_1_indices.last_mut().unwrap() = arr_count - 1;
//         for i in (0..arr_count - 1).rev() {
//             stage_1_indices[i as usize] = if stage1_best > nums[i as usize] {
//                 stage_1_indices[i as usize + 1]
//             } else {
//                 stage1_best = nums[i as usize];
//                 i
//             };
//             if i + k < arr_count {
//                 let stage2_val =
//                     nums[i as usize] + nums[stage_1_indices[(i + k) as usize] as usize];
//                 stage_2_indices[i as usize] = if stage2_best > stage2_val {
//                     stage_2_indices[i as usize + 1]
//                 } else {
//                     stage2_best = stage2_val;
//                     [i, stage_1_indices[(i + k) as usize]]
//                 };
//             }
//             if i + k + k < arr_count {
//                 let relative_stage2 = stage_2_indices[(i + k) as usize];
//                 let stage3_val = nums[i as usize]
//                     + nums[relative_stage2[0] as usize]
//                     + nums[relative_stage2[1] as usize];
//                 stage_3_indices[i as usize] = if stage3_best > stage3_val {
//                     stage_3_indices[i as usize + 1]
//                 } else {
//                     stage3_best = stage3_val;
//                     [i, relative_stage2[0], relative_stage2[1]]
//                 };
//             }
//         }
//         stage_3_indices[0].map(|x| x as i32).to_vec()
//     }
// }

// DP Sol'n (Working)
impl Solution {
    pub fn max_sum_of_three_subarrays(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        assert!(nums.len() <= 20_000);
        assert!((k as usize) < nums.len());
        let n = nums.len() as u16;
        let k = k as u16;
        let arr_count = nums.len() as u16 - (k - 1);
        {
            // Precompute subarray sums
            let mut s = 0;
            for i in 0..n {
                s += nums[i as usize];
                if i >= k - 1 {
                    let tmp = nums[(i - (k - 1)) as usize];
                    nums[(i - (k - 1)) as usize] = s;
                    s -= tmp;
                }
            }
            unsafe {
                nums.set_len(arr_count as usize);
            }
        }
        // Compute answer (Not pretty, not space-saving, but damn is she fast.)
        let mut stage1_indices = vec![0u16; arr_count as usize];
        let mut stage1_best = *nums.last().unwrap();
        let mut stage2_indices = vec![[0u16; 2]; (arr_count - k) as usize];
        let mut stage2_best = 0;
        let mut stage3_indices = vec![[0u16; 3]; (arr_count - k - k) as usize];
        let mut stage3_best = 0;
        *stage1_indices.last_mut().unwrap() = arr_count - 1;
        for i in (0..arr_count - 1).rev() {
            stage1_indices[i as usize] = if stage1_best > nums[i as usize] {
                stage1_indices[i as usize + 1]
            } else {
                stage1_best = nums[i as usize];
                i
            };
            if i + k < arr_count {
                let stage2_val = nums[i as usize] + nums[stage1_indices[(i + k) as usize] as usize];
                stage2_indices[i as usize] = if stage2_best > stage2_val {
                    stage2_indices[i as usize + 1]
                } else {
                    stage2_best = stage2_val;
                    [i, stage1_indices[(i + k) as usize]]
                };
            }
            if i + k + k < arr_count {
                let relative_stage2 = stage2_indices[(i + k) as usize];
                let stage3_val = nums[i as usize]
                    + nums[relative_stage2[0] as usize]
                    + nums[relative_stage2[1] as usize];
                stage3_indices[i as usize] = if stage3_best > stage3_val {
                    stage3_indices[i as usize + 1]
                } else {
                    stage3_best = stage3_val;
                    [i, relative_stage2[0], relative_stage2[1]]
                };
            }
        }
        stage3_indices[0].map(|x| x as i32).to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u16], k: i32, expected: [u16; 3]) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 20_000);
        assert!(k >= 1);
        assert!(k <= nums.len() as i32 / 3);
        for &num in nums {
            assert!(num >= 1);
        }
        assert_eq!(
            Solution::max_sum_of_three_subarrays(nums.iter().map(|&x| x as i32).collect(), k),
            expected.map(|x| x as i32)
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 1, 2, 6, 7, 5, 1], 2, [0, 3, 5])
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 1, 2, 1, 2, 1, 2, 1], 2, [0, 2, 4])
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 2, 3, 4], 1, [1, 2, 3]);
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 2, 1, 2, 2, 2, 2, 2], 2, [0, 3, 5])
    }

    #[test]
    fn discussion_case3() {
        test(&[4, 5, 10, 6, 11, 17, 4, 11, 1, 3], 1, [4, 5, 7])
    }

    #[test]
    fn discussion_case4() {
        test(&[18, 11, 14, 7, 16, 3, 18, 11, 3, 8], 3, [0, 4, 7])
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                12, 15, 4, 6, 8, 3, 12, 11, 2, 10, 14, 6, 7, 18, 11, 16, 15, 17, 14, 10, 5, 17, 3,
                3, 8, 4, 14, 13, 10, 11, 17, 8, 19, 6, 12, 8, 16, 3, 12, 12, 19, 9, 10, 20, 3, 1,
                10, 20, 4, 5, 5, 13, 16, 19, 2, 15, 11, 1, 8, 8, 2, 5, 7, 4, 14, 3, 14, 4, 19, 5,
                19, 3, 14, 13, 7, 16, 1, 16, 4, 8, 17, 11, 20, 9, 5, 20, 2, 7, 12, 8, 15, 15, 11,
                1, 9, 5, 1, 4, 6, 20,
            ],
            15,
            [7, 29, 68],
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            &[
                10, 5, 9, 4, 14, 10, 16, 9, 17, 19, 2, 11, 15, 10, 17, 20, 3, 14, 19, 17, 14, 20,
                10, 17, 9, 14, 18, 2, 7, 5, 7, 20, 11, 1, 13, 6, 3, 11, 18, 3, 5, 6, 14, 10, 10, 4,
                12, 7, 5, 7, 5, 15, 14, 20, 13, 15, 6, 20, 13, 6, 9, 15, 2, 6, 16, 13, 2, 9, 14, 3,
                6, 2, 10, 9, 7, 10, 5, 3, 14, 14, 6, 16, 1, 19, 12, 5, 16, 9, 2, 16, 11, 19, 9, 2,
                17, 19, 6, 5, 6, 6,
            ],
            20,
            [7, 42, 78],
        )
    }

    #[test]
    fn discussion_case7() {
        test(
            &[
                62368, 43415, 6411, 30431, 47476, 43988, 57619, 4131, 15320, 8470, 12424, 32500,
                37914, 256, 54892, 64542, 44556, 63623, 32488, 38391, 21821, 814, 17407, 25008,
                5503, 20233, 44981, 21616, 17906, 34260, 33045, 6535, 64008, 53390, 1376, 44651,
                16181, 18292, 50643, 15050, 45411, 5124, 30452, 9018, 31760, 7046, 39294, 51894,
                43549, 15092, 42683, 31095, 982, 40803, 44869, 30685, 58595, 54124, 5746, 49021,
                37208, 45952, 34455, 1884, 19642, 63556, 21510, 17701, 25556, 52905, 29963, 2716,
                17966, 22775, 56535, 15474, 11496, 15975, 34816, 14486, 53517, 55132, 2444, 7493,
                50678, 40509, 20774, 35932, 9936, 12255, 54904, 3246, 60422, 39236, 21118, 10657,
                50107, 53038, 4168, 53669, 26736, 40270, 45504, 35101, 1857, 34941, 63136, 30522,
                52761, 36214, 41522, 51187, 25016, 62798, 55606, 17470, 12428, 49736, 53277, 37848,
                45604, 21964, 58413, 54931, 38205, 19175, 51970, 9769, 4683, 50149, 23165, 15010,
                6584, 55412, 59918, 53641, 64475, 18216, 47808, 21116, 29770, 9262, 31961, 24894,
                20219, 16035, 60933, 19904, 15193, 33039, 12544, 14650, 29748, 22824, 6541, 48083,
                3027, 30777, 55442, 33284, 54358, 17329, 10445, 30616, 16699, 10617, 41450, 52177,
                60769, 30161, 31747, 1537, 8882, 26213, 380, 58124, 16460, 37154, 45252, 65258,
                8795, 36606, 35192, 34215, 21100, 39898, 17820, 16225, 31017, 23330, 19812, 14809,
                56587, 50279, 46723, 25526, 57230, 15351, 10101, 55003,
            ],
            40,
            [26, 99, 157],
        )
    }

    #[test]
    fn discussion_case8() {
        test(
            &[
                780, 13064, 12173, 1276, 23860, 39384, 21772, 34161, 61405, 41752, 18187, 50465,
                52713, 36613, 6284, 41896, 62018, 35662, 39318, 43667, 27325, 53359, 25428, 21587,
                5038, 13444, 28434, 22077, 47751, 59482, 23999, 5423, 3308, 31759, 41732, 47742,
                64560, 50014, 58182, 63547, 65480, 60979, 60704, 30082, 46270, 51002, 12160, 5324,
                18914, 48153, 12996, 62814, 29771, 10443, 55406, 61804, 1999, 34662, 56965, 54278,
                42731, 32035, 9692, 31986, 16860, 19829, 18683, 33799, 48826, 2084, 50986, 31129,
                19019, 38917, 21731, 30201, 21927, 64496, 40215, 49382, 48382, 9297, 32969, 35819,
                6604, 18729, 19512, 1383, 52777, 56971, 25882, 62064, 6784, 50830, 21420, 53795,
                8744, 51452, 46643, 39592, 24684, 37063, 19890, 53821, 56982, 968, 36547, 33891,
                2132, 21822, 34472, 1435, 43279, 25793, 10291, 28097, 49156, 41, 45384, 64910,
                56377, 46774, 31633, 14469, 41444, 6473, 30490, 8989, 64223, 7371, 46473, 5742,
                5506, 34715, 21890, 19507, 11804, 43473, 24346, 57437, 52825, 30888, 47668, 29345,
                17053, 27932, 24937, 33517, 21, 38869, 56607, 17741, 28237, 5557, 47448, 60341,
                18589, 59251, 8375, 1393, 23675, 13630, 49315, 65393, 13170, 46951, 34849, 63649,
                52933, 23848, 45052, 41644, 6473, 18544, 43507, 8899, 1815, 20438, 22433, 53503,
                10472, 446, 54022, 3446, 53385, 19018, 37833, 51111, 48772, 31993, 32789, 9979,
                27696, 53864, 53510, 5286, 8681, 33874, 46843, 43669, 19404, 20031, 58248, 45655,
                55274, 45952, 37587, 53716, 8739, 6858, 48440, 64472, 52393, 34187, 14667, 31744,
                58472, 12227, 43802, 34813, 43230, 63661, 12220, 31879, 18372, 34912, 1931, 54289,
                64798, 10154, 4812, 37933, 33569, 10814, 18129, 64170, 54203, 25465, 48078, 44624,
                49913, 59895, 35795, 58702, 32061, 37130, 37800, 36289, 46650, 61935, 52204, 10753,
                9524, 47098, 19326, 2679, 6726, 41766, 12883, 46579, 55945, 52092, 40869, 49045,
                59358, 32605, 27789, 59012, 36421, 38579, 10628, 49790, 14706, 13488, 15004, 49158,
                46424, 7528, 40608, 27528, 64720, 32236, 5606, 59699, 27671, 35413, 60866, 14719,
                47927, 5969, 30254, 6511, 30826, 20063, 26823, 15905, 55884, 21071, 27893, 48304,
                63498, 14586, 44877, 33306, 20250, 64971, 41653, 5141, 31025, 15848, 1250, 4937,
                20561, 8123, 1617, 18292, 20594, 42088, 3076, 5469, 34967, 39105, 63498, 52919,
                9057, 19980, 43796, 64789, 2607, 17534, 45384, 37116, 3161, 58081, 31110, 49466,
                61438, 1614, 52519, 33719, 8135, 1186, 63629, 12105, 20436, 15514, 26058, 30151,
                63263, 12509, 60882, 42176, 37124, 40481, 9917, 18402, 54354, 38534, 34391, 55489,
                28843, 8572, 41518, 25472, 4296, 39575, 19269, 39540, 59858, 24054, 15452, 32635,
                22047, 27389, 162, 38464, 1272, 29570, 16289, 35465, 37427, 23847, 57523, 33186,
                44679, 53306, 38870, 54918, 22468, 49286, 12602, 49760, 56569, 1704, 39962, 22454,
                24489, 13048, 52944, 64722,
            ],
            6,
            [36, 238, 259],
        )
    }

    #[test]
    fn failing_case1() {
        test(
            &[
                1, 9, 16, 14, 13, 18, 18, 9, 1, 17, 6, 3, 8, 2, 20, 16, 10, 17, 19, 3, 8, 14, 11,
                19, 8, 19, 18, 1, 14, 7, 17, 8, 16, 15, 9, 5, 11, 11, 9, 18,
            ],
            12,
            [1, 14, 28],
        )
    }

    #[test]
    fn failing_case1_1() {
        test(
            &[
                9, 16, 14, 13, 18, 18, 9, 1, 17, 6, 3, 8, 2, 20, 16, 10, 17, 19, 3, 8, 14, 11, 19,
                8, 19, 18, 1, 14, 7, 17, 8, 16, 15, 9, 5, 11, 11, 9, 18,
            ],
            12,
            [0, 13, 27],
        )
    }

    #[test]
    fn failing_case1_2() {
        test(
            &[
                9, 16, 14, 13, 18, 18, 9, 1, 17, 6, 3, // Group 1
                2, // Out
                20, 16, 10, 17, 19, 3, 8, 14, 11, 19, 8, // Group 2
                18, 1, // Out
                14, 7, 17, 8, 16, 15, 9, 5, 11, 11, 9, // Group 3
            ],
            11,
            [0, 12, 25],
        )
    }

    #[test]
    fn failing_case1_3() {
        test(
            &[
                9, 16, 14, 13, 18, 18, 9, 1, 17, 6, // Group 1
                2, // Out
                20, 16, 10, 17, 19, 3, 8, 14, 11, 19, // Group 2
                18, 1, // Out
                14, 7, 17, 8, 16, 15, 9, 5, 11, 11, // Group 3
            ],
            10,
            [0, 11, 23],
        )
    }

    #[test]
    fn failing_case1_5() {
        test(
            &[
                9, 16, 14, // Group 1
                2,  // Out
                20, 16, 10, // Group 2
                18, 1, // Out
                14, 7, 17, // Group 3
            ],
            3,
            [0, 4, 9],
        )
    }

    #[test]
    fn failing_case1_6() {
        test(
            &[
                9, 16, 14, // Group 1
                2,  // Out
                20, 16, 10, // Group 2
                1, 1, // Out
                14, 7, 17, // Group 3
            ],
            3,
            [0, 4, 9],
        )
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[u16::MAX; 20_000], 6666, [0, 6666, 6666 + 6666])
    }

    #[test]
    fn my_extreme_ex2() {
        // O(n^3) defeater
        test(&[u16::MAX; 20_000], 1, [0, 1, 2])
    }
}
