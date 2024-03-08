// https://leetcode.com/problems/array-of-doubled-pairs/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn can_reorder_doubled(mut arr: Vec<i32>) -> bool {
//         arr.sort_unstable();
//         let mut map = std::collections::HashMap::new();
//         for &n in arr.iter() {
//             *map.entry(n).or_insert(0u16) += 1;
//         }
//         for n in arr.into_iter() {
//             match map.get_mut(&n) {
//                 Some(n_ref) if *n_ref == 0 => continue,
//                 Some(n_ref) => *n_ref -= 1,
//                 None => unreachable!("Array element not added to map."),
//             }
//             let pair_el = if n < 0 {
//                 if n % 2 != 0 {
//                     return false;
//                 }
//                 n / 2
//             } else {
//                 n * 2
//             };
//             match map.get_mut(&pair_el) {
//                 Some(pair_ref) if *pair_ref == 0 => return false,
//                 Some(pair_ref) => *pair_ref -= 1,
//                 None => return false,
//             }
//         }
//         true
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn can_reorder_doubled(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let mut map = std::collections::BTreeMap::new();
        for n in arr {
            *map.entry(n).or_insert(0u16) += 1;
        }
        while let Some((n, count)) = map.pop_first() {
            let pair_el = match 0.cmp(&n) {
                std::cmp::Ordering::Equal => {
                    if count % 2 != 0 {
                        return false;
                    }
                    continue;
                }
                std::cmp::Ordering::Less => n * 2,
                std::cmp::Ordering::Greater => {
                    if n % 2 != 0 {
                        return false;
                    }
                    n / 2
                }
            };
            let std::collections::btree_map::Entry::Occupied(mut entry) = map.entry(pair_el) else {
                return false;
            };
            let pair_count = *entry.get();
            if pair_count < count {
                return false;
            }
            if pair_count == count {
                entry.remove();
            } else {
                *entry.get_mut() -= count;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let arr = vec![3, 1, 3, 6];
        assert_eq!(Solution::can_reorder_doubled(arr), false);
    }

    #[test]
    fn ex2() {
        let arr = vec![2, 1, 2, 6];
        assert_eq!(Solution::can_reorder_doubled(arr), false);
    }

    #[test]
    fn ex3() {
        let arr = vec![4, -2, 2, -4];
        assert_eq!(Solution::can_reorder_doubled(arr), true);
    }

    #[test]
    fn discussion_case1() {
        let arr = vec![2, 4, 0, 0, 8, 1];
        assert_eq!(Solution::can_reorder_doubled(arr), true);
    }

    #[test]
    fn discussion_case2() {
        let arr = vec![2, 2, 2, 4, 4, 4];
        assert_eq!(Solution::can_reorder_doubled(arr), true);
    }

    #[test]
    fn myex1() {
        let arr = vec![1, 2, 4, 8];
        assert_eq!(Solution::can_reorder_doubled(arr), true);
    }

    #[test]
    fn myex2() {
        let arr = vec![1, 2, 4, 8, 16, 32];
        assert_eq!(Solution::can_reorder_doubled(arr), true);
    }
}
