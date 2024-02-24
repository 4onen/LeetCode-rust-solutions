// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/

pub struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by(|a, b| {
            let ord = a.count_ones().cmp(&b.count_ones());
            if ord == std::cmp::Ordering::Equal {
                a.cmp(b)
            } else {
                ord
            }
        });
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![0, 1, 2, 4, 8, 3, 5, 6, 7];
        assert_eq!(Solution::sort_by_bits(input), expected);
    }

    #[test]
    fn ex2() {
        let input = vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
        let expected = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        assert_eq!(Solution::sort_by_bits(input), expected);
    }
}
