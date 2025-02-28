// https://leetcode.com/problems/shortest-common-supersequence/

pub struct Solution;

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        let mut dp = vec![vec![0u16; s2.len() + 1]; s1.len() + 1];
        for i in 0..s1.len() as u16 {
            for j in 0..s2.len() as u16 {
                dp[i as usize + 1][j as usize + 1] = if s1[i as usize] == s2[j as usize] {
                    dp[i as usize][j as usize] + 1
                } else {
                    dp[i as usize + 1][j as usize].max(dp[i as usize][j as usize + 1])
                }
            }
        }
        let mut i = s1.len() as u16;
        let mut j = s2.len() as u16;
        let mut res = std::vec::Vec::with_capacity(dp[i as usize][j as usize] as usize);
        while i > 0 || j > 0 {
            if i == 0 {
                res.insert(0, s2[j as usize - 1]);
                j -= 1;
                continue;
            } else if j == 0 {
                res.insert(0, s1[i as usize - 1]);
                i -= 1;
            } else if s1[i as usize - 1] == s2[j as usize - 1] {
                res.insert(0, s1[i as usize - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i as usize - 1][j as usize] > dp[i as usize][j as usize - 1] {
                res.insert(0, s1[i as usize - 1]);
                i -= 1;
            } else {
                res.insert(0, s2[j as usize - 1]);
                j -= 1;
            }
        }
        unsafe { std::string::String::from_utf8_unchecked(res) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(str1: &str, str2: &str, expected: &[&str]) {
        assert!(str1.len() >= 1);
        assert!(str1.len() <= 1000);
        assert!(str2.len() >= 1);
        assert!(str2.len() <= 1000);
        for &b in str1.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        for &b in str2.as_bytes() {
            assert!(b >= b'a');
            assert!(b <= b'z');
        }
        let result_forward =
            Solution::shortest_common_supersequence(str1.to_owned(), str2.to_owned());
        assert!(
            expected.contains(&result_forward.as_str()),
            "Could not find forward {result_forward:?} in {expected:?}"
        );
        let result_backward =
            Solution::shortest_common_supersequence(str1.to_owned(), str2.to_owned());
        assert!(
            expected.contains(&result_backward.as_str()),
            "Could not find forward {result_backward:?} in {expected:?}"
        );
    }

    #[test]
    fn ex1() {
        test("abac", "cab", &["cabac"])
    }

    #[test]
    fn ex2() {
        test("aaaaaaaa", "aaaaaaaa", &["aaaaaaaa"])
    }

    #[test]
    fn discussion_case1() {
        test("bbabacaa", "cccababab", &["bbcccabacabab"])
    }

    #[test]
    fn discussion_case2() {
        test("horse", "ros", &["horose"])
    }

    #[test]
    fn discussion_case_big() {
        test(
            "atdznrqfwlfbcqkezrltzyeqvqemikzgghxkzenhtapwrmrovwtpzzsyiwongllqmvptwammerobtgmkpowndejvbuwbporfyroknrjoekdgqqlgzxiisweeegxajqlradgcciavbpgqjzwtdetmtallzyukdztoxysggrqkliixnagwzmassthjecvfzmyonglocmvjnxkcwqqvgrzpsswnigjthtkuawirecfuzrbifgwolpnhcapzxwmfhvpfmqapdxgmddsdlhteugqoyepbztspgojbrmpjmwmhnldunskpvwprzrudbmtwdvgyghgprqcdgqjjbyfsujnnssfqvjhnvcotynidziswpzhkdszbblustoxwtlhkowpatbypvkmajumsxqqunlxxvfezayrolwezfzfyzmmneepwshpemynwzyunsxgjflnqmfghsvwpknqhclhrlmnrljwabwpxomwhuhffpfinhnairblcayygghzqmotwrywqayvvgohmujneqlzurxcpnwdipldofyvfdurbsoxdurlofkqnrjomszjimrxbqzyazakkizojwkuzcacnbdifesoiesmkbyffcxhqgqyhwyubtsrqarqagogrnaxuzyggknksrfdrmnoxrctntngdxxechxrsbyhtlbmzgmcqopyixdomhnmvnsafphpkdgndcscbwyhueytaeodlhlzczmpqqmnilliydwtxtpedbncvsqauopbvygqdtcwehffagxmyoalogetacehnbfxlqhklvxfzmrjqofaesvuzfczeuqegwpcmahhpzodsmpvrvkzxxtsdsxwixiraphjlqawxinlwfspdlscdswtgjpoiixbvmpzilxrnpdvigpccnngxmlzoentslzyjjpkxemyiemoluhqifyonbnizcjrlmuylezdkkztcphlmwhnkdguhelqzjgvjtrzofmtpuhifoqnokonhqtzxmimp", 
            "xjtuwbmvsdeogmnzorndhmjoqnqjnhmfueifqwleggctttilmfokpgotfykyzdhfafiervrsyuiseumzmymtvsdsowmovagekhevyqhifwevpepgmyhnagjtsciaecswebcuvxoavfgejqrxuvnhvkmolclecqsnsrjmxyokbkesaugbydfsupuqanetgunlqmundxvduqmzidatemaqmzzzfjpgmhyoktbdgpgbmjkhmfjtsxjqbfspedhzrxavhngtnuykpapwluameeqlutkyzyeffmqdsjyklmrxtioawcrvmsthbebdqqrpphncthosljfaeidboyekxezqtzlizqcvvxehrcskstshupglzgmbretpyehtavxegmbtznhpbczdjlzibnouxlxkeiedzoohoxhnhzqqaxdwetyudhyqvdhrggrszqeqkqqnunxqyyagyoptfkolieayokryidtctemtesuhbzczzvhlbbhnufjjocporuzuevofbuevuxhgexmckifntngaohfwqdakyobcooubdvypxjjxeugzdmapyamuwqtnqspsznyszhwqdqjxsmhdlkwkvlkdbjngvdmhvbllqqlcemkqxxdlldcfthjdqkyjrrjqqqpnmmelrwhtyugieuppqqtwychtpjmloxsckhzyitomjzypisxzztdwxhddvtvpleqdwamfnhhkszsfgfcdvakyqmmusdvihobdktesudmgmuaoovskvcapucntotdqxkrovzrtrrfvoczkfexwxujizcfiqflpbuuoyfuoovypstrtrxjuuecpjimbutnvqtiqvesaxrvzyxcwslttrgknbdcvvtkfqfzwudspeposxrfkkeqmdvlpazzjnywxjyaquirqpinaennweuobqvxnomuejansapnsrqivcateqngychblywxtdwntancarldwnloqyywrxrganyehkglbdeyshpodpmdchbcc",
            &["axjtuwbmvsdzeogmnzorndhmjoqnqjnhmfwlueifbcqkezrwltzyeqvqemggctttilmfokzgpghxotfykyzendhtfapwrmfierovwtpzzrsyuiwongllqseumvpzmymtvsdsowammerobtvagmekpowndhejvbuwbporfyroknrjoekdgqqlgzxihisfweevpepgxajqlrmyhnadgcjtsciavbpgqjzecswtdetmtallzybcukdztovxysgoavfgrejqkliirxuvnagwzmassthjecvfzkmyonglocmvjnxklecwqqvgrzpsswnigsrjthtmxyokuawirbkecfsauzrgbiydfsupuqanetgwounlpqmunhcapzdxwmfhvpfmduqmzidatemaqmzzzfjpdxgmddsdlhyokteubdgqoyepgbzmjkhmfjtspgoxjqbrmfspjmwmedhzrxavhnldugtnsuykpvwaprzrwludbameeqlutwdvgkyghgprqcdgqjjbzyefsujnnssfmqvjhnvcotynidziswpzhjykdszbblustomrxwtlhkiowpatbypwcrvkmajumsxthbebdqqurpphncthoslxxvjfezayreidbolwyekxezfqtzfylizmmneqcvvxepwhrcskstshpemynwzyunsxpgjflnqmfzgmbretpyehstavwpkxegmbtznqhpbclhrzdjlmzibnrouxljwabwpxkeiedzomwhuohffpfinoxhnairblcayygghzqmotwrywqayvvgohmujneqlzurxcpnwdiplwetyudofhyqvfduhrbsoxduggrlofszqeqkqqnrjomszjimrunxbqzyazyakgyoptfkolizeayojwkryidtctemtesuhbzcacnzzvhlbdibhnufjjocporuzuesvoifbuesvuxhgexmckbyfifcxhqntngqyaohfwyubtsrqarqdagkyogrnaxbcoouzbdvygpxjjxeugknksrfzdrmnoxrcapyamuwqtntqspszngdxxecyszhwqdqjxrsbymhtdlkwkvlkdbmzjngvdmhvbllqqlcemkqopyixxdomhnmvnsalldcfpthpkjdgqkyjrrjqqqpndcscbmmelrwyhtyugieyuppqqtaeodlwychtpjmlzoxsckhzyitomjzypqqmnilliysxzztdwtxhddvtvpleqdbwamfnhhkszsfgfcdvsqakyqmmusdvihopbvygqdktcwehffasudmgxmyoualogetovskvcapucehnbfxltotdqhklvxfzmkrjqofaesvuzrtrrfvoczeuqkfegxwpxujizcmahhfiqflpzbuuodsmpyfuoovypstrvkzxxtsdsxwixiraxjuuecphjlimbutnvqtiqvesawxinlrvzyxcwfspdlsttrgknbdcvvtkfqfzwudswtgjpepoiisxbvrfkkeqmdvlpazilzjnywxjyaquirnqpdvigpccnaengnweuobqvxmlznomuejantslzyjjapkxemynsrqivcatemoluhqifyonbnizgycjrhblmuylezwxtdkkzwntancpharlmdwhnkdguhelqzjgvjtrzofmtpuhifoqyywrxrganoyehkonglbdeyshqtzxmimpodpmdchbcc"], // ?
        )
    }
}
