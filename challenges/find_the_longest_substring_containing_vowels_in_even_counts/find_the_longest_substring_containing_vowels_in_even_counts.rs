// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/

pub struct Solution;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut state = 0i8;
        let mut seen = [-1; 32];
        seen[0] = 0;
        for (i, c) in s.bytes().enumerate() {
            if seen[state as usize] >= 0 {
                ans = std::cmp::max(ans, i as i32 - seen[state as usize]);
            } else {
                seen[state as usize] = i as i32;
            }
            state ^= 1
                << match c {
                    b'a' => 0,
                    b'e' => 1,
                    b'i' => 2,
                    b'o' => 3,
                    b'u' => 4,
                    _ => continue,
                };
        }
        if seen[state as usize] >= 0 {
            ans = std::cmp::max(ans, s.len() as i32 - seen[state as usize]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(expected >= 0);
        assert!(s.len() >= expected as usize);
        assert!(s.len() <= 500_000);
        for b in s.bytes() {
            assert!(b >= b'a' && b <= b'z');
        }
        assert_eq!(
            Solution::find_the_longest_substring(s.to_string()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("eleetminicoworoep", 13)
    }

    #[test]
    fn ex1_1() {
        test("eleet", 4)
    }

    #[test]
    fn ex2() {
        test("leetcodeisgreat", 5)
    }

    #[test]
    fn ex3() {
        test("bcbcbc", 6)
    }

    #[test]
    fn discussion_case1() {
        test("yopumzgd", 4)
    }

    #[test]
    fn discussion_case2() {
        test("tttttttttttttttttttttt", 22)
    }

    #[test]
    fn discussion_case3() {
        test("z",1)
    }

    #[test]
    fn myex1() {
        test("aeiou", 0)
    }

    #[test]
    fn myex2() {
        test("aeiouaeiou", 10)
    }
}
