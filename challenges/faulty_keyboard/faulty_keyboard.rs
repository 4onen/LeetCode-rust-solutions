// https://leetcode.com/problems/faulty-keyboard/

pub struct Solution;

// Initial sol'n (Slower, but I like it better)
impl Solution {
    pub fn final_string(s: String) -> String {
        let bs = s.as_bytes();
        let mut deque = std::collections::VecDeque::with_capacity(bs.len());
        let mut flipped = false;
        for &b in bs {
            if b == b'i' {
                flipped = !flipped;
            } else if flipped {
                deque.push_front(b);
            } else {
                deque.push_back(b);
            }
        }
        let mut res: Vec<u8> = deque.into();
        if flipped {
            res.reverse();
        }
        unsafe { std::string::String::from_utf8_unchecked(res) }
    }
}

// Less-thinky-more-doey sol'n
// impl Solution {
//     pub fn final_string(s: String) -> String {
//         let mut res = std::vec::Vec::with_capacity(s.len());
//         for &b in s.as_bytes() {
//             if b != b'i' {
//                 res.push(b);
//                 continue;
//             }
//             res.reverse()
//         }
//         unsafe { std::string::String::from_utf8_unchecked(res) }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: &str) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 100);
        assert_ne!(s.as_bytes()[0], b'i');
        let mut chars = [0; 26];
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
            chars[(b - b'a') as usize] += 1;
        }
        assert!(expected.len() == s.len() - chars[(b'i' - b'a') as usize]);
        let mut expected_chars = [0; 26];
        for &b in expected.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
            assert_ne!(b, b'i');
            expected_chars[(b - b'a') as usize] += 1;
        }
        expected_chars[(b'i' - b'a') as usize] = chars[(b'i' - b'a') as usize];
        assert_eq!(chars, expected_chars);
        assert_eq!(Solution::final_string(s.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test("string", "rtsng")
    }

    #[test]
    fn ex2() {
        test("poiinter", "ponter")
    }

    #[test]
    fn ex3() {
        test(
            "aibicidieifig",
            // "a"
            // "a"
            // "ab"
            // "ba"
            // "bac"
            // "cab"
            // "cabd"
            // "dbac"
            // "dbace"
            // "ecabd"
            // "ecabdf"
            // "fdbace"
            "fdbaceg",
        )
    }

    #[test]
    fn ex4() {
        test("abcidef", "cbadef")
    }
}
