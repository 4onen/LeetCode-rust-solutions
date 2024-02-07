// https://leetcode.com/problems/sort-characters-by-frequency/

pub struct Solution;

// Failed hash map solution
// impl Solution {
//     pub fn frequency_sort(s: String) -> String {
//         let mut counts: std::collections::HashMap<u8, i32> = std::collections::HashMap::new();
//         let mut s = s.into_bytes();
//         for &c in s.iter() {
//             *counts.entry(c).or_insert(0) += 1;
//         }
//         s.sort_unstable_by_key(|c| -(counts[&c]));
//         unsafe { String::from_utf8_unchecked(s) }
//     }
// }

// Counting sort solution
// impl Solution {
//     pub fn frequency_sort(s: String) -> String {
//         const ASCII_SIZE: u8 = b'z' - b'0' + 1;
//         let mut counts = [0i32; ASCII_SIZE as usize];
//         let mut s = s.into_bytes();
//         for &c in s.iter() {
//             counts[(c - b'0') as usize] += 1;
//         }
//         let mut chars = (b'0'..=b'z').collect::<Vec<u8>>();
//         chars.sort_unstable_by_key(|&c| -(counts[(c - b'0') as usize]));
//         let mut output_idx = 0;
//         for char in chars.into_iter() {
//             let count = counts[(char - b'0') as usize];
//             for _ in 0..count {
//                 s[output_idx] = char;
//                 output_idx += 1;
//             }
//         }
//         unsafe { String::from_utf8_unchecked(s) }
//     }
// }

// Counting sort sol'n v2 (stack me up baby)
// const ASCII_SIZE: u8 = b'z' - b'0' + 1;
// const fn make_empty_counts() -> [(i32, u8); ASCII_SIZE as usize] {
//     let mut initial = [(0, b'0'); ASCII_SIZE as usize];
//     let mut i = 0;
//     while i < ASCII_SIZE {
//         initial[i as usize].1 = b'0' + i;
//         i += 1;
//     }
//     initial
// }
// const INITIAL_COUNTS: [(i32, u8); ASCII_SIZE as usize] = make_empty_counts();
// impl Solution {
//     pub fn frequency_sort(s: String) -> String {
//         let mut counts = INITIAL_COUNTS;
//         let mut s = s.into_bytes();
//         for &c in s.iter() {
//             counts[(c - b'0') as usize].0 += 1;
//         }
//         let mut counts = counts.map(|(count, char)| std::cmp::Reverse((count, char)));
//         counts.sort_unstable();
//         let mut output_idx = 0;
//         // All entries are zero until the first non-zero entry, so find the
//         // first non-zero entry from the end of the array
//         for std::cmp::Reverse((count, char)) in counts {
//             for _ in 0..count {
//                 s[output_idx] = char;
//                 output_idx += 1;
//             }
//         }
//         unsafe { String::from_utf8_unchecked(s) }
//     }
// }

// Counting sort sol'n v3 (Struct of arrays is a great pattern) (0ms!)
// const ASCII_SIZE: u8 = b'z' - b'0' + 1;
// struct Counts {
//     pub counts: [u32; ASCII_SIZE as usize],
//     pub chars: [u8; ASCII_SIZE as usize],
// }
// impl Counts {
//     const fn new() -> Self {
//         let mut initial = [0; ASCII_SIZE as usize];
//         let mut i = 0;
//         while i < ASCII_SIZE {
//             initial[i as usize] = b'0' + i;
//             i += 1;
//         }
//         Self {
//             counts: [0; ASCII_SIZE as usize],
//             chars: initial,
//         }
//     }
//     fn fill_from_bytes(&mut self, bytes: &[u8]) {
//         for &c in bytes.iter() {
//             self.counts[(c - b'0') as usize] += 1;
//         }
//     }
//     fn sort(&mut self) {
//         let chars = &mut self.chars;
//         let counts = &self.counts;
//         chars.sort_unstable_by_key(|&c| std::cmp::Reverse(counts[(c - b'0') as usize]));
//     }
// }
// impl Solution {
//     pub fn frequency_sort(s: String) -> String {
//         let mut s = s.into_bytes();
//         let mut counts = Counts::new();
//         counts.fill_from_bytes(&s);
//         counts.sort();
//         let mut output_idx = 0;
//         for &char in counts.chars.iter() {
//             let count = counts.counts[(char - b'0') as usize];
//             for _ in 0..count {
//                 s[output_idx] = char;
//                 output_idx += 1;
//             }
//         }
//         unsafe { String::from_utf8_unchecked(s) }
//     }
// }

// Counting sort sol'n v4 (Minimizing memory use with a filter)
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        const ASCII_SIZE: u8 = b'z' - b'0' + 1;
        let mut s = s.into_bytes();
        let mut counts = [0u32; ASCII_SIZE as usize];
        for &c in s.iter() {
            counts[(c - b'0') as usize] += 1;
        }
        let mut counts = IntoIterator::into_iter(counts)
            .enumerate()
            .map(|(idx, count)| (count, idx as u8))
            .filter(|&(count, _)| count > 0)
            .collect::<Vec<(_, u8)>>();
        counts.sort_unstable_by_key(|&(count, _)| std::cmp::Reverse(count));
        let mut output_idx = 0;
        for (count, idx) in counts.into_iter() {
            let c = b'0' + idx;
            s[output_idx..output_idx + count as usize].fill(c);
            output_idx += count as usize;
        }
        unsafe { String::from_utf8_unchecked(s) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        const INPUT: &str = "tree";
        const EXPECTED: &[&str] = &["eert", "eetr"];
        let result = Solution::frequency_sort(INPUT.to_string());
        assert!(
            EXPECTED.contains(&result.as_str()),
            "\"{}\" was not accepted.",
            result
        );
    }

    #[test]
    fn ex2() {
        const INPUT: &str = "cccaaa";
        const EXPECTED: &[&str] = &["aaaccc", "cccaaa"];
        let result = Solution::frequency_sort(INPUT.to_string());
        assert!(EXPECTED.contains(&result.as_str()));
    }

    #[test]
    fn ex3() {
        const INPUT: &str = "Aabb";
        const EXPECTED: &[&str] = &["bbAa", "bbaA"];
        let result = Solution::frequency_sort(INPUT.to_string());
        assert!(EXPECTED.contains(&result.as_str()));
    }

    #[test]
    fn discussion_case1() {
        const INPUT: &str = "ApsY9AQF2xsz1pbHl5Po7Q1yKNOthVVY9wkGrvktI1dcbXirB5k82sK1lFrx2Wv3mc1viEfvt31TRor6HFCe4M40hztHX3pVSAoi";
        const EXPECTED: &[&str] = &["111111rrrrttttvvvvHHHAAAiiikkkooopppsss333222FFFVVVbbcchhKKll994455QQxxzzXXYY8OyNRIw7PTMCW0dfmSEGe6B", "111111vvvvttttrrrrkkk222333VVVHHHFFFAAAiiissspppoooll4499hhccbbYYxxXXzz55QQKKTI6yMNOPRS7W0Gw8mBCfedE","111111vvvvttttrrrrssspppoookkkiiiVVVHHHFFFAAA333222zzxxllhhccbbYYXXQQKK995544ywmfedWTSRPONMIGECB8760","111111vvvvrrrrttttiiipppVVVkkk222oooAAAHHHsss333FFF554499KKllxxXXhhQQccbbYYzzCW6TSdefRPONmMIGE0Bw8y7"];
        let result = Solution::frequency_sort(INPUT.to_string());
        assert!(EXPECTED.contains(&result.as_str()));
    }

    #[test]
    fn discussion_case2() {
        const INPUT: &str = "IkWBhGISvgpEEvw5XdEp25kwMtnf2ww7sbbEqoOx9BXz2pS2TE7YolAw42hioODDph5FfadcWyFMpln381SinvWe6Lzz5b8hkCvnIU38ph0T1iXPp2elYNunfqH1wH1luqfgZ51LQIGkOVT9JnNSgYnBzooJBcXKeet4NnjfuEIhQBl5C4M4pE4DJSlJ6YZilH7qvSjdnixjJDin6Xpk3xaMd2TRSFGAZo7HqSu0NmlKrzgEK3n5jfNkm0SGvaFNyUTAQVcEnLx4omizRJg0lB72y4yNh4GpBddD6SiP9yuPjo96U3QZJMFlAFNP4psWR3MoCKwmIlzyEWdcgDdXnoDBWYqpTMXGx90pDMpezvvQ99HKuUhJLWgGu7EEU49SnpebahRXtpuGchpBRnzfY6ZMMGkjhEIe5lFEiNNNagfhgMRf6I4udgP9y84YyLXWETO40xVOc5ntXqvzvko63FzugCWxzugJzKA2AmdCFNs1mZQSW8oJwWc58TnmaatX5P7ub98O9boTTncWPS7zd5xaVutjPhtg6VLew3Rq0UXOz1Pl2BeNLwJJ9SUuGewwVgRqpECIn0Y9VLq5cE5QKsVJxcRDiqtOP9Aan4u8UgypcTa0ngDw4ejJFDNofwybaOD53vFI99cCMnhFUkKV8DsqFtx8gLnw3L5uL60BWJ0aLPpW46LvUzPjbWjTRJHzWj7P6z4nNDYNoMg9OoZU2fkyELtz4Vq8R5BsIcC4lrC12AMjfPju1Oy4rmGqDcZZMNy5bG9Z8qvgIc6bhu6qfVB2P4i0XtHw9Fsr4pv8rY5WgXv8XYcL2Fj3cnNgGj2a9IoFHWQvt8SWxG33LOW7NU8P39Zoh13C84bm4JfyDG6pcByhPAEFURTfJn0aNiMyGo1zyD6G2K4uvpnOvhCJGB591HcnxQw34Jju0Ag1fEatI4lQWj8XewxpuDzzjMXaUkApHhRpmJXzepzH9e0rFb0uL6B7uhZbYAKCX7QX";
        const EXPECTED: &[&str] = &["nnnnnnnnnnnnnnnnnnnnnnnnnnnn444444444444444444444444444pppppppppppppppppppppppppppzzzzzzzzzzzzzzzzzzzzzzzz99999999999999999999999uuuuuuuuuuuuuuuuuuuuuuuggggggggggggggggggggggWWWWWWWWWWWWWWWWWWWWWJJJJJJJJJJJJJJJJJJJJJXXXXXXXXXXXXXXXXXXXXhhhhhhhhhhhhhhhhhhhhNNNNNNNNNNNNNNNNNNNN55555555555555555555EEEEEEEEEEEEEEEEEEEEcccccccccccccccccccoooooooooooooooooooGGGGGGGGGGGGGGGGGGFFFFFFFFFFFFFFFFFF888888888888888888jjjjjjjjjjjjjjjjjjvvvvvvvvvvvvvvvvvvwwwwwwwwwwwwwwwwwwDDDDDDDDDDDDDDDDDDLLLLLLLLLLLLLLLLLMMMMMMMMMMMMMMMMMPPPPPPPPPPPPPPPPP66666666666666666yyyyyyyyyyyyyyyyy0000000000000000aaaaaaaaaaaaaaaa3333333333333333ffffffffffffffffBBBBBBBBBBBBBBBB2222222222222222qqqqqqqqqqqqqqqqSSSSSSSSSSSSSSSlllllllllllllllIIIIIIIIIIIIIIeeeeeeeeeeeeeeUUUUUUUUUUUUUURRRRRRRRRRRRR1111111111111bbbbbbbbbbbbbtttttttttttttTTTTTTTTTTTTTOOOOOOOOOOOOOxxxxxxxxxxxxxCCCCCCCCCCCC777777777777YYYYYYYYYYYYiiiiiiiiiiiiAAAAAAAAAAAAZZZZZZZZZZZZQQQQQQQQQQQkkkkkkkkkkkdddddddddddVVVVVVVVVVVHHHHHHHHHHHmmmmmmmmmmKKKKKKKKKKsssssssrrrrrr", "nnnnnnnnnnnnnnnnnnnnnnnnnnnn444444444444444444444444444pppppppppppppppppppppppppppzzzzzzzzzzzzzzzzzzzzzzzz99999999999999999999999uuuuuuuuuuuuuuuuuuuuuuuggggggggggggggggggggggWWWWWWWWWWWWWWWWWWWWWJJJJJJJJJJJJJJJJJJJJJEEEEEEEEEEEEEEEEEEEEXXXXXXXXXXXXXXXXXXXXNNNNNNNNNNNNNNNNNNNNhhhhhhhhhhhhhhhhhhhh55555555555555555555oooooooooooooooooooccccccccccccccccccc888888888888888888wwwwwwwwwwwwwwwwwwvvvvvvvvvvvvvvvvvvjjjjjjjjjjjjjjjjjjDDDDDDDDDDDDDDDDDDFFFFFFFFFFFFFFFFFFGGGGGGGGGGGGGGGGGGyyyyyyyyyyyyyyyyyPPPPPPPPPPPPPPPPPMMMMMMMMMMMMMMMMM66666666666666666LLLLLLLLLLLLLLLLLBBBBBBBBBBBBBBBBffffffffffffffff22222222222222220000000000000000aaaaaaaaaaaaaaaaqqqqqqqqqqqqqqqq3333333333333333lllllllllllllllSSSSSSSSSSSSSSSUUUUUUUUUUUUUUeeeeeeeeeeeeeeIIIIIIIIIIIIIIxxxxxxxxxxxxx1111111111111tttttttttttttTTTTTTTTTTTTTOOOOOOOOOOOOObbbbbbbbbbbbbRRRRRRRRRRRRRZZZZZZZZZZZZ777777777777AAAAAAAAAAAACCCCCCCCCCCCiiiiiiiiiiiiYYYYYYYYYYYYQQQQQQQQQQQdddddddddddHHHHHHHHHHHkkkkkkkkkkkVVVVVVVVVVVKKKKKKKKKKmmmmmmmmmmsssssssrrrrrr","nnnnnnnnnnnnnnnnnnnnnnnnnnnnppppppppppppppppppppppppppp444444444444444444444444444zzzzzzzzzzzzzzzzzzzzzzzzuuuuuuuuuuuuuuuuuuuuuuu99999999999999999999999ggggggggggggggggggggggWWWWWWWWWWWWWWWWWWWWWJJJJJJJJJJJJJJJJJJJJJhhhhhhhhhhhhhhhhhhhhXXXXXXXXXXXXXXXXXXXXNNNNNNNNNNNNNNNNNNNNEEEEEEEEEEEEEEEEEEEE55555555555555555555ooooooooooooooooooocccccccccccccccccccwwwwwwwwwwwwwwwwwwvvvvvvvvvvvvvvvvvvjjjjjjjjjjjjjjjjjjGGGGGGGGGGGGGGGGGGFFFFFFFFFFFFFFFFFFDDDDDDDDDDDDDDDDDD888888888888888888yyyyyyyyyyyyyyyyyPPPPPPPPPPPPPPPPPMMMMMMMMMMMMMMMMMLLLLLLLLLLLLLLLLL66666666666666666qqqqqqqqqqqqqqqqffffffffffffffffaaaaaaaaaaaaaaaaBBBBBBBBBBBBBBBB333333333333333322222222222222220000000000000000lllllllllllllllSSSSSSSSSSSSSSSeeeeeeeeeeeeeeUUUUUUUUUUUUUUIIIIIIIIIIIIIIxxxxxxxxxxxxxtttttttttttttbbbbbbbbbbbbbTTTTTTTTTTTTTRRRRRRRRRRRRROOOOOOOOOOOOO1111111111111iiiiiiiiiiiiZZZZZZZZZZZZYYYYYYYYYYYYCCCCCCCCCCCCAAAAAAAAAAAA777777777777kkkkkkkkkkkdddddddddddVVVVVVVVVVVQQQQQQQQQQQHHHHHHHHHHHmmmmmmmmmmKKKKKKKKKKsssssssrrrrrr","nnnnnnnnnnnnnnnnnnnnnnnnnnnnppppppppppppppppppppppppppp444444444444444444444444444zzzzzzzzzzzzzzzzzzzzzzzz99999999999999999999999uuuuuuuuuuuuuuuuuuuuuuuggggggggggggggggggggggWWWWWWWWWWWWWWWWWWWWWJJJJJJJJJJJJJJJJJJJJJXXXXXXXXXXXXXXXXXXXXhhhhhhhhhhhhhhhhhhhh55555555555555555555NNNNNNNNNNNNNNNNNNNNEEEEEEEEEEEEEEEEEEEEooooooooooooooooooocccccccccccccccccccwwwwwwwwwwwwwwwwwwDDDDDDDDDDDDDDDDDDvvvvvvvvvvvvvvvvvvFFFFFFFFFFFFFFFFFFGGGGGGGGGGGGGGGGGGjjjjjjjjjjjjjjjjjj888888888888888888yyyyyyyyyyyyyyyyyMMMMMMMMMMMMMMMMMPPPPPPPPPPPPPPPPPLLLLLLLLLLLLLLLLL66666666666666666qqqqqqqqqqqqqqqq00000000000000002222222222222222aaaaaaaaaaaaaaaaffffffffffffffffBBBBBBBBBBBBBBBB3333333333333333SSSSSSSSSSSSSSSlllllllllllllllUUUUUUUUUUUUUUeeeeeeeeeeeeeeIIIIIIIIIIIIIITTTTTTTTTTTTTtttttttttttttbbbbbbbbbbbbbxxxxxxxxxxxxx1111111111111RRRRRRRRRRRRROOOOOOOOOOOOOYYYYYYYYYYYYiiiiiiiiiiii777777777777AAAAAAAAAAAAZZZZZZZZZZZZCCCCCCCCCCCCQQQQQQQQQQQHHHHHHHHHHHdddddddddddVVVVVVVVVVVkkkkkkkkkkkKKKKKKKKKKmmmmmmmmmmsssssssrrrrrr"];
        let result = Solution::frequency_sort(INPUT.to_string());
        assert!(EXPECTED.contains(&result.as_str()));
    }

    #[test]
    fn my_extrema_ex1() {
        let input = unsafe { String::from_utf8_unchecked(vec![b'a'; 100000]) };
        let result = Solution::frequency_sort(input.clone());
        assert_eq!(result, input);
    }
}
