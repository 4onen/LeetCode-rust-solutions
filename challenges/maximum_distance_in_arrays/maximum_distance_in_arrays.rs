// https://leetcode.com/problems/maximum-distance-in-arrays/

pub struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min = arrays[0][0];
        let mut max = arrays[0][arrays[0].len() - 1];
        let mut result = 0;
        for arr in arrays.into_iter().skip(1) {
            let curr_min = arr[0];
            let curr_max = arr[arr.len() - 1];
            result = std::cmp::max(result, std::cmp::max((curr_max - min).abs(), (max - curr_min).abs()));
            min = std::cmp::min(min, curr_min);
            max = std::cmp::max(max, curr_max);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &[&[i32]], expected: u16) {
        assert!(input.len() >= 2);
        assert!(input.len() <= 100_000);
        let mut total_len = 0;
        for &arr in input {
            assert!(arr.len() >= 1);
            assert!(arr.len() <= 500);
            total_len += arr.len();
            for val in arr {
                assert!(*val >= -10_000);
                assert!(*val <= 10_000);
            }
            assert!(total_len <= 100_000);
        }
        let arrays = input.iter().map(|v| v.to_vec()).collect();
        assert_eq!(Solution::max_distance(arrays), expected as i32);
    }

    #[test]
    fn ex1() {
        test(&[&[1, 2, 3], &[4, 5], &[1, 2, 3]], 4)
    }

    #[test]
    fn ex2() {
        test(&[&[1], &[1]], 0)
    }

    #[test]
    fn failing_case1() {
        test(&[&[-1,1],&[-3,1,4],&[-2,-1,0,2]], 6)
    }

    #[test]
    fn my_failing_case1() {
        test(&[&[-3,1,4],&[-2,-1,0,2]], 6)
    }

    #[test]
    fn my_failing_case2() {
        test(&[&[-1,1],&[-3,4],&[-2,2]], 6)
    }

    #[test]
    fn my_failing_case3() {
        test(&[&[1],&[-3,4],&[-2,2]], 6)
    }

    #[test]
    fn discussion_case1() {
        test(&[&[-3,-3],&[-3,-2]], 1)
    }

    #[test]
    fn discussion_case2() {
        test(&[&[1,4],&[0,5]], 4)
    }

    #[test]
    fn discussion_case3() {
        test(&[&[5, 7, 10], &[1, 2, 3], &[15, 20, 25]], 24)
    }

    #[test]
    fn discussion_case4() {
        test(&[&[0,9],&[1,3]], 8)
    }
}
