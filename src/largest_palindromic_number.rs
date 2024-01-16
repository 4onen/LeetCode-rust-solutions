// https://leetcode.com/problems/largest-palindromic-number/

pub struct Solution;

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let max_digits = num.len();
        let mut digits: Vec<u8> = num.as_bytes().into_iter().map(|b| b - b'0').collect();
        digits.sort_unstable();

        // Now that the digits are lined up, we need to count the number of
        // pairs of each digit.
        let mut counts = [0usize; 10];
        for digit in digits {
            counts[digit as usize] += 1;
        }
        let max_odd: Option<u8> = counts
            .into_iter()
            .rposition(|c| c & 1 == 1)
            .map(|i| i as u8);
        let pairs: [usize; 10] = counts.map(|c| c >> 1);

        if pairs[1..].iter().any(|c| *c > 0) {
            // We have at least one nonzero pair.
            // Now we need to build the palindrome.
            let mut palindrome = Vec::with_capacity(max_digits);
            for i in (0..10).into_iter().rev() {
                palindrome.extend(std::iter::repeat(i as u8 + b'0').take(pairs[i]));
            }
            if let Some(max_odd) = max_odd {
                palindrome.push(max_odd + b'0');
            }
            for i in (0..10).into_iter() {
                palindrome.extend(std::iter::repeat(i as u8 + b'0').take(pairs[i]));
            }
            unsafe { String::from_utf8_unchecked(palindrome) }
        } else {
            // We have no nonzero pairs. The `max_odd` should be populated
            // or we got an even number of zeros.
            if let Some(max_odd) = max_odd {
                unsafe { String::from_utf8_unchecked(vec![max_odd + b'0']) }
            } else {
                "0".into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::largest_palindromic("444947137".into()), "7449447");
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::largest_palindromic("0009".into()), "9");
    }

    #[test]
    fn failing_case() {
        assert_eq!(Solution::largest_palindromic("0000".into()), "0");
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::largest_palindromic("1234".into()), "4");
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::largest_palindromic("123456789".into()), "9");
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::largest_palindromic("1234567890".into()), "9");
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::largest_palindromic("12345678901".into()), "191");
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::largest_palindromic("123456789012".into()),
            "21912"
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::largest_palindromic("1234567890123".into()),
            "3219123"
        );
    }
}
