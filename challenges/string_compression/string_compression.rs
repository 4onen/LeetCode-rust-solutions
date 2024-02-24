// https://leetcode.com/problems/string-compression/

pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut original_idx = 0;
        let mut new_idx = 0;
        while original_idx < chars.len() {
            let mut original_idx2 = original_idx;
            while original_idx2 < chars.len() && chars[original_idx2] == chars[original_idx] {
                original_idx2 += 1;
            }
            chars[new_idx] = chars[original_idx];
            new_idx += 1;
            if original_idx2 - original_idx > 1 {
                let mut count = original_idx2 - original_idx;
                let mut count_digits = 0;
                while count > 0 {
                    count /= 10;
                    count_digits += 1;
                }
                let mut count = original_idx2 - original_idx;
                for i in (0..count_digits).rev() {
                    chars[new_idx + i] = ((count % 10) as u8 + 48u8) as char;
                    count /= 10;
                }
                new_idx += count_digits;
            }
            original_idx = original_idx2;
        }
        unsafe { chars.set_len(new_idx) };
        new_idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(Solution::compress(&mut chars), 6);
        assert_eq!(chars, vec!['a', '2', 'b', '2', 'c', '3']);
    }

    #[test]
    fn ex2() {
        let mut chars = vec!['a'];
        assert_eq!(Solution::compress(&mut chars), 1);
        assert_eq!(chars, vec!['a']);
    }

    #[test]
    fn ex3() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(Solution::compress(&mut chars), 4);
        assert_eq!(chars, vec!['a', 'b', '1', '2']);
    }

    #[test]
    fn ex4() {
        let mut chars = vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'];
        assert_eq!(Solution::compress(&mut chars), 6);
        assert_eq!(chars, vec!['a', '3', 'b', '2', 'a', '2']);
    }

    #[test]
    fn myex1() {
        let mut chars = vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'];
        assert_eq!(Solution::compress(&mut chars), 3);
        assert_eq!(chars, vec!['a', '1', '0']);
    }
}
