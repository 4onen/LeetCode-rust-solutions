// https://leetcode.com/problems/strange-printer/

pub struct Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        type Idx = u16;
        type Cnt = u8;
        fn recurse(chars: &[u8], i: Idx, j: Idx, dp: &mut [Vec<Cnt>]) -> Cnt {
            if i > j {
                return 0;
            }
            if i == j {
                return 1;
            }
            if dp[i as usize][j as usize] != 0 {
                return dp[i as usize][j as usize];
            }
            let mut res = 1 + recurse(chars, i + 1, j, dp);
            for m in i + 1..=j {
                if chars[i as usize] == chars[m as usize] {
                    let new_res = recurse(chars, i + 1, m - 1, dp) + recurse(chars, m, j, dp);
                    if new_res < res {
                        res = new_res;
                    }
                }
            }
            dp[i as usize][j as usize] = res;
            res
        }
        let chars: Vec<u8> = s.into_bytes();
        let n = chars.len() as Idx;
        let mut dp = vec![vec![0 as Cnt; n as usize]; n as usize];
        recurse(&chars, 0, n as Idx - 1, &mut dp) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 100);
        for b in s.bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert_eq!(Solution::strange_printer(s.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test("aaabbb", 2)
    }

    #[test]
    fn ex2() {
        test("aba", 2)
    }

    #[test]
    fn discussion_case1() {
        test("abbaabb", 3)
    }

    #[test]
    fn discussion_case2() {
        test("zuvckrvtmihlhnbbgycnxthqtskcjgakbypnrkhduqqcdsfksjzscjivbtzmbzxezosrabwurnywhdizmktqtcnuxmjyoidpwxg", 74)
    }

    #[test]
    fn discussion_case3() {
        test("tbgtgb", 4)
    }

    #[test]
    fn discussion_case4() {
        test("bhjdfbdj", 6)
    }

    #[test]
    fn discussion_case5() {
        // Compression equivalence
        test("aaabbbcccaa", 3);
        test("abca", 3);
    }

    #[test]
    fn discussion_case6() {
        test("abcabcabc", 7)
    }
}
