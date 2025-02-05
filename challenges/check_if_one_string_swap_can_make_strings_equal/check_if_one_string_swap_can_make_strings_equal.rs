// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/

pub struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        assert_eq!(s1.len(), s2.len());
        let mut out_of_place_idx: i8 = -1;
        for i in 0..s1.len() as i8 {
            let b1 = s1[i as usize];
            let b2 = s2[i as usize];
            if b1 != b2 {
                match out_of_place_idx {
                    -1 => {
                        out_of_place_idx = i;
                    }
                    j if j >= 0 => {
                        if s1[j as usize] != b2 || s2[j as usize] != b1 {
                            return false;
                        }
                        out_of_place_idx = -2;
                    }
                    _ => {
                        return false;
                    }
                }
            }
        }
        out_of_place_idx < 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s1: &str, s2: &str, expected: bool) {
        assert_eq!(s1.len(), s2.len());
        assert!(s1.len() >= 1);
        assert!(s1.len() <= 100);
        for &b in s1.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        for &b in s2.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert_eq!(
            Solution::are_almost_equal(s1.to_owned(), s2.to_owned()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("bank", "kanb", true)
    }

    #[test]
    fn ex2() {
        test("attack", "defend", false)
    }

    #[test]
    fn ex3() {
        test("kelb", "kelb", true)
    }

    #[test]
    fn myex0() {
        test("ab", "ba", true)
    }

    #[test]
    fn myex1() {
        test("ab", "ab", true)
    }

    #[test]
    fn myex2() {
        test("ab", "ak", false)
    }

    #[test]
    fn myex3() {
        test("ab", "ka", false)
    }

    #[test]
    fn myex4() {
        test("a", "b", false)
    }

    #[test]
    fn myex5() {
        test("a", "a", true)
    }

    #[test]
    fn discussion_case1() {
        test("abcd", "dcba", false)
    }

    #[test]
    fn discussion_case2() {
        test("rywib", "rywib", true)
    }

    #[test]
    fn discussion_case3() {
        test("xlkrjiiklmmtezp", "xnfbamkizybugtm", false)
    }

    #[test]
    fn discussion_case4() {
        test("gemvejmohpmkg", "hemvejmogpmkg", true)
    }

    #[test]
    fn discussion_case5() {
        test(
            "siyolsdcjthwsiplccjbuceoxmpjgrauocx",
            "siyolsdcjthwsiplccpbuceoxmjjgrauocx",
            true,
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            "djrxsradfumetmknyvsyydhnibbtvphmhcjmeemgczdjttkbmayivbfowsnlhytqy",
            "djrxsradfumetmknxvsyydhnibbtvphmhcjmeemgcydjttkbmayivbfowsnlhytqy",
            false,
        )
    }

    #[test]
    fn discussion_case7() {
        test(
            "xbmkpymqnuhknexweggxfrygymgauyrlbrgqbphyefzkeblqlgqwzvnenlzhdoxpwptuibvjkcb",
            "xbmkpymqnnhkuexweggxfrygymgauyrlbrgqbphyefzkeblqlgqwzvnenlzhdoxpwptuibvjkcb",
            true,
        )
    }
}
