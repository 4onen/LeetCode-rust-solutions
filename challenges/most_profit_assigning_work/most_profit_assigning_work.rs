// https://leetcode.com/problems/most-profit-assigning-work/

pub struct Solution;

// Initial sol'n (being too clever)
// impl Solution {
//     pub fn max_profit_assignment(
//         mut difficulty: Vec<i32>,
//         mut profit: Vec<i32>,
//         mut worker: Vec<i32>,
//     ) -> i32 {
//         assert!(difficulty.len() > 0);
//         assert!(difficulty.len() == profit.len());
//         assert!(worker.len() > 0);
//         let mut jobs: Vec<(i32, i32)> = std::iter::Iterator::zip(
//             difficulty.drain(..),
//             profit.drain(..),
//         )
//         .fold(Vec::new(), |mut jobs, (difficulty, profit)| {
//             // Insertion sort with deduplication
//             let mut i = 0;
//             while i < jobs.len() && jobs[i].0 < difficulty {
//                 i += 1;
//             }
//             if i < jobs.len() && jobs[i].0 == difficulty {
//                 jobs[i].1 = std::cmp::max(jobs[i].1, profit);
//             } else {
//                 jobs.insert(i, (difficulty, profit));
//             }
//             jobs
//         });
//         // Now we do a scan to ensure that the profit monotonically increases.
//         // If a job has a lower profit than one previous, erase it.
//         let mut prev_profit = 0;
//         jobs.retain(|&(_, profit)| {
//             if profit > prev_profit {
//                 prev_profit = profit;
//                 true
//             } else {
//                 false
//             }
//         });
//         worker.sort_unstable();
//         let mut last_difficulty_index = 0;
//         let mut result = 0;
//         for ability in worker {
//             while last_difficulty_index < jobs.len() && jobs[last_difficulty_index].0 <= ability {
//                 last_difficulty_index += 1;
//             }
//             if last_difficulty_index > 0 {
//                 let job = jobs[last_difficulty_index - 1];
//                 result += job.1;
//             }
//         }
//         result
//     }
// }

// Optimized (braindead) sol'n
impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        assert!(difficulty.len() > 0);
        assert!(difficulty.len() == profit.len());
        assert!(worker.len() > 0);
        let mut jobs: Vec<(i32, i32)> =
            std::iter::Iterator::zip(difficulty.into_iter(), profit).collect();
        jobs.sort_unstable();
        worker.sort_unstable();
        let mut max_profit = 0;
        let mut last_difficulty_index = 0;
        let mut result = 0;
        for ability in worker {
            while last_difficulty_index < jobs.len() && jobs[last_difficulty_index].0 <= ability {
                max_profit = std::cmp::max(max_profit, jobs[last_difficulty_index].1);
                last_difficulty_index += 1;
            }
            result += max_profit;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(difficulty: &[i32], profit: &[i32], worker: &[i32], expected: i32) {
        assert_eq!(
            Solution::max_profit_assignment(difficulty.to_vec(), profit.to_vec(), worker.to_vec()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[2, 4, 6, 8, 10], &[10, 20, 30, 40, 50], &[4, 5, 6, 7], 100)
    }

    #[test]
    fn ex2() {
        test(&[85, 47, 57], &[24, 66, 99], &[40, 25, 25], 0)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                66, 1, 28, 73, 53, 35, 45, 60, 100, 44, 59, 94, 27, 88, 7, 18, 83, 18, 72, 63,
            ],
            &[
                66, 20, 84, 81, 56, 40, 37, 82, 53, 45, 43, 96, 67, 27, 12, 54, 98, 19, 47, 77,
            ],
            &[
                61, 33, 68, 38, 63, 45, 1, 10, 53, 23, 66, 70, 14, 51, 94, 18, 28, 78, 100, 16,
            ],
            1392,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                2, 17, 19, 20, 24, 29, 33, 43, 50, 51, 57, 67, 70, 72, 73, 75, 80, 82, 87, 90,
            ],
            &[
                6, 7, 10, 17, 18, 29, 30, 31, 34, 39, 40, 42, 48, 54, 57, 78, 78, 78, 83, 88,
            ],
            &[
                12, 9, 11, 41, 11, 87, 48, 6, 48, 93, 76, 73, 7, 50, 55, 97, 47, 33, 46, 10,
            ],
            693,
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[68, 35, 52, 47, 86],
            &[67, 17, 1, 81, 3],
            &[92, 10, 85, 84, 82],
            324,
        )
    }
}
