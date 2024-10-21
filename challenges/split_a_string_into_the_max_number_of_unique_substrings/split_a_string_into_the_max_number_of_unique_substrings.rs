// https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn max_unique_split(s: String) -> i32 {
//         fn dfs<'a>(s: &'a str, set: &mut std::collections::HashSet<&'a str>, max: &mut i32) {
//             if s.is_empty() {
//                 *max = std::cmp::max(*max, set.len() as i32);
//                 return;
//             }
//             for i in 1..=s.len() {
//                 let (left, right) = s.split_at(i);
//                 if set.insert(left) {
//                     dfs(right, set, max);
//                     set.remove(left);
//                 }
//             }
//         }
//         let mut set = std::collections::HashSet::new();
//         let mut max = 0;
//         dfs(&s, &mut set, &mut max);
//         max
//     }
// }

// Returning sol'n (Faster... somehow)
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        fn dfs<'a>(s: &'a str, set: &mut std::collections::HashSet<&'a str>) -> i32 {
            if s.is_empty() {
                return set.len() as i32;
            }
            let mut max = 0;
            for i in 1..=s.len() {
                let (left, right) = s.split_at(i);
                if set.insert(left) {
                    max = std::cmp::max(max, dfs(right, set));
                    set.remove(left);
                }
            }
            max
        }
        let mut set = std::collections::HashSet::new();
        dfs(&s, &mut set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, expected: u8) {
        assert!(s.len() >= 1);
        assert!(s.len() <= 16);
        assert!(expected >= 1);
        assert!(expected <= s.len() as u8);
        assert_eq!(Solution::max_unique_split(s.to_string()), expected as i32);
    }

    #[test]
    fn ex1() {
        test("ababccc", 5)
    }

    #[test]
    fn ex2() {
        test("aba", 2)
    }

    #[test]
    fn ex3() {
        test("aa", 1)
    }

    #[test]
    fn myex1() {
        test("abcd", 4)
    }

    #[test]
    fn discussion_case1() {
        test("wwwzfvedwfvhsww", 11)
    }

    #[test]
    fn my_extreme_ex1() {
        // a aa aaa aaaa aaaaaa
        test("aaaaaaaaaaaaaaaa", 5)
    }

    #[test]
    fn my_extreme_ex2() {
        // a aa aaa aaaa aaaaa
        test("aaaaaaaaaaaaaaa", 5)
    }
}
