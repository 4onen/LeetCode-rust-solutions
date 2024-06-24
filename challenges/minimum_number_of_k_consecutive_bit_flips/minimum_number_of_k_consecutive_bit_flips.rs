// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
//         assert!(k > 0);
//         let k = k as usize;
//         assert!(k <= nums.len());
//         const fn map_bin_to_bool(num: i32) -> bool {
//             match num {
//                 0 => false,
//                 1 => true,
//                 _ => unreachable!(),
//             }
//         }
//         let mut nums_iter = nums.into_iter();
//         let mut ring_buf: Vec<bool> = nums_iter
//             .by_ref()
//             .take(k as usize)
//             .map(map_bin_to_bool)
//             .collect();
//         let mut ring_start = 0;
//         let mut flips = 0;
//         let mut is_flipped = false;
//         for x in nums_iter {
//             if ring_buf[ring_start] == is_flipped {
//                 flips += 1;
//                 is_flipped = !is_flipped;
//             }
//             let x = map_bin_to_bool(x);
//             ring_buf[ring_start] = x ^ is_flipped;
//             ring_start = (ring_start + 1) % k;
//         }
//         // If all values in the final ring buffer are the same,
//         // then we're good. Add a flip if the last value needs.
//         let exemplar = ring_buf[0];
//         if ring_buf.into_iter().all(|x| x == exemplar) {
//             if exemplar == is_flipped {
//                 flips += 1;
//             }
//             flips
//         } else {
//             -1
//         }
//     }
// }

// Optimized type sizes
impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        assert!(k > 0);
        assert!(nums.len() <= 100_000);
        assert!(k <= nums.len() as i32);
        let k = k as u32;
        const fn map_bin_to_bool(num: i32) -> bool {
            match num {
                0 => false,
                1 => true,
                _ => unreachable!(),
            }
        }
        let mut nums_iter = nums.into_iter();
        let mut ring_buf: Vec<bool> = nums_iter
            .by_ref()
            .take(k as usize)
            .map(map_bin_to_bool)
            .collect();
        let mut ring_start = 0u32;
        let mut flips = 0;
        let mut is_flipped = false;
        for x in nums_iter {
            if ring_buf[ring_start as usize] == is_flipped {
                flips += 1;
                is_flipped = !is_flipped;
            }
            let x = map_bin_to_bool(x);
            ring_buf[ring_start as usize] = x ^ is_flipped;
            ring_start = (ring_start + 1) % k;
        }
        // If all values in the final ring buffer are the same,
        // then we're good. Add a flip if the last value needs.
        let exemplar = ring_buf[0];
        if ring_buf.into_iter().all(|x| x == exemplar) {
            if exemplar == is_flipped {
                flips += 1;
            }
            flips
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: u32, expected: i32) {
        assert!(k > 0);
        assert!(nums.len() <= 100_000);
        assert!(k <= nums.len() as u32);
        assert_eq!(Solution::min_k_bit_flips(nums.to_vec(), k as i32), expected)
    }

    #[test]
    fn ex1() {
        test(&[0, 1, 0], 1, 2)
    }

    #[test]
    fn ex2() {
        test(&[1, 1, 0], 2, -1)
    }

    #[test]
    fn ex3() {
        test(&[0, 0, 0, 1, 0, 1, 1, 0], 3, 3)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0,
                0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0,
                0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1,
                1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0,
                0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0,
                0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1,
                1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0,
            ],
            5,
            126,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0,
            ],
            4,
            -1,
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[
                0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0,
            ],
            10,
            -1,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0,
            ],
            1,
            39,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            &[
                0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0,
            ],
            2,
            -1,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[0; 100_000], 100, 1000)
    }
}
