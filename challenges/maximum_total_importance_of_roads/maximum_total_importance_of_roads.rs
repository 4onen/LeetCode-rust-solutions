// https://leetcode.com/problems/maximum-total-importance-of-roads/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        assert!(n >= 2);
        assert!(n <= 50_000);
        let mut city_degrees = vec![0u16; n as usize];
        for road in roads {
            city_degrees[road[0] as usize] += 1;
            city_degrees[road[1] as usize] += 1;
        }
        city_degrees.sort_unstable();
        city_degrees
            .into_iter()
            .enumerate()
            .map(|(i, d)| (i as i64 + 1) * d as i64)
            .sum()
    }
}

// Reduce size of final loop iterator dtype (Whoops, slower)
// impl Solution {
//     pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
//         assert!(n >= 2);
//         assert!(n <= 50_000);
//         let n = n as u16;
//         let mut city_degrees = vec![0u16; n as usize];
//         for road in roads {
//             city_degrees[road[0] as usize] += 1;
//             city_degrees[road[1] as usize] += 1;
//         }
//         city_degrees.sort_unstable();
//         let mut total_importance = 0i64;
//         for i in 0..n {
//             total_importance += (i as i64 + 1) * city_degrees[i as usize] as i64;
//         }
//         total_importance
//     }
// }

// Use a fold to make initial vec
// impl Solution {
//     pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
//         assert!(n >= 2);
//         assert!(n <= 50_000);
//         let mut city_degrees = roads.into_iter().fold(vec![0u16; n as usize], |mut acc, road| {
//             acc[road[0] as usize] += 1;
//             acc[road[1] as usize] += 1;
//             acc
//         });
//         city_degrees.sort_unstable();
//         city_degrees
//             .into_iter()
//             .enumerate()
//             .map(|(i, d)| (i as i64 + 1) * d as i64)
//             .sum()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, roads: &[[i32; 2]], expected: i64) {
        assert!(n >= 2);
        assert!(n <= 50_000);
        assert!(roads.len() >= 1);
        assert!(roads.len() <= 50_000);
        for (i, [a, b]) in roads.iter().enumerate() {
            assert_ne!(*a, *b);
            assert!(*a >= 0);
            assert!(*a < n);
            assert!(*b >= 0);
            assert!(*b < n);
            for j in 0..i {
                assert_ne!(roads[j], [*a, *b]);
                assert_ne!(roads[j], [*b, *a]);
            }
        }
        let roads = roads.iter().map(|r| r.to_vec()).collect();
        assert_eq!(Solution::maximum_importance(n, roads), expected);
    }

    #[test]
    fn ex1() {
        test(5, &[[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]], 43);
    }

    #[test]
    fn ex2() {
        test(5, &[[0, 3], [2, 4], [1, 3]], 20);
    }

    #[test]
    fn discussion_case1() {
        test(5, &[[0, 1]], 9);
    }
}
