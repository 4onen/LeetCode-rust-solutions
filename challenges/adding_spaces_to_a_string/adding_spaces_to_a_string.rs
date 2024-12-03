// https://leetcode.com/problems/adding-spaces-to-a-string/

pub struct Solution;

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let s = s.as_bytes();
        let mut input_idx = 0;
        let mut space_idx = 0; // Spaces are sorted in ascending order
        let mut result = std::vec::Vec::with_capacity(s.len() + spaces.len());
        while input_idx < s.len() {
            if space_idx < spaces.len() && spaces[space_idx] == input_idx as i32 {
                result.push(b' ');
                space_idx += 1;
            }
            result.push(s[input_idx]);
            input_idx += 1;
        }
        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, spaces: &[i32], expected: &str) {
        assert_eq!(
            Solution::add_spaces(s.to_string(), spaces.to_vec()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            "LeetcodeHelpsMeLearn",
            &[8, 13, 15],
            "Leetcode Helps Me Learn",
        )
    }

    #[test]
    fn ex2() {
        test("icodeinpython", &[1, 5, 7, 9], "i code in py thon")
    }

    #[test]
    fn ex3() {
        test("spacing", &[0, 1, 2, 3, 4, 5, 6], " s p a c i n g")
    }
}
