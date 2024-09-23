// https://leetcode.com/problems/extra-characters-in-a-string/

pub struct Solution;

// Initial sol'n w/ slice compare
// impl Solution {
//     pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
//         let s = s.as_bytes();
//         let mut dp = vec![0; s.len() + 1];
//         for i in 1..=s.len() {
//             dp[i] = dp[i - 1] + 1;
//             for word in &dictionary {
//                 let wb = word.as_bytes();
//                 if i >= wb.len() && &s[i - word.len()..i] == wb {
//                     dp[i] = std::cmp::min(dp[i], dp[i - word.len()]);
//                 }
//             }
//         }
//         dp[s.len()] as i32
//     }
// }

// Initial sol'n w/ byte iter compare
impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![0; s.len() + 1];
        for i in 1..=s.len() {
            dp[i] = dp[i - 1] + 1;
            for word in &dictionary {
                if i >= word.len() {
                    let mut j = 0;
                    while j < word.len()
                        && j < i
                        && s[i - j - 1] == word.as_bytes()[word.len() - j - 1]
                    {
                        j += 1;
                    }
                    if j == word.len() {
                        dp[i] = std::cmp::min(dp[i], dp[i - word.len()]);
                    }
                }
            }
        }
        dp[s.len()] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, dictionary: &[&str], expected: i32) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 50);
        assert!(dictionary.len() >= 1);
        assert!(dictionary.len() <= 50);
        let mut seen_set = std::collections::HashSet::new();
        for word in dictionary {
            assert!(word.len() >= 1);
            assert!(word.len() <= 50);
            for b in word.as_bytes() {
                assert!(*b >= b'a' && *b <= b'z');
            }
            assert!(seen_set.insert(word));
        }
        // If "expected" is less than zero, this is just a "doesn't crash" test
        // and we don't need to check the result.
        let result = Solution::min_extra_char(
            s.to_string(),
            dictionary.iter().map(|&s| s.to_string()).collect(),
        );
        if expected >= 0 {
            assert_eq!(result, expected)
        }
    }

    #[test]
    fn ex1() {
        test("leetscode", &["leet", "code", "leetcode"], 1)
    }

    #[test]
    fn ex2() {
        test("sayhelloworld", &["hello", "world"], 3)
    }

    #[test]
    fn discussion_case1() {
        test(
            "dwmodizxvvbosxxw",
            &[
                "ox", "lb", "diz", "gu", "v", "ksv", "o", "nuq", "r", "txhe", "e", "wmo", "cehy",
                "tskz", "ds", "kzbu",
            ],
            7,
        )
    }

    #[test]
    fn discussion_case2() {
        // tc 1991
        test(
            "azvzulhlwxwobowijiyebeaskecvtjqwkmaqnvnaomaqnvf",
            &[
                "na", "i", "edd", "wobow", "kecv", "b", "n", "or", "jj", "zul", "vk", "yeb",
                "qnfac", "azv", "grtjba", "yswmjn", "xowio", "u", "xi", "pcmatm", "maqnv",
            ],
            15,
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            "kevlplxozaizdhxoimmraiakbak",
            &[
                "yv", "bmab", "hv", "bnsll", "mra", "jjqf", "g", "aiyzi", "ip", "pfctr", "flr",
                "ybbcl", "biu", "ke", "lpl", "iak", "pirua", "ilhqd", "zdhx", "fux", "xaw",
                "pdfvt", "xf", "t", "wq", "r", "cgmud", "aokas", "xv", "jf", "cyys", "wcaz",
                "rvegf", "ysg", "xo", "uwb", "lw", "okgk", "vbmi", "v", "mvo", "fxyx", "ad", "e",
            ],
            9,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            "rkmsilizktprllwoimafyuqmeqrujxdzgp",
            &[
                "afy", "lyso", "ymdt", "uqm", "cfybt", "lwoim", "hdzeg", "th", "rkmsi", "d", "e",
                "tp", "r", "jx", "tofxe", "etjx", "llqs", "cpir", "p", "ncz", "ofeyx", "eqru", "l",
                "demij", "tjky", "jgodm", "y", "ernt", "jfns", "akjtl", "wt", "tk", "zg", "lxoi",
                "kt",
            ],
            2,
        )
    }

    #[test]
    fn discussion_case5() {
        // tc 2020
        test(
            "aakodubkrlauvfkzje",
            &[
                "ix", "qoqw", "ax", "ar", "v", "hxpl", "nxcg", "thr", "kod", "pns", "cdo", "euy",
                "es", "rf", "bxcx", "xe", "ua", "vws", "vumr", "zren", "bzt", "qwxn", "ami",
                "rrbk", "ak", "uan", "g", "vfk", "jxmg", "fhb", "nqgd", "fau", "rl", "h", "r",
                "jxvo", "tv", "smfp", "lmck", "od",
            ],
            9,
        )
    }

    #[test]
    fn discussion_case6() {
        test("leetsleet", &["leet"], 1)
    }

    #[test]
    fn discussion_case7() {
        test(
            "zzyyxxwwvvuuttssrrqqppoonnmmllkkjjiihhggffeeddccbb",
            &[
                "ffeed", "kkjji", "ppoon", "uutts", "zzyyx", "ccb", "dcc", "ddc", "eed", "fee",
                "ffe", "ggf", "hgg", "hhg", "iih", "jii", "jji", "kkj", "lkk", "llk", "mml", "nmm",
                "nnm", "oon", "poo", "ppo", "qqp", "rqq", "rrq", "ssr", "tss", "tts", "uut", "vuu",
                "vvu", "wwv", "xww", "xxw", "yyx", "zyy", "zzy",
            ],
            8,
        )
    }

    #[test]
    fn discussion_case8() {
        test(
            "metzeaencgpgvsckjrqafkxgyzbe",
            &[
                "zdzz", "lgrhy", "r", "ohk", "zkowk", "g", "zqpn", "anoni", "ka", "qafkx", "t",
                "jr", "xdye", "mppc", "bqqb", "encgp", "yf", "vl", "ctsxk", "gn", "cujh", "ce",
                "rwrpq", "tze", "zxhg", "yzbe", "c", "o", "hnk", "gv", "uzbc", "xn", "kk", "ujjd",
                "vv", "mxhmv", "ugn", "at", "kumr", "ensv", "x", "uy", "gb", "ae", "jljuo",
                "xqkgj",
            ],
            5,
        )
    }

    #[test]
    fn discussion_case9() {
        test("leetsleetscode", &["leets", "code"], 0)
    }

    #[test]
    fn myex1() {
        test("leetscode", &["leet", "code", "zs"], 1)
    }
}
