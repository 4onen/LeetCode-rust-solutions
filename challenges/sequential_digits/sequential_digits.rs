// https://leetcode.com/problems/sequential-digits/

pub struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        const LUT: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut result = Vec::new();
        for len in 2..=9 {
            for window in LUT.windows(len) {
                let num = window.iter().fold(0, |acc, &x| acc * 10 + x as i32);
                if num < low {
                    continue;
                } else if num > high {
                    break;
                } else {
                    result.push(num);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234]);
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::sequential_digits(1000, 13000),
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::sequential_digits(100, 1000),
            vec![123, 234, 345, 456, 567, 678, 789]
        );
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::sequential_digits(200, 1000),
            vec![234, 345, 456, 567, 678, 789]
        );
    }
}
