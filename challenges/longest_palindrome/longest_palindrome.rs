// https://leetcode.com/problems/longest-palindrome/

pub struct Solution;

// Initial sol'n (but with AND instead of %)
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counts = [0u16; (b'z' - b'A') as usize + 1];
        for b in s.bytes() {
            counts[(b - b'A') as usize] += 1;
        }
        let mut found_odd = 0;
        let mut result = 0;
        for count in counts.into_iter() {
            let is_odd = count & 1;
            result += count - is_odd;
            found_odd |= is_odd;
        }
        (result + found_odd) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: u16) {
        assert!(s.len() > 0);
        assert!(s.len() <= 2000);
        assert_eq!(Solution::longest_palindrome(s.to_string()), expected as i32);
    }

    #[test]
    fn ex1() {
        test("abccccdd", 7)
    }

    #[test]
    fn ex2() {
        test("a", 1)
    }

    #[test]
    fn discussion_case1() {
        test("abcabcabcabcabcabc", 18)
    }

    #[test]
    fn discussion_case2() {
        test("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth", 983)
    }
}
