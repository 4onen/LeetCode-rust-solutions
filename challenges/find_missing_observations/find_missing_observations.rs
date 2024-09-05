// https://leetcode.com/problems/find-missing-observations/

pub struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        match mean {
            1|6 => {
                if rolls.into_iter().all(|x| x == mean) {
                    vec![mean; n as usize]
                } else {
                    vec![]
                }
            }
            2..=5 => {
                let sum: i32 = rolls.iter().sum();
                let total = mean * (rolls.len() as i32 + n);
                let missing_sum = total - sum;
                if missing_sum < n || missing_sum > 6 * n {
                    vec![]
                } else {
                    let mut result = vec![missing_sum / n; n as usize];
                    for i in 0..(missing_sum % n) as usize {
                        result[i] += 1;
                    }
                    result
                }
            }
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(rolls: &[u8], mean: u8, n: u32, has_answer: bool) {
        let i32_rolls: Vec<i32> = rolls.iter().map(|&x| x as i32).collect();
        let result = Solution::missing_rolls(i32_rolls.clone(), mean as i32, n as i32);
        if !has_answer {
            assert_eq!(result, []);
        } else {
            assert_eq!(result.len(), n as usize);
            let sum: i32 = i32_rolls.iter().sum::<i32>() + result.iter().sum::<i32>();
            let expected_sum = (mean as i32) * (rolls.len() as i32 + n as i32);
            assert_eq!(sum, expected_sum);
        }
    }

    #[test]
    fn ex1() {
        test(
            &[3, 2, 4, 3],
            4,
            2,
            true,
        )
    }

    #[test]
    fn ex2() {
        test(
            &[1, 5, 6],
            3,
            4,
            true,
        )
    }

    #[test]
    fn ex3() {
        test(
            &[1, 2, 3, 4],
            6,
            4,
            false,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        test(
            &[6; 100_000],
            6,
            100_000,
            true,
        )
    }

    #[test]
    fn my_extreme_ex2() {
        test(
            &[1; 100_000],
            1,
            100_000,
            true,
        )
    }

    #[test]
    fn my_extreme_ex3() {
        test(
            &[6; 100_000],
            3,
            100_000,
            false,
        )
    }

    #[test]
    fn my_extreme_ex4() {
        test(
            &[1; 100_000],
            3,
            100_000,
            true,
        )
    }

    #[test]
    fn my_extreme_ex5() {
        test(
            &[6; 100_000],
            5,
            100_000,
            true,
        )
    }

    #[test]
    fn my_extreme_ex6() {
        test(
            &[1; 100_000],
            5,
            100_000,
            false,
        )
    }
}
