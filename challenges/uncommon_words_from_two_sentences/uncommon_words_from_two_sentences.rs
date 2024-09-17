// https://leetcode.com/problems/uncommon-words-from-two-sentences/

pub struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut seen = std::collections::HashMap::new();
        let mut trailing: u8 = 0;
        for i in 0..s1.len() as u8 {
            match s1[i as usize] {
                b' ' => {
                    seen.entry(&s1[trailing as usize..i as usize])
                        .and_modify(|s| *s = true)
                        .or_insert(false);
                    trailing = i + 1;
                }
                _ => {}
            }
        }
        seen.entry(&s1[trailing as usize..])
            .and_modify(|s| *s = true)
            .or_insert(false);
        let mut trailing: u8 = 0;
        for i in 0..s2.len() as u8 {
            match s2[i as usize] {
                b' ' => {
                    seen.entry(&s2[trailing as usize..i as usize])
                        .and_modify(|s| *s = true)
                        .or_insert(false);
                    trailing = i + 1;
                }
                _ => {}
            }
        }
        seen.entry(&s2[trailing as usize..])
            .and_modify(|s| *s = true)
            .or_insert(false);
        seen.into_iter()
            .filter_map(|(k, double_seen)| {
                if double_seen {
                    None
                } else {
                    Some(unsafe { std::str::from_utf8_unchecked(k) }.to_string())
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s1: &str, s2: &str, expected: &[&str]) {
        assert!(s1.len() >= 1);
        assert!(s1.len() <= 200);
        assert!(s2.len() >= 1);
        assert!(s2.len() <= 200);
        assert_eq!(
            Solution::uncommon_from_sentences(s1.to_string(), s2.to_string()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            "this apple is sweet",
            "this apple is sour",
            &["sweet", "sour"],
        )
    }

    #[test]
    fn ex2() {
        test("apple apple", "banana", &["banana"])
    }
}
