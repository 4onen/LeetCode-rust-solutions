// https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/

pub struct Solution;

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        // Array of (cycle_member1, cycle_member2)
        // Can add cycle_id to specify connected components of
        // each of these chains ending in 2-cycles.
        // We also know this is sorted order of cycle_id, for free.
        let mut two_cycle_ccs = vec![];
        // Find largest cycle
        let mut cycle_id = vec![-1i32; favorite.len()];
        let mut max_cycle = 0;
        for i in 0..favorite.len() as i32 {
            if cycle_id[i as usize] >= 0 {
                continue;
            }
            let mut j = i;
            let mut depth = 0;
            while cycle_id[j as usize] < 0 {
                cycle_id[j as usize] = i as i32;
                j = favorite[j as usize];
                depth += 1;
            }
            let true_cycle_id = cycle_id[j as usize];
            let mut k = i;
            let mut depth_to_cycle = 0;
            while cycle_id[k as usize] == i && k != j {
                cycle_id[k as usize] = true_cycle_id as i32;
                k = favorite[k as usize];
                depth_to_cycle += 1;
            }
            if k == j {
                // Found a cycle
                let len = depth - depth_to_cycle;
                if len > max_cycle {
                    max_cycle = len;
                }
                if len == 2 {
                    two_cycle_ccs.push((j, favorite[j as usize]));
                }
            }
        }
        // Open (non-cycle) seating arrangement
        // -- combine all longest terminating in 2-cycles
        let mut inverted = vec![vec![]; favorite.len()];
        for (i, x) in favorite.into_iter().enumerate() {
            // TODO: Try limiting this to the two-cycle connected components
            //       to improve memory usage.
            inverted[x as usize].push(i as i32);
        }
        let mut open_seating_count = 0;
        for (a, b) in two_cycle_ccs {
            // For each 2-cycle connected component, find the longest path
            // between any two nodes. Add it to the seating chart.
            fn bfs(src: i32, friend: i32, inverted: &Vec<Vec<i32>>) -> i32 {
                let mut queue = std::collections::VecDeque::new();
                let mut max_len = 0;
                queue.push_back((src, 0));
                while let Some((node, len)) = queue.pop_front() {
                    if len > max_len {
                        max_len = len;
                    }
                    for &neighbor in inverted[node as usize].iter() {
                        if neighbor == friend {
                            continue;
                        }
                        queue.push_back((neighbor, len + 1));
                    }
                }
                max_len
            }
            open_seating_count += 2 + bfs(a, b, &inverted) + bfs(b, a, &inverted);
        }
        // Return the larger of the two.
        std::cmp::max(max_cycle, open_seating_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(favorite: &[i32], expected: i32) {
        assert!(favorite.len() >= 2);
        assert!(favorite.len() <= 100_000);
        for (i, &x) in favorite.iter().enumerate() {
            assert!(x >= 0);
            assert!(x <= favorite.len() as i32);
            assert!(x != i as i32);
        }
        assert!(expected >= 0);
        assert!(expected <= favorite.len() as i32);
        assert_eq!(Solution::maximum_invitations(favorite.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[2, 2, 1, 2], 3)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 0], 3)
    }

    #[test]
    fn ex3() {
        test(&[3, 0, 1, 4, 1], 4)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 0], 2)
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 2, 3, 4, 5, 0], 6)
    }

    #[test]
    fn discussion_case3() {
        test(
            &[6, 4, 4, 5, 0, 3, 3],
            // 1 -> 4 -> 0 -> 6 -> 3 -> 5
            6,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[1, 0, 0, 2, 1, 4, 7, 8, 9, 6, 7, 10, 8],
            // 5 -> 4 -> 1 -> 0 -> 2 -> 3
            6,
        )
    }

    #[test]
    fn discussion_case5() {
        test(&[1, 0, 3, 2, 5, 6, 7, 4, 9, 8, 11, 10, 11, 12, 10], 11)
    }

    #[test]
    fn discussion_case6() {
        test(
            &[
                131, 316, 117, 308, 345, 374, 103, 135, 319, 422, 376, 58, 175, 269, 17, 265, 375,
                374, 419, 77, 404, 190, 48, 2, 25, 91, 37, 150, 42, 119, 182, 62, 408, 24, 359,
                358, 17, 243, 243, 53, 271, 48, 395, 45, 367, 38, 96, 44, 302, 143, 351, 360, 163,
                70, 353, 391, 224, 201, 366, 381, 173, 414, 92, 57, 398, 36, 338, 219, 418, 221,
                324, 314, 335, 353, 88, 120, 54, 117, 379, 200, 240, 92, 83, 306, 81, 236, 420, 9,
                195, 70, 152, 156, 162, 410, 288, 344, 125, 9, 217, 359, 84, 161, 195, 377, 174,
                136, 22, 351, 71, 411, 319, 77, 228, 368, 264, 80, 156, 47, 365, 34, 325, 133, 174,
                169, 237, 140, 176, 421, 21, 72, 301, 210, 286, 212, 316, 247, 287, 93, 350, 117,
                372, 374, 283, 192, 130, 0, 276, 265, 122, 266, 420, 416, 360, 83, 21, 126, 250,
                97, 246, 382, 376, 388, 378, 286, 403, 256, 322, 411, 238, 211, 25, 345, 361, 322,
                222, 37, 417, 363, 258, 286, 170, 118, 148, 272, 151, 83, 213, 127, 207, 338, 7,
                277, 124, 238, 8, 147, 190, 117, 231, 60, 6, 254, 187, 83, 193, 239, 312, 325, 287,
                404, 191, 389, 234, 152, 149, 45, 397, 136, 74, 77, 265, 216, 389, 330, 90, 380,
                385, 358, 15, 379, 339, 162, 13, 285, 120, 30, 413, 49, 328, 181, 356, 259, 149,
                316, 163, 61, 301, 85, 118, 152, 217, 21, 118, 244, 345, 363, 104, 240, 3, 46, 128,
                219, 410, 185, 186, 96, 385, 245, 4, 79, 228, 362, 346, 309, 90, 228, 132, 94, 280,
                35, 241, 207, 305, 229, 53, 154, 212, 336, 307, 397, 59, 99, 152, 115, 408, 333,
                21, 176, 320, 24, 212, 328, 96, 237, 169, 368, 356, 141, 386, 219, 25, 189, 370,
                232, 334, 181, 372, 139, 277, 5, 348, 251, 15, 28, 145, 187, 51, 83, 112, 300, 300,
                215, 104, 217, 393, 215, 242, 16, 134, 352, 420, 45, 291, 132, 151, 422, 369, 335,
                114, 318, 362, 287, 192, 76, 39, 52, 305, 284, 307, 223, 326, 74, 296, 212, 99,
                101, 176, 411, 50, 16, 392, 421, 8, 322, 143, 330, 122, 389, 259, 59, 226, 156,
                372, 393, 349, 223, 357, 270, 182, 114, 82, 318, 75, 182, 123, 38, 236, 296, 41,
                14, 293, 100, 320, 87, 158, 276, 353, 77, 103, 50, 321, 422, 274, 299, 250, 183,
                45, 374, 24, 124, 260, 371, 306, 217, 75,
            ],
            38,
        )
    }

    #[test]
    fn myex1() {
        // Shows a graph with two cycles.
        // 0 -> 1 -> 2 -> 3 -> 0
        // 4 -> 5 -> 6 -> 4
        // The larger cycle wins the answer.
        test(&[1, 2, 3, 0, 5, 6, 4], 4)
    }

    #[test]
    fn myex1_1() {
        // jff, add a couple nodes hanging off the smaller cycle.
        // Won't change the answer is the larger.
        test(&[1, 2, 3, 0, 5, 6, 4, 5, 6], 4)
    }

    #[test]
    fn my_extreme_ex1() {
        // Big cycle, nothing else
        let mut input = vec![0; 100_000];
        for i in 0..100_000 {
            input[i] = (i + 1) as i32 % 100_000;
        }
        test(&input, 100_000)
    }

    #[test]
    fn my_extreme_ex2() {
        // Big chain terminating in a 2-cycle
        let mut input = vec![0; 100_000];
        for i in 0..99_999 {
            input[i] = i as i32 + 1;
        }
        input[99_999] = 99_998;
        test(&input, 100_000)
    }

    #[test]
    fn my_extreme_ex3() {
        // Two big chains terminating in 2-cycles
        let mut input = vec![0; 100_000];
        for i in 0..99_999 {
            input[i] = i as i32 + 1;
        }
        input[99_999] = 99_998;
        input[49_999] = 49_998;
        test(&input, 100_000)
    }

    #[test]
    fn my_extreme_ex4() {
        // Big cycle and big chain terminating in 2-cycle
        // 2-cycle should win.
        let mut input = vec![0; 100_000];
        for i in 0..99_999 {
            input[i] = i as i32 + 1;
        }
        input[99_999] = 99_998;
        input[49_998] = 0;
        test(&input, 50_001)
    }
}
