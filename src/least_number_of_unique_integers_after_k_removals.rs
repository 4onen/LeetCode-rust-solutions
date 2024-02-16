// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/

pub struct Solution;

// Initial vec sort sol'n
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut k = k as u32;
        let mut map = std::collections::HashMap::new();
        for n in arr {
            *map.entry(n).or_insert(0) += 1u32;
        }
        let mut frequencies = map.into_values().collect::<Vec<_>>();
        frequencies.sort_unstable();
        let mut i = 0;
        while i < frequencies.len() {
            if k < frequencies[i] {
                break;
            }
            k -= frequencies[i];
            i += 1;
        }
        frequencies.len().saturating_sub(i) as i32
    }
}

// Binary heap sol'n
// impl Solution {
//     pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
//         let mut k = k as u32;
//         let mut map = std::collections::HashMap::new();
//         for n in arr {
//             *map.entry(n).or_insert(0) += 1u32;
//         }
//         let mut heap = map
//             .into_values()
//             .map(|v| std::cmp::Reverse(v))
//             .collect::<std::collections::BinaryHeap<_>>();
//         while let Some(&v) = heap.peek() {
//             if k < v.0 {
//                 break;
//             }
//             k -= v.0;
//             heap.pop();
//         }
//         heap.len() as i32
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
            2
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::find_least_num_of_unique_ints(vec![1], 1), 0);
    }

    #[test]
    fn failing_case() {
        assert_eq!(
            Solution::find_least_num_of_unique_ints(vec![2, 4, 1, 8, 3, 5, 1, 3], 3),
            3
        );
    }
}
