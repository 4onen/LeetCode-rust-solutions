// https://leetcode.com/problems/word-subsets/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn word_subsets(mut words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
//         const fn to_count(s: &str) -> [u8; 26] {
//             let sb = s.as_bytes();
//             let mut res = [0u8; 26];
//             let mut i = 0;
//             while i < sb.len() {
//                 res[(sb[i as usize] - b'a') as usize] += 1;
//                 i += 1;
//             }
//             res
//         }
//         let words2_counts: std::collections::HashSet<_> =
//             words2.iter().map(|x| to_count(&x)).collect();
//         let mut i = 0;
//         'outer: while i < words1.len() {
//             let count = to_count(&words1[i]);
//             for subset in words2_counts.iter() {
//                 for letter in 0..26 {
//                     if subset[letter] > count[letter] {
//                         words1.swap_remove(i);
//                         continue 'outer;
//                     }
//                 }
//             }
//             i += 1;
//         }
//         words1
//     }
// }

// Optimization: Only need max count of each letter
// impl Solution {
//     pub fn word_subsets(mut words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
//         const fn to_count(s: &str) -> [u8; 26] {
//             let sb = s.as_bytes();
//             let mut res = [0u8; 26];
//             let mut i = 0;
//             while i < sb.len() {
//                 res[(sb[i as usize] - b'a') as usize] += 1;
//                 i += 1;
//             }
//             res
//         }
//         let words2_counts = words2.iter().fold([0u8; 26], |mut acc, word| {
//             let count = to_count(&word);
//             for letter in 0..26 {
//                 if count[letter] > acc[letter] {
//                     acc[letter] = count[letter];
//                 }
//             }
//             acc
//         });
//         let mut i = 0;
//         'outer: while i < words1.len() {
//             let count = to_count(&words1[i]);
//             for letter in 0..26 {
//                 if words2_counts[letter] > count[letter] {
//                     words1.swap_remove(i);
//                     continue 'outer;
//                 }
//             }
//             i += 1;
//         }
//         words1
//     }
// }

// Optimization: Remove iterator use & counting function abstraction
impl Solution {
    pub fn word_subsets(mut words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut words2_counts = [0u8; 26];
        for word in words2 {
            let mut count = [0u8; 26];
            for &b in word.as_bytes() {
                count[(b - b'a') as usize] += 1;
            }
            for letter in 0..26 {
                if count[letter] > words2_counts[letter] {
                    words2_counts[letter] = count[letter];
                }
            }
        }
        let mut i = 0;
        'outer: while i < words1.len() {
            let mut count = [0u8;26];
            for &b in words1[i].as_bytes() {
                count[(b - b'a') as usize] += 1;
            }
            for letter in 0..26 {
                if words2_counts[letter] > count[letter] {
                    words1.swap_remove(i);
                    continue 'outer;
                }
            }
            i += 1;
        }
        words1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words1: &[&str], words2: &[&str], expected: &[&str]) {
        assert!(words1.len() >= 1);
        assert!(words1.len() <= 10_000);
        assert!(words2.len() >= 1);
        assert!(words2.len() <= 10_000);
        let mut seen = std::collections::HashSet::new();
        for &word in words1 {
            assert!(word.len() >= 1);
            assert!(word.len() <= 10);
            for &b in word.as_bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
            assert!(seen.insert(word));
        }
        for &word in words2 {
            assert!(word.len() >= 1);
            assert!(word.len() <= 10);
            for &b in word.as_bytes() {
                assert!(b >= b'a');
                assert!(b <= b'z');
            }
        }

        let mut res = Solution::word_subsets(
            words1.iter().map(|&x| x.to_owned()).collect(),
            words2.iter().map(|&x| x.to_owned()).collect(),
        );
        let mut expected_res = expected.to_vec();
        res.sort_unstable();
        expected_res.sort_unstable();
        assert_eq!(res, expected_res);
    }

    #[test]
    fn ex1() {
        test(
            &["amazon", "apple", "facebook", "google", "leetcode"],
            &["e", "o"],
            &["facebook", "google", "leetcode"],
        )
    }

    #[test]
    fn ex2() {
        test(
            &["amazon", "apple", "facebook", "google", "leetcode"],
            &["l", "e"],
            &["apple", "google", "leetcode"],
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                "pjvmqekfpa",
                "pqekvfmlac",
                "nkvamfpsqc",
                "vafqppqmkh",
                "qhpfmkbvas",
                "mvapkbnqaf",
                "vpiqmfzjak",
                "hunjgeaolc",
                "avkdqlpmft",
                "avskfpmmlq",
                "aecglrhxsg",
                "aapmqkfvlf",
                "mfpakqavap",
                "kavfwqepdm",
                "mytxzbmqma",
                "ozvmspkfaq",
                "zafjmpkqpv",
                "faaapumkvq",
                "amdkqpfijv",
                "qpafqvfkpm",
                "mqfckamqvp",
                "fwapsmokvq",
                "bkfzqamvzp",
                "qckmpfsfva",
                "zbmnfykeyw",
                "pjmqavfkjf",
                "scvakpcfmq",
                "vpjmfawqke",
                "nfzkpmvaaq",
                "afqmekopva",
                "vpmvpafkqy",
                "xfupkvmqad",
                "aggvkpmbfq",
                "cmpqvxatfk",
                "qkmfpvnamq",
                "utlusqhbfy",
                "oppjejmmcy",
                "wndhvprmoz",
                "kxpsawmvqf",
                "pxsvamnkqf",
                "vfmqlkplah",
                "avppfkqudm",
                "ksyvfpaqgm",
                "fmkhfpdqva",
                "pqlrvxamfk",
                "dfqvimjkap",
                "qpvmfwakxb",
                "rrkfcvqapm",
                "vhqrakfvmp",
                "msqhkfapvw",
                "qpfmktpmav",
                "okzckhugsd",
                "fpxkqdmagv",
                "aqpykvofqm",
                "avaqzpfkwm",
                "kvfpamvqzx",
                "pqfkmvacgm",
                "qvmgftkupa",
                "kelxxnfiei",
                "fkpakvqmau",
                "qkxxamvpfa",
                "kqqmcmfpav",
                "mqafvubkvp",
                "fcikmvxapq",
                "kpnfarmvqs",
                "vpmakfnpiq",
                "avkfwmspqk",
                "pmnfkwvqak",
                "avlfkqpvkm",
                "pymkvfqjpa",
                "qmfkaxpfrv",
                "qkkvffapwm",
                "fcwenlxtla",
                "qhpkvfmyaj",
                "fajhfkpqvm",
                "smvkpsaqzf",
                "fpthakqmxv",
                "tmqfawkjvp",
                "vkyaytqpmf",
                "apmtsqvwfk",
                "nujagbkcqj",
                "qwpfkamvpf",
                "mlvqkmpfal",
                "dduzzxmskv",
                "kjkggnwvar",
                "kqsvpamwxf",
                "apqfwekmmv",
                "mvbfpikeqa",
                "qviknmaifp",
                "sokvapfqbm",
                "mvpqaxfqfk",
                "klfvlqmppa",
                "azqkixfpvm",
                "gtframkvqp",
                "wkspfmavkq",
                "mpafjcvykq",
                "rmpxfqkiav",
                "gyaifdnbli",
                "wwnzulopnh",
                "vpafvcmkaq",
            ],
            &[
                "v", "vm", "vma", "vkq", "av", "vapmq", "fmqk", "qam", "k", "kqmvf", "aqpfkv",
                "kmqpfa", "fva", "mqfa", "vamkfq", "pa", "qpk", "qfka", "qa", "f", "pmq", "qmk",
                "mk", "fpamkv", "vk", "vpkm", "mva", "v", "kvaf", "aqvkmf", "qm", "v", "vapmf",
                "fvk", "mp", "avkmqf", "maf", "kfvqm", "k", "kaq", "a", "vk", "mafp", "k", "kpa",
                "pfqvm", "aq", "fa", "pqfkav", "p", "kvfpm", "q", "kfmqp", "a", "pvk", "q",
                "pfkvqa", "vkfamq", "vkfmpq", "afk", "p", "kvpam", "vkpaq", "vkqfap", "qk", "q",
                "qk", "vkamp", "ma", "v", "qvapmk", "vmqafk", "afqpv", "kavfmq", "f", "pvma",
                "paqvmk", "v", "amqkv", "mkpfv", "pkmv", "mkqvpf", "q", "vm", "v", "mfav", "fm",
                "f", "pmfvq", "qk", "fpqmk", "pamfv", "fv", "pv", "apfmvq", "qv", "v", "kvm",
                "pvmaq", "fvp",
            ],
            &[
                "pjvmqekfpa",
                "pqekvfmlac",
                "nkvamfpsqc",
                "vafqppqmkh",
                "qhpfmkbvas",
                "mvapkbnqaf",
                "vpiqmfzjak",
                "avkdqlpmft",
                "avskfpmmlq",
                "aapmqkfvlf",
                "mfpakqavap",
                "kavfwqepdm",
                "ozvmspkfaq",
                "zafjmpkqpv",
                "faaapumkvq",
                "amdkqpfijv",
                "qpafqvfkpm",
                "mqfckamqvp",
                "fwapsmokvq",
                "bkfzqamvzp",
                "qckmpfsfva",
                "pjmqavfkjf",
                "scvakpcfmq",
                "vpjmfawqke",
                "nfzkpmvaaq",
                "afqmekopva",
                "vpmvpafkqy",
                "xfupkvmqad",
                "aggvkpmbfq",
                "cmpqvxatfk",
                "qkmfpvnamq",
                "kxpsawmvqf",
                "pxsvamnkqf",
                "vfmqlkplah",
                "avppfkqudm",
                "ksyvfpaqgm",
                "fmkhfpdqva",
                "pqlrvxamfk",
                "dfqvimjkap",
                "qpvmfwakxb",
                "rrkfcvqapm",
                "vhqrakfvmp",
                "msqhkfapvw",
                "qpfmktpmav",
                "fpxkqdmagv",
                "aqpykvofqm",
                "avaqzpfkwm",
                "kvfpamvqzx",
                "pqfkmvacgm",
                "qvmgftkupa",
                "fkpakvqmau",
                "qkxxamvpfa",
                "kqqmcmfpav",
                "mqafvubkvp",
                "fcikmvxapq",
                "kpnfarmvqs",
                "vpmakfnpiq",
                "avkfwmspqk",
                "pmnfkwvqak",
                "avlfkqpvkm",
                "pymkvfqjpa",
                "qmfkaxpfrv",
                "qkkvffapwm",
                "qhpkvfmyaj",
                "fajhfkpqvm",
                "smvkpsaqzf",
                "fpthakqmxv",
                "tmqfawkjvp",
                "vkyaytqpmf",
                "apmtsqvwfk",
                "qwpfkamvpf",
                "mlvqkmpfal",
                "kqsvpamwxf",
                "apqfwekmmv",
                "mvbfpikeqa",
                "qviknmaifp",
                "sokvapfqbm",
                "mvpqaxfqfk",
                "klfvlqmppa",
                "azqkixfpvm",
                "gtframkvqp",
                "wkspfmavkq",
                "mpafjcvykq",
                "rmpxfqkiav",
                "vpafvcmkaq",
            ],
        )
    }
}
