// https://leetcode.com/problems/unique-number-of-occurrences

pub struct Solution;

// Braindead hashmap solution
// impl Solution {
//     pub fn unique_occurrences(arr: Vec<i32>) -> bool {
//         let mut map = std::collections::HashMap::new();
//         for i in arr {
//             let count = map.entry(i).or_insert(0);
//             *count += 1;
//         }
//         let mut set = std::collections::HashSet::new();
//         for &v in map.values() {
//             if set.contains(&v) {
//                 return false;
//             }
//             set.insert(v);
//         }
//         true
//     }
// }

// Replacing the set with a vector
// impl Solution {
//     pub fn unique_occurrences(arr: Vec<i32>) -> bool {
//         let mut map = std::collections::HashMap::new();
//         for i in arr {
//             let count = map.entry(i).or_insert(0);
//             *count += 1;
//         }
//         let mut vec = vec![];
//         for &v in map.values() {
//             if vec.contains(&v) {
//                 return false;
//             }
//             vec.push(v);
//         }
//         true
//     }
// }

// Replacing the constant set comparisons with a length check
// impl Solution {
//     pub fn unique_occurrences(arr: Vec<i32>) -> bool {
//         let mut map = std::collections::HashMap::new();
//         for i in arr {
//             let count = map.entry(i).or_insert(0);
//             *count += 1;
//         }
//         let mut set = std::collections::HashSet::new();
//         for &v in map.values() {
//             set.insert(v);
//         }
//         set.len() == map.len()
//     }
// }

// Constant space solution (following LeetCode input bounds) (fastest)
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counts = [0u16; 2001];
        let mut occurrences = [false; 1000];
        for i in arr {
            counts[(i + 1000) as usize] += 1;
        }
        for v in counts {
            if v == 0 {
                continue;
            }
            let index = v as usize - 1;
            if occurrences[index] {
                return false;
            }
            occurrences[index] = true;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
            true
        );
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(
            Solution::unique_occurrences(vec![
                26, 2, 16, 16, 5, 5, 26, 2, 5, 20, 20, 5, 2, 20, 2, 2, 20, 2, 16, 20, 16, 17, 16,
                2, 16, 20, 26, 16
            ]),
            false
        );
    }
}
