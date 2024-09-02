// https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/

pub struct Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, mut k: i32) -> i32 {
        let mut sum = 0;
        for i in 0..chalk.len() {
            sum += chalk[i];
            k -= chalk[i];
            if k < 0 {
                return i as i32
            }
        }
        return Self::chalk_replacer(chalk, k % sum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(chalk: &[u32], k: u32, expected: u32) {
        assert!(chalk.len() >= 1);
        assert!(chalk.len() <= 100_000);
        for &c in chalk {
            assert!(c >= 1);
            assert!(c <= 100_000);
        }
        assert!(k >= 1);
        assert!(k <= 1_000_000_000);
        assert_eq!(Solution::chalk_replacer(chalk.into_iter().map(|&x| x as i32).collect(), k as i32), expected as i32);
    }

    #[test]
    fn ex1() {
        test(
            &[5, 1, 5],
            22,
            0,
        )
    }

    #[test]
    fn ex2() {
        test(
            &[3, 4, 1, 2],
            25,
            1,
        )
    }
}
