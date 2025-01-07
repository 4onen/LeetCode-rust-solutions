// https://leetcode.com/problems/find-words-containing-character/

pub struct Solution;

// Initial sol'n (Not fast enough)
// impl Solution {
//     pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
//         assert!(words.len() <= 50);
//         words
//             .into_iter()
//             .enumerate()
//             .filter_map(|(i, s)| if s.contains(x) { Some(i as i32) } else { None })
//             .collect()
//     }
// }

// Dtype compressed sol'n
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        (0..words.len() as u8)
            .into_iter()
            .filter_map(|i| {
                if words[i as usize].contains(x) {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], x: char, expected: &[i32]) {
        assert!(words.len() >= 1);
        assert!(words.len() <= 50);
        assert!(x >= 'a');
        assert!(x <= 'z');
        for word in words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 50);
            for &b in word.as_bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
        }
        assert_eq!(
            Solution::find_words_containing(
                words.iter().map(|&x| x.to_owned()).collect(),
                x as char
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&["leet", "code"], 'e', &[0, 1])
    }

    #[test]
    fn ex2() {
        test(&["abc", "bcd", "aaaa", "cbc"], 'a', &[0, 2])
    }

    #[test]
    fn ex3() {
        test(&["abc", "bcd", "aaaa", "cbc"], 'z', &[])
    }
}
