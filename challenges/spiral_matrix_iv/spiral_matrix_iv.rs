// https://leetcode.com/problems/spiral-matrix-iv/

use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        struct ListDrainIter {
            head: Option<Box<ListNode>>
        }
        impl Iterator for ListDrainIter {
            type Item = i32;
            fn next(&mut self) -> Option<Self::Item> {
                if let Some(node) = std::mem::take(&mut self.head) {
                    self.head = node.next;
                    Some(node.val)
                } else {
                    None
                }
            }
        }
        let mut iter = ListDrainIter { head };
        let mut result = vec![vec![-1; n as usize]; m as usize];
        let mut top = 0;
        let mut bottom = m - 1;
        let mut left = 0;
        let mut right = n - 1;
        'outer: while top <= bottom && left <= right {
            for i in left..=right {
                let Some(x) = iter.next() else { break 'outer; };
                result[top as usize][i as usize] = x;
            }
            top += 1;
            for i in top..=bottom {
                let Some(x) = iter.next() else { break 'outer; };
                result[i as usize][right as usize] = x;
            }
            right -= 1;
            for i in (left..=right).rev() {
                let Some(x) = iter.next() else { break 'outer; };
                result[bottom as usize][i as usize] = x;
            }
            bottom -= 1;
            for i in (top..=bottom).rev() {
                let Some(x) = iter.next() else { break 'outer; };
                result[i as usize][left as usize] = x;
            }
            left += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(m: i32, n: i32, input: &[i32], expected: Vec<Vec<i32>>) {
        assert!(m >= 1);
        assert!(m <= 100_000);
        assert!(n >= 1);
        assert!(n <= 100_000);
        assert!(m*n <= 100_000);
        assert!(input.len() >= 1);
        assert!(input.len() <= m as usize*n as usize);
        for &x in input {
            assert!(x >= 0);
            assert!(x <= 1000);
        }
        let head = ListNode::from_slice(input);
        assert_eq!(Solution::spiral_matrix(m, n, head), expected);
    }

    #[test]
    fn ex1() {
        test(
            3,
            5,
            &[3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0],
            vec![
                vec![3, 0, 2, 6, 8],
                vec![5, 0, -1, -1, 1],
                vec![5, 2, 4, 9, 7],
            ],
        )
    }

    #[test]
    fn ex2() {
        test(
            1,
            4,
            &[0,1,2],
            vec![vec![0, 1, 2, -1]],
        )
    }

    #[test]
    fn myex2() {
        test(
            1,
            4,
            &[0,1,2,3],
            vec![vec![0, 1, 2, 3]],
        )
    }
}
