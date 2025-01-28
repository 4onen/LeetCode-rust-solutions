// https://leetcode.com/problems/find-the-difference/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_the_difference(s: String, t: String) -> char {
//         let mut a = [0u16; 26];
//         for &b in s.as_bytes() {
//             a[(b - b'a') as usize] += 1;
//         }
//         for &b in t.as_bytes() {
//             if a[(b - b'a') as usize] > 0 {
//                 a[(b - b'a') as usize] -= 1;
//             } else {
//                 return b as char;
//             }
//         }
//         unreachable!();
//     }
// }

// XOR sol'n
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut a = 0u8;
        for &b in s.as_bytes() {
            a ^= b;
        }
        for &b in t.as_bytes() {
            a ^= b;
        }
        a as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, t: &str, expected: u8) {
        assert!(s.len() <= 1000);
        assert_eq!(t.len(), s.len() + 1);
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        for &b in t.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert_eq!(
            Solution::find_the_difference(s.to_owned(), t.to_owned()) as u8,
            expected
        );
    }

    #[test]
    fn ex1() {
        test("abcd", "abcde", b'e')
    }

    #[test]
    fn ex2() {
        test("", "y", b'y')
    }

    #[test]
    fn discussion_case1() {
        test(
            "ymbgaraibkfmvocpizdydugvalagaivdbfsfbepeyccqfepzvtpyxtbadkhmwmoswrcxnargtlswqemafandgkmydtimuzvjwxvlfwlhvkrgcsithaqlcvrihrwqkpjdhgfgreqoxzfvhjzojhghfwbvpfzectwwhexthbsndovxejsntmjihchaotbgcysfdaojkjldprwyrnischrgmtvjcorypvopfmegizfkvudubnejzfqffvgdoxohuinkyygbdzmshvyqyhsozwvlhevfepdvafgkqpkmcsikfyxczcovrmwqxxbnhfzcjjcpgzjjfateajnnvlbwhyppdleahgaypxidkpwmfqwqyofwdqgxhjaxvyrzupfwesmxbjszolgwqvfiozofncbohduqgiswuiyddmwlwubetyaummenkdfptjczxemryuotrrymrfdxtrebpbjtpnuhsbnovhectpjhfhahbqrfbyxggobsweefcwxpqsspyssrmdhuelkkvyjxswjwofngpwfxvknkjviiavorwyfzlnktmfwxkvwkrwdcxjfzikdyswsuxegmhtnxjraqrdchaauazfhtklxsksbhwgjphgbasfnlwqwukprgvihntsyymdrfovaszjywuqygpvjtvlsvvqbvzsmgweiayhlubnbsitvfxawhfmfiatxvqrcwjshvovxknnxnyyfexqycrlyksderlqarqhkxyaqwlwoqcribumrqjtelhwdvaiysgjlvksrfvjlcaiwrirtkkxbwgicyhvakxgdjwnwmubkiazdjkfmotglclqndqjxethoutvjchjbkoasnnfbgrnycucfpeovruguzumgmgddqwjgdvaujhyqsqtoexmnfuluaqbxoofvotvfoiexbnprrxptchmlctzgqtkivsilwgwgvpidpvasurraqfkcmxhdapjrlrnkbklwkrvoaziznlpor",
            "qhxepbshlrhoecdaodgpousbzfcqjxulatciapuftffahhlmxbufgjuxstfjvljybfxnenlacmjqoymvamphpxnolwijwcecgwbcjhgdybfffwoygikvoecdggplfohemfypxfsvdrseyhmvkoovxhdvoavsqqbrsqrkqhbtmgwaurgisloqjixfwfvwtszcxwktkwesaxsmhsvlitegrlzkvfqoiiwxbzskzoewbkxtphapavbyvhzvgrrfriddnsrftfowhdanvhjvurhljmpxvpddxmzfgwwpkjrfgqptrmumoemhfpojnxzwlrxkcafvbhlwrapubhveattfifsmiounhqusvhywnxhwrgamgnesxmzliyzisqrwvkiyderyotxhwspqrrkeczjysfujvovsfcfouykcqyjoobfdgnlswfzjmyucaxuaslzwfnetekymrwbvponiaojdqnbmboldvvitamntwnyaeppjaohwkrisrlrgwcjqqgxeqerjrbapfzurcwxhcwzugcgnirkkrxdthtbmdqgvqxilllrsbwjhwqszrjtzyetwubdrlyakzxcveufvhqugyawvkivwonvmrgnchkzdysngqdibhkyboyftxcvvjoggecjsajbuqkjjxfvynrjsnvtfvgpgveycxidhhfauvjovmnbqgoxsafknluyimkczykwdgvqwlvvgdmufxdypwnajkncoynqticfetcdafvtqszuwfmrdggifokwmkgzuxnhncmnsstffqpqbplypapctctfhqpihavligbrutxmmygiyaklqtakdidvnvrjfteazeqmbgklrgrorudayokxptswwkcircwuhcavhdparjfkjypkyxhbgwxbkvpvrtzjaetahmxevmkhdfyidhrdeejapfbafwmdqjqszwnwzgclitdhlnkaiyldwkwwzvhyorgbysyjbxsspnjdewjxbhpsvj",
            b't',
        )
    }

    #[test]
    fn discussion_case2() {
        test("a", "aa", b'a')
    }

    #[test]
    fn discussion_case3() {
        test("aabbcd", "aabbcda", b'a')
    }
}
