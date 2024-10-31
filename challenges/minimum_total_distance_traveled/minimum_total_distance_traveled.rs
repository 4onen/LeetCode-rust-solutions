// https://leetcode.com/problems/minimum-total-distance-traveled/

pub struct Solution;

// Naive DFS sol'n. (Too slow for large inputs, as expected)
// impl Solution {
//     pub fn minimum_total_distance(mut robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
//         // First, convert factories to vec of tuples
//         let mut all_factory_limit = 0;
//         let mut factory = factory
//             .into_iter()
//             .map(|x| (x[0], x[1]))
//             .filter(|&(_, limit)| {
//                 all_factory_limit += limit;
//                 limit > 0
//             })
//             .collect::<Vec<_>>();
//         // Next, sort the robots and factories
//         robot.sort_unstable();
//         factory.sort_unstable();
//         let mut factory_fix_positions = Vec::with_capacity(all_factory_limit as usize);
//         for (position, limit) in factory.iter() {
//             for _ in 0..*limit {
//                 factory_fix_positions.push(*position);
//             }
//         }
//         assert_eq!(factory_fix_positions.len(), all_factory_limit as usize);
//         // Now the DP question we have is which robot should be assigned to
//         // which factory. First, let's try a DFS sol'n. (Won't be fast enough
//         // but we'll turn it into a memoized array later.)
//         // Each robot needs to decide if it stops at the current factory or
//         // the next factory, and what the cost to all future robots
//         // would be if it chooses each. Then it picks the minimum of these.
//         fn dfs(robots: &[i32], factories: &[i32], robot_index: usize, factory_index: usize) -> i64 {
//             if robot_index >= robots.len() {
//                 return 0;
//             }
//             if factories.len() - factory_index < robots.len() - robot_index {
//                 return i64::MAX;
//             }
//             std::cmp::min(
//                 // Stop at this factory
//                 dfs(robots, factories, robot_index + 1, factory_index + 1)
//                     .saturating_add((robots[robot_index] - factories[factory_index]).abs() as i64),
//                 // Don't stop at this factory
//                 dfs(robots, factories, robot_index, factory_index + 1),
//             )
//         }
//         dfs(&robot, &factory_fix_positions, 0, 0)
//     }
// }

// DP sol'n
impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        // First, convert factories to vec of tuples
        let mut all_factory_limit = 0;
        let mut factory = factory
            .into_iter()
            .map(|x| (x[0], x[1]))
            .filter(|&(_, limit)| {
                all_factory_limit += limit;
                limit > 0
            })
            .collect::<Vec<_>>();
        // Next, sort the robots and factories
        robot.sort_unstable();
        factory.sort_unstable();
        let mut factory_fix_positions = Vec::with_capacity(all_factory_limit as usize);
        for (position, limit) in factory.iter() {
            for _ in 0..*limit {
                factory_fix_positions.push(*position);
            }
        }
        assert_eq!(factory_fix_positions.len(), all_factory_limit as usize);
        // Now the DP question we have is which robot should be assigned to
        // which factory.
        let mut dp = vec![vec![u64::MAX; factory_fix_positions.len() + 1]; robot.len() + 1];
        dp[robot.len()][robot.len()..].fill(0);
        for robot_index in (0..robot.len()).rev() {
            for factory_index in (0..factory_fix_positions.len()).rev() {
                if factory_fix_positions.len() - factory_index < robot.len() - robot_index {
                    // More robots than factories left
                    dp[robot_index][factory_index] = u64::MAX;
                    continue;
                }
                dp[robot_index][factory_index] = std::cmp::min(
                    // Stop at this factory
                    dp[robot_index + 1][factory_index + 1]
                        .saturating_add(
                            (robot[robot_index] - factory_fix_positions[factory_index]).abs() as u64,
                        ),
                    // Don't stop at this factory
                    dp[robot_index][factory_index + 1],
                );
            }
        }
        dp[0][0] as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(robot: &[i32], factory: &[[i32; 2]], expected: i64) {
        assert!(robot.len() >= 1);
        assert!(robot.len() <= 100);
        assert!(factory.len() >= 1);
        assert!(factory.len() <= 100);
        for &robo in robot.iter() {
            assert!(robo >= -1_000_000_000);
            assert!(robo <= 1_000_000_000);
        }
        let mut all_factory_limit = 0;
        for &[position, limit] in factory.iter() {
            assert!(position >= -1_000_000_000);
            assert!(position <= 1_000_000_000);
            assert!(limit >= 0);
            assert!(limit <= robot.len() as i32);
            all_factory_limit += limit;
        }
        assert!(all_factory_limit >= robot.len() as i32);
        assert_eq!(
            Solution::minimum_total_distance(
                robot.to_vec(),
                factory.iter().map(|&x| x.to_vec()).collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[0, 4, 6], &[[2, 2], [6, 2]], 4)
    }

    #[test]
    fn ex2() {
        test(&[1, -1], &[[-2, 1], [2, 1]], 2)
    }

    #[test]
    fn my_trivial_ex1() {
        test(&[0], &[[0, 1]], 0)
    }

    #[test]
    fn my_trivial_ex1_1() {
        test(&[0], &[[0, 1],[100, 1]], 0)
    }

    #[test]
    fn my_trivial_ex2() {
        test(&[0], &[[1, 1]], 1)
    }

    #[test]
    fn my_trivial_ex3() {
        test(&[1], &[[0, 1]], 1)
    }

    #[test]
    fn my_trivial_ex4() {
        test(&[1, 10], &[[1, 1], [10, 1]], 0)
    }

    #[test]
    fn discussion_case1() {
        // 9 will be repared by 7          +2
        // 11 will be repared by 10        +1
        // 99 will be repared by 100       +1
        // 101 will be repared by 103      +2
        test(
            &[9, 11, 99, 101],
            &[[10, 1], [7, 1], [14, 1], [100, 1], [96, 1], [103, 1]],
            6,
        )
    }

    #[test]
    fn discussion_case2() {
        test(&[7], &[[0, 1]], 7)
    }

    #[test]
    fn discussion_case3() {
        test(
            &[-40, -14, -8, 1, 3, 5, 39],
            &[[-34, 5], [28, 2], [-12, 3]],
            92,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[9, 11, 99, 101],
            &[[10, 1], [7, 1], [14, 1], [100, 1], [96, 1], [103, 1]],
            1 + 2 + 1 + 2,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[-551, -510, -344, -264, -242, -50, 202, 185, 700, 947],
            &[[-79, 5], [-534, 5]],
            3172,
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            &[
                403625544, 670355988, 886437985, 224430896, 126139936, -477101480, -868159607,
                -293937930,
            ],
            &[
                [333473422, 7],
                [912209329, 7],
                [468372740, 7],
                [-765827269, 4],
                [155827122, 4],
                [635462096, 2],
                [-300275936, 2],
                [-115627659, 0],
            ],
            509199280,
        )
    }

    #[test]
    fn discussion_case7() {
        test(
            &[
                9486709, 305615257, 214323605, 282129380, 763907021, -662831631, 628054452,
                -132239126, 50488558, 381544523, -656107497, 810414334, 421675516, -304916551,
                571202823, 478460906, -972398628, 325714139, -86452967, 660743346,
            ],
            &[
                [-755430217, 18],
                [382914340, 2],
                [977509386, 4],
                [94299927, 9],
                [32194684, 16],
                [-371001457, 2],
                [-426296769, 13],
                [-284404215, 8],
                [-421288725, 0],
                [-893030428, 2],
                [291827872, 17],
                [-766616824, 8],
                [-730172656, 17],
                [-722387876, 1],
                [510570520, 20],
                [756326049, 7],
                [-242350340, 14],
                [6585224, 19],
                [-173457765, 15],
            ],
            925405949,
        )
    }

    #[test]
    fn discussion_case8() {
        test(&[5, 3], &[[0, 1], [4, 1], [7, 1]], 3)
    }

    #[test]
    fn discussion_case9() {
        test(
            &[
                -333539942, 359275673, 89966494, 949684497, -733065249, 241002388, 325009248,
                403868412, -390719486, -670541382, 563735045, 119743141, 323190444, 534058139,
                -684109467, 425503766, 761908175,
            ],
            &[
                [-590277115, 0],
                [-80676932, 3],
                [396659814, 0],
                [480747884, 9],
                [118956496, 10],
            ],
            4412966458,
        )
    }
}
