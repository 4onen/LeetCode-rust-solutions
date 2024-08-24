// https://leetcode.com/problems/find-the-closest-palindrome/

pub struct Solution;

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        fn mirror_number(mut n: u64, even: bool) -> u64 {
            let mut result = n;
            if !even {
                n /= 10;
            }
            while n > 0 {
                result = result * 10 + n % 10;
                n /= 10;
            }
            result
        }
        assert!(n.len() >= 1);
        if n.len() == 1 {
            return (n.as_bytes()[0].wrapping_sub(1) as char).to_string();
        }
        assert!(n.len() <= 18);
        let num = n.parse::<u64>().unwrap();
        let middle = (n.len() + 1) / 2;
        let first_half = n[..middle].parse::<u64>().unwrap();
        let power_of_ten = 10u64.pow(n.len() as u32 - 1);
        let evenness = n.len() % 2 == 0;
        let candidates = [
            power_of_ten - 1,
            power_of_ten * 10 + 1,
            mirror_number(first_half, evenness),
            mirror_number(first_half - 1, evenness),
            mirror_number(first_half + 1, evenness),
        ];

        candidates
            .into_iter()
            .min_by_key(|&x| {
                let diff = if x > num {
                    x - num
                } else if x == num {
                    return (u64::MAX, x);
                } else {
                    num - x
                };
                (diff, x)
            })
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: &str, expected: &str) {
        assert!(n.len() >= 1);
        assert!(n.len() <= 18);
        for b in n.bytes() {
            assert!(b >= b'0');
            assert!(b <= b'9');
        }
        assert_ne!(n.as_bytes()[0], b'0');
        assert!(expected.len() >= n.len() - 1);
        assert!(expected.len() <= n.len() + 1);
        assert_eq!(Solution::nearest_palindromic(n.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("123", "121")
    }

    #[test]
    fn ex2() {
        test("1", "0")
    }

    #[test]
    fn discussion_case1() {
        test("10", "9")
    }

    #[test]
    fn discussion_case2() {
        test("100", "99")
    }

    #[test]
    fn discussion_case3() {
        test("1000", "999")
    }

    #[test]
    fn discussion_case4() {
        test("1283", "1331")
    }

    #[test]
    fn discussion_case5() {
        test("88", "77")
    }

    #[test]
    fn discussion_case6() {
        test("1837722381", "1837667381")
    }

    #[test]
    fn discussion_case7() {
        test("807045053224792883", "807045053350540708")
    }

    #[test]
    fn myex1() {
        test("1", "0")
    }

    #[test]
    fn myex2() {
        test("2", "1")
    }

    #[test]
    fn my_extreme_ex1() {
        test("100000000000000000", "99999999999999999")
    }

    #[test]
    fn my_extreme_ex2() {
        test("100000000000000002", "100000000000000001")
    }

    #[test]
    fn my_extreme_ex3() {
        test("999999999999999999", "1000000000000000001")
    }

    #[test]
    fn my_extreme_ex9() {
        test("876543210987654321", "876543211112345678")
    }
}
