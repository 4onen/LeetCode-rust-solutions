// https://leetcode.com/problems/count-elements-with-maximum-frequency/

pub struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut num_counts = [0u8; 101];
        for num in nums {
            num_counts[num as usize] += 1;
        }
        let max_count = *num_counts.iter().max().unwrap();
        num_counts
            .into_iter()
            .filter(|&x| x == max_count)
            .sum::<u8>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![1, 2, 2, 3, 1, 4];
        assert_eq!(Solution::max_frequency_elements(nums), 4);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_frequency_elements(nums), 5);
    }
}
