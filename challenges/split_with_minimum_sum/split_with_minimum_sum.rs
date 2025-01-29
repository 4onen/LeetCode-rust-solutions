// https://leetcode.com/problems/split-with-minimum-sum/

pub struct Solution;

impl Solution {
    pub fn split_num(mut num: i32) -> i32 {
        let mut digits = [0u8; 10];
        while num > 0 {
            let (div, rem) = (num / 10, num % 10);
            num = div;
            digits[rem as usize] += 1;
        }
        let mut digit = 1;
        let mut num1 = 0;
        let mut num2 = 0;
        loop {
            while digit < digits.len() as u8 && digits[digit as usize] <= 0 {
                digit += 1;
            }
            if digit >= digits.len() as u8 {
                break;
            }
            num1 = num1 * 10 + digit as i32;
            digits[digit as usize] -= 1;
            while digit < digits.len() as u8 && digits[digit as usize] <= 0 {
                digit += 1;
            }
            if digit >= digits.len() as u8 {
                break;
            }
            num2 = num2 * 10 + digit as i32;
            digits[digit as usize] -= 1;
        }
        num1 + num2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(num: u32, expected: i32) {
        assert!(num >= 10);
        assert!(num <= 1_000_000_000);
        assert_eq!(Solution::split_num(num as i32), expected);
    }

    #[test]
    fn ex1() {
        test(4325, 59)
    }

    #[test]
    fn ex2() {
        test(687, 75)
    }
}
