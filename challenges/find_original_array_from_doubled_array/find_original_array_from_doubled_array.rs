// https://leetcode.com/problems/find-original-array-from-doubled-array/

pub struct Solution;

// Initial solution (slow)
// impl Solution {
//     pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
//         let mut arr = changed;
//         let mut map = std::collections::BTreeMap::new();
//         for n in arr.drain(..) {
//             *map.entry(n).or_insert(0u32) += 1;
//         }
//         while let Some((n, count)) = map.pop_first() {
//             if n == 0 {
//                 if count % 2 != 0 {
//                     return vec![];
//                 }
//                 arr.extend(std::iter::repeat(0).take((count / 2) as usize));
//                 continue;
//             }
//             let std::collections::btree_map::Entry::Occupied(mut entry) = map.entry(n * 2) else {
//                 return vec![];
//             };
//             let pair_count = *entry.get();
//             if pair_count < count {
//                 return vec![];
//             }
//             if pair_count == count {
//                 entry.remove();
//             } else {
//                 *entry.get_mut() -= count;
//             }
//             arr.extend(std::iter::repeat(n).take(count as usize));
//         }
//         arr
//     }
// }

// Optimized sol'n
// impl Solution {
//     pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
//         let changed_len = changed.len();
//         let mut map = std::collections::BTreeMap::new();
//         for n in changed {
//             *map.entry(n).or_insert(0u32) += 1;
//         }
//         let mut arr = Vec::with_capacity(changed_len / 2);
//         while let Some((n, count)) = map.pop_first() {
//             if n == 0 {
//                 if count % 2 != 0 {
//                     return vec![];
//                 }
//                 arr.extend(std::iter::repeat(0).take((count / 2) as usize));
//                 continue;
//             }
//             let std::collections::btree_map::Entry::Occupied(mut entry) = map.entry(n * 2) else {
//                 return vec![];
//             };
//             let pair_count = *entry.get();
//             if pair_count < count {
//                 return vec![];
//             }
//             if pair_count == count {
//                 entry.remove();
//             } else {
//                 *entry.get_mut() -= count;
//             }
//             arr.extend(std::iter::repeat(n).take(count as usize));
//         }
//         arr
//     }
// }

// Cheesy solution
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let changed_len = changed.len();
        if changed_len % 2 != 0 {
            return vec![];
        }
        let mut map = [0u32; 100_001];
        let mut min = 100_000;
        let mut max = 0u32;
        for n in changed {
            let n = n as u32;
            map[n as usize] += 1;
            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
        }
        let mut arr = Vec::with_capacity(changed_len / 2);
        arr.extend(std::iter::repeat(0).take((map[0] / 2) as usize));
        map[0] = 0;
        for n in (min..=max).rev() {
            let count = map[n as usize];
            if count == 0 {
                continue;
            }
            let pair_count = map[n as usize / 2];
            if n % 2 != 0 || pair_count < count {
                return vec![];
            } else {
                map[n as usize / 2] -= count;
            }
            arr.extend(std::iter::repeat((n / 2) as i32).take(count as usize));
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut result = Solution::find_original_array(vec![1, 3, 4, 2, 6, 8]);
        result.sort_unstable();
        assert_eq!(result, vec![1, 3, 4]);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::find_original_array(vec![6, 3, 0, 1]), vec![]);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::find_original_array(vec![1]), vec![]);
    }

    #[test]
    fn discussion_case1() {
        let mut result = Solution::find_original_array(vec![4, 4, 16, 20, 8, 8, 2, 10]);
        result.sort_unstable();
        assert_eq!(result, vec![2, 4, 8, 10]);
    }

    #[test]
    fn myex1() {
        let mut result = Solution::find_original_array(vec![6, 3, 0, 0, 2, 1]);
        result.sort_unstable();
        assert_eq!(result, vec![0, 1, 3]);
    }

    #[test]
    fn failing_case1() {
        assert_eq!(Solution::find_original_array(vec![4, 2, 0]), vec![]);
    }
}
