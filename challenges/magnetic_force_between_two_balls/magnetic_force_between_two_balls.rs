// https://leetcode.com/problems/magnetic-force-between-two-balls/

pub struct Solution;

// Initial sol'n plus dividing max distance
// impl Solution {
//     pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
//         const fn pass(positions: &[i32], m: i32, guess: i32) -> bool {
//             let mut count = 1;
//             let mut last = positions[0];
//             let mut i = 1;
//             while i < positions.len() {
//                 if positions[i] - last >= guess {
//                     count += 1;
//                     last = positions[i];
//                 }
//                 i += 1
//             }
//             count >= m
//         }
//         position.sort_unstable();
//         let max_distance = position[position.len() - 1] - position[0];
//         if m <= 2 {
//             return max_distance;
//         }
//         let mut min_distance = 1;
//         let mut max_distance = max_distance / (m - 1);
//         while min_distance < max_distance {
//             let mid = (min_distance + max_distance ) / 2 + 1;
//             if pass(&position, m, mid) {
//                 min_distance = mid;
//             } else {
//                 max_distance = mid - 1;
//             }
//         }
//         min_distance
//     }
// }

// Optimized sol'n using iterator folding
// impl Solution {
//     pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
//         position.sort_unstable();
//         let position = position;
//         let first = position[0];
//         let max_distance = position[position.len() - 1] - position[0];
//         if m <= 2 {
//             return max_distance;
//         }
//         let mut min_distance = 1;
//         let mut max_distance = max_distance / (m - 1);
//         while min_distance < max_distance {
//             let guess = (min_distance + max_distance ) / 2 + 1;
//             let passes = position.iter().fold((1, first), |(count, prev), &pos| {
//                 if pos - prev >= guess {
//                     (count + 1, pos)
//                 } else {
//                     (count, prev)
//                 }
//             }).0 >= m;
//             if passes {
//                 min_distance = guess;
//             } else {
//                 max_distance = guess - 1;
//             }
//         }
//         min_distance
//     }
// }

// Initial sol'n but now pass has early exit
impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        const fn pass(positions: &[i32], m: i32, guess: i32) -> bool {
            let mut count = 1;
            let mut last = positions[0];
            let mut i = 1;
            while i < positions.len() {
                if positions[i] - last >= guess {
                    count += 1;
                    if count >= m {
                        return true;
                    }
                    last = positions[i];
                }
                i += 1
            }
            false
        }
        position.sort_unstable();
        let max_distance = position[position.len() - 1] - position[0];
        if m <= 2 {
            return max_distance;
        }
        let mut min_distance = 1;
        let mut max_distance = max_distance / (m - 1);
        while min_distance < max_distance {
            let mid = (min_distance + max_distance ) / 2 + 1;
            if pass(&position, m, mid) {
                min_distance = mid;
            } else {
                max_distance = mid - 1;
            }
        }
        min_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(position: &[i32], m: i32, expected: i32) {
        assert!(position.len() >= 2);
        assert!(position.len() <= 100_000);
        assert!(m >= 2);
        assert!(m <= position.len() as i32);
        for &pos in position {
            assert!(pos >= 1);
            assert!(pos <= 1_000_000_000);
        }
        let pos_set = std::collections::HashSet::<i32>::from_iter(position.iter().copied());
        assert_eq!(pos_set.len(), position.len());
        assert_eq!(Solution::max_distance(position.to_vec(), m), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3, 4, 7], 3, 3);
    }

    #[test]
    fn ex2() {
        test(&[5, 4, 3, 2, 1, 1000000000], 2, 999999999);
    }

    #[test]
    fn discussion_case1() {
        test(&[1,2,3,4,5], 3, 2);
    }

    #[test]
    fn discussion_case2() {
        test(&[1,2,4,999999998,999999999,1000000000], 3, 3);
    }

    #[test]
    fn myex1() {
        test(&[1,2,3,4,5], 4, 1);
    }

    #[test]
    fn myex2() {
        test(&[1,2,3,4,5], 5, 1);
    }
}
