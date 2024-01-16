// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/

pub struct Solution;

// Recursive sol'n
type Difficulty = i32;
type DifficultySum = i32;
type DayCount = i32;
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<Difficulty>, d: DayCount) -> DifficultySum {
        if d > job_difficulty.len() as DayCount {
            return -1;
        }

        fn recurse(
            job_difficulty: &[Difficulty],
            splits_remaining: DayCount,
            max_today: Difficulty,
            memo: &mut std::collections::HashMap<(usize, DayCount, Difficulty), DifficultySum>,
        ) -> DifficultySum {
            // Building subarray days_remaining+1
            if splits_remaining == 0 {
                // No more splits to make
                return std::cmp::max(Some(max_today), job_difficulty.iter().max().copied())
                    .unwrap();
            }

            // At least one more split to make.
            // If there's not enough jobs left to split, return MAX so we don't
            // consider this path
            if splits_remaining > job_difficulty.len() as DayCount {
                return DifficultySum::MAX;
            }

            let cache_id = (job_difficulty.len(), splits_remaining, max_today);
            if let Some(cached) = memo.get(&cache_id) {
                return *cached;
            }

            // If we're here, we have at least one more split to make, and
            // there's enough jobs left to split.
            // Try splitting here and not splitting here, and return the min.
            let result = std::cmp::min(
                recurse(
                    &job_difficulty[1..],
                    splits_remaining,
                    std::cmp::max(max_today, job_difficulty[0]),
                    memo,
                ),
                max_today.saturating_add(recurse(
                    &job_difficulty[1..],
                    splits_remaining - 1,
                    job_difficulty[0],
                    memo,
                )),
            );
            memo.insert(cache_id, result);
            result
        }

        let mut memo = std::collections::HashMap::new();
        recurse(&job_difficulty[1..], d - 1, job_difficulty[0], &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::min_difficulty(vec![6, 5, 4, 3, 2, 1], 2), 7);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::min_difficulty(vec![9, 9, 9], 4), -1);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::min_difficulty(vec![1, 1, 1], 3), 3);
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(
            Solution::min_difficulty(
                vec![
                    186, 398, 479, 206, 885, 423, 805, 112, 925, 656, 16, 932, 740, 292, 671, 360,
                ],
                4,
            ),
            1803,
        )
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::min_difficulty(vec![1, 2, 3, 4, 5, 6], 2), 7);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::min_difficulty(vec![9, 9, 9, 1], 2), 10);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::min_difficulty(vec![1, 1, 9, 1, 9, 1], 2), 10);
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::min_difficulty(vec![6, 1], 2), 7);
    }

    #[test]
    fn myex5() {
        assert_eq!(Solution::min_difficulty(vec![6, 1, 2], 2), 8);
    }

    #[test]
    fn myex6() {
        assert_eq!(Solution::min_difficulty(vec![6], 1), 6);
    }

    #[test]
    fn myex7() {
        assert_eq!(Solution::min_difficulty(vec![6, 1, 2, 3], 1), 6);
    }

    #[test]
    fn myex8() {
        let input = (1..=300).rev().collect();
        assert_eq!(Solution::min_difficulty(input, 3), 303);
    }

    #[test]
    fn myex9() {
        let input = (1..=300).rev().cycle().take(900).collect();
        assert_eq!(Solution::min_difficulty(input, 3), 303);
    }
}
