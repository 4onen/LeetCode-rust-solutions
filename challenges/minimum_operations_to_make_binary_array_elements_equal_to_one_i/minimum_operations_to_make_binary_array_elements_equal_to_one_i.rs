// https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/

pub struct Solution;

// Copied from challenges/minimum_number_of_k_consecutive_bit_flips/minimum_number_of_k_consecutive_bit_flips.rs
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        const K: u32 = 3;
        assert!(nums.len() <= 100_000);
        assert!(nums.len() as u32 >= K);
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
            .take(K as usize)
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
            ring_start = (ring_start + 1) % K;
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

    fn test(nums: &[i32], expected: i32) {
        assert_eq!(Solution::min_operations(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[0, 1, 1, 1, 0, 0], 3);
    }

    #[test]
    fn ex2() {
        test(&[0, 1, 1, 1], -1);
    }
}
