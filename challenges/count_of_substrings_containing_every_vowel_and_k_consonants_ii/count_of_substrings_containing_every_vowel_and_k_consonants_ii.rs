// https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii/

pub struct Solution;

// Naive O(n^2) sol'n
// impl Solution {
//     pub fn count_of_substrings(word: String, k: i32) -> i64 {
//         let k = k as u32;
//         let word = word.as_bytes();
//         let mut count = 0;
//         for left in 0..word.len() as u32 {
//             let mut vowel_set = 0u8;
//             let mut consonant_count = 0;
//             for b in &word[left as usize..] {
//                 match b {
//                     b'a' => {
//                         vowel_set |= 1;
//                     }
//                     b'e' => {
//                         vowel_set |= 2;
//                     }
//                     b'i' => {
//                         vowel_set |= 4;
//                     }
//                     b'o' => {
//                         vowel_set |= 8;
//                     }
//                     b'u' => {
//                         vowel_set |= 16;
//                     }
//                     _ => {
//                         consonant_count += 1;
//                     }
//                 }
//                 if vowel_set == (1 | 2 | 4 | 8 | 16) && consonant_count == k {
//                     count += 1;
//                 }
//             }
//         }
//         count
//     }
// }

// Optimized O(n) sol'n
impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        const fn at_least_k_substrings(word: &[u8], k: u32) -> i64 {
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
                    count += (len - right) as i64;
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

    fn test(word: &str, k: i32, expected: i64) {
        assert!(word.len() >= 5);
        assert!(word.len() <= 200_000);
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

    #[test]
    fn myex3_1() {
        test("ieaouq", 1, 1)
    }

    #[test]
    fn myex3_2() {
        test("qieaou", 1, 1)
    }

    #[test]
    fn myex3_2_1() {
        test("qieaouq", 1, 2)
    }

    #[test]
    fn myex3_3() {
        test("ieaouq", 1, 1)
    }

    #[test]
    fn myex3_3_1() {
        test("ieaouqq", 1, 1)
    }

    #[test]
    fn discussion_case1() {
        test("iqeaouqi", 2, 3)
    }

    #[test]
    fn discusssion_case2() {
        test("aeouih", 0, 1)
    }

    #[test]
    fn discussion_case3() {
        test("aadieuoh", 1, 2)
    }

    #[test]
    fn discussion_case3_1() {
        test("aadieuoh", 2, 2)
    }

    #[test]
    fn discussion_case4() {
        test("euaoei", 1, 0)
    }

    #[test]
    fn discussion_case4_1() {
        test("euaoei", 0, 2)
    }

    #[test]
    fn discussion_case5() {
        test("aoaiuefi", 1, 4)
    }

    #[test]
    fn discussion_case6() {
        test("iiiiqeaouqiiiiiii", 2, 5 * 8 - 1)
    }

    #[test]
    fn discussion_case6_1() {
        test("iqeaouqiiiiiii", 2, 2 * 8 - 1)
    }

    #[test]
    fn discussion_case6_1_1() {
        test("aiqeouqiiiiiii", 2, 8)
    }

    #[test]
    fn discussion_case6_1_2() {
        test("iqeaouqi", 2, 3)
    }

    #[test]
    fn discussion_case6_1_3() {
        test("iqeaouqii", 2, 2 * 3 - 1)
    }

    #[test]
    fn discussion_case6_2() {
        test("iiqeaouqi", 2, 3 * 2 - 1)
    }

    #[test]
    fn my_extreme_ex1() {
        let input = vec![b'a'; 200_000];
        test(unsafe { std::str::from_utf8_unchecked(&input) }, 1, 0)
    }

    #[test]
    fn my_extreme_ex2() {
        let input = vec![b'a'; 200_000];
        test(unsafe { std::str::from_utf8_unchecked(&input) }, 0, 0)
    }

    #[test]
    fn my_extreme_ex3() {
        let input = "aeiou".repeat(200_000 / 5);
        test(&input, 1, 0)
    }

    #[test]
    fn my_extreme_ex4() {
        let input = "aeiouh".repeat(33135);
        test(&input, 1, 695815)
    }

    #[test]
    fn my_extreme_ex5() {
        let input = "aeiouh".repeat(33135);
        test(&input, 2, 1192794)
    }
}
