// https://leetcode.com/problems/walking-robot-simulation/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
//         #[derive(Debug, Clone, Copy)]
//         enum Facing {
//             North,
//             East,
//             South,
//             West,
//         }
//         impl Facing {
//             fn left(self) -> Self {
//                 match self {
//                     Facing::North => Facing::West,
//                     Facing::West => Facing::South,
//                     Facing::South => Facing::East,
//                     Facing::East => Facing::North,
//                 }
//             }
//             fn right(self) -> Self {
//                 match self {
//                     Facing::North => Facing::East,
//                     Facing::East => Facing::South,
//                     Facing::South => Facing::West,
//                     Facing::West => Facing::North,
//                 }
//             }
//             fn step(self) -> [i32; 2] {
//                 match self {
//                     Facing::North => [0, 1],
//                     Facing::East => [1, 0],
//                     Facing::South => [0, -1],
//                     Facing::West => [-1, 0],
//                 }
//             }
//         }
//         let obstacles = obstacles
//             .into_iter()
//             .map(|v| [v[0], v[1]])
//             .collect::<std::collections::HashSet<_>>();
//         let mut facing = Facing::North;
//         let mut x = 0;
//         let mut y = 0;
//         let mut best_dist = 0;
//         for mut command in commands {
//             let step = facing.step();
//             match command {
//                 -2 => facing = facing.left(),
//                 -1 => facing = facing.right(),
//                 1..=9 => loop {
//                     let newx = x + step[0];
//                     let newy = y + step[1];
//                     if obstacles.contains(&[newx, newy]) {
//                         break;
//                     }
//                     x = newx;
//                     y = newy;
//                     best_dist = std::cmp::max(best_dist, newx * newx + newy * newy);
//                     command -= 1;
//                     if command < 1 {
//                         break;
//                     }
//                 },
//                 _ => unreachable!(),
//             }
//         }
//         best_dist
//     }
// }

// Optimized sol'n 1
// impl Solution {
//     pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
//         const STEPX: [i8; 4] = [0, 1, 0, -1];
//         const STEPY: [i8; 4] = [1, 0, -1, 0];
//         let obstacles = obstacles
//             .into_iter()
//             .map(|v| [v[0], v[1]])
//             .collect::<std::collections::HashSet<_>>();
//         let mut facing = 0u8;
//         let mut x = 0;
//         let mut y = 0;
//         let mut best_dist = 0;
//         for mut command in commands {
//             match command {
//                 -2 => facing = facing.checked_sub(1).unwrap_or(3),
//                 -1 => facing = (facing + 1) % 4,
//                 1..=9 => {
//                     let stepx = STEPX[facing as usize];
//                     let stepy = STEPY[facing as usize];
//                     loop {
//                         let newx = x + stepx as i32;
//                         let newy = y + stepy as i32;
//                         if obstacles.contains(&[newx, newy]) {
//                             break;
//                         }
//                         x = newx;
//                         y = newy;
//                         best_dist = std::cmp::max(best_dist, newx * newx + newy * newy);
//                         command -= 1;
//                         if command < 1 {
//                             break;
//                         }
//                     }
//                 }
//                 _ => unreachable!(),
//             }
//         }
//         best_dist
//     }
// }

// Optimized sol'n 2
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        fn simpl_hash(x: i32, y: i32) -> i64 {
            ((x as i64) << 32) ^ (y as i64)
        }
        const STEPX: [i8; 4] = [0, 1, 0, -1];
        const STEPY: [i8; 4] = [1, 0, -1, 0];
        let obstacles = obstacles
            .into_iter()
            .map(|v| simpl_hash(v[0], v[1]))
            .collect::<std::collections::HashSet<_>>();
        let mut facing = 0u8;
        let mut x = 0;
        let mut y = 0;
        let mut best_dist = 0;
        for mut command in commands {
            match command {
                -2 => facing = facing.checked_sub(1).unwrap_or(3),
                -1 => facing = (facing + 1) % 4,
                0.. => {
                    let stepx = STEPX[facing as usize];
                    let stepy = STEPY[facing as usize];
                    while command > 0 {
                        let newx = x + stepx as i32;
                        let newy = y + stepy as i32;
                        if obstacles.contains(&simpl_hash(newx, newy)) {
                            break;
                        }
                        x = newx;
                        y = newy;
                        best_dist = std::cmp::max(best_dist, newx * newx + newy * newy);
                        command -= 1;
                        if command < 1 {
                            break;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        best_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(commands: &[i32], obstacles: &[[i32; 2]], expected: i32) {
        assert_eq!(
            Solution::robot_sim(
                commands.to_vec(),
                obstacles.iter().map(|a| a.to_vec()).collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[4, -1, 3], &[], 25)
    }

    #[test]
    fn ex2() {
        test(&[4, -1, 4, -2, 4], &[[2, 4]], 65)
    }

    #[test]
    fn ex3() {
        test(&[6, -1, -1, 6], &[], 36)
    }

    #[test]
    fn failing_case1() {
        // Accidentally took left steps wrong
        test(
            &[-2,-1,8,9,6],
            &[[-1,3],[0,1],[-1,5],[-2,-4],[5,4],[-2,-3],[5,-1],[1,-1],[5,5],[5,2]],
            0,
        )
    }
}
