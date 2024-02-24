// https://leetcode.com/problems/merge-intervals/

pub struct Solution;

// impl Solution {
//     pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
//         let mut result = vec![];
//         let mut intervals = intervals.into_iter();
//         let mut current = intervals.next().unwrap();
//         for interval in intervals {
//             if interval[0] <= current[1] {
//                 current[1] = std::cmp::max(current[1], interval[1]);
//             } else {
//                 result.push(current);
//                 current = interval;
//             }
//         }
//         result.push(current);
//         result
//     }
// }

// impl Solution {
//     pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then_with(|| a[1].cmp(&b[1]).reverse()));
//         let mut intervals = intervals.into_iter();
//         let mut result = vec![intervals.next().unwrap()];
//         let mut current_end = &mut result.last_mut().unwrap()[1];
//         for interval in intervals {
//             if interval[0] <= *current_end {
//                 *current_end = std::cmp::max(*current_end, interval[1]);
//             } else {
//                 result.push(interval);
//                 current_end = &mut result.last_mut().unwrap()[1];
//             }
//         }
//         result
//     }
// }

// in-place sol'n
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut last_merged = 0;
        for idx in 1..intervals.len() {
            let curr_start = intervals[idx][0];
            let curr_end = intervals[idx][1];
            let prev = &mut intervals[last_merged][1];
            if curr_start <= *prev {
                *prev = std::cmp::max(*prev, curr_end);
            } else {
                last_merged += 1;
                intervals.swap(last_merged, idx);
            }
        }
        intervals.truncate(last_merged + 1);
        intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(Solution::merge(intervals), expected);
    }

    #[test]
    fn ex2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let expected = vec![vec![1, 5]];
        assert_eq!(Solution::merge(intervals), expected);
    }

    #[test]
    fn discussion_case1() {
        let intervals = vec![
            vec![1, 1],
            vec![2, 2],
            vec![0, 0],
            vec![2, 3],
            vec![1, 3],
            vec![3, 5],
            vec![2, 3],
            vec![3, 5],
        ];
        let expected = vec![vec![0, 0], vec![1, 5]];
        assert_eq!(Solution::merge(intervals), expected);
    }
}
