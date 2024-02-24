// https://leetcode.com/problems/longest-palindromic-substring/

pub struct Solution;

// O(N^2) solution
// impl Solution {
//     fn is_palindrome(s: &str) -> bool {
//         let halfrange = s.len() / 2;
//         s.bytes()
//             .take(halfrange)
//             .eq(s.bytes().rev().take(halfrange))
//     }
//     pub fn longest_palindrome(s: String) -> String {
//         let mut longest = &s[0..1];
//         for i in 0..s.len() - 1 {
//             let mut j = i + longest.len() + 1;
//             while j <= s.len() {
//                 let sub = &s[i..j];
//                 if Self::is_palindrome(sub) {
//                     longest = sub;
//                 }
//                 j += 1;
//             }
//         }
//         longest.to_string()
//     }
// }

// O(N) solution using Manacher's algorithm
// https://en.wikipedia.org/wiki/Longest_palindromic_substring#Manacher's_algorithm
// impl Solution {
//     pub fn longest_palindrome(s: String) -> String {
//         let bytes = s.as_bytes();
//         let mut s_prime = vec![b'#'; 2 * s.len() + 1];
//         for i in 0..s.len() {
//             s_prime[2 * i + 1] = bytes[i];
//         }
//         let mut palindrome_radii: Vec<u16> = vec![0; s_prime.len()];
//         let mut center: u16 = 0;
//         let mut radius: u16 = 0;
//         while (center as usize) < s_prime.len() {
//             // Determine the longest palindrome starting at center-radius
//             // and going to center+radius
//             while radius + 1 <= center
//                 && ((center + radius + 1) as usize) < s_prime.len()
//                 && s_prime[(center - (radius + 1)) as usize]
//                     == s_prime[(center + (radius + 1)) as usize]
//             {
//                 radius += 1;
//             }
//             palindrome_radii[center as usize] = radius;
//             let old_center = center;
//             let old_radius = radius;
//             center += 1;
//             radius = 0; // Default if we reach the end of the following loop.
//             while center <= old_center + old_radius {
//                 let mirrored_center = old_center - (center - old_center);
//                 let max_mirrored_radius = (old_center + old_radius - center) as u16;
//                 let mirrored_radius = palindrome_radii[mirrored_center as usize];
//                 match mirrored_radius.cmp(&max_mirrored_radius) {
//                     std::cmp::Ordering::Less => {
//                         palindrome_radii[center as usize] = mirrored_radius;
//                         center += 1;
//                     }
//                     std::cmp::Ordering::Greater => {
//                         palindrome_radii[center as usize] = max_mirrored_radius;
//                         center += 1;
//                     }
//                     std::cmp::Ordering::Equal => {
//                         radius = max_mirrored_radius;
//                         break;
//                     }
//                 }
//             }
//         }
//         let (idx, max_palindrome_radius) = palindrome_radii
//             .into_iter()
//             .enumerate()
//             .max_by_key(|t| t.1)
//             .unwrap();
//         let start = (idx - max_palindrome_radius as usize) / 2;
//         let end = start + max_palindrome_radius as usize;
//         String::from_utf8(bytes[start..end].to_vec()).unwrap()
//     }
// }

// O(N) solution using Manacher's algorithm without instantiating the s_prime vector
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        struct BogusCharIndexer<'a> {
            s: &'a [u8],
        }

        impl BogusCharIndexer<'_> {
            fn len(&self) -> u16 {
                self.s.len() as u16 * 2 + 1
            }
        }

        impl<'a> std::ops::Index<usize> for BogusCharIndexer<'a> {
            type Output = u8;

            fn index(&self, index: usize) -> &Self::Output {
                if index & 1 == 0 {
                    &b'#' // Eh, probably safe enough for leetcode work.
                } else {
                    &self.s[index >> 1]
                }
            }
        }

        impl<'a> std::ops::Index<u16> for BogusCharIndexer<'a> {
            type Output = u8;

            fn index(&self, index: u16) -> &Self::Output {
                if index & 1 == 0 {
                    &b'#' // Eh, probably safe enough for leetcode work.
                } else {
                    &self.s[(index >> 1) as usize]
                }
            }
        }

        let bytes = s.as_bytes();
        let s_prime = BogusCharIndexer { s: bytes };

        let mut palindrome_radii: Vec<u16> = vec![0; s_prime.len() as usize];
        let mut center: u16 = 0;
        let mut radius: u16 = 0;
        while center < s_prime.len() {
            // Determine the longest palindrome starting at center-radius
            // and going to center+radius
            while radius + 1 <= center
                && (center + radius + 1) < s_prime.len()
                && s_prime[center - (radius + 1)] == s_prime[center + (radius + 1)]
            {
                radius += 1;
            }
            palindrome_radii[center as usize] = radius;
            let old_center = center;
            let old_radius = radius;
            center += 1;
            radius = 0; // Default if we reach the end of the following loop.
            while center <= old_center + old_radius {
                let mirrored_center = old_center - (center - old_center);
                let max_mirrored_radius = (old_center + old_radius - center) as u16;
                let mirrored_radius = palindrome_radii[mirrored_center as usize];
                match mirrored_radius.cmp(&max_mirrored_radius) {
                    std::cmp::Ordering::Less => {
                        palindrome_radii[center as usize] = mirrored_radius;
                        center += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        palindrome_radii[center as usize] = max_mirrored_radius;
                        center += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        radius = max_mirrored_radius;
                        break;
                    }
                }
            }
        }
        let (idx, max_palindrome_radius) = palindrome_radii
            .into_iter()
            .enumerate()
            .max_by_key(|t| t.1)
            .unwrap();
        let start = (idx - max_palindrome_radius as usize) / 2;
        let end = start + max_palindrome_radius as usize;
        String::from_utf8(bytes[start..end].to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let result = Solution::longest_palindrome(String::from("babad"));
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::longest_palindrome(String::from("cbbd")), "bb");
    }

    #[test]
    fn discussion_case1() {
        let input = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabcaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert_eq!(Solution::longest_palindrome(String::from(input)), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    }

    #[test]
    fn discussion_case2() {
        let input = "aacabdkacaa";
        assert_eq!(Solution::longest_palindrome(String::from(input)), "aca");
    }

    #[test]
    fn discussion_case3() {
        let input = "abcda";
        let result = Solution::longest_palindrome(String::from(input));
        assert!(result == "a" || result == "b" || result == "c" || result == "d");
    }

    #[test]
    fn discussion_case4() {
        let input = "aacdefcaa";
        assert_eq!(Solution::longest_palindrome(String::from(input)), "aa");
    }

    #[test]
    fn discussion_case5() {
        let input = "dddddddd";
        assert_eq!(Solution::longest_palindrome(String::from(input)), input);
    }

    #[test]
    fn discussion_case6() {
        let input = "nmngaowrbsssvihklwmuqshcddwlxrywrlwtennwfvrevgvhsvgeccfulmuvrcksdmgeqrblnlwoepefhcwhmgyvgcoyyygrmttyfycxwbqktpurlcfhzlakhmrddsydgygganpmaglaxyhfwjusukzcnakznygqplngnkhcowavxoiwrfycxwdkxqfcjqwyqutcpyedbnuogedwobsktgioqdczxhikjrbkmqspnxcpngfdwdaboscqbkwforihzqdcppxjksiujfvlpdjryewaxgmdgigvxdlstxwngtbdrrkfudjinzyxbdmkautclvvyguekuzwwetmsxittgtxbnvvrgasvnlogdiepltweaehubwelznidltzlbzdsrxmhjpkmylnwkdsxnpkplkdzywioluaqguowtbaoqzqgjfewphqcvlnwlojbxgomvxxkhwwykawegxubjiobizicuxzeafgautefsurgjlbhcfevqzsbhwxycrcaibdsgluczcuewzqupakbzmcvzsfodbmgtugnihyhqkvyeboqhqldifbxuaxqzxtyejoswikbzpsvzkxcndgeyvfnyrfbkhlalzpqjueibnodamgpnxlkvwvliouvejcpnakllfxepldfmdzszagkyhdgqqbkb";
        assert_eq!(Solution::longest_palindrome(String::from(input)), "uczcu");
    }

    #[test]
    fn myex1() {
        let input = "babadada";
        assert_eq!(Solution::longest_palindrome(String::from(input)), "adada");
    }

    #[test]
    fn myex2() {
        let input = "a";
        assert_eq!(Solution::longest_palindrome(String::from(input)), "a");
    }
}
