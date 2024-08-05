// https://leetcode.com/problems/kth-distinct-string-in-an-array/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn kth_distinct(mut arr: Vec<String>, k: i32) -> String {
//         assert!(k >= 1);
//         let k = k as u32;
//         let mut hash_map = std::collections::HashMap::new();
//         for s in arr.iter() {
//             hash_map.entry(s).and_modify(|v| *v = true).or_insert(false);
//         }
//         let mut i = 0;
//         let mut i_after_filter = 0;
//         while i < arr.len() {
//             if *hash_map
//                 .get(&arr[i])
//                 .expect("Word appeared that was not previously processed")
//             {
//                 // Ignore
//                 i += 1;
//                 continue;
//             }
//             i_after_filter += 1;
//             if i_after_filter >= k {
//                 return std::mem::take(&mut arr[i]);
//             }
//             i += 1;
//         }
//         return "".to_owned();
//     }
// }

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        assert!(k >= 1);
        let k = k as u32;
        let mut hash_map = std::collections::HashMap::new();
        for s in arr.iter() {
            hash_map.entry(s).and_modify(|v| *v = true).or_insert(false);
        }
        let mut i_after_filter = 0;
        for s in arr.iter() {
            match hash_map.get(&s) {
                Some(true) => {}
                _ => {
                    i_after_filter += 1;
                    if i_after_filter >= k {
                        return s.to_owned();
                    }
                }
            }
        }
        return "".to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr: &[&str], k: u16, expected: &str) {
        assert!(arr.len() <= 1000);
        assert!(arr.len() as u16 >= k);
        assert!(k >= 1);
        for s in arr {
            assert!(s.len() >= 1);
            assert!(s.len() <= 5);
        }
        let input: Vec<String> = arr.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::kth_distinct(input, k as i32), expected);
    }

    #[test]
    fn ex1() {
        test(&["d", "b", "c", "b", "c", "a"], 2, "a")
    }

    #[test]
    fn ex2() {
        test(&["aaa", "aa", "a"], 1, "aaa")
    }

    #[test]
    fn ex3() {
        test(&["a", "b", "a"], 3, "")
    }
}
