// https://leetcode.com/problems/alternating-groups-ii/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn number_of_alternating_groups(mut colors: Vec<i32>, k: i32) -> i32 {
//         colors.extend_from_within(0..k as usize - 1);
//         let mut alternating_tile_count = 1;
//         let mut group_count = 0;
//         for i in 1..colors.len() {
//             if colors[i] != colors[i - 1] {
//                 alternating_tile_count += 1;
//                 if alternating_tile_count >= k {
//                     group_count += 1;
//                     alternating_tile_count -= 1;
//                 }
//             } else {
//                 alternating_tile_count = 1;
//             }
//         }
//         group_count
//     }
// }

// Only reset accumulator at group ends.
impl Solution {
    pub fn number_of_alternating_groups(mut colors: Vec<i32>, k: i32) -> i32 {
        colors.extend_from_within(0..k as usize - 1);
        let mut alternating_tile_count = 1;
        let mut group_count = 0;
        for i in 1..colors.len() {
            if colors[i] != colors[i - 1] {
                alternating_tile_count += 1;
            } else {
                if alternating_tile_count >= k {
                    group_count += alternating_tile_count - k + 1;
                }
                alternating_tile_count = 1;
            }
        }
        if alternating_tile_count >= k {
            group_count += alternating_tile_count - k + 1;
        }
        group_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(colors: &[u8], k: u32, expected: u32) {
        assert!(colors.len() >= 3);
        assert!(colors.len() <= 100_000);
        assert!(k >= 3);
        assert!(k as usize <= colors.len());
        let colors_vec: Vec<i32> = colors.iter().map(|&x| x as i32).collect();
        assert_eq!(
            Solution::number_of_alternating_groups(colors_vec.clone(), k as i32),
            expected as i32
        );
        // Rotating the colors array should not change the result
        // Try all rotations up to the length of the array or, like, 10
        let mut rotated_colors = colors_vec;
        for _ in 1..(colors.len().min(10) as usize) {
            rotated_colors.rotate_right(1);
            assert_eq!(
                Solution::number_of_alternating_groups(rotated_colors.clone(), k as i32),
                expected as i32
            );
        }
        // Flipping the colors array should not change the result
        // (Yes, we're using the rotated colors, but that also shouldn't change it.)
        let mut flipped_colors = rotated_colors;
        flipped_colors.iter_mut().for_each(|x| *x = 1 - *x);
        assert_eq!(
            Solution::number_of_alternating_groups(flipped_colors.clone(), k as i32),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test(&[0, 1, 0, 1, 0], 3, 3)
    }

    #[test]
    fn ex2() {
        test(&[0, 1, 0, 0, 1, 0, 1], 6, 2)
    }

    #[test]
    fn ex3() {
        test(&[1, 1, 0, 1], 4, 0)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 0, 1, 0], 4, 4)
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0,
                0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0,
            ],
            34,
            0,
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[
                1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1,
                0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1,
                1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0,
                0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0,
                1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0,
                1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1,
                0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
                0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1,
                0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0,
                0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0,
                0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0,
                1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1,
                0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1,
                0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0,
                1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1,
                1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0,
                0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0,
                1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0,
                0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0,
                0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1,
                1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0,
                0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1,
                0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1,
                0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1,
                1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1,
                0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1,
                0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0,
                1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0,
            ],
            10,
            9,
        )
    }
}
