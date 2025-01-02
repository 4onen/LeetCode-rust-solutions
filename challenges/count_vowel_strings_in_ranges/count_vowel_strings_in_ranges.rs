// https://leetcode.com/problems/count-vowel-strings-in-ranges/

pub struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const fn is_vowel(b: u8) -> bool {
            const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
            let mut i: u8 = 0;
            while i < VOWELS.len() as u8 {
                if b == VOWELS[i as usize] {
                    return true;
                }
                i += 1;
            }
            false
        }
        let mut prefix_sum: Vec<i32> = std::vec::Vec::with_capacity(words.len() + 1);
        let mut so_far = 0;
        for word in words {
            let bs = word.as_bytes();
            let first = bs[0];
            let last = bs[bs.len() - 1];
            let vowel_string: bool = is_vowel(first) && is_vowel(last);
            so_far += vowel_string as i32;
            prefix_sum.push(so_far);
        }
        queries
            .into_iter()
            .map(|q| {
                let end = prefix_sum[q[1] as usize];
                if q[0] == 0 {
                    end
                } else {
                    end - prefix_sum[q[0] as usize - 1]
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(words: &[&str], queries: &[[i32; 2]], expected: &[i32]) {
        assert!(words.len() >= 1);
        assert!(words.len() <= 100_000);
        {
            let mut s = 0;
            for word in words {
                assert!(word.len() >= 1);
                assert!(word.len() <= 40);
                for &b in word.as_bytes() {
                    assert!(b >= b'a');
                    assert!(b <= b'z');
                }
                s += word.len() as u32;
            }
            assert!(s <= 300_000);
        }
        assert!(queries.len() >= 1);
        assert!(queries.len() <= 100_000);
        for query in queries {
            assert!(query[0] >= 0);
            assert!(query[0] <= query[1]);
            assert!(query[1] <= words.len() as i32);
        }
        assert_eq!(
            Solution::vowel_strings(
                words.iter().map(|&s| s.to_owned()).collect(),
                queries.iter().map(|&q| q.to_vec()).collect(),
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &["aba", "bcb", "ece", "aa", "e"],
            &[[0, 2], [1, 4], [1, 1]],
            &[2, 3, 0],
        )
    }

    #[test]
    fn ex2() {
        test(&["a", "e", "i"], &[[0, 2], [0, 1], [2, 2]], &[3, 2, 1])
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                "b",
                "rmivyakd",
                "kddwnexxssssnvrske",
                "vceguisunlxtldqenxiyfupvnsxdubcnaucpoi",
                "nzwdiataxfkbikbtsjvcbjxtr",
                "wlelgybcaakrxiutsmwnkuyanvcjczenuyaiy",
                "eueryyiayq",
                "bghegfwmwdoayakuzavnaucpur",
                "ukorsxjfkdojcxgjxgmxbghno",
                "pmgbiuzcwbsakwkyspeikpzhnyiqtqtfyephqhl",
                "gsjdpelkbsruooeffnvjwtsidzw",
                "ugeqzndjtogxjkmhkkczdpqzwcu",
                "ppngtecadjsirj",
                "rvfeoxunxaqezkrlr",
                "adkxoxycpinlmcvmq",
                "gfjhpxlzmokcmvhjcrbrpfakspscmju",
                "rgmzhaj",
                "ychktzwdhfuruhpvdjwfsqjhztshcxdey",
                "yifrzmmyzvfk",
                "mircixfzzobcficujgbj",
                "d",
                "pxcmwnqknyfkmafzbyajjildngccadudfziknos",
                "dxmlikjoivggmyasaktllgmfhqpyznc",
                "yqdbiiqexkemebyuitve",
            ],
            &[
                [5, 21],
                [17, 22],
                [19, 23],
                [13, 15],
                [20, 23],
                [21, 23],
                [6, 20],
                [1, 8],
                [15, 20],
                [17, 22],
                [6, 6],
                [1, 2],
                [4, 11],
                [14, 23],
                [7, 10],
                [16, 22],
                [20, 22],
                [21, 22],
                [15, 18],
                [5, 16],
                [17, 23],
            ],
            &[
                2, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 2, 0, 1, 0, 0, 0, 0, 2, 0,
            ],
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                "bzmxvzjxfddcuznspdcbwiojiqf",
                "mwguoaskvramwgiweogzulcinycosovozppl",
                "uigevazgbrddbcsvrvnngfrvkhmqszjicpieahs",
                "uivcdsboxnraqpokjzaayedf",
                "yalc",
                "bbhlbmpskgxmxosft",
                "vigplemkoni",
                "krdrlctodtmprpxwditvcps",
                "gqjwokkskrb",
                "bslxxpabivbvzkozzvdaykaatzrpe",
                "qwhzcwkchluwdnqjwhabroyyxbtsrsxqjnfpadi",
                "siqbezhkohmgbenbkikcxmvz",
                "ddmaireeouzcvffkcohxus",
                "kjzguljbwsxlrd",
                "gqzuqcljvcpmoqlnrxvzqwoyas",
                "vadguvpsubcwbfbaviedr",
                "nxnorutztxfnpvmukpwuraen",
                "imgvujjeygsiymdxp",
                "rdzkpk",
                "cuap",
                "qcojjumwp",
                "pyqzshwykhtyzdwzakjejqyxbganow",
                "cvxuskhcloxykcu",
                "ul",
                "axzscbjajazvbxffrydajapweci",
            ],
            &[
                [5, 21],
                [17, 22],
                [19, 23],
                [13, 15],
                [20, 23],
                [21, 23],
                [6, 20],
                [1, 8],
                [15, 20],
                [17, 22],
                [6, 6],
                [1, 2],
                [4, 11],
                [14, 23],
                [7, 10],
                [16, 22],
                [20, 22],
                [21, 22],
                [15, 18],
                [5, 16],
                [17, 23],
            ],
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        )
    }
}
