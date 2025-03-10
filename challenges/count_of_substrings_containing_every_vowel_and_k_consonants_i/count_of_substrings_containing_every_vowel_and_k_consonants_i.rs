// https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-i/

// Identical to ../count_of_substrings_containing_every_vowel_and_k_consonants_ii
// but the constraints on this problem are much smaller

pub struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        const fn at_least_k_substrings(word: &[u8], k: u32) -> i32 {
            let mut count = 0;
            let mut vowel_set = 0u8;
            let mut vowel_count = [0u32; 5];
            let mut consonant_count = 0;
            let mut left: u32 = 0;
            let mut right: u32 = 0;
            let len = word.len() as u32;
            while right < len {
                let b = word[right as usize];
                match b {
                    b'a' => {
                        vowel_count[0] += 1;
                        vowel_set |= 1;
                    }
                    b'e' => {
                        vowel_count[1] += 1;
                        vowel_set |= 2;
                    }
                    b'i' => {
                        vowel_count[2] += 1;
                        vowel_set |= 4;
                    }
                    b'o' => {
                        vowel_count[3] += 1;
                        vowel_set |= 8;
                    }
                    b'u' => {
                        vowel_count[4] += 1;
                        vowel_set |= 16;
                    }
                    _ => {
                        consonant_count += 1;
                    }
                }
                while left < right && vowel_set == (1 | 2 | 4 | 8 | 16) && consonant_count >= k {
                    count += (len - right) as i32;
                    let b = word[left as usize];
                    left += 1;
                    match b {
                        b'a' => {
                            vowel_count[0] -= 1;
                            if vowel_count[0] == 0 {
                                vowel_set &= !1;
                                break;
                            }
                        }
                        b'e' => {
                            vowel_count[1] -= 1;
                            if vowel_count[1] == 0 {
                                vowel_set &= !2;
                                break;
                            }
                        }
                        b'i' => {
                            vowel_count[2] -= 1;
                            if vowel_count[2] == 0 {
                                vowel_set &= !4;
                                break;
                            }
                        }
                        b'o' => {
                            vowel_count[3] -= 1;
                            if vowel_count[3] == 0 {
                                vowel_set &= !8;
                                break;
                            }
                        }
                        b'u' => {
                            vowel_count[4] -= 1;
                            if vowel_count[4] == 0 {
                                vowel_set &= !16;
                                break;
                            }
                        }
                        _ => {
                            consonant_count -= 1;
                            if consonant_count < k {
                                break;
                            }
                        }
                    }
                }
                right += 1;
            }
            count
        }
        let word = word.as_bytes();
        let k = k as u32;
        at_least_k_substrings(word, k) - at_least_k_substrings(word, k + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(word: &str, k: i32, expected: i32) {
        assert!(word.len() >= 5);
        assert!(word.len() <= 250);
        for &b in word.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert!(k >= 0);
        assert!(k <= word.len() as i32 - 5);
        assert_eq!(
            Solution::count_of_substrings(word.to_owned(), k as i32),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("aeioqq", 1, 0)
    }

    #[test]
    fn ex2() {
        test("aeiou", 0, 1)
    }

    #[test]
    fn ex3() {
        test("ieaouqqieaouqq", 1, 3)
    }
}
