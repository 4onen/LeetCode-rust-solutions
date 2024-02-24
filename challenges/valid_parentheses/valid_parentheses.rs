pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::new();
        for c in s.into_bytes() {
            match c {
                b'(' | b'[' | b'{' => stack.push(c),
                b')' => {
                    if stack.pop() != Some(b'(') {
                        return false;
                    }
                }
                b']' => {
                    if stack.pop() != Some(b'[') {
                        return false;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(b'{') {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }

    #[test]
    fn test_is_invalid() {
        assert_eq!(Solution::is_valid("[".to_string()), false);
        assert_eq!(Solution::is_valid("]".to_string()), false);
        assert_eq!(Solution::is_valid("".to_string()), true);
    }
}
