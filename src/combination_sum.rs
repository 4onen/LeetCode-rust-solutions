// https://leetcode.com/problems/combination-sum/

pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
                    for i in start..candidates.len() {
                        let candidate = candidates[i];
                        *path.last_mut().unwrap() = candidate;
                        backtrack(candidates, target - candidate, i, path, result);
                    }
                    path.pop();
                }
            }
        }
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::with_capacity(2 * candidates.len());
        backtrack(&candidates, target, 0, &mut path, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let expected = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(Solution::combination_sum(candidates, target), expected);
    }

    #[test]
    fn ex2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(Solution::combination_sum(candidates, target), expected);
    }

    #[test]
    fn ex3() {
        let candidates = vec![2];
        let target = 1;
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combination_sum(candidates, target), expected);
    }

    #[test]
    fn myex1() {
        let candidates = vec![1];
        let target = 1;
        let expected = vec![vec![1]];
        assert_eq!(Solution::combination_sum(candidates, target), expected);
    }

    #[test]
    fn myex2() {
        let candidates = vec![7, 6, 4, 3, 2];
        let target = 7;
        let expected = vec![vec![7], vec![4, 3], vec![3, 2, 2]];
        assert_eq!(Solution::combination_sum(candidates, target), expected);
    }
}
