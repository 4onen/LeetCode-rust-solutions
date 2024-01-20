// https://leetcode.com/problems/sum-of-subarray-minimums

pub struct Solution;

// impl Solution {
//     pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
//         const MODULO: u32 = 1_000_000_007;
//         let mut stack = vec![];
//         let mut res: u32 = 0;
//         for i in 0..arr.len() {
//             while stack.last().map_or(false, |&j| arr[j] > arr[i]) {
//                 let j: usize = stack.pop().unwrap();
//                 let k = stack.last().map_or(-1, |&k| k as i32);
//                 res = (res + arr[j] as u32 * (i as u32 - j as u32) * ((j as i32 - k) as u32))
//                     % MODULO;
//             }
//             stack.push(i)
//         }
//         while let Some(j) = stack.pop() {
//             let k = stack.last().map_or(-1, |&k| k as i32);
//             println!("j: {}, k: {}, arr[j]: {}", j, k, arr[j]);
//             res =
//                 (res + arr[j] as u32 * ((arr.len() - j) as u32) * ((j as i32 - k) as u32)) % MODULO;
//         }
//         res as i32
//     }
// }

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MODULO: u32 = 1_000_000_007;
        let mut stack: Vec<i32> = vec![];
        let mut res: u32 = 0;
        for i in 0..arr.len() {
            while stack.last().map_or(false, |&j| arr[j as usize] > arr[i]) {
                let j = stack.pop().unwrap();
                let k = *stack.last().unwrap_or(&-1);
                res = (res
                    + arr[j as usize] as u32
                        * ((i as i32 - j as i32) as u32)
                        * ((j as i32 - k) as u32))
                    % MODULO;
            }
            stack.push(i as i32)
        }
        while let Some(j) = stack.pop() {
            let k = *stack.last().unwrap_or(&-1);
            res = (res
                + arr[j as usize] as u32
                    * ((arr.len() as i32 - j) as u32)
                    * ((j as i32 - k) as u32))
                % MODULO;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::sum_subarray_mins(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            0 + 1
                + 2
                + 3
                + 4
                + 5
                + 6
                + 7
                + 8
                + 9
                + 1
                + 2
                + 3
                + 4
                + 5
                + 6
                + 7
                + 8
                + 1
                + 2
                + 3
                + 4
                + 5
                + 6
                + 7
                + 1
                + 2
                + 3
                + 4
                + 5
                + 6
                + 1
                + 2
                + 3
                + 4
                + 5
                + 1
                + 2
                + 3
                + 4
                + 1
                + 2
                + 3
                + 1
                + 2
                + 1
        )
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::sum_subarray_mins(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
            0 + 1
                + 2
                + 3
                + 4
                + 5
                + 6
                + 7
                + 8
                + 9
                + 1
                + 2
                + 3
                + 4
                + 5
                + 6
                + 7
                + 8
                + 1
                + 2
                + 3
                + 4
                + 5
                + 6
                + 7
                + 1
                + 2
                + 3
                + 4
                + 5
                + 6
                + 1
                + 2
                + 3
                + 4
                + 5
                + 1
                + 2
                + 3
                + 4
                + 1
                + 2
                + 3
                + 1
                + 2
                + 1
        )
    }

    #[test]
    fn myex3() {
        assert_eq!(
            Solution::sum_subarray_mins(vec![2, 1, 2]),
            2 + 1 + 2 + 1 + 1 + 1
        )
    }

    #[test]
    fn myex4() {
        assert_eq!(
            Solution::sum_subarray_mins(vec![2, 1, 2, 3]),
            2 + 1 + 2 + 3 + 1 + 1 + 2 + 1 + 1 + 1
        )
    }

    #[test]
    fn myex5() {
        assert_eq!(
            Solution::sum_subarray_mins(vec![2, 1, 2, 1]),
            2 + 1 + 2 + 1 + 1 + 1 + 1 + 1 + 1 + 1
        )
    }

    #[test]
    fn myex6() {
        assert_eq!(
            Solution::sum_subarray_mins(vec![3, 2, 3, 1]),
            3 + 2 + 3 + 1 + 2 + 2 + 1 + 2 + 1 + 1
        )
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::sum_subarray_mins(vec![71, 55, 82, 55]), 593)
    }

    #[test]
    fn failing_case1() {
        let input = include_str!("sum_of_subarray_minimums_failing_case.txt");
        let input: Vec<i32> = input
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(Solution::sum_subarray_mins(input), 667452382)
    }
}
