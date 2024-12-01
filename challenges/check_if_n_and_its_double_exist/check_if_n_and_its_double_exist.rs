// https://leetcode.com/problems/check-if-n-and-its-double-exist/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn check_if_exist(arr: Vec<i32>) -> bool {
//         let mut seen = std::collections::HashSet::new();
//         for num in arr {
//             if seen.contains(&(num * 2)) || (num % 2 == 0 && seen.contains(&(num / 2))) {
//                 return true;
//             }
//             seen.insert(num);
//         }
//         false
//     }
// }

// Limits
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen = std::collections::HashSet::new();
        for num in arr {
            assert!(num >= -1000);
            assert!(num <= 1000);
            let num = num as i16;
            if seen.contains(&(num * 2)) || (num % 2 == 0 && seen.contains(&(num / 2))) {
                return true;
            }
            seen.insert(num);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr: &[i32], expected: bool) {
        assert_eq!(Solution::check_if_exist(arr.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[10, 2, 5, 3], true)
    }

    #[test]
    fn ex2() {
        test(&[3, 1, 7, 11], false)
    }
}
