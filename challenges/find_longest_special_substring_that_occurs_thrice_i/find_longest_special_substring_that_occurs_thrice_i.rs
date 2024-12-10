// https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-i/

pub struct Solution;

// Initial hashmap sol'n (Broken for myex1)
// impl Solution {
//     pub fn maximum_length(s: String) -> i32 {
//         let s = s.as_bytes();
//         let mut maps = std::collections::HashMap::new();
//         {
//             let mut cnt = 0i8;
//             let mut last = s[0];
//             for i in 0..s.len() {
//                 if s[i] == last {
//                     cnt += 1;
//                 } else {
//                     maps.entry((last, cnt)).and_modify(|e| *e += 1).or_insert(1);
//                     last = s[i];
//                     cnt = 1;
//                 }
//             }
//             maps.entry((last, cnt)).and_modify(|e| *e += 1).or_insert(1);
//         }
//         let mut max = -1;
//         for ((_, n), v) in maps.into_iter() {
//             if v >= 3 {
//                 if n > max {
//                     max = n;
//                 }
//             } else {
//                 if n - 2 > max {
//                     max = n - 2;
//                 }
//             }
//         }
//         max as i32
//     }
// }

// Initial working sol'n
impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut maps = std::collections::HashMap::new();
        {
            let mut cnt = 0i8;
            let mut last = s[0];
            for i in 0..s.len() {
                cnt += 1;
                if s[i] != last {
                    last = s[i];
                    cnt = 1;
                }
                for j in 1..=cnt {
                    maps.entry((last, j)).and_modify(|e| *e += 1).or_insert(1);
                }
            }
        }
        let mut max = -1;
        for ((_, n), v) in maps.into_iter() {
            if v >= 3 {
                if n > max {
                    max = n;
                }
            } else {
                if n > 2 && n - 2 > max {
                    max = n - 2;
                }
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: i32) {
        assert!(s.len() >= 3);
        assert!(s.len() <= 50);
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert_eq!(Solution::maximum_length(s.to_string()), expected);
    }

    #[test]
    fn ex1() {
        test("aaaa", 2)
    }

    #[test]
    fn ex2() {
        test("abcdef", -1)
    }

    #[test]
    fn ex3() {
        test("abcaba", 1)
    }

    #[test]
    fn discussion_case1() {
        test("eccdnmcnkl", 1)
    }

    #[test]
    fn discussion_case2() {
        test("aac", -1)
    }

    #[test]
    fn discussion_case3() {
        test("aaa", 1)
    }

    #[test]
    fn discussion_case4() {
        test("abcccccddddabcccccddddabcccccdddd", 5)
    }

    #[test]
    fn discussion_case5() {
        test("jinhhhtttttttefffffjjjjjjjjjfffffjjjjjjjjjzvvvvvvg", 8)
    }

    #[test]
    fn discussion_case6() {
        test("aaaaaaaaaaaaccccccccccccccccccccccccccaaaaaaaaaaaa", 24)
    }

    #[test]
    fn discussion_case7() {
        test("aaaaaaaaaaaaaaaaaaaabbbbbbbbbbaaaaaaaaaaaaaaaaaaaa", 19)
    }

    #[test]
    fn discussion_case8() {
        test("zzzzzzzzzzzzzzzzzfffffdddddddddiiiiiiiiiiiiiiiiiii", 17)
    }

    #[test]
    fn discussion_case9() {
        test("zzzzzzzzzzzsssssssssssssssssqppppppppppppppnqmosat", 15)
    }

    #[test]
    fn myex1() {
        test("aaabaabaa", 2)
    }

    #[test]
    fn myex50() {
        test(unsafe { std::str::from_utf8_unchecked(&[b'a'; 50]) }, 48)
    }
}
