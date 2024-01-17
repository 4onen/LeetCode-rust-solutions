// https://leetcode.com/problems/insert-interval/

pub struct Solution;

// Merge afterward sol'n (Technically in place)
// impl Solution {
//     pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         // intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
//         let mut last_merged = 0;
//         for idx in 1..intervals.len() {
//             let curr_start = intervals[idx][0];
//             let curr_end = intervals[idx][1];
//             let prev = &mut intervals[last_merged][1];
//             if curr_start <= *prev {
//                 *prev = std::cmp::max(*prev, curr_end);
//             } else {
//                 last_merged += 1;
//                 intervals.swap(last_merged, idx);
//             }
//         }
//         intervals.truncate(last_merged + 1);
//         intervals
//     }
//     pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
//         let first_idx = intervals
//             .binary_search_by(|interval| interval[0].cmp(&new_interval[0]))
//             .unwrap_or_else(|idx| idx);
//         intervals.insert(first_idx, new_interval);
//         Self::merge(intervals)
//     }
// }

// Copying iterator sol'n
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = std::vec::Vec::with_capacity(intervals.len() + 1);
        let mut intervals = intervals.into_iter().peekable();
        while let Some(interval) = intervals.next_if(|interval| interval[1] < new_interval[0]) {
            result.push(interval);
        }
        let mut new_start = new_interval[0];
        let mut new_end = new_interval[1];
        while let Some(interval) = intervals.next_if(|interval| interval[0] <= new_interval[1]) {
            new_start = std::cmp::min(new_start, interval[0]);
            new_end = std::cmp::max(new_end, interval[1]);
        }
        result.push(vec![new_start, new_end]);
        result.extend(intervals);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let expected = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn ex2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let expected = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn myex1() {
        let intervals = vec![vec![4, 5], vec![6, 7]];
        let new_interval = vec![1, 2];
        let expected = vec![vec![1, 2], vec![4, 5], vec![6, 7]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn myex2() {
        let intervals = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];
        let new_interval = vec![0, 8];
        let expected = vec![vec![0, 8], vec![9, 10]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn myex3() {
        let intervals = vec![
            vec![1, 1],
            vec![2, 3],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7],
            vec![8, 9],
        ];
        let new_interval = vec![3, 7];
        let expected = vec![vec![1, 1], vec![2, 7], vec![8, 9]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    // Some discussion cases:
    #[test]
    fn discussion_case1() {
        // [[1,5],[6,8]] // [5,6]
        let intervals = vec![vec![1, 5], vec![6, 8]];
        let new_interval = vec![5, 6];
        let expected = vec![vec![1, 8]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn discussion_case2() {
        // [] // [5,7]
        let intervals: Vec<Vec<i32>> = vec![];
        let new_interval = vec![5, 7];
        let expected = vec![vec![5, 7]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn discussion_case3() {
        // [[1,5]] // [2,7]
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 7];
        let expected = vec![vec![1, 7]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn discussion_case4() {
        // [[1,5]] // [6,8]
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![6, 8];
        let expected = vec![vec![1, 5], vec![6, 8]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn discussion_case5() {
        // [[1,5]] // [0,3]
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![0, 3];
        let expected = vec![vec![0, 5]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }

    #[test]
    fn discussion_case6() {
        // [[1,5]] // [0,0]
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![0, 0];
        let expected = vec![vec![0, 0], vec![1, 5]];
        assert_eq!(Solution::insert(intervals, new_interval), expected);
    }
}
