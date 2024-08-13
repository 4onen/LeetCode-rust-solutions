// https://leetcode.com/problems/combination-sum-ii/

pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            candidates: &[i32],
            target: i32,
            start: usize,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            match target.cmp(&0i32) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {
                    result.push(path.clone());
                }
                std::cmp::Ordering::Greater => {
                    path.push(0);
                    let mut last = 0;
                    for i in start..candidates.len() {
                        let candidate = candidates[i];
                        if candidate == last {
                            continue;
                        }
                        *path.last_mut().unwrap() = candidate;
                        last = candidate;
                        backtrack(candidates, target - candidate, i + 1, path, result);
                    }
                    path.pop();
                }
            }
        }
        candidates.sort_unstable();
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::with_capacity(candidates.len());
        match candidates.iter().rev().position(|&x| x <= target) {
            Some(i) => candidates.truncate(candidates.len() - i),
            None => return result,
        }
        backtrack(&candidates, target, 0, &mut path, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(candidates: &[i32], target: u8, expected: &[&[i32]]) {
        assert!(candidates.len() >= 1);
        assert!(candidates.len() <= 100);
        for &candidate in candidates {
            assert!(candidate >= 1);
            assert!(candidate <= 50);
        }
        assert!(target >= 1);
        assert!(target <= 30);
        let mut result = Solution::combination_sum2(candidates.to_vec(), target as i32)
            .into_iter()
            .map(|mut v| {
                v.sort_unstable();
                v
            })
            .collect::<Vec<_>>();
        result.sort_unstable();
        let mut expected = expected
            .iter()
            .map(|v| {
                let mut v = v.to_vec();
                v.sort_unstable();
                v
            })
            .collect::<Vec<_>>();
        expected.sort_unstable();
    }

    fn expected_from_str(s: &str) -> Vec<Vec<i32>> {
        s[2..s.len() - 3]
            .split("],[")
            .map(|s| {
                s.split(',')
                    .map(|s| s.parse().expect(&format!("Failed to unwrap {}", s)))
                    .collect()
            })
            .collect()
    }

    #[test]
    fn ex1() {
        test(
            &[10, 1, 2, 7, 6, 1, 5],
            8,
            &[&[1, 1, 6], &[1, 2, 5], &[1, 7], &[2, 6]],
        )
    }

    #[test]
    fn ex2() {
        test(&[2, 5, 2, 1, 2], 5, &[&[1, 2, 2], &[5]])
    }

    #[test]
    fn discussion_case1() {
        test(
            &[2, 2, 2, 2, 3, 6, 7],
            9,
            &[&[2, 2, 2, 3], &[2, 7], &[3, 6]],
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[1, 2, 2, 3, 3, 4, 5, 6, 7, 8, 9, 10],
            25,
            &[
                &[1, 2, 2, 3, 3, 4, 10],
                &[1, 2, 2, 3, 3, 5, 9],
                &[1, 2, 2, 3, 3, 6, 8],
                &[1, 2, 2, 3, 4, 5, 8],
                &[1, 2, 2, 3, 4, 6, 7],
                &[1, 2, 2, 3, 7, 10],
                &[1, 2, 2, 3, 8, 9],
                &[1, 2, 2, 4, 6, 10],
                &[1, 2, 2, 4, 7, 9],
                &[1, 2, 2, 5, 6, 9],
                &[1, 2, 2, 5, 7, 8],
                &[1, 2, 3, 3, 4, 5, 7],
                &[1, 2, 3, 3, 6, 10],
                &[1, 2, 3, 3, 7, 9],
                &[1, 2, 3, 4, 5, 10],
                &[1, 2, 3, 4, 6, 9],
                &[1, 2, 3, 4, 7, 8],
                &[1, 2, 3, 5, 6, 8],
                &[1, 2, 3, 9, 10],
                &[1, 2, 4, 5, 6, 7],
                &[1, 2, 4, 8, 10],
                &[1, 2, 5, 7, 10],
                &[1, 2, 5, 8, 9],
                &[1, 2, 6, 7, 9],
                &[1, 3, 3, 4, 5, 9],
                &[1, 3, 3, 4, 6, 8],
                &[1, 3, 3, 5, 6, 7],
                &[1, 3, 3, 8, 10],
                &[1, 3, 4, 7, 10],
                &[1, 3, 4, 8, 9],
                &[1, 3, 5, 6, 10],
                &[1, 3, 5, 7, 9],
                &[1, 3, 6, 7, 8],
                &[1, 4, 5, 6, 9],
                &[1, 4, 5, 7, 8],
                &[1, 5, 9, 10],
                &[1, 6, 8, 10],
                &[1, 7, 8, 9],
                &[2, 2, 3, 3, 4, 5, 6],
                &[2, 2, 3, 3, 5, 10],
                &[2, 2, 3, 3, 6, 9],
                &[2, 2, 3, 3, 7, 8],
                &[2, 2, 3, 4, 5, 9],
                &[2, 2, 3, 4, 6, 8],
                &[2, 2, 3, 5, 6, 7],
                &[2, 2, 3, 8, 10],
                &[2, 2, 4, 7, 10],
                &[2, 2, 4, 8, 9],
                &[2, 2, 5, 6, 10],
                &[2, 2, 5, 7, 9],
                &[2, 2, 6, 7, 8],
                &[2, 3, 3, 4, 5, 8],
                &[2, 3, 3, 4, 6, 7],
                &[2, 3, 3, 7, 10],
                &[2, 3, 3, 8, 9],
                &[2, 3, 4, 6, 10],
                &[2, 3, 4, 7, 9],
                &[2, 3, 5, 6, 9],
                &[2, 3, 5, 7, 8],
                &[2, 4, 5, 6, 8],
                &[2, 4, 9, 10],
                &[2, 5, 8, 10],
                &[2, 6, 7, 10],
                &[2, 6, 8, 9],
                &[3, 3, 4, 5, 10],
                &[3, 3, 4, 6, 9],
                &[3, 3, 4, 7, 8],
                &[3, 3, 5, 6, 8],
                &[3, 3, 9, 10],
                &[3, 4, 5, 6, 7],
                &[3, 4, 8, 10],
                &[3, 5, 7, 10],
                &[3, 5, 8, 9],
                &[3, 6, 7, 9],
                &[4, 5, 6, 10],
                &[4, 5, 7, 9],
                &[4, 6, 7, 8],
                &[6, 9, 10],
                &[7, 8, 10],
            ],
        )
    }

    #[test]
    fn discussion_case3() {
        let expected = expected_from_str(include_str!("discussion_case3.txt"));
        let refs = expected.iter().map(|v| v.as_slice()).collect::<Vec<_>>();
        test(
            &[
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 33, 3, 3, 3, 4, 4,
                4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 44, 4, 4, 4, 5, 5, 5, 5,
                5, 5, 5, 5, 5, 5, 5, 49, 5, 5, 5, 5, 6, 6, 6, 6,
            ],
            29,
            &refs,
        )
    }

    #[test]
    fn discussion_case4() {
        let expected = expected_from_str(include_str!("discussion_case4.txt"));
        let refs = expected.iter().map(|v| v.as_slice()).collect::<Vec<_>>();
        test(
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
                45, 46, 47, 48, 49, 50,
            ],
            30,
            &refs,
        )
    }

    #[test]
    fn discussion_case5() {
        let expected = expected_from_str(include_str!("discussion_case5.txt"));
        let refs = expected.iter().map(|v| v.as_slice()).collect::<Vec<_>>();
        test(
            &[
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3,
                3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            ],
            15,
            &refs,
        )
    }

    #[test]
    fn discussion_case6() {
        test(&[1; 100], 30, &[&[1; 30]])
    }

    #[test]
    fn my_discussion_case1() {
        let mut nums = [1; 100];
        nums[0] = 2;
        let mut expected = vec![vec![1; 30], vec![1; 30]];
        expected[0][0] = 2;
        let refs = expected.iter().map(|v| v.as_slice()).collect::<Vec<_>>();
        test(&nums, 30, &refs)
    }
}
