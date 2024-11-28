// https://leetcode.com/problems/minimum-obstacle-removal-to-reach-corner/

pub struct Solution;

// Using djikstra's algorithm
// impl Solution {
//     pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
//         // To minimize walls removed, we can give the walls a weight greater
//         // than the maximum possible path length. Given our constraints, no
//         // clear path can be greater than the maximal area of the grid: 100_000
//         // Now, however many multiples of MAX_CLEAR_PATH in the answer
//         // will be the number of walls removed.
//         // Worst-case, we need to remove all walls except the first and last,
//         // so we need to remove MAX_CLEAR_PATH * MAX_CLEAR_PATH - 2 walls.
//         // This requires a 64-bit integer type to represent.
//         type Pathlen = i64; // TODO: Consider unsigned
//         const MAX_CLEAR_PATH: Pathlen = 100_001;
//         let mut queue = std::collections::BinaryHeap::new();
//         let n = grid.len() as u32;
//         let m = grid[0].len() as u32;
//         let mut dist = vec![vec![std::i64::MAX; m as usize]; n as usize];
//         dist[0][0] = 0;
//         queue.push((std::cmp::Reverse(0 as Pathlen), (0u32, 0u32)));
//         while let Some((std::cmp::Reverse(d), (x, y))) = queue.pop() {
//             if x == n - 1 && y == m - 1 {
//                 return (d / MAX_CLEAR_PATH) as i32;
//             }
//             if d > dist[x as usize][y as usize] {
//                 continue;
//             }
//             for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
//                 let (nx, ny) = (x as i32 + dx, y as i32 + dy);
//                 if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
//                     continue;
//                 }
//                 let (nx, ny) = (nx as u32, ny as u32);
//                 let nd = d + 1 + (grid[nx as usize][ny as usize] as Pathlen * MAX_CLEAR_PATH);
//                 if nd < dist[nx as usize][ny as usize] {
//                     dist[nx as usize][ny as usize] = nd;
//                     queue.push((std::cmp::Reverse(nd), (nx, ny)));
//                 }
//             }
//         }
//         -1
//     }
// }

// Using two-queue inpainting
// impl Solution {
//     pub fn minimum_obstacles(mut grid: Vec<Vec<i32>>) -> i32 {
//         let n = grid.len() as u32;
//         let m = grid[0].len() as u32;
//         if n < 1 || m < 1 {
//             return 0;
//         }
//         if n == 1 || m == 1 {
//             // Just count the number of walls
//             return grid.into_iter().flatten().filter(|&x| x != 0).count() as i32;
//         }
//         let mut q = std::collections::VecDeque::new();
//         let mut q2 = std::collections::VecDeque::new();
//         q.push_back((0u32, 0u32));
//         let mut steps = 0;
//         loop {
//             // This loop must end because the grid is finite
//             while let Some((x, y)) = q.pop_front() {
//                 for (nx, ny) in [
//                     (x, y + 1),
//                     (x + 1, y),
//                     (x, y.wrapping_sub(1)),
//                     (x.wrapping_sub(1), y),
//                 ] {
//                     if nx > n - 1 || ny > m - 1 {
//                         continue;
//                     }
//                     if nx == n - 1 && ny == m - 1 {
//                         return steps;
//                     }
//                     let cell = &mut grid[nx as usize][ny as usize];
//                     match cell {
//                         0 => q.push_back((nx, ny)),
//                         1 => q2.push_back((nx, ny)),
//                         _ => continue,
//                     }
//                     *cell = 2;
//                 }
//             }
//             std::mem::swap(&mut q, &mut q2);
//             steps += 1;
//         }
//     }
// }

// Using two stacks inpainting
impl Solution {
    pub fn minimum_obstacles(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as u32;
        let m = grid[0].len() as u32;
        if n < 1 || m < 1 {
            return 0;
        }
        if n == 1 || m == 1 {
            // Just count the number of walls
            return grid.into_iter().flatten().filter(|&x| x != 0).count() as i32;
        }
        let mut q = std::vec::Vec::new();
        let mut q2 = std::vec::Vec::new();
        q.push((0u32, 0u32));
        let mut steps = 0;
        loop {
            // This loop must end because the grid is finite
            while let Some((x, y)) = q.pop() {
                for (nx, ny) in [
                    (x, y + 1),
                    (x + 1, y),
                    (x, y.wrapping_sub(1)),
                    (x.wrapping_sub(1), y),
                ] {
                    if nx > n - 1 || ny > m - 1 {
                        continue;
                    }
                    if nx == n - 1 && ny == m - 1 {
                        return steps;
                    }
                    let cell = &mut grid[nx as usize][ny as usize];
                    match cell {
                        0 => q.push((nx, ny)),
                        1 => q2.push((nx, ny)),
                        _ => continue,
                    }
                    *cell = 2;
                }
            }
            std::mem::swap(&mut q, &mut q2);
            steps += 1;
            assert!(!q.is_empty());
        }
    }
}

// Using two stacks inpainting with paint-on-read (Slower)
// impl Solution {
//     pub fn minimum_obstacles(mut grid: Vec<Vec<i32>>) -> i32 {
//         let n = grid.len() as u32;
//         let m = grid[0].len() as u32;
//         if n < 1 || m < 1 {
//             return 0;
//         }
//         if n == 1 || m == 1 {
//             // Just count the number of walls
//             return grid.into_iter().flatten().filter(|&x| x != 0).count() as i32;
//         }
//         let mut q = std::vec::Vec::new();
//         let mut q2 = std::vec::Vec::new();
//         q.push((0u32, 0u32));
//         let mut steps = 0;
//         loop {
//             // This loop must end because the grid is finite
//             while let Some((x, y)) = q.pop() {
//                 let cell = &mut grid[x as usize][y as usize];
//                 if *cell == 2 {
//                     continue;
//                 }
//                 *cell = 2;
//                 for (nx, ny) in [
//                     (x, y + 1),
//                     (x + 1, y),
//                     (x, y.wrapping_sub(1)),
//                     (x.wrapping_sub(1), y),
//                 ] {
//                     if nx > n - 1 || ny > m - 1 {
//                         continue;
//                     }
//                     if nx == n - 1 && ny == m - 1 {
//                         return steps;
//                     }
//                     match grid[nx as usize][ny as usize] {
//                         0 => q.push((nx, ny)),
//                         1 => q2.push((nx, ny)),
//                         _ => continue,
//                     }
//                 }
//             }
//             std::mem::swap(&mut q, &mut q2);
//             steps += 1;
//             assert!(!q.is_empty());
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&[i32]], expected: i32) {
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 100_000);
        let m = grid[0].len();
        assert!(m >= 1);
        assert!(m <= 100_000);
        for row in grid {
            assert_eq!(row.len(), m);
            for &x in *row {
                assert!(x == 0 || x == 1);
            }
        }
        assert!(grid.len() * m >= 2);
        assert!(grid.len() * m <= 100_000);
        assert!(grid[0][0] == 0);
        assert!(grid[grid.len() - 1][m - 1] == 0);

        assert_eq!(
            Solution::minimum_obstacles(grid.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[&[0, 1, 1], &[1, 1, 0], &[1, 1, 0]], 2)
    }

    #[test]
    fn ex2() {
        test(&[&[0, 1, 0, 0, 0], &[0, 1, 0, 1, 0], &[0, 0, 0, 1, 0]], 0)
    }

    #[test]
    fn my_extreme_ex1() {
        let input_row: &[i32] = &[0; 100_000];
        let input = &[input_row; 1];
        test(input, 0);
    }

    #[test]
    fn my_extreme_ex2() {
        let input_row: &[i32] = &[0; 1];
        let input = &[input_row; 100_000];
        test(input, 0);
    }

    #[test]
    fn my_extreme_ex3() {
        let input_row: &mut [i32] = &mut [1; 100_000];
        input_row[0] = 0;
        input_row[99_999] = 0;
        let input: &[&[i32]] = &[&*input_row; 1];
        test(input, 100_000 - 2);
    }

    #[test]
    fn my_extreme_ex4() {
        let first_row: &mut [i32] = &mut [1; 50_000];
        first_row[0] = 0;
        let second_row: &mut [i32] = &mut [1; 50_000];
        second_row[49_999] = 0;
        let input: &[&[i32]] = &[first_row, second_row];
        test(input, 49_999);
    }
}
