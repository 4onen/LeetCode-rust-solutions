// https://leetcode.com/problems/number-of-wonderful-substrings/

pub struct Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        // Initial sol'n
        // let mut count = vec![0u32; 1024];
        // Stack alloc instead (Nope, slower)
        let mut count = [0u32; 1024];
        count[0] = 1;
        let mut mask = 0u16;
        let mut result = 0;
        for c in word.bytes() {
            mask ^= 1 << (c - b'a');
            result += count[mask as usize] as i64;
            for i in 0..10u8 {
                result += count[(mask ^ (1 << i)) as usize] as i64;
            }
            count[mask as usize] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(word: &str, expected: i64) {
        assert!(word.len() > 0);
        assert!(word.len() <= 10usize.pow(5));
        const ALLOWED_CHARS: &[u8] = b"abcdefghij";
        assert!(word.bytes().all(|c| ALLOWED_CHARS.contains(&c)));
        assert!(expected > 0); // Even one letter is a wonderful substring
        assert_eq!(Solution::wonderful_substrings(word.to_string()), expected);
    }

    #[test]
    fn ex0() {
        test("ccjjc", 13)
        // c, c, j, j, c -> 5
        // cc, jj -> 2
        // ccj, cjj, jjc -> 3
        // ccjj, cjjc -> 2
        // ccjjc -> 1
    }

    #[test]
    fn ex1() {
        test("aba", 4)
    }

    #[test]
    fn ex2() {
        test("aabb", 9)
    }

    #[test]
    fn ex3() {
        test("he", 2)
    }

    #[test]
    fn ex4() {
        test("a", 1)
    }

    #[test]
    fn myex1() {
        test("ab", 2)
    }

    #[test]
    fn myex2() {
        test("abc", 3)
        // a, b, c -> 3
    }

    #[test]
    fn myex3() {
        test("abcd", 4)
        // a, b, c, d -> 4
    }

    #[test]
    fn myex4() {
        test("abcc", 6)
        // a, b, c, c -> 4
        // cc -> 1
        // bcc -> 1
    }

    #[test]
    fn my_extreme_ex1() {
        const N: i64 = 10i64.pow(5);
        test("a".repeat(N as usize).as_str(), N * (N + 1) / 2)
    }

    #[test]
    fn my_extreme_ex2() {
        const N: i64 = 10i64.pow(5);
        test("ab".repeat((N / 2) as usize).as_str(), 3750025000)
    }
}
