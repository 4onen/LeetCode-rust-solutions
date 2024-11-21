// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/

pub struct Solution;

// Initial sol'n (broken for unknown reasons)
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        const fn idx(x: i32, y: i32, m: i32) -> usize {
            (y * m + x) as usize
        }
        // We're going to treat each map cell as a bitset state machine
        // 0b000 = unguarded
        // 0b001 = guarded vertically
        // 0b010 = guarded horizontally
        // 0b011 = guarded vertically and horizontally
        // 0b100 = wall or guard
        let mut map = vec![0u8; m as usize * n as usize];
        // Because walls don't affect the world around them, we can place
        // all walls in the map immediately without affecting the rest of the
        // algorithm
        for wall in walls {
            map[idx(wall[0], wall[1], m)] = 0b100;
        }
        // We can also place all guards in the map immediately, as they
        // obstruct each others' vision cones:
        for guard in &guards {
            map[idx(guard[0], guard[1], m)] = 0b100;
        }
        // Now we can place each guard. We'll have to iterate the guard's
        // north, south, east, and west directions to mark the cells as
        // guarded. Luckily, if we run into a wall, another guard, or a guarding
        // track matching our current direction, we can stop iterating in that
        // direction.
        for guard in guards {
            let x = guard[0];
            let y = guard[1];
            // Guard vertically
            for i in (0..y).rev() {
                if map[idx(x, i, m)] & 0b101 != 0 {
                    break;
                }
                map[idx(x, i, m)] = 0b001;
            }
            for i in (y + 1)..n {
                if map[idx(x, i, m)] & 0b101 != 0 {
                    break;
                }
                map[idx(x, i, m)] = 0b001;
            }
            // Guard horizontally
            for i in (0..x).rev() {
                if map[idx(i, y, m)] & 0b110 != 0 {
                    break;
                }
                map[idx(i, y, m)] = 0b010;
            }
            for i in x + 1..m {
                if map[idx(i, y, m)] & 0b110 != 0 {
                    break;
                }
                map[idx(i, y, m)] = 0b010;
            }
        }
        // Now we can count the unguarded cells
        map.into_iter().filter(|&x| x == 0).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(m: i32, n: i32, guards: &[[i32; 2]], walls: &[[i32; 2]], expected: i32) {
        assert!(m >= 1);
        assert!(m <= 100_000);
        assert!(n >= 1);
        assert!(n <= 100_000);
        assert!(m * n >= 1);
        assert!(m * n <= 100_000);
        assert!(guards.len() >= 1);
        assert!(guards.len() <= 50_000);
        assert!(walls.len() >= 1);
        assert!(walls.len() <= 50_000);
        assert!(guards.len() + walls.len() >= 2);
        assert!(guards.len() + walls.len() <= m as usize * n as usize);
        let mut seen = std::collections::HashSet::new();
        for guard in guards {
            assert!(guard[0] >= 0);
            assert!(guard[0] < m);
            assert!(guard[1] >= 0);
            assert!(guard[1] < n);
            assert!(seen.insert((guard[0], guard[1])));
        }
        for wall in walls {
            assert!(wall[0] >= 0);
            assert!(wall[0] < m);
            assert!(wall[1] >= 0);
            assert!(wall[1] < n);
            assert!(seen.insert((wall[0], wall[1])));
        }
        assert_eq!(
            Solution::count_unguarded(
                m,
                n,
                guards.into_iter().map(|&x| x.to_vec()).collect(),
                walls.into_iter().map(|&x| x.to_vec()).collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            4,
            6,
            &[[0, 0], [1, 1], [2, 3]],
            &[[0, 1], [2, 2], [1, 4]],
            7,
        )
    }

    #[test]
    fn ex2() {
        test(3, 3, &[[1, 1]], &[[0, 1], [1, 0], [2, 1], [1, 2]], 4)
    }

    #[test]
    fn myex1() {
        test(
            5,
            5,
            &[[2, 2]],
            &[[0, 0], [1, 1], [3, 3], [4, 4]],
            2 + 4 + 4 + 2,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        test(
            1_000,
            100,
            &[[0, 0], [999, 99]],
            &[[1, 1], [998, 98]],
            998 * 98 - 2,
        )
    }
}
