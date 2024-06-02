// https://leetcode.com/problems/reverse-string/

pub struct Solution;

// Initial solution
// impl Solution {
//     pub fn reverse_string(s: &mut Vec<char>) {
//         s.reverse();
//     }
// }

// Hand (optimized) solution
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        assert!(s.len() > 0);
        assert!(s.len() <= 100_000);
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            let temp = s[i];
            s[i] = s[j];
            s[j] = temp;
            i += 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str) {
        let mut input = s.chars().collect();
        Solution::reverse_string(&mut input);
        assert_eq!(
            input.iter().collect::<String>(),
            s.chars().rev().collect::<String>()
        );
    }

    #[test]
    fn ex1() {
        test("hello");
    }

    #[test]
    fn ex2() {
        test("Hannah");
    }
}
