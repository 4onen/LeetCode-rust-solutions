// https://leetcode.com/problems/replace-words/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
//         struct MinMatchTrie {
//             children: [Option<Box<MinMatchTrie>>; 26],
//             is_word: bool,
//         }
//         impl MinMatchTrie {
//             fn new() -> Self {
//                 MinMatchTrie {
//                     children: std::array::from_fn(|_| None),
//                     is_word: false,
//                 }
//             }
//             fn insert(&mut self, word: &str) {
//                 let mut node = self;
//                 for b in word.bytes() {
//                     let index = (b - b'a') as usize;
//                     node =
//                         node.children[index].get_or_insert_with(|| Box::new(MinMatchTrie::new()));
//                     if node.is_word {
//                         break; // Stop if a shorter word is already in the trie
//                     }
//                 }
//                 node.is_word = true;
//             }
//             fn search<'a>(&self, word: &'a str) -> &'a str {
//                 let mut node = self;
//                 for (i, b) in word.bytes().enumerate() {
//                     let index = (b - b'a') as usize;
//                     match node.children[index] {
//                         Some(ref child) if child.is_word => return &word[..=i],
//                         Some(ref child) => node = child,
//                         None => break,
//                     }
//                 }
//                 word
//             }
//         }
//         let mut trie = MinMatchTrie::new();
//         for word in dictionary {
//             trie.insert(&word);
//         }
//         sentence
//             .split_ascii_whitespace()
//             .map(|word| trie.search(word))
//             .collect::<Vec<&str>>()
//             .join(" ")
//     }
// }

// Two pointer no word formatting sol'n
// We're guaranteed the solution is always shorter so we can just use a two pointer approach
// Tried a version without lifetimes in search(), but returning usize was slower somehow
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        struct MinMatchTrie {
            children: [Option<Box<MinMatchTrie>>; 26],
            is_word: bool,
        }
        impl MinMatchTrie {
            fn new() -> Self {
                MinMatchTrie {
                    children: std::array::from_fn(|_| None),
                    is_word: false,
                }
            }
            fn insert(&mut self, word: &str) {
                let mut node = self;
                for b in word.bytes() {
                    let index = (b - b'a') as usize;
                    node =
                        node.children[index].get_or_insert_with(|| Box::new(MinMatchTrie::new()));
                    if node.is_word {
                        break; // Stop if a shorter word is already in the trie
                    }
                }
                node.is_word = true;
            }
            fn search<'a>(&self, word: &'a str) -> &'a str {
                let mut node = self;
                for (i, b) in word.bytes().enumerate() {
                    let index = (b - b'a') as usize;
                    match node.children[index] {
                        Some(ref child) if child.is_word => return &word[..=i],
                        Some(ref child) => node = child,
                        None => break,
                    }
                }
                word
            }
        }
        let mut trie = MinMatchTrie::new();
        for word in dictionary {
            trie.insert(&word);
        }
        let mut sentence = sentence.into_bytes();
        let mut read_ptr = 0;
        let mut write_ptr = 0;
        while read_ptr < sentence.len() {
            if sentence[read_ptr] == b' ' {
                sentence[write_ptr] = b' ';
                write_ptr += 1;
                read_ptr += 1;
            } else {
                let start = read_ptr;
                while read_ptr < sentence.len() && sentence[read_ptr] != b' ' {
                    read_ptr += 1;
                }
                let replacement_len = trie
                    .search(unsafe { std::str::from_utf8_unchecked(&sentence[start..read_ptr]) })
                    .len();
                let replacement_range = start..start + replacement_len;
                sentence.copy_within(replacement_range, write_ptr);
                write_ptr += replacement_len;
            }
        }
        unsafe {
            sentence.set_len(write_ptr);
            String::from_utf8_unchecked(sentence)
        }
    }
}

// Simplify two-ptr approach further with removing search fn (Not actually simpler, nor faster)
// impl Solution {
//     pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
//         struct MinMatchTrie {
//             children: [Option<Box<MinMatchTrie>>; 26],
//             is_word: bool,
//         }
//         impl MinMatchTrie {
//             fn new() -> Self {
//                 MinMatchTrie {
//                     children: std::array::from_fn(|_| None),
//                     is_word: false,
//                 }
//             }
//             fn insert(&mut self, word: &str) {
//                 let mut node = self;
//                 for b in word.bytes() {
//                     let index = (b - b'a') as usize;
//                     node =
//                         node.children[index].get_or_insert_with(|| Box::new(MinMatchTrie::new()));
//                     if node.is_word {
//                         break; // Stop if a shorter word is already in the trie
//                     }
//                 }
//                 node.is_word = true;
//             }
//         }
//         let mut trie = MinMatchTrie::new();
//         for word in dictionary {
//             trie.insert(&word);
//         }
//         let mut sentence = sentence.into_bytes();
//         let mut read_ptr = 0;
//         let mut write_ptr = 0;
//         while read_ptr < sentence.len() {
//             if sentence[read_ptr] == b' ' {
//                 sentence[write_ptr] = b' ';
//                 write_ptr += 1;
//                 read_ptr += 1;
//             } else {
//                 let start = read_ptr;
//                 let mut node = &trie;
//                 while read_ptr < sentence.len() && sentence[read_ptr] != b' ' {
//                     let index = (sentence[read_ptr] - b'a') as usize;
//                     match node.children.get(index) {
//                         Some(Some(child)) if child.is_word => {
//                             read_ptr += 1;
//                             break;
//                         }
//                         Some(Some(child)) => node = child,
//                         _ => {
//                             // Not a word in the trie, so copy the word as is
//                             while read_ptr < sentence.len() && sentence[read_ptr] != b' ' {
//                                 read_ptr += 1;
//                             }
//                             break;
//                         }
//                     }
//                     read_ptr += 1;
//                 }
//                 sentence.copy_within(start..read_ptr, write_ptr);
//                 write_ptr += read_ptr - start;
//                 while read_ptr < sentence.len() && sentence[read_ptr] != b' ' {
//                     read_ptr += 1;
//                 }
//             }
//         }
//         unsafe {
//             sentence.set_len(write_ptr);
//             String::from_utf8_unchecked(sentence)
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(dictionary: &[&str], sentence: &str, expected: &str) {
        assert!(dictionary.len() > 0);
        assert!(dictionary.len() <= 1000);
        for word in dictionary {
            assert!(word.len() > 0);
            assert!(word.len() <= 100);
            for b in word.bytes() {
                assert!(b >= b'a' && b <= b'z');
            }
        }
        assert!(sentence.len() > 0);
        assert!(sentence.len() <= 1_000_000);
        for b in sentence.bytes() {
            assert!(b == b' ' || (b >= b'a' && b <= b'z'));
        }
        let sentence_word_count = sentence.split_ascii_whitespace().count();
        assert!(sentence_word_count > 0);
        assert!(sentence_word_count <= 1000);
        sentence.split_ascii_whitespace().for_each(|word| {
            assert!(word.len() > 0);
            assert!(word.len() <= 1000);
        });
        for b in expected.bytes() {
            assert!(b == b' ' || (b >= b'a' && b <= b'z'));
        }
        assert!(*sentence.as_bytes().first().unwrap() != b' ');
        assert!(*sentence.as_bytes().last().unwrap() != b' ');

        let dictionary = dictionary.iter().map(|s| s.to_string()).collect();
        assert_eq!(
            Solution::replace_words(dictionary, sentence.to_string()),
            expected.to_string()
        );
    }

    #[test]
    fn ex1() {
        test(
            &["cat", "bat", "rat"],
            "the cattle was rattled by the battery",
            "the cat was rat by the bat",
        );
    }

    #[test]
    fn ex2() {
        test(&["a", "b", "c"], "aadsfasf absbs bbab cadsfafs", "a a b c");
    }

    #[test]
    fn discussion_case1() {
        test(
            &["a", "aa", "aaa", "aaaa"],
            "a a a a a a a a b b a",
            "a a a a a a a a b b a",
        );
    }

    #[test]
    fn discussion_case2() {
        test(
            &["a", "aa", "aaa", "aaaa"],
            "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa",
            "a a a a a a a a bbb baba a",
        );
    }

    #[test]
    fn discussion_case3() {
        test(
            &["catt", "cat", "bat", "rat"],
            "the cattle was rattled by the battery",
            "the cat was rat by the bat",
        );
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                "e", "k", "c", "harqp", "h", "gsafc", "vn", "lqp", "soy", "mr", "x", "iitgm", "sb",
                "oo", "spj", "gwmly", "iu", "z", "f", "ha", "vds", "v", "vpx", "fir", "t", "xo",
                "apifm", "tlznm", "kkv", "nxyud", "j", "qp", "omn", "zoxp", "mutu", "i", "nxth",
                "dwuer", "sadl", "pv", "w", "mding", "mubem", "xsmwc", "vl", "farov", "twfmq",
                "ljhmr", "q", "bbzs", "kd", "kwc", "a", "buq", "sm", "yi", "nypa", "xwz", "si",
                "amqx", "iy", "eb", "qvgt", "twy", "rf", "dc", "utt", "mxjfu", "hm", "trz", "lzh",
                "lref", "qbx", "fmemr", "gil", "go", "qggh", "uud", "trnhf", "gels", "dfdq",
                "qzkx", "qxw",
            ],
            "ikkbp miszkays wqjferqoxjwvbieyk gvcfldkiavww vhokchxz dvypwyb bxahfzcfanteibiltins ueebf lqhflvwxksi dco kddxmckhvqifbuzkhstp wc ytzzlm gximjuhzfdjuamhsu gdkbmhpnvy ifvifheoxqlbosfww mengfdydekwttkhbzenk wjhmmyltmeufqvcpcxg hthcuovils ldipovluo aiprogn nusquzpmnogtjkklfhta klxvvlvyh nxzgnrveghc mpppfhzjkbucv cqcft uwmahhqradjtf iaaasabqqzmbcig zcpvpyypsmodtoiif qjuiqtfhzcpnmtk yzfragcextvx ivnvgkaqs iplazv jurtsyh gzixfeugj rnukjgtjpim hscyhgoru aledyrmzwhsz xbahcwfwm hzd ygelddphxnbh rvjxtlqfnlmwdoezh zawfkko iwhkcddxgpqtdrjrcv bbfj mhs nenrqfkbf spfpazr wrkjiwyf cw dtd cqibzmuuhukwylrnld dtaxhddidfwqs bgnnoxgyynol hg dijhrrpnwjlju muzzrrsypzgwvblf zbugltrnyzbg hktdviastoireyiqf qvufxgcixvhrjqtna ipfzhuvgo daee r nlipyfszvxlwqw yoq dewpgtcrzausqwhh qzsaobsghgm ichlpsjlsrwzhbyfhm ksenb bqprarpgnyemzwifqzz oai pnqottd nygesjtlpala qmxixtooxtbrzyorn gyvukjpc s mxhlkdaycskj uvwmerplaibeknltuvd ocnn frotscysdyclrc ckcttaceuuxzcghw pxbd oklwhcppuziixpvihihp",
            "i miszkays w gvcfldkiavww v dvypwyb bxahfzcfanteibiltins ueebf lqhflvwxksi dc k w ytzzlm gximjuhzfdjuamhsu gdkbmhpnvy i mengfdydekwttkhbzenk w h ldipovluo a nusquzpmnogtjkklfhta k nxzgnrveghc mpppfhzjkbucv c uwmahhqradjtf i z q yzfragcextvx i i j gzixfeugj rnukjgtjpim h a x h ygelddphxnbh rvjxtlqfnlmwdoezh z i bbfj mhs nenrqfkbf spfpazr w c dtd c dtaxhddidfwqs bgnnoxgyynol h dijhrrpnwjlju muzzrrsypzgwvblf z h q i daee r nlipyfszvxlwqw yoq dewpgtcrzausqwhh q i k bqprarpgnyemzwifqzz oai pnqottd nygesjtlpala q gyvukjpc s mxhlkdaycskj uvwmerplaibeknltuvd ocnn f c pxbd oklwhcppuziixpvihihp",
        );
    }
}
