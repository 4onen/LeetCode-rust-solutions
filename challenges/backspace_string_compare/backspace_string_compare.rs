// https://leetcode.com/problems/backspace-string-compare/

pub struct Solution;

// In-place two-pointer sol'n
impl Solution {
    fn apply_backspaces(bytes: &mut [u8]) -> &[u8] {
        let mut i: u8 = 0;
        let mut j: u8 = 0;
        while j < bytes.len() as u8 {
            match bytes[j as usize] {
                b'#' => i = i.saturating_sub(1),
                _ => {
                    bytes[i as usize] = bytes[j as usize];
                    i += 1;
                }
            }
            j += 1;
        }
        &bytes[..i as usize]
    }
    pub fn backspace_compare(mut s: String, mut t: String) -> bool {
        unsafe {
            Self::apply_backspaces(s.as_bytes_mut()) == Self::apply_backspaces(t.as_bytes_mut())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::backspace_compare("a##c".to_string(), "#a#c".to_string()),
            true
        );
    }
}
