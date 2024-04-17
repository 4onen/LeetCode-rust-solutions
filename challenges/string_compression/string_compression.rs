// https://leetcode.com/problems/string-compression/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn compress(chars: &mut Vec<char>) -> i32 {
//         let mut original_idx = 0;
//         let mut new_idx = 0;
//         while original_idx < chars.len() {
//             let mut original_idx2 = original_idx;
//             while original_idx2 < chars.len() && chars[original_idx2] == chars[original_idx] {
//                 original_idx2 += 1;
//             }
//             chars[new_idx] = chars[original_idx];
//             new_idx += 1;
//             if original_idx2 - original_idx > 1 {
//                 let mut count = original_idx2 - original_idx;
//                 let mut count_digits = 0;
//                 while count > 0 {
//                     count /= 10;
//                     count_digits += 1;
//                 }
//                 let mut count = original_idx2 - original_idx;
//                 for i in (0..count_digits).rev() {
//                     chars[new_idx + i] = ((count % 10) as u8 + 48u8) as char;
//                     count /= 10;
//                 }
//                 new_idx += count_digits;
//             }
//             original_idx = original_idx2;
//         }
//         unsafe { chars.set_len(new_idx) };
//         new_idx as i32
//     }
// }

// Updated, faster sol'n
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        fn write_count(chars: &mut Vec<char>, count: u16, new_len: &mut u16) {
            if count < 2 {
                return;
            }
            let mut digit_count = count;
            let mut count_digits = 0;
            while digit_count > 0 {
                digit_count /= 10;
                count_digits += 1;
            }
            let mut count = count;
            for i in (0..count_digits).rev() {
                chars[*new_len as usize + i as usize] = ((count % 10) as u8 + 48u8) as char;
                count /= 10;
            }
            *new_len += count_digits;
        }
        let mut new_len = 0u16;
        let mut last_idx = 0u16;
        for original_idx in 0..chars.len() as u16 {
            if chars[last_idx as usize] != chars[original_idx as usize] {
                chars[new_len as usize] = chars[last_idx as usize];
                new_len += 1;
                write_count(chars, original_idx - last_idx, &mut new_len);
                last_idx = original_idx;
            }
        }
        // Write the last character
        chars[new_len as usize] = chars[last_idx as usize];
        new_len += 1;
        write_count(chars, chars.len() as u16 - last_idx, &mut new_len);
        unsafe { chars.set_len(new_len as usize) };
        new_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(chars: &[char], expected: &[char]) {
        let mut chars = chars.to_vec();
        assert_eq!(Solution::compress(&mut chars), expected.len() as i32);
        assert_eq!(chars, expected);
    }

    #[test]
    fn ex1() {
        test(
            &['a', 'a', 'b', 'b', 'c', 'c', 'c'],
            &['a', '2', 'b', '2', 'c', '3'],
        )
    }

    #[test]
    fn ex2() {
        test(&['a'], &['a'])
    }

    #[test]
    fn ex3() {
        test(
            &[
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
            ],
            &['a', 'b', '1', '2'],
        )
    }

    #[test]
    fn ex4() {
        test(
            &['a', 'a', 'a', 'b', 'b', 'a', 'a'],
            &['a', '3', 'b', '2', 'a', '2'],
        )
    }

    #[test]
    fn myex1() {
        test(
            &['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
            &['a', '1', '0'],
        )
    }
}
