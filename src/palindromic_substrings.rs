// https://leetcode.com/problems/palindromic-substrings/

pub struct Solution;

// Odd- and Even-centered palindromes
// impl Solution {
//     fn odd_palindromes_centered_at(s: &[u8], i: usize) -> u32 {
//         let n = s.len();
//         let mut count = 1;
//         let mut l = i;
//         let mut r = i;
//         while l > 0 && r < n - 1 && s[l - 1] == s[r + 1] {
//             l -= 1;
//             r += 1;
//             count += 1;
//         }
//         count
//     }
//     fn even_palindromes_centered_at(s: &[u8], i: usize) -> u32 {
//         let nm1 = s.len() - 1;
//         let mut count = 0;
//         let mut l = i + 1;
//         let mut r = i;
//         while l > 0 && r < nm1 && s[l - 1] == s[r + 1] {
//             l -= 1;
//             r += 1;
//             count += 1;
//         }
//         count
//     }
//     pub fn count_substrings(s: String) -> i32 {
//         let s = s.into_bytes();
//         let n = s.len();
//         let mut count = 0;
//         for i in 0..n {
//             count += Self::odd_palindromes_centered_at(&s, i);
//             count += Self::even_palindromes_centered_at(&s, i);
//         }
//         count as i32
//     }
// }

// One-helper optimization
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        const fn palindromes_from(s: &[u8], mut l: usize, mut r: usize) -> u32 {
            let n = s.len();
            let mut count = 0;
            while l > 0 && r < n - 1 && s[l - 1] == s[r + 1] {
                l -= 1;
                r += 1;
                count += 1;
            }
            count
        }
        let s = s.into_bytes();
        let n = s.len();
        let mut count = n as u32;
        for i in 0..n.saturating_sub(1) {
            count += palindromes_from(&s, i, i);
            count += palindromes_from(&s, i + 1, i);
        }
        count as i32
    }
}

// Lambda overoptimization (slower)
// impl Solution {
//     pub fn count_substrings(s: String) -> i32 {
//         let s = s.into_bytes();
//         let n = s.len();
//         let mut count = n as u32;
//         let nm1 = n.saturating_sub(1);
//         let palindromes_from = |mut l: usize, mut r: usize| -> u32 {
//             let mut count = 0;
//             while l > 0 && r < nm1 && s[l - 1] == s[r + 1] {
//                 l -= 1;
//                 r += 1;
//                 count += 1;
//             }
//             count
//         };
//         for i in 0..nm1 {
//             count += palindromes_from(i, i);
//             count += palindromes_from(i + 1, i);
//         }
//         count as i32
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    }

    #[test]
    fn discussion_case1() {
        // Need to narrow this to debug it.
        assert_eq!(Solution::count_substrings("n".to_string()), 1);
        assert_eq!(Solution::count_substrings("in".to_string()), 2);
        assert_eq!(Solution::count_substrings("ein".to_string()), 3);
        assert_eq!(Solution::count_substrings("nein".to_string()), 4);
        assert_eq!(Solution::count_substrings("onein".to_string()), 5);
        assert_eq!(Solution::count_substrings("wonein".to_string()), 6);
        assert_eq!(Solution::count_substrings("bwonein".to_string()), 7);
        assert_eq!(Solution::count_substrings("cbwonein".to_string()), 8);
        assert_eq!(Solution::count_substrings("ncbwonein".to_string()), 9);
        assert_eq!(Solution::count_substrings("nncbwonein".to_string()), 11);
        assert_eq!(Solution::count_substrings("d".to_string()), 1);
        assert_eq!(Solution::count_substrings("dn".to_string()), 2);
        assert_eq!(Solution::count_substrings("dnn".to_string()), 4);
        assert_eq!(Solution::count_substrings("dnnc".to_string()), 5);
        assert_eq!(Solution::count_substrings("dnncb".to_string()), 6);
        assert_eq!(Solution::count_substrings("dnncbw".to_string()), 7);
        assert_eq!(Solution::count_substrings("dnncbwo".to_string()), 8);
        assert_eq!(Solution::count_substrings("dnncbwon".to_string()), 9);
        assert_eq!(Solution::count_substrings("dnncbwone".to_string()), 10);
        assert_eq!(Solution::count_substrings("dnncbwonei".to_string()), 11);
        assert_eq!(Solution::count_substrings("dnncbwonein".to_string()), 12);
        assert_eq!(Solution::count_substrings("dnncbwoneino".to_string()), 13);
        assert_eq!(Solution::count_substrings("dnncbwoneinop".to_string()), 14);
        assert_eq!(Solution::count_substrings("dnncbwoneinopl".to_string()), 15);
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoply".to_string()),
            16
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplyp".to_string()),
            17
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypw".to_string()),
            18
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwg".to_string()),
            19
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgb".to_string()),
            20
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbw".to_string()),
            21
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwk".to_string()),
            22
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwkt".to_string()),
            23
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktm".to_string()),
            24
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmv".to_string()),
            25
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvk".to_string()),
            26
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvko".to_string()),
            27
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoi".to_string()),
            28
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoim".to_string()),
            29
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimc".to_string()),
            30
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimco".to_string()),
            31
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcoo".to_string()),
            33
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooy".to_string()),
            34
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyi".to_string()),
            35
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiw".to_string()),
            36
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwi".to_string()),
            38
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwir".to_string()),
            39
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwirg".to_string()),
            40
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwirgb".to_string()),
            41
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwirgbx".to_string()),
            42
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxl".to_string()),
            43
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlc".to_string()),
            44
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlct".to_string()),
            45
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlctt".to_string()),
            47
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttg".to_string()),
            48
        );
        assert_eq!(
            Solution::count_substrings("dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgt".to_string()),
            50
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgte".to_string()
            ),
            51
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteq".to_string()
            ),
            52
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqt".to_string()
            ),
            53
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqth".to_string()
            ),
            54
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthc".to_string()
            ),
            55
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcv".to_string()
            ),
            56
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvy".to_string()
            ),
            57
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyo".to_string()
            ),
            58
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyou".to_string()
            ),
            59
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoue".to_string()
            ),
            60
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyouey".to_string()
            ),
            61
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyf".to_string()
            ),
            62
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyft".to_string()
            ),
            63
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyfti".to_string()
            ),
            64
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyftiw".to_string()
            ),
            65
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyftiwg".to_string()
            ),
            66
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyftiwgw".to_string()
            ),
            68
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyftiwgww".to_string()
            ),
            70
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyftiwgwwx".to_string()
            ),
            71
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyftiwgwwxv".to_string()
            ),
            72
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyftiwgwwxvx".to_string()
            ),
            74
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyftiwgwwxvxv".to_string()
            ),
            76
        );
        assert_eq!(
            Solution::count_substrings(
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyftiwgwwxvxvg".to_string()
            ),
            77
        );
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(Solution::count_substrings("aaaaa".to_string()), 15);
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(
            Solution::count_substrings(
                "tlthpowwythupxaszmxhqbfbxegiqzdxzesppfjgycyprjyscubntihrfwbeebqgeclzdccxwxezasfzclndmnfjjqoplbxuygtopqtnpatixyydboldybmdoyfwkevwgxmsrdkwjiyoksilsorcbotqitujdaavjbvrjjwtnimpldrnkfuftxnhzfiwzkhslzolbfmdkqhulpjxmbmzqhawiztcbbaggcccttokwkznsctemmdgutldvpybalridbjmupbjijmexzjuvdfntqxuvdoijbbmbpwhbtdbxlhrvbxrfcoyjbrfsowdamohdsoojivwgoopfjwzdjhlzelkdxpsrkfdkjktptahoeanuuuujdybotiitmttzpnbyrxtjetxhydhlvlsmveddtvaobbvdkwxzoyugojhoapbaghgcanazkpauaorgjluszsezbnaqjxtvoxfitnpwsmywlxdimemymuyehyabrpedfkgrwtgyjvzkvahcbekqsofcvseswvtdixaxrjwjinvrruoskqlxcnxrvaqvsnpxdwjpjaupdyfaaxqsnrcrmkmzmtnndniqxglucqwargfzzqwxvaopxwafbzuifptayzoedznsljslpaoytiqnnlxeegbebslvbbsfoqlbokxakkaxdqyttxkdermidtoxjnjwibnlrsuvdkfcvoeagzpsogmcoeihbvyvjcdirnbbpqhdgoirclqapqzsvuesezbhdjoumxwhtwwnxnwyrnyhpaeqzbirnqxsufritrjkgbftmnjwpoakrzokpopwmwjsimwkvblwplsammgwonxrdkfbongodjnvadspxuvcyxlwvwhonvagznjsslbfayoxpqwrizxdhwgskewymhdlurtbekqsmghgzufkmsvrchskdoudtllfflromzwwahigprsrydcsyionczumedayyvldefctkuzafmwsvbifaxzyqywhzpqbeun".to_string()
            ),
            1089
        );
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(
            Solution::count_substrings(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()
            ),
            500500
        );
    }

    #[test]
    fn myex0() {
        assert_eq!(Solution::count_substrings("".to_string()), 0);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::count_substrings("a".to_string()), 1);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::count_substrings("ab".to_string()), 2);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::count_substrings("aba".to_string()), 4);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::count_substrings("abba".to_string()), 6);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::count_substrings("abcba".to_string()), 7);
    }

    #[test]
    fn myex6() {
        assert_eq!(Solution::count_substrings("abccba".to_string()), 9);
    }

    #[test]
    fn myex7() {
        assert_eq!(Solution::count_substrings("abccbaa".to_string()), 11);
    }

    #[test]
    fn myex8() {
        assert_eq!(Solution::count_substrings("abccbaaa".to_string()), 14);
    }

    #[test]
    fn myex9() {
        assert_eq!(Solution::count_substrings("abcbaaa".to_string()), 12);
    }

    #[test]
    fn myex10() {
        assert_eq!(Solution::count_substrings("abcbaaaa".to_string()), 16);
    }
}
