// https://leetcode.com/problems/rank-transform-of-an-array/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
//         let mut sorted = arr.clone();
//         sorted.sort_unstable();
//         let mut map = std::collections::HashMap::new();
//         let mut rank = 1;
//         for &num in &sorted {
//             map.entry(num).or_insert_with(|| {
//                 let r = rank;
//                 rank += 1;
//                 r
//             });
//         }
//         arr.iter_mut().for_each(|num| {
//             *num = *map.get(num).unwrap();
//         });
//         arr
//     }
// }

// Optimized sol'n using dedup
impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let mut sorted = arr.clone();
        sorted.sort_unstable();
        sorted.dedup();
        let map: std::collections::HashMap<_, _> = sorted
            .into_iter()
            .enumerate()
            .map(|(i, num)| (num, (i + 1) as i32))
            .collect();
        arr.iter_mut().for_each(|num| {
            *num = *map.get(num).unwrap();
        });
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr: &[i32], expected: &[i32]) {
        assert_eq!(arr.len(), expected.len());
        assert!(arr.len() >= 1);
        assert!(arr.len() <= 100_000);
        for i in 0..arr.len() {
            assert!(arr[i] >= -1_000_000_000);
            assert!(arr[i] <= 1_000_000_000);
            assert!(expected[i] >= 0);
            assert!(expected[i] <= 100_000);
        }
        let result = Solution::array_rank_transform(arr.to_vec());
        assert_eq!(result, expected)
    }

    #[test]
    fn ex1() {
        test(&[40, 10, 20, 30], &[4, 1, 2, 3])
    }

    #[test]
    fn ex2() {
        test(&[100, 100, 100], &[1, 1, 1])
    }

    #[test]
    fn ex3() {
        test(
            &[37, 12, 28, 9, 100, 56, 80, 5, 12],
            &[5, 3, 4, 2, 8, 6, 7, 1, 3],
        )
    }

    #[test]
    fn discussion_case1() {
        test(&[40, 10, 10, 20, 30], &[4, 1, 1, 2, 3])
    }
}
