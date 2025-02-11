// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/

pub struct Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut left = 0;
        let mut result = s.into_bytes();
        let part = part.as_bytes();
        for right in 0..result.len() {
            result[left] = result[right];
            left += 1;
            if left >= part.len() && &result[left - part.len()..left] == part {
                left -= part.len();
            }
        }
        result.truncate(left);
        unsafe { std::string::String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, part: &str, expected: &str) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 1000);
        assert!(part.len() >= 1);
        assert!(part.len() <= s.len());
        assert!(expected.len() <= s.len());
        for &b in s.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z')
        }
        for &b in part.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        assert_eq!(
            Solution::remove_occurrences(s.to_owned(), part.to_owned()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("daabcbaabcbc", "abc", "dab")
    }

    #[test]
    fn ex2() {
        test("axxxxyyyyb", "xy", "ab")
    }

    #[test]
    fn discussion_case1() {
        test("aaa", "aa", "a")
    }

    #[test]
    fn discussion_case2() {
        test("eemckxmckx", "emckx", "")
    }

    #[test]
    fn discussion_case3() {
        test("ccctltctlltlb", "ctl", "b")
    }

    #[test]
    fn discussion_case4() {
        test("yjyjqnaxlbqnaxlbfss", "yjqnaxlb", "fss")
    }

    #[test]
    fn discussion_case5() {
        test("gjzgbpggjzgbpgsvpwdk", "gjzgbpg", "svpwdk")
    }

    #[test]
    fn discussion_case6() {
        test("hhvhvaahvahvhvaavhvaasshvahvaln", "hva", "ssln")
    }

    #[test]
    fn discussion_case7() {
        test("totmejyatprpawuabfkorqfaamcznovsauicaynoxszfdhsgvortexhcouwcinlxdjxmsybkwxslhejgycxtotmejyatprpawuabfkorqfaamcznovsauicaynoxszfdhsgvortexhcouwcinlxdjxmsybkwxslhejgycxlkjhnbjwxfgzcudsfvloriblhwuqcaxstzrofsdgdscexjdevsmmisbovaefkefzlytoqgxyiowmodrtnifrelddgmstxcmsownhxiwzbidhezpcvdgbfacstynokhtlkjhnbjwxfgzcudsfvloriblhwuqcaxstzrofsdgdscexjdevsmmisbovaefkefzlytoqgxyiowmodrtnifrelddgmstxcmsownhxiwzbidhezpcvdgbfacstynokhtuoekjxjgycjnvhphixsgyzfwcqyfuyskgsrheuftaakrkufuuwkhwttydjdbjmutbnxeruqjhaargonhirtlsyysidknonutscislmuvkhuipijixqqk", "totmejyatprpawuabfkorqfaamcznovsauicaynoxszfdhsgvortexhcouwcinlxdjxmsybkwxslhejgycxlkjhnbjwxfgzcudsfvloriblhwuqcaxstzrofsdgdscexjdevsmmisbovaefkefzlytoqgxyiowmodrtnifrelddgmstxcmsownhxiwzbidhezpcvdgbfacstynokht", "uoekjxjgycjnvhphixsgyzfwcqyfuyskgsrheuftaakrkufuuwkhwttydjdbjmutbnxeruqjhaargonhirtlsyysidknonutscislmuvkhuipijixqqk")
    }

    #[test]
    fn discussion_case8() {
        test("avtjogzavshnomyochrrwtxedldenqyszxejcgqoksmyzgfifjiwrzimsgcyoqcultmxvolfhwcsjipjlvaceeoxavtjogzavshnomyochrrwtxedldenqyszxejcgqoksmyzgfifjiwrzimsgcyoqcultmxvolfhwcsjipjlvaceeoxexaehmmneivlscizwzqckdqusbhwosqitxbkgtvhfjpzchwwuvflpdlksldqigioembrrekuwamhhvkrakrhlbbnhjtvcxjmpygnvszgwvjxrszawpwhyuykbzyyeecscusbvdcjolammskjhalzcozqpdobzmmggrefyyzgxqnrdxwzswlboynqbgbiwyyiukqnqnmmvjfpacmwmwxaacuejcwjjocotacgkgtahsrsqgqknwukyolxhaaezivyezpjabsdkhdthqmbwqwjdablqkcxpxwgjwdtzmgqyhjhmfukhzmqdzebbwbsjyvbstijoraxbicpkuehjkqjhxlexaehmmneivlscizwzqckdqusbhwosqitxbkgtvhfjpzchwwuvflpdlksldqigioembrrekuwamhhvkrakrhlbbnhjtvcxjmpygnvszgwvjxrszawpwhyuykbzyyeecscusbvdcjolammskjhalzcozqpdobzmmggrefyyzgxqnrdxwzswlboynqbgbiwyyiukqnqnmmvjfpacmwmwxaacuejcwjjocotacgkgtahsrsqgqknwukyolxhaaezivyezpjabsdkhdthqmbwqwjdablqkcxpxwgjwdtzmgqyhjhmfukhzmqdzebbwbsjyvbstijoraxbicpkuehjkqjhxlepgnzrlbzytiaasnnahnbjiqgwiossjxsowggbguovdikhqrdcpheamfysdjgvykvxhqxyxmnylkftkqstpqbspuoyckpkvttagylycvpxgdbaemzhbgjmlkcccaniibqetjy", "avtjogzavshnomyochrrwtxedldenqyszxejcgqoksmyzgfifjiwrzimsgcyoqcultmxvolfhwcsjipjlvaceeoxexaehmmneivlscizwzqckdqusbhwosqitxbkgtvhfjpzchwwuvflpdlksldqigioembrrekuwamhhvkrakrhlbbnhjtvcxjmpygnvszgwvjxrszawpwhyuykbzyyeecscusbvdcjolammskjhalzcozqpdobzmmggrefyyzgxqnrdxwzswlboynqbgbiwyyiukqnqnmmvjfpacmwmwxaacuejcwjjocotacgkgtahsrsqgqknwukyolxhaaezivyezpjabsdkhdthqmbwqwjdablqkcxpxwgjwdtzmgqyhjhmfukhzmqdzebbwbsjyvbstijoraxbicpkuehjkqjhxl", "epgnzrlbzytiaasnnahnbjiqgwiossjxsowggbguovdikhqrdcpheamfysdjgvykvxhqxyxmnylkftkqstpqbspuoyckpkvttagylycvpxgdbaemzhbgjmlkcccaniibqetjy")
    }
}
