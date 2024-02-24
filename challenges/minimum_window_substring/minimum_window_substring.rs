// https://leetcode.com/problems/minimum-window-substring/

pub struct Solution;

// Sliding window alpahbet solution
// impl Solution {
//     pub fn min_window(s: String, t: String) -> String {
//         struct Alphabet {
//             counts: [u32; 58],
//         }
//         impl Alphabet {
//             const fn new() -> Alphabet {
//                 Alphabet { counts: [0; 58] }
//             }
//             fn add(&mut self, byte: u8) {
//                 self.counts[(byte - b'A') as usize] += 1;
//             }
//             fn remove(&mut self, byte: u8) {
//                 self.counts[(byte - b'A') as usize] -= 1;
//             }
//             fn includes(&self, other: &Alphabet) -> bool {
//                 std::iter::zip(self.counts.iter(), other.counts.iter())
//                     .all(|(mycount, tgtcount)| mycount >= tgtcount)
//             }
//         }
//         let target_bytes = t.as_bytes();
//         let target_len = target_bytes.len() as u32;
//         let target = target_bytes
//             .into_iter()
//             .fold(Alphabet::new(), |mut acc, &byte| {
//                 acc.add(byte);
//                 acc
//             });
//         let bytes = s.into_bytes();
//         let l = bytes.len() as u32;
//         let mut current_start: u32 = 0;
//         let mut current_end: u32 = 0;
//         let mut current_alphabet = Alphabet::new();
//         let mut best_range: Option<(u32, u32)> = None;
//         loop {
//             if current_alphabet.includes(&target) {
//                 // The current range is a possible valid range
//                 // Check if it's smaller than the best
//                 best_range = Some(match best_range {
//                     None => (current_start, current_end),
//                     Some((a, b)) => {
//                         let current_len = current_end - current_start;
//                         if b - a > current_len {
//                             (current_start, current_end)
//                         } else {
//                             (a, b)
//                         }
//                     }
//                 });
//                 // Now that we've adjusted our best range,
//                 // delete a character from the start and
//                 // try again.
//                 current_alphabet.remove(bytes[current_start as usize]);
//                 current_start += 1
//             } else {
//                 if current_end < l {
//                     // We haven't reached the end, but our substring isn't
//                     // valid yet. Add a character to the end and try again.
//                     current_alphabet.add(bytes[current_end as usize]);
//                     current_end += 1;
//                 } else {
//                     // We reached the end
//                     break;
//                 }
//             }
//         }
//         if let Some((a, b)) = best_range {
//             let result_bytes = bytes[a as usize..b as usize].to_vec();
//             unsafe { std::string::String::from_utf8_unchecked(result_bytes) }
//         } else {
//             String::new()
//         }
//     }
// }

// Remove alphabet abstraction
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        const ALPHABET_SIZE: usize = 58;
        let target_bytes = t.into_bytes();
        let target_len = target_bytes.len() as u32;
        let bytes = s.into_bytes();
        let l = bytes.len() as u32;
        if target_len > l {
            // The target is longer than the string, bail.
            return String::new();
        }
        let target = target_bytes
            .into_iter()
            .fold([0u32; ALPHABET_SIZE], |mut acc, byte| {
                acc[(byte - b'A') as usize] += 1;
                acc
            });
        let mut current_start: u32 = 0;
        let mut current_end: u32 = 0;
        let mut current_alphabet = [0u32; ALPHABET_SIZE];
        let mut best_range: Option<(u32, u32)> = None;
        loop {
            if std::iter::zip(current_alphabet.iter(), target.iter())
                .all(|(mycount, tgtcount)| mycount >= tgtcount)
            {
                // The current range is a possible valid range
                // Check if it's smaller than the best
                best_range = Some(match best_range {
                    None => (current_start, current_end),
                    Some((a, b)) => {
                        let current_len = current_end - current_start;
                        if b - a > current_len {
                            (current_start, current_end)
                        } else {
                            (a, b)
                        }
                    }
                });
                // if current_len == target_len {
                //     break; // We can't get any smaller
                // }
                // Now that we've adjusted our best range,
                // delete a character from the start and
                // try again.
                current_alphabet[(bytes[current_start as usize] - b'A') as usize] -= 1;
                current_start += 1
            } else {
                if current_end < l {
                    // We haven't reached the end, but our substring isn't
                    // valid yet. Add a character to the end and try again.
                    current_alphabet[(bytes[current_end as usize] - b'A') as usize] += 1;
                    current_end += 1;
                } else {
                    // We reached the end
                    break;
                }
            }
        }
        if let Some((a, b)) = best_range {
            let result_bytes = bytes[a as usize..b as usize].to_vec();
            unsafe { std::string::String::from_utf8_unchecked(result_bytes) }
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a");
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
    }

    #[test]
    fn discussion_case1() {
        // Test 192
        assert_eq!(
            Solution::min_window("aaaaaaaaaaaabbbbbcdd".to_string(), "abcdd".to_string()),
            "abbbbbcdd",
        );
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(
            Solution::min_window(
                "caccaacaaaabbcaccaccc".to_string(),
                "acccacbccc".to_string()
            ),
            "caaaabbcaccaccc"
        )
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(
            Solution::min_window("aaaaaaaaaaaabbbbbcdd".to_string(), "abcdd".to_string()),
            "abbbbbcdd"
        )
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(
            Solution::min_window(
                "cbbbacccccbbbacbabbabacbabbbabaacbaccccbcbcbcca".to_string(),
                "abcbcabaacccababacbabcacbc".to_string()
            ),
            "acbabbabacbabbbabaacbaccccbcbc"
        )
    }

    #[test]
    fn discussion_case5() {
        assert_eq!(
            Solution::min_window("babb".to_string(), "baba".to_string()),
            ""
        )
    }

    #[test]
    fn discussion_case6() {
        assert_eq!(
            Solution::min_window("cabwefgewcwaefgcf".to_string(), "cae".to_string()),
            "cwae"
        )
    }

    #[test]
    fn discussion_case7() {
        assert_eq!(
            Solution::min_window("wegdtzwabazduwwdysdetrrctotpcepalxdewzezbfewbabbseinxbqqplitpxtcwwhuyntbtzxwzyaufihclztckdwccpeyonumbpnuonsnnsjscrvpsqsftohvfnvtbphcgxyumqjzltspmphefzjypsvugqqjhzlnylhkdqmolggxvneaopadivzqnpzurmhpxqcaiqruwztroxtcnvhxqgndyozpcigzykbiaucyvwrjvknifufxducbkbsmlanllpunlyohwfsssiazeixhebipfcdqdrcqiwftutcrbxjthlulvttcvdtaiwqlnsdvqkrngvghupcbcwnaqiclnvnvtfihylcqwvderjllannflchdklqxidvbjdijrnbpkftbqgpttcagghkqucpcgmfrqqajdbynitrbzgwukyaqhmibpzfxmkoeaqnftnvegohfudbgbbyiqglhhqevcszdkokdbhjjvqqrvrxyvvgldtuljygmsircydhalrlgjeyfvxdstmfyhzjrxsfpcytabdcmwqvhuvmpssingpmnpvgmpletjzunewbamwiirwymqizwxlmojsbaehupiocnmenbcxjwujimthjtvvhenkettylcoppdveeycpuybekulvpgqzmgjrbdrmficwlxarxegrejvrejmvrfuenexojqdqyfmjeoacvjvzsrqycfuvmozzuypfpsvnzjxeazgvibubunzyuvugmvhguyojrlysvxwxxesfioiebidxdzfpumyon".to_string(),"ozgzyywxvtublcl".to_string()),
            "tcnvhxqgndyozpcigzykbiaucyvwrjvknifufxducbkbsmlanl"
        )
    }

    #[test]
    fn myex1() {
        // Case sensitivity
        assert_eq!(Solution::min_window("A".to_string(), "a".to_string()), "");
    }

    #[test]
    fn myex2() {
        // Case range
        assert_eq!(
            Solution::min_window("AzZa".to_string(), "a".to_string()),
            "a"
        );
    }
}
