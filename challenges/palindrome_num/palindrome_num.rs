pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        !x.is_negative() && {
            let digits: Vec<u8> = {
                let mut digits = Vec::with_capacity(10);
                let mut num = x;
                while num > 0 {
                    digits.push((num % 10) as u8);
                    num = num / 10;
                }
                digits
            };

            let len = digits.len();
            let to_check = len / 2;
            let front = digits.iter().take(to_check);
            let back = digits.iter().rev().take(to_check);
            let mut zipped = Iterator::zip(front, back);
            zipped.all(|(a, b)| a == b)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0() {
        assert!(Solution::is_palindrome(0));
    }
    #[test]
    fn test_121() {
        assert!(Solution::is_palindrome(121));
    }
    #[test]
    fn test_neg_121() {
        assert!(!Solution::is_palindrome(-121));
    }
    #[test]
    fn test_10() {
        assert!(!Solution::is_palindrome(10));
    }
    #[test]
    fn test_1234554321() {
        assert!(Solution::is_palindrome(1234554321));
    }
    #[test]
    fn test_1234564321() {
        assert!(!Solution::is_palindrome(1234564321));
    }
    #[test]
    fn test_123454321() {
        assert!(Solution::is_palindrome(123454321));
    }
    #[test]
    fn test_123456321() {
        assert!(!Solution::is_palindrome(123456321));
    }
    #[test]
    fn test_123654321() {
        assert!(!Solution::is_palindrome(123654321));
    }
}
