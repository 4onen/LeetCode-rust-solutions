// https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs/

pub struct Solution;

// My sol'n (Longest time first -- incorrect)
// This sol'n is incorrect because this is an NP-hard problem.
// See the refutation test for an example where this sol'n fails.
// impl Solution {
//     pub fn minimum_time_required(mut jobs: Vec<i32>, k: i32) -> i32 {
//         let n = jobs.len() as u8;
//         assert!(n >= 1);
//         assert!(n <= 12);
//         if k == 1 {
//             return jobs.into_iter().sum();
//         }
//         assert!(k > 1);
//         if k == n as i32 {
//             return jobs.into_iter().max().unwrap();
//         }
//         assert!(k < n as i32);
//         // I don't see why we can't be greedy here as long as
//         // we're smart about it.
//         // Sort the jobs by time and assign them to the workers
//         // in order of the _most_ time taken.
//         // If we always assign to the next free worker,
//         // all workers are saturated as evenly as possible.
//         // I think.
//         jobs.sort_unstable();
//         // We'll use a min heap to keep track of the workers' next
//         // available time.
//         let mut workers = std::collections::BinaryHeap::from_iter(
//             (0..k).into_iter().map(|_| std::cmp::Reverse(0)),
//         );
//         for job in jobs.into_iter().rev() {
//             let std::cmp::Reverse(next) = workers.pop().unwrap();
//             // println!(
//             //     "Worker at {} takes job {} to work until {}",
//             //     next,
//             //     job,
//             //     next + job
//             // );
//             workers.push(std::cmp::Reverse(next + job));
//         }
//         // The worker with the most time taken is the time taken.
//         workers
//             .into_iter()
//             .map(|std::cmp::Reverse(n)| n)
//             .max()
//             .unwrap()
//     }
// }

// Exponential sol'n (Full-rank DP)
// impl Solution {
//     pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
//         let n = jobs.len() as u8;
//         assert!(n >= 1);
//         assert!(n <= 12);
//         if k == 1 {
//             return jobs.into_iter().sum();
//         }
//         assert!(k > 1);
//         if k == n as i32 {
//             return jobs.into_iter().max().unwrap();
//         }
//         assert!(k < n as i32);
//         // Precompute the time taken for each subset of jobs.
//         let job_assignment_cost = (0..(1 << n))
//             .map(|i| {
//                 (0..n)
//                     .filter(|&j| i & 1 << j != 0)
//                     .map(|j| jobs[j as usize])
//                     .sum::<i32>()
//             })
//             .collect::<Vec<_>>();
//         // Track the minimum time taken for each subset of jobs
//         // for each number of workers.
//         let mut dp = vec![vec![0; 1 << n]; k as usize];
//         dp[0] = job_assignment_cost.clone(); // 1 worker
//         for i in 1..k as u8 {
//             for j in 0..1u16 << n {
//                 let mut min = std::i32::MAX;
//                 for l in 0..1u16 << n {
//                     if j & l == l {
//                         min = std::cmp::min(
//                             min,
//                             std::cmp::max(
//                                 dp[i as usize - 1][j as usize - l as usize],
//                                 job_assignment_cost[l as usize],
//                             ),
//                         );
//                     }
//                 }
//                 dp[i as usize][j as usize] = min;
//             }
//         }
//         dp[k as usize - 1][(1 << n) - 1]
//     }
// }

// Exponential sol'n (two-row DP)
// impl Solution {
//     pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
//         let n = jobs.len() as u8;
//         assert!(n >= 1);
//         assert!(n <= 12);
//         if k == 1 {
//             return jobs.into_iter().sum();
//         }
//         assert!(k > 1);
//         if k == n as i32 {
//             return jobs.into_iter().max().unwrap();
//         }
//         assert!(k < n as i32);
//         // Precompute the time taken for each subset of jobs.
//         let job_assignment_cost = (0..(1 << n))
//             .map(|i| {
//                 (0..n)
//                     .filter(|&j| i & 1 << j != 0)
//                     .map(|j| jobs[j as usize])
//                     .sum::<i32>()
//             })
//             .collect::<Vec<_>>();
//         // Track the minimum time taken for each subset of jobs
//         // for each number of workers.
//         let mut curr = vec![0; 1 << n];
//         let mut prev = job_assignment_cost.clone(); // 1 worker
//         for _ in 1..k as u8 {
//             for j in 0..1u16 << n {
//                 let mut min = std::i32::MAX;
//                 for l in 0..1u16 << n {
//                     if j & l == l {
//                         min = std::cmp::min(
//                             min,
//                             std::cmp::max(
//                                 prev[j as usize - l as usize],
//                                 job_assignment_cost[l as usize],
//                             ),
//                         );
//                     }
//                 }
//                 curr[j as usize] = min;
//             }
//             std::mem::swap(&mut curr, &mut prev);
//         }
//         prev[(1 << n) - 1]
//     }
// }

// Exponential sol'n (two-row DP) iterator optimizations
impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let n = jobs.len() as u8;
        assert!(n >= 1);
        assert!(n <= 12);
        if k == 1 {
            return jobs.into_iter().sum();
        }
        assert!(k > 1);
        if k == n as i32 {
            return jobs.into_iter().max().unwrap();
        }
        assert!(k < n as i32);
        // Precompute the time taken for each subset of jobs.
        let job_assignment_cost = (0..(1 << n))
            .map(|i| {
                (0..n)
                    .filter(|&j| i & 1 << j != 0)
                    .map(|j| jobs[j as usize])
                    .sum::<i32>()
            })
            .collect::<Vec<_>>();
        // Track the minimum time taken for each subset of jobs
        // for each number of workers.
        let mut curr = vec![0; 1 << n];
        let mut prev = job_assignment_cost.clone(); // 1 worker
        for _ in 1..k as u8 {
            for j in 0..1u16 << n {
                let min = (0..1u16 << n)
                    .filter(|&l| j & l == l)
                    .map(|l| {
                        let v = std::cmp::max(
                            prev[j as usize - l as usize],
                            job_assignment_cost[l as usize],
                        );
                        unsafe { std::num::NonZeroI32::new_unchecked(v) }
                    })
                    .min()
                    .unwrap_or(unsafe { std::num::NonZeroI32::new_unchecked(i32::MAX) })
                    .get();
                curr[j as usize] = min;
            }
            std::mem::swap(&mut curr, &mut prev);
        }
        prev[(1 << n) - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let jobs = vec![3, 2, 3];
        let k = 3;
        assert_eq!(Solution::minimum_time_required(jobs, k), 3);
    }

    #[test]
    fn ex2() {
        let jobs = vec![1, 2, 4, 7, 8];
        let k = 2;
        assert_eq!(Solution::minimum_time_required(jobs, k), 11);
    }

    #[test]
    fn myex1() {
        let jobs = vec![1, 2, 4, 7, 8];
        let k = 3;
        assert_eq!(Solution::minimum_time_required(jobs, k), 8);
    }

    #[test]
    fn myex2() {
        let jobs = vec![1, 2, 4, 7, 8];
        let k = 1;
        assert_eq!(Solution::minimum_time_required(jobs, k), 22);
    }

    #[test]
    fn myex3() {
        let jobs = vec![1, 2, 4, 7, 8];
        let k = 5;
        assert_eq!(Solution::minimum_time_required(jobs, k), 8);
    }

    #[test]
    fn myex4() {
        let jobs = vec![10, 9, 9, 1, 1];
        // Worker 1: 10, 1, 1
        // Worker 2: 9, 9
        let k = 2;
        assert_eq!(Solution::minimum_time_required(jobs, k), 18);
    }

    #[test]
    fn myex5() {
        let jobs = vec![10, 9, 9, 9, 1, 1];
        // Worker 1: 10, 9, 1
        // Worker 2: 9, 9, 1
        let k = 2;
        assert_eq!(Solution::minimum_time_required(jobs, k), 20);
    }

    #[test]
    fn myex6() {
        let jobs = vec![10, 9, 9, 9, 1, 1, 1];
        // Worker 1: 10, 9, 1
        // Worker 2: 9, 9, 1, 1
        let k = 2;
        assert_eq!(Solution::minimum_time_required(jobs, k), 20);
    }

    #[test]
    fn myex7() {
        let jobs = vec![10, 5, 6, 7];
        // Worker 1: 10
        // Worker 2: 7
        // Worker 2: 6, 5
        let k = 3;
        assert_eq!(Solution::minimum_time_required(jobs, k), 11);
    }

    #[test]
    fn myex8() {
        let jobs = vec![10, 5, 6, 7];
        // Worker 1: 10, 5
        // Worker 2: 7, 6
        let k = 2;
        assert_eq!(Solution::minimum_time_required(jobs, k), 15);
    }

    #[test]
    fn wikipedia_longest_time_first_refutation() {
        let jobs = vec![4, 5, 6, 7, 8];
        // Worker 1: 8, 7
        // Worker 2: 6, 5, 4
        let k = 2;
        // https://en.wikipedia.org/wiki/Longest-processing-time-first_scheduling
        assert_eq!(Solution::minimum_time_required(jobs, k), 15);
    }

    #[test]
    fn my_extrema_case1() {
        let jobs = (0..12).map(|_| 10_000_000).collect();
        let k = 1;
        assert_eq!(Solution::minimum_time_required(jobs, k), 120_000_000);
    }

    #[test]
    fn my_extrema_case2() {
        let jobs = (0..12).map(|_| 10_000_000).collect();
        let k = 12;
        assert_eq!(Solution::minimum_time_required(jobs, k), 10_000_000);
    }
}
