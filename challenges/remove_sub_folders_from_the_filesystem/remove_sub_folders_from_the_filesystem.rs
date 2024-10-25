// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/

pub struct Solution;

// Initial allocation
// impl Solution {
//     pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
//         folder.sort_unstable();
//         let mut res = std::vec::Vec::with_capacity(folder.len());
//         let mut prev = "//";
//         for i in 0..folder.len() {
//             if !folder[i].starts_with(&format!("{}/", prev)) {
//                 res.push(folder[i].clone());
//                 prev = &folder[i];
//             }
//         }
//         res
//     }
// }

// Backload all string allocations
impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();
        let mut res = std::vec::Vec::with_capacity(folder.len());
        let mut prev = "//";
        for i in 0..folder.len() {
            if !folder[i].starts_with(&format!("{}/", prev)) {
                res.push(&folder[i]);
                prev = &folder[i];
            }
        }
        res.into_iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(folder: &[&str], expected: &[&str]) {
        let expected_sorted = {
            let mut v = expected.to_vec();
            v.sort_unstable();
            v
        };
        let result = Solution::remove_subfolders(folder.iter().map(|s| s.to_string()).collect());
        let result_sorted = {
            let mut v = result.iter().map(|s| s.as_str()).collect::<Vec<_>>();
            v.sort_unstable();
            v
        };
        assert_eq!(result_sorted, expected_sorted);
    }

    #[test]
    fn ex1() {
        test(
            &["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"],
            &["/a", "/c/d", "/c/f"],
        )
    }
}
