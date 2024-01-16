// https://leetcode.com/problems/string-compression-ii/

pub struct Solution;

// Greedy sol'n (Misses 3 test cases)
// impl Solution {
//     pub fn get_length_of_optimal_compression(s: String, mut k: i32) -> i32 {
//         type RunLengthEncoding = (u8, u32);
//         let mut rle: Vec<RunLengthEncoding> = Vec::with_capacity(s.len());
//         for c in s.bytes() {
//             if rle.is_empty() || rle.last().unwrap().0 != c {
//                 rle.push((c, 1));
//             } else {
//                 rle.last_mut().unwrap().1 += 1;
//             }
//         }
//         // Now we have to reduce the length of runs to reduce the cost.
//         // We do this by greedily deleting from the run with the lowest
//         // number of deletions needed to reduce the cost.
//         fn deletions_to_reduce_cost(rle: &RunLengthEncoding) -> u32 {
//             if rle.1 < 2 {
//                 1
//             } else if rle.1 < 10 {
//                 rle.1 - 1
//             } else if rle.1 < 100 {
//                 rle.1 - 9
//             } else {
//                 rle.1 - 99
//             }
//         }
//         while k > 0 {
//             let mut min_cost = u32::MAX;
//             let mut min_cost_idx = 0;
//             for (i, r) in rle.iter().enumerate() {
//                 let cost = deletions_to_reduce_cost(r);
//                 if cost <= min_cost {
//                     min_cost = cost;
//                     min_cost_idx = i;
//                 }
//             }
//             if min_cost > k as u32 {
//                 break;
//             }
//             k -= min_cost as i32;
//             rle[min_cost_idx].1 -= min_cost;
//             if rle[min_cost_idx].1 == 0 {
//                 rle.remove(min_cost_idx);
//             }
//         }
//         fn char_cost(rle: &RunLengthEncoding) -> u32 {
//             if rle.1 == 1 {
//                 1
//             } else if rle.1 < 10 {
//                 2
//             } else if rle.1 < 100 {
//                 3
//             } else {
//                 4
//             }
//         }
//         rle.iter().map(char_cost).sum::<u32>() as i32
//     }
// }

// Recursive sol'n
impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        fn is_length_incrementing_char(prev_cnt: u32) -> bool {
            prev_cnt == 1 || prev_cnt == 9 || prev_cnt == 99
        }

        type CacheId = (usize, i32, u8, u32);

        fn recurse(
            s: &[u8],
            k: i32,
            prev: u8,
            prev_cnt: u32,
            memo: &mut std::collections::HashMap<CacheId, i32>,
        ) -> i32 {
            if s.is_empty() {
                return 0;
            }

            let cache_id = (s.len(), k, prev, prev_cnt);
            if let Some(&result) = memo.get(&cache_id) {
                return result;
            }

            let result = if s[0] == prev {
                recurse(&s[1..], k, prev, prev_cnt + 1, memo)
                    + if is_length_incrementing_char(prev_cnt) {
                        1
                    } else {
                        0
                    }
            } else {
                if k == 0 {
                    1 + recurse(&s[1..], k, s[0], 1, memo)
                } else {
                    std::cmp::min(
                        recurse(&s[1..], k - 1, prev, prev_cnt, memo),
                        1 + recurse(&s[1..], k, s[0], 1, memo),
                    )
                }
            };

            memo.insert(cache_id, result);
            result
        }

        let mut memo = std::collections::HashMap::new();
        let s = s.as_bytes();
        recurse(s, k, 0, 0, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaabcccd".to_string(), 2),
            4
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aabbaa".to_string(), 2),
            2
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaaaaaaaaaa".to_string(), 0),
            3
        );
    }

    #[test]
    fn pastex1() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aabbcc".to_string(), 0),
            6
        );
    }

    #[test]
    fn pastex2() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("a".to_string(), 0),
            1
        );
    }

    #[test]
    fn pastex3() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("abbbbbbbbbbbb".to_string(), 0),
            4
        );
    }

    #[test]
    fn pastex4() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaabbaa".to_string(), 0),
            6
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("a".to_string(), 1),
            0
        );
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("a".to_string(), 2),
            0
        );
    }

    #[test]
    fn myex3() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaaaaaaaaa".to_string(), 0),
            3
        );
    }

    #[test]
    fn myex4() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaaaaaaaaa".to_string(), 1),
            2
        );
    }

    #[test]
    fn myex5() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("abbbbbbbbbbbb".to_string(), 1),
            3
        );
    }

    #[test]
    fn myex6() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("abbbbbbbbbbbb".to_string(), 2),
            3
        );
    }

    #[test]
    fn myex7() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("abbbbbbbbbbbb".to_string(), 3),
            3
        );
    }

    #[test]
    fn myex8() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("abbbbbbbbbbbb".to_string(), 4),
            2
        );
    }

    #[test]
    fn myex9() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aaabcccdccc".to_string(), 1),
            5
        );
    }

    #[test]
    fn myex10() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aa".to_string(), 2),
            0
        );
    }

    #[test]
    fn myex11() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aa".to_string(), 3),
            0
        );
    }

    #[test]
    fn myex12() {
        assert_eq!(
            Solution::get_length_of_optimal_compression("aabdaa".to_string(), 2),
            2
        );
    }

    #[test]
    fn myex13() {
        assert_eq!(
            Solution::get_length_of_optimal_compression(
                "a".chars().cycle().take(100).collect::<String>(),
                1
            ),
            3
        )
    }

    #[test]
    fn tle_case1() {
        assert_eq!(
            Solution::get_length_of_optimal_compression(
                "jejjflbakmfdbkadfjdmhlfbeiidib".to_string(),
                26
            ),
            2
        );
    }

    #[test]
    fn myex14() {
        assert_eq!(
            Solution::get_length_of_optimal_compression(
                "ababababababababababababababababab".to_string(),
                16,
            ),
            4,
        );
    }
}
