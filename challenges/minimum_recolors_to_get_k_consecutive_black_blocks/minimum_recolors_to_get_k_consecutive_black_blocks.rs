// https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/

pub struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let k = k as u8;
        let mut white_blocks_count = 0;
        for i in 0..k {
            white_blocks_count += (blocks[i as usize] == b'W') as u8;
        }
        let mut min_recolors = white_blocks_count;
        for i in k..blocks.len() as u8 {
            white_blocks_count += (blocks[i as usize] == b'W') as u8;
            white_blocks_count -= (blocks[(i - k) as usize] == b'W') as u8;
            if white_blocks_count < min_recolors {
                min_recolors = white_blocks_count;
            }
            if min_recolors == 0 {
                break;
            }
        }
        min_recolors as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(blocks: &str, k: u8, expected: u8) {
        assert!(blocks.len() >= 1);
        assert!(blocks.len() <= 100);
        assert!(k >= 1);
        assert!(k <= blocks.len() as u8);
        assert!(expected <= k);
        assert_eq!(
            Solution::minimum_recolors(blocks.to_owned(), k as i32),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test("WBBWWBBWBW", 7, 3)
    }

    #[test]
    fn ex2() {
        test("WBWBBBW", 2, 0)
    }
}
