// https://leetcode.com/problems/lemonade-change/

pub struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut fives: u32 = 0;
        let mut tens: u32 = 0;
        for bill in bills {
            match bill {
                5 => fives += 1,
                10 if fives > 0 => {
                    fives -= 1;
                    tens += 1;
                }
                10 => {
                    return false;
                }
                20 if tens > 0 && fives > 0 => {
                    tens -= 1;
                    fives -= 1;
                }
                20 if fives >= 3 => {
                    fives -= 3;
                }
                20 => {
                    return false;
                }
                _ => unreachable!(),
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(bills: &[i32], expected: bool) {
        assert!(bills.len() >= 1);
        assert!(bills.len() <= 100_000);
        for &bill in bills {
            assert!(bill == 5 || bill == 10 || bill == 20);
        }
        assert_eq!(Solution::lemonade_change(bills.to_vec()), expected)
    }

    #[test]
    fn ex1() {
        test(&[5, 5, 5, 10, 20], true)
    }

    #[test]
    fn ex2() {
        test(&[5, 5, 10, 10, 20], false)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[5; 100_000], true)
    }

    #[test]
    fn my_extreme_ex2() {
        let mut input = [5; 100_000];
        input[0] = 10;
        test(&input, false)
    }

    #[test]
    fn my_extreme_ex3() {
        let mut input = [5; 100_000];
        input[1] = 10;
        test(&input, true)
    }

    #[test]
    fn myex10() {
        test(&[10], false)
    }
}
