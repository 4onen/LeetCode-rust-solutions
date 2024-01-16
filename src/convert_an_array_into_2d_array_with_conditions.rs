// https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/

pub struct Solution;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut num_counts: Vec<u8> = vec![0; nums.len()];

        for num in nums {
            num_counts[(num - 1) as usize] += 1;
        }

        let max_count = num_counts.iter().max().unwrap().clone();
        let mut positive_count = num_counts.iter().filter(|&x| *x > 0).count();

        let mut result = std::vec::Vec::with_capacity(max_count as usize);

        while positive_count > 0 {
            let mut row = std::vec::Vec::with_capacity(positive_count);
            for i in 0..num_counts.len() as u8 {
                if num_counts[i as usize] > 0 {
                    row.push((i + 1) as i32);
                    num_counts[i as usize] -= 1;
                    if num_counts[i as usize] == 0 {
                        positive_count -= 1;
                    }
                }
            }
            result.push(row);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![1, 3, 4, 1, 2, 3, 1];
        let expected = vec![vec![1, 2, 3, 4], vec![1, 3], vec![1]];
        assert_eq!(Solution::find_matrix(nums), expected);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![vec![1, 2, 3, 4]];
        assert_eq!(Solution::find_matrix(nums), expected);
    }
}
