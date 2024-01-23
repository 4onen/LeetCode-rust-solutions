// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/

pub struct Solution;

#[allow(unused_imports)]
use std::convert::TryFrom;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct Alphabet {
    bits: u32,
}
impl Alphabet {
    fn new() -> Self {
        Self { bits: 0 }
    }
    fn from_str(s: &str) -> Option<Self> {
        let mut alphabet = Self::new();
        for c in s.bytes() {
            if !alphabet.add(c) {
                return None;
            }
        }
        Some(alphabet)
    }
    fn add(&mut self, c: u8) -> bool {
        if c < b'a' || c > b'z' {
            unreachable!("invalid char: {}", c)
        }
        let bit = 1 << (c as u8 - b'a');
        let new_bits = self.bits | bit;
        if new_bits == self.bits {
            false
        } else {
            self.bits = new_bits;
            true
        }
    }
    fn merge(&self, other: &Self) -> Option<Self> {
        if self.bits & other.bits != 0 {
            None
        } else {
            Some(Alphabet {
                bits: self.bits | other.bits,
            })
        }
    }
    fn len(&self) -> u8 {
        self.bits.count_ones() as u8
    }
}

// DFS sol'n
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let alphabets: Vec<Alphabet> = arr
            .into_iter()
            .filter_map(|s| Alphabet::from_str(&s))
            .collect();
        let mut max_len = 0;
        let mut stack = vec![(Alphabet::new(), 0)];
        while let Some((alphabet, i)) = stack.pop() {
            if i == alphabets.len() {
                max_len = max_len.max(alphabet.len());
            } else {
                stack.push((alphabet, i + 1));
                let new_alphabet = alphabet.merge(&alphabets[i]);
                if let Some(new_alphabet) = new_alphabet {
                    stack.push((new_alphabet, i + 1));
                }
            }
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let arr = &["un", "iq", "ue"];
        assert_eq!(
            Solution::max_length(arr.iter().map(|s| s.to_string()).collect()),
            4
        )
    }

    #[test]
    fn ex2() {
        let arr = &["cha", "r", "act", "ers"];
        assert_eq!(
            Solution::max_length(arr.iter().map(|s| s.to_string()).collect()),
            6
        )
    }

    #[test]
    fn ex3() {
        let arr = &["abcdefghijklmnopqrstuvwxyz"];
        assert_eq!(
            Solution::max_length(arr.iter().map(|s| s.to_string()).collect()),
            26
        )
    }

    #[test]
    fn discussion_case1() {
        let arr = &[
            "fui", "lo", "yr", "i", "hxo", "rou", "q", "spu", "d", "lo", "p", "xjb", "idm", "bwj",
            "s", "ec",
        ];
        assert_eq!(
            Solution::max_length(arr.iter().map(|s| s.to_string()).collect()),
            17
        )
    }

    #[test]
    fn discussion_case2() {
        let arr = &[
            "dw", "q", "ux", "j", "he", "ev", "ly", "zix", "tth", "x", "t", "r", "ty", "n", "sei",
            "mb",
        ];
        assert_eq!(
            Solution::max_length(arr.iter().map(|s| s.to_string()).collect()),
            16
        )
    }

    #[test]
    fn discussion_case3() {
        let arr = &[
            "z",
            "chgtccakarmgp",
            "ieyfhzxtcczjhs",
            "i",
            "kxowcdbynshauqikgg",
            "aklbjxkczzjiqldciekn",
            "cvabiynubojuwa",
            "ctmszammcjwdkyigd",
            "vswykwxueeo",
            "ua",
            "rmwest",
            "jmjivmbnoexaat",
            "obbar",
            "cyek",
            "vvfxooaacpxdjzsstzbn",
            "t",
        ];
        assert_eq!(
            Solution::max_length(arr.iter().map(|s| s.to_string()).collect()),
            10
        )
    }

    #[test]
    fn discussion_case4() {
        let arr = &[
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
        ];
        assert_eq!(
            Solution::max_length(arr.iter().map(|s| s.to_string()).collect()),
            16
        )
    }

    #[test]
    fn discussion_case5() {
        let arr = &["ab", "cd", "cde", "cdef", "efg", "fgh", "abxyz"];
        assert_eq!(
            Solution::max_length(arr.iter().map(|s| s.to_string()).collect()),
            11
        )
    }
}
