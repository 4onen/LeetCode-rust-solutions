// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let sb = s.as_bytes();
        let mut min_idx = [u32::MAX; 26];
        let mut max_idx = [u32::MAX; 26];
        for i in 0..s.len() as u32 {
            let b = sb[i as usize] - b'a';
            if min_idx[b as usize] == u32::MAX {
                min_idx[b as usize] = i;
            }
            max_idx[b as usize] = i;
        }
        let mut res = 0;
        for letter in 0..=b'z' - b'a' {
            let end = max_idx[letter as usize];
            if end == u32::MAX {
                continue;
            }
            let start = min_idx[letter as usize];
            if start == end {
                continue;
            }
            let mut bitset = 0u32;
            for &b in &sb[(start as usize + 1)..(end as usize)] {
                bitset |= 1 << (b - b'a');
            }
            res += bitset.count_ones() as i32;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(s.len() >= 3);
        assert!(s.len() <= 100_000);
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert_eq!(
            Solution::count_palindromic_subsequence(s.to_owned()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("aabca", 3)
    }

    #[test]
    fn ex2() {
        test("adc", 0)
    }

    #[test]
    fn ex3() {
        test("bbcbaba", 4)
    }
}
