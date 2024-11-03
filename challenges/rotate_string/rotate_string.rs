// https://leetcode.com/problems/rotate-string/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let mut s = s.into_bytes();
        s.extend_from_within(..);
        unsafe { std::str::from_utf8_unchecked(&s) }.contains(&goal)
    }
}

// Iterator chain incomplete sol'n
// impl Solution {
//     pub fn rotate_string(s: String, goal: String) -> bool {
//         let s = s.as_bytes();
//         let g = goal.as_bytes();
//         if s.len() != g.len() {
//             return false;
//         }
//         let mut m = s.into_iter().copied().chain(s.into_iter().copied());
//         // No easy way to do this search problem
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, goal: &str, expected: bool) {
        assert_eq!(
            Solution::rotate_string(s.to_string(), goal.to_string()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("abcde", "cdeab", true)
    }

    #[test]
    fn ex2() {
        test("abcde", "abced", false)
    }

    #[test]
    fn failing_case1() {
        // Accidentally removed uneqal length check
        test("aa", "a", false)
    }
}
