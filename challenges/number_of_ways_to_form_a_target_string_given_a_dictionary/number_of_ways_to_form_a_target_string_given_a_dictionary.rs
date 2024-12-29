// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary/

pub struct Solution;

// Recursive implementation (Good initial sol'n, not fast enough, doesn't support big inputs.)
// impl Solution {
//     pub fn num_ways(words: Vec<String>, target: String) -> i32 {
//         fn rec(words: &[String], target: &[u8], k: u16, j: u16) -> i64 {
//             if j >= target.len() as u16 {
//                 return 1;
//             }
//             if target.len() as u16 - j > words[0].len() as u16 - k {
//                 return 0;
//             }
//             let target_byte = target[j as usize];
//             let ways_here = words
//                 .iter()
//                 .filter(|&word| word.as_bytes()[k as usize] == target_byte)
//                 .count() as i64;
//             let ways_select_here = rec(words, target, k + 1, j + 1);
//             let ways_skip_here = rec(words, target, k + 1, j);
//             ways_here * ways_select_here + ways_skip_here
//         }
//         const MODULO: i64 = 1_000_000_007;
//         (rec(&words, target.as_bytes(), 0, 0) % MODULO) as i32
//     }
// }

// DP Sol'n (doesn't support big inputs.)
// impl Solution {
//     pub fn num_ways(words: Vec<String>, target: String) -> i32 {
//         let target = target.as_bytes();
//         let wordlen = words[0].len() as u16;
//         let targetlen = target.len() as u16;
//         let mut dp = vec![vec![0u64; (targetlen + 1) as usize]; (wordlen + 1) as usize];
//         for k in 0..=wordlen {
//             dp[k as usize][targetlen as usize] = 1;
//         }
//         for j in (0..targetlen).rev() {
//             let target_byte = target[j as usize];
//             for k in (0..wordlen).rev() {
//                 let ways_here = words
//                     .iter()
//                     .filter(|&word| word.as_bytes()[k as usize] == target_byte)
//                     .count() as u64;
//                 let ways_select_here = dp[(k + 1) as usize][(j + 1) as usize];
//                 let ways_skip_here = dp[(k + 1) as usize][j as usize];
//                 dp[k as usize][j as usize] = ways_here * ways_select_here + ways_skip_here;
//             }
//         }
//         const MODULO: u64 = 1_000_000_007;
//         (dp[0][0] % MODULO) as i32
//     }
// }

// DP Sol'n (appears to work, painfully slow.)
// impl Solution {
//     pub fn num_ways(words: Vec<String>, target: String) -> i32 {
//         type DPCount = u64;
//         const MODULO: DPCount = 1_000_000_007;
//         let target = target.as_bytes();
//         let wordlen = words[0].len() as u16;
//         let targetlen = target.len() as u16;
//         let mut dp = vec![vec![0 as DPCount; (targetlen + 1) as usize]; (wordlen + 1) as usize];
//         for k in 0..=wordlen {
//             dp[k as usize][targetlen as usize] = 1;
//         }
//         for j in (0..targetlen).rev() {
//             let target_byte = target[j as usize];
//             for k in (0..wordlen).rev() {
//                 let ways_here = words
//                     .iter()
//                     .filter(|&word| word.as_bytes()[k as usize] == target_byte)
//                     .count() as DPCount;
//                 let ways_select_here = dp[(k + 1) as usize][(j + 1) as usize];
//                 let ways_skip_here = dp[(k + 1) as usize][j as usize];
//                 dp[k as usize][j as usize] =
//                     (ways_here * ways_select_here + ways_skip_here) % MODULO;
//             }
//         }
//         dp[0][0] as i32
//     }
// }

// 2-row DP sol'n with early row exit (TLE)
// impl Solution {
//     pub fn num_ways(words: Vec<String>, target: String) -> i32 {
//         type DPCount = u64;
//         type DPIntermediate = DPCount;
//         const MODULO: DPCount = 1_000_000_007;
//         let target = target.as_bytes();
//         let wordlen = words[0].len() as u16;
//         let targetlen = target.len() as u16;
//         let mut dpjp1 = vec![1 as DPCount; (wordlen + 1) as usize];
//         let mut dpj = vec![0 as DPCount; (wordlen + 1) as usize];
//         for j in (0..targetlen).rev() {
//             let target_byte = target[j as usize];
//             for k in (0..wordlen).rev() {
//                 if k > wordlen - (targetlen - j) {
//                     dpj[k as usize] = 0;
//                     continue;
//                 }
//                 let ways_here = words
//                     .iter()
//                     .filter(|&word| word.as_bytes()[k as usize] == target_byte)
//                     .count() as DPCount;
//                 let ways_select_here = dpjp1[(k + 1) as usize];
//                 let ways_skip_here = dpj[(k + 1) as usize];
//                 dpj[k as usize] = ((ways_here as DPIntermediate
//                     * ways_select_here as DPIntermediate
//                     + ways_skip_here as DPIntermediate)
//                     % MODULO as DPIntermediate) as DPCount;
//             }
//             std::mem::swap(&mut dpjp1, &mut dpj);
//         }
//         dpjp1[0] as i32
//     }
// }

// Remove iterators from word position calculations (Speedup but still too slow)
// impl Solution {
//     pub fn num_ways(words: Vec<String>, target: String) -> i32 {
//         type DPCount = u64;
//         const MODULO: DPCount = 1_000_000_007;
//         let wordlen = words[0].len() as u16;
//         let target = target.as_bytes();
//         let targetlen = target.len() as u16;
//         let mut dpjp1 = vec![1 as DPCount; (wordlen + 1) as usize];
//         let mut dpj = vec![0 as DPCount; (wordlen + 1) as usize];
//         for j in (0..targetlen).rev() {
//             let target_byte = target[j as usize];
//             for k in (0..wordlen).rev() {
//                 if k > wordlen - (targetlen - j) {
//                     dpj[k as usize] = 0;
//                     continue;
//                 }
//                 let ways_here = {
//                     let mut ways_here = 0;
//                     for word in &words {
//                         ways_here += (word.as_bytes()[k as usize] == target_byte) as DPCount;
//                     }
//                     ways_here
//                 };
//                 let ways_select_here = dpjp1[(k + 1) as usize];
//                 let ways_skip_here = dpj[(k + 1) as usize];
//                 let mut res = ways_here * ways_select_here + ways_skip_here;
//                 if (res >> 54) > 0 {
//                     res = res % MODULO;
//                 }
//                 dpj[k as usize] = res;
//             }
//             std::mem::swap(&mut dpjp1, &mut dpj);
//         }
//         (dpjp1[0] % MODULO) as i32
//     }
// }

// Flip loops to k-outer (No faster, maybe slightly slower)
// impl Solution {
//     pub fn num_ways(words: Vec<String>, target: String) -> i32 {
//         type DPCount = u64;
//         const MODULO: DPCount = 1_000_000_007;
//         let wordlen = words[0].len() as u16;
//         let target = target.as_bytes();
//         let targetlen = target.len() as u16;
//         let mut dpkp1 = vec![0 as DPCount; (targetlen + 1) as usize];
//         let mut dpk = vec![0 as DPCount; (targetlen + 1) as usize];
//         dpk[targetlen as usize] = 1;
//         dpkp1[targetlen as usize] = 1;
//         for k in (0..wordlen).rev() {
//             for j in 0..targetlen {
//                 let target_byte = target[j as usize];
//                 if k + targetlen > j + wordlen {
//                     dpk[j as usize] = 0;
//                     continue;
//                 }
//                 let ways_here = {
//                     let mut ways_here = 0;
//                     for word in &words {
//                         ways_here += (word.as_bytes()[k as usize] == target_byte) as DPCount;
//                     }
//                     ways_here
//                 };
//                 let ways_select_here = dpkp1[(j + 1) as usize];
//                 let ways_skip_here = dpkp1[j as usize];
//                 let mut res = ways_here * ways_select_here + ways_skip_here;
//                 if (res >> 54) > 0 {
//                     res = res % MODULO;
//                 }
//                 dpk[j as usize] = res;
//             }
//             std::mem::swap(&mut dpkp1, &mut dpk);
//         }
//         (dpkp1[0] % MODULO) as i32
//     }
// }

// Precompute character frequencies in each _column_ of words
// (Massive speedup, but I can see room for improvement with k on the outside)
// impl Solution {
//     pub fn num_ways(words: Vec<String>, target: String) -> i32 {
//         type DPCount = u64;
//         const MODULO: DPCount = 1_000_000_007;
//         let wordlen = words[0].len() as u16;
//         let word_col_frequencies = {
//             let mut cols = vec![[0u16; 26]; wordlen as usize];
//             for word in words {
//                 for (col, b) in word.into_bytes().into_iter().enumerate() {
//                     cols[col][(b - b'a') as usize] += 1;
//                 }
//             }
//             cols
//         };
//         let target = target.as_bytes();
//         let targetlen = target.len() as u16;
//         let mut dpjp1 = vec![1 as DPCount; (wordlen + 1) as usize];
//         let mut dpj = vec![0 as DPCount; (wordlen + 1) as usize];
//         for j in (0..targetlen).rev() {
//             let target_byte = target[j as usize] - b'a';
//             for k in (0..wordlen).rev() {
//                 if k > wordlen - (targetlen - j) {
//                     dpj[k as usize] = 0;
//                     continue;
//                 }
//                 let ways_here = word_col_frequencies[k as usize][target_byte as usize];
//                 let ways_select_here = dpjp1[(k + 1) as usize];
//                 let ways_skip_here = dpj[(k + 1) as usize];
//                 let mut res = ways_here as DPCount * ways_select_here + ways_skip_here;
//                 if (res >> 54) > 0 {
//                     res = res % MODULO;
//                 }
//                 dpj[k as usize] = res;
//             }
//             std::mem::swap(&mut dpjp1, &mut dpj);
//         }
//         (dpjp1[0] % MODULO) as i32
//     }
// }

// K-outer precompute character frequencies
impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        type DPCount = u64;
        const MODULO: DPCount = 1_000_000_007;
        let wordlen = words[0].len() as u16;
        let word_col_frequencies = {
            let mut cols = vec![[0u16; 26]; wordlen as usize];
            for word in words {
                for (col, b) in word.into_bytes().into_iter().enumerate() {
                    cols[col][(b - b'a') as usize] += 1;
                }
            }
            cols
        };
        let target = target.as_bytes();
        let targetlen = target.len() as u16;
        let mut dpkp1 = vec![0 as DPCount; (targetlen + 1) as usize];
        let mut dpk = vec![0 as DPCount; (targetlen + 1) as usize];
        dpk[targetlen as usize] = 1;
        dpkp1[targetlen as usize] = 1;
        for k in (0..wordlen).rev() {
            let letter_freqs = word_col_frequencies[k as usize];
            for j in 0..targetlen {
                let target_byte = target[j as usize] - b'a';
                if j + wordlen < k + targetlen {
                    dpk[j as usize] = 0;
                    continue;
                }
                let ways_here = letter_freqs[target_byte as usize];
                let ways_select_here = dpkp1[(j + 1) as usize];
                let ways_skip_here = dpkp1[j as usize];
                dpk[j as usize] =
                    (ways_here as DPCount * ways_select_here + ways_skip_here) % MODULO;
            }
            std::mem::swap(&mut dpkp1, &mut dpk);
        }
        (dpkp1[0] % MODULO) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], target: &str, expected: i32) {
        assert!(words.len() >= 1);
        assert!(words.len() <= 1000);
        for &word in words {
            assert!(word.len() >= 1);
            assert!(word.len() <= 1000);
            assert_eq!(word.len(), words[0].len());
            for &b in word.as_bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
        }
        assert!(target.len() >= 1);
        assert!(target.len() <= 1000);
        for &b in target.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert!(expected >= 0);
        assert!(expected <= 1_000_000_007);
        assert_eq!(
            Solution::num_ways(
                words.iter().map(|x| x.to_string()).collect(),
                target.to_string()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&["acca", "bbbb", "caca"], "aba", 6)
    }

    #[test]
    fn ex2() {
        test(&["abba", "baab"], "bab", 4)
    }

    #[test]
    fn discussion_case1() {
        test(&["abc", "abc", "abc"], "abc", 27)
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                "abasghiijjjjjhhhhgyisghiijjjjjhhhhgyiba",
                "baawwsghiijjjjjhhhhgyiwwweerrrssttuygab",
            ],
            "baa",
            14,
        )
    }

    #[test]
    fn discussion_case2_1() {
        test(&["ababa", "baaab"], "baa", 14)
    }

    #[test]
    fn discussion_case3() {
        test(
            &[
                "asghiijjjjjhhhhgyisghiijjjjjhhhhgyi",
                "awwsghiijjjjjhhhhgyiwwweerrrssttuyg",
                "asghiijjjjjhhhhgyillkjjjjhhiiiuuyyt",
            ],
            "jhhhh",
            40129,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                "asghiijjjjjhhhhgyi",
                "awwwwweerrrssttuyg",
                "allkjjjjhhiiiuuyyt",
            ],
            "jjjjjl",
            0,
        )
    }

    #[test]
    fn discussion_case5() {
        test(&["aaaabcabaxda", "absghabababa"], "aaa", 200)
    }

    #[test]
    fn discussion_case6() {
        test(&["aaaabcaa", "abbbcaba"], "aba", 75)
    }

    #[test]
    fn discussion_case7() {
        test(
            &[
                "cbabddddbc",
                "addbaacbbd",
                "cccbacdccd",
                "cdcaccacac",
                "dddbacabbd",
                "bdbdadbccb",
                "ddadbacddd",
                "bbccdddadd",
                "dcabaccbbd",
                "ddddcddadc",
                "bdcaaaabdd",
                "adacdcdcdd",
                "cbaaadbdbb",
                "bccbabcbab",
                "accbdccadd",
                "dcccaaddbc",
                "cccccacabd",
                "acacdbcbbc",
                "dbbdbaccca",
                "bdbddbddda",
                "daabadbacb",
                "baccdbaada",
                "ccbabaabcb",
                "dcaabccbbb",
                "bcadddaacc",
                "acddbbdccb",
                "adbddbadab",
                "dbbcdcbcdd",
                "ddbabbadbb",
                "bccbcbbbab",
                "dabbbdbbcb",
                "dacdabadbb",
                "addcbbabab",
                "bcbbccadda",
                "abbcacadac",
                "ccdadcaada",
                "bcacdbccdb",
            ],
            "bcbbcccc",
            677452090,
        )
    }

    #[test]
    fn discussion_case7_1() {
        test(
            &[
                "cbabddd", "addbaac", "cccbacd", "cdcacca", "dddbaca", "bdbdadb", "ddadbac",
                "bbccddd", "dcabacc", "ddddcdd", "bdcaaaa", "adacdcd", "cbaaadb", "bccbabc",
                "accbdcc", "dcccaad", "cccccac", "acacdbc", "dbbdbac", "bdbddbd", "daabadb",
                "baccdba", "ccbabaa", "dcaabcc", "bcaddda", "acddbbd", "adbddba", "dbbcdcb",
                "ddbabba", "bccbcbb", "dabbbdb", "dacdaba", "addcbba", "bcbbcca", "abbcaca",
                "ccdadca", "bcacdbc",
            ],
            "bcbbc",
            2358588,
        )
    }

    #[test]
    fn discussion_case7_2() {
        test(
            &[
                "cbabdddd", "addbaacb", "cccbacdc", "cdcaccac", "dddbacab", "bdbdadbc", "ddadbacd",
                "bbccddda", "dcabaccb", "ddddcdda", "bdcaaaab", "adacdcdc", "cbaaadbd", "bccbabcb",
                "accbdcca", "dcccaadd", "cccccaca", "acacdbcb", "dbbdbacc", "bdbddbdd", "daabadba",
                "baccdbaa", "ccbabaab", "dcaabccb", "bcadddaa", "acddbbdc", "adbddbad", "dbbcdcbc",
                "ddbabbad", "bccbcbbb", "dabbbdbb", "dacdabad", "addcbbab", "bcbbccad", "abbcacad",
                "ccdadcaa", "bcacdbcc",
            ],
            "bcbbcc",
            26891884,
        )
    }

    #[test]
    fn discussion_case7_3() {
        test(
            &[
                "cbabddddb",
                "addbaacbb",
                "cccbacdcc",
                "cdcaccaca",
                "dddbacabb",
                "bdbdadbcc",
                "ddadbacdd",
                "bbccdddad",
                "dcabaccbb",
                "ddddcddad",
                "bdcaaaabd",
                "adacdcdcd",
                "cbaaadbdb",
                "bccbabcba",
                "accbdccad",
                "dcccaaddb",
                "cccccacab",
                "acacdbcbb",
                "dbbdbaccc",
                "bdbddbddd",
                "daabadbac",
                "baccdbaad",
                "ccbabaabc",
                "dcaabccbb",
                "bcadddaac",
                "acddbbdcc",
                "adbddbada",
                "dbbcdcbcd",
                "ddbabbadb",
                "bccbcbbba",
                "dabbbdbbc",
                "dacdabadb",
                "addcbbaba",
                "bcbbccadd",
                "abbcacada",
                "ccdadcaad",
                "bcacdbccd",
            ],
            "bcbbccc",
            288395512,
        )
    }

    #[test]
    fn discussion_case7_4() {
        test(
            &[
                "cbabddddbc",
                "addbaacbbd",
                "cccbacdccd",
                "cdcaccacac",
                "dddbacabbd",
                "bdbdadbccb",
                "ddadbacddd",
                "bbccdddadd",
                "dcabaccbbd",
                "ddddcddadc",
                "bdcaaaabdd",
                "adacdcdcdd",
                "cbaaadbdbb",
                "bccbabcbab",
                "accbdccadd",
                "dcccaaddbc",
                "cccccacabd",
                "acacdbcbbc",
                "dbbdbaccca",
                "bdbddbddda",
                "daabadbacb",
                "baccdbaada",
                "ccbabaabcb",
                "dcaabccbbb",
                "bcadddaacc",
                "acddbbdccb",
                "adbddbadab",
                "dbbcdcbcdd",
                "ddbabbadbb",
                "bccbcbbbab",
                "dabbbdbbcb",
                "dacdabadbb",
                "addcbbabab",
                "bcbbccadda",
                "abbcacadac",
                "ccdadcaada",
                "bcacdbccdb",
            ],
            "bcbbccc",
            731575564,
        )
    }

    #[test]
    fn discussion_case7_5() {
        test(
            &[
                "cbabddddbc",
                "addbaacbbd",
                "cccbacdccd",
                "cdcaccacac",
                "dddbacabbd",
                "bdbdadbccb",
                "ddadbacddd",
                "bbccdddadd",
                "dcabaccbbd",
                "ddddcddadc",
                "bdcaaaabdd",
                "adacdcdcdd",
                "cbaaadbdbb",
                "bccbabcbab",
                "accbdccadd",
                "dcccaaddbc",
                "cccccacabd",
                "acacdbcbbc",
                "dbbdbaccca",
                "bdbddbddda",
                "daabadbacb",
                "baccdbaada",
                "ccbabaabcb",
                "dcaabccbbb",
                "bcadddaacc",
                "acddbbdccb",
                "adbddbadab",
                "dbbcdcbcdd",
                "ddbabbadbb",
                "bccbcbbbab",
                "dabbbdbbcb",
                "dacdabadbb",
                "addcbbabab",
                "bcbbccadda",
                "abbcacadac",
                "ccdadcaada",
            ],
            "bcbbcccc",
            882468805,
        )
    }

    #[test]
    fn discussion_case7_6() {
        test(
            &[
                "cbabddddbc",
                "addbaacbbd",
                "cccbacdccd",
                "cdcaccacac",
                "dddbacabbd",
                "bdbdadbccb",
                "ddadbacddd",
                "bbccdddadd",
                "dcabaccbbd",
                "ddddcddadc",
                "bdcaaaabdd",
                "adacdcdcdd",
                "cbaaadbdbb",
                "bccbabcbab",
                "accbdccadd",
                "dcccaaddbc",
                "cccccacabd",
            ],
            "bcbbcccc",
            1679808,
        )
    }

    #[test]
    fn discussion_case8() {
        test(
            &[
                "abcdabcdefghijklmnopqrstuabcdabcdefghijklmnopqrstuvwxyzefghijkabcdefghijklmnopqrstuvwxyzlmnopqrstuvwxyzvwxyzefghijkabcdefghijklmnopqrstuvwxyzlmnopqrstuvwxyz",
                "abcdabcdefghijklmnopqrstuabcdabcdefghijklmnopqrstuvwxyzefghijkabcdefghijklmnopqrstuvwxyzlmnopqrstuvwxyzvwxyzefghijkabcdefghijklmnopqrstuvwxyzlmnopqrstuvwxyz",
                "abcdabcdefghijklmnopqrstuabcdabcdefghijklmnopqrstuvwxyzefghijkabcdefghijklmnopqrstuvwxyzlmnopqrstuvwxyzvwxyzefghijkabcdefghijklmnopqrstuvwxyzlmnopqrstuvwxyz"
            ],
            "mn",
            189,
        )
    }

    #[test]
    fn discussion_case9() {
        test(
            &[
                "asghiijjjjjhhhhgyisghiijjjjjhhhhgyiabsghaasghiijjjjjhhhhgyisghiijjjjjhhhhgyiabsghababababababa",
                "absghabababaawwsghiijjjjjhhhhgyiwwweerrrsasghiijjjjjhhhhgyisghiijjjjjhhhhgyiabsghabababasttuyg",
                "asghiijjjjjhhhhgyillkjjjjhhiiiuuyytabsghaasghiijjjjjhhhhgyisghiijjjjjhhhhgyiabsghababababababa"
            ],
            "hi",
            893,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        test(
            &[
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; 1
            ],
            "a",
            1000,
        )
    }

    #[test]
    fn my_extreme_ex10() {
        test(
            &[
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; 10
            ],
            "a",
            10000,
        )
    }

    #[test]
    fn my_extreme_ex1000() {
        test(
            &[
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; 1000
            ],
            "a",
            1000*1000,
        )
    }

    #[test]
    fn my_extreme_ex1000_2() {
        test(
            &[
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; 1000
            ],
            "aa",
            499996507,
        )
    }

    #[test]
    fn my_extreme_ex1000_3() {
        test(
            &[
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; 1000
            ],
            "aaa",
            836831014,
        )
    }

    #[test]
    fn my_extreme_ex10_1000() {
        test(
            &[
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; 10
            ],
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            221730025,
        )
    }

    #[test]
    fn my_extreme_ex1000_1000() {
        test(
            &[
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; 1000
            ],
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            524700271,
        )
    }

    #[test]
    fn tle_case1() {
        test(&["vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv";1000],
            "vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv",
            524700271,
        )
    }
}
