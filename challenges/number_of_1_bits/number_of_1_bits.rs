// https://leetcode.com/problems/number-of-1-bits/

pub struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    pub const fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}
