// https://leetcode.com/problems/clear-digits/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn clear_digits(s: String) -> String {
//         let s = s.as_bytes();
//         let mut result = std::vec::Vec::with_capacity(s.len());
//         for &b in s {
//             match b {
//                 (b'0'..=b'9') => {
//                     result.pop();
//                 }
//                 (b'a'..=b'z') => {
//                     result.push(b);
//                 }
//                 _ => unreachable!(),
//             }
//         }
//         unsafe { std::string::String::from_utf8_unchecked(result) }
//     }
// }

// Two-pointer sol'n
impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut left = 0;
        let mut result = s.into_bytes();
        for right in 0..result.len() {
            if result[right].is_ascii_digit() {
                left -= 1;
            } else {
                result[left] = result[right];
                left += 1;
            }
        }
        result.truncate(left);
        unsafe { std::string::String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: &str) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 100);
        assert!(expected.len() <= s.len());
        for &b in s.as_bytes() {
            assert!((b >= b'a' || b <= b'z') || b.is_ascii_digit());
        }
        for &b in expected.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        let result = Solution::clear_digits(s.to_owned());
        assert_eq!(result, expected);
    }

    #[test]
    fn ex1() {
        test("abc", "abc")
    }

    #[test]
    fn ex2() {
        test("cb34", "")
    }

    #[test]
    fn discussion_case1() {
        test("a", "a")
    }

    #[test]
    fn discussion_case2() {
        test("xzuzr2ghilydk", "xzuzghilydk")
    }

    #[test]
    fn discussion_case3() {
        test(
            "qm93xjkpaaovhqckjhg5j1o4rndtg3bobgeke",
            "xjkpaaovhqckjhrndtbobgeke",
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            "pl5v0jttxe9acvd0t9vtxwrhvwajpasfe2nhtws48pweam4vsomd79nw14ed",
            "pjttxacvvtxwrhvwajpasfnhtpweavsoed",
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            "rrytkz6w5qwniszr6duuec0lqbeds3qdbzimoszqwxng5c02vxse03hpoarbu",
            "rrytkqwniszduuelqbedqdbzimoszqwxvxhpoarbu",
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            "wezofxhb44upfwyci72gbi2jwdxxank64yxvr677aegwl7jzgqlof4z9neisjq88pj1pvq98q",
            "wezofxupfwygbjwdxxayaegwjzgqloneisppq",
        )
    }

    #[test]
    fn discussion_case7() {
        test(
            "g8dsianl4u49d3froonbnkrw83qzmdbh114lidlc2bv95s2pzwzuimi3ef443txu5d6h8ng5j5stu32y",
            "dsiafroonbnkqzmlidlpzwzuitxnsy",
        )
    }

    #[test]
    fn discussion_case8() {
        test(
            "sl2yqa6z8i0eqyqnla250rh8olipeu4oie5t16n8p2n67e391n01s7ol11qg93u6tuoy55no117861sj4wfno27p65212",
            "syqeqyq"
        )
    }
}
