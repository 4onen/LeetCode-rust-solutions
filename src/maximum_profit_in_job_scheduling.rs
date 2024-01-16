// https://leetcode.com/problems/maximum-profit-in-job-scheduling/

pub struct Solution;

// DP with index sorting
// impl Solution {
//     pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
//         let mut job_indices = (0u16..end_time.len() as u16).collect::<Vec<_>>();
//         job_indices.sort_by_key(|&i| end_time[i as usize]);

//         let mut dp = vec![0; start_time.len()];
//         dp[0] = profit[job_indices[0] as usize];
//         for i in 1..start_time.len() {
//             let mut profit = profit[job_indices[i] as usize];
//             for j in (0..i).rev() {
//                 if end_time[job_indices[j] as usize] <= start_time[job_indices[i] as usize] {
//                     profit += dp[j];
//                     break;
//                 }
//             }
//             dp[i] = std::cmp::max(dp[i - 1], profit);
//         }
//         dp[start_time.len() - 1]
//     }
// }

// DP with Array of Structs sorting and binary search
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        struct Job {
            end: i32,
            start: i32,
            profit: i32,
        }

        let mut jobs = Iterator::zip(
            Iterator::zip(end_time.into_iter(), start_time.into_iter()),
            profit.into_iter(),
        )
        .map(|((e, s), p)| Job {
            end: e,
            start: s,
            profit: p,
        })
        .collect::<Vec<_>>();
        jobs.sort_by_key(|j| j.end);

        // Will use the profit field to store the dp results as we go
        for i in 1..jobs.len() {
            let mut profit = jobs[i].profit;

            // Binary search for the first job that ends before the current job starts
            let mut start = 0;
            let mut end = i - 1;
            while start < end {
                let mid = (start + end + 1) / 2;
                if jobs[i].start < jobs[mid].end {
                    end = mid - 1;
                } else {
                    start = mid;
                }
            }

            if jobs[start].end <= jobs[i].start {
                profit += jobs[start].profit;
            }

            jobs[i].profit = std::cmp::max(jobs[i - 1].profit, profit);
        }
        jobs[jobs.len() - 1].profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120,
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150,
        )
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
            6,
        )
    }

    #[test]
    fn discussion_example1() {
        assert_eq!(
            Solution::job_scheduling(
                vec![6, 24, 45, 27, 13, 43, 47, 36, 14, 11, 11, 12],
                vec![31, 27, 48, 46, 44, 46, 50, 49, 24, 42, 13, 27],
                vec![14, 4, 16, 12, 20, 3, 18, 6, 9, 1, 2, 8]
            ),
            45,
        )
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 70, 40]),
            120,
        )
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150,
        )
    }
}
