// https://leetcode.com/problems/max-chunks-to-make-sorted/

pub struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut max_seen = -1;
        let mut chunks = 0;
        for i in 0..arr.len() as u8 {
            if max_seen < arr[i as usize] {
                max_seen = arr[i as usize];
            }
            chunks += (max_seen == i as i32) as u8;
        }
        chunks as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr: &[i32], expected: i32) {
        assert!(arr.len() >= 1);
        assert!(arr.len() <= 10);
        let mut seen = std::collections::HashSet::with_capacity(arr.len());
        for &val in arr {
            assert!(val >= 0);
            assert!(val < arr.len() as i32);
            assert!(seen.insert(val));
        }
        assert_eq!(Solution::max_chunks_to_sorted(arr.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[4, 3, 2, 1, 0], 1)
    }

    #[test]
    fn ex2() {
        test(&[1, 0, 2, 3, 4], 4)
    }

    #[test]
    fn myex3_permutations() {
        test(&[0, 1, 2], 3);
        test(&[0, 2, 1], 2);
        test(&[1, 0, 2], 2);
        test(&[1, 2, 0], 1);
        test(&[2, 0, 1], 1);
        test(&[2, 1, 0], 1);
    }

    #[test]
    fn myex4_permutations() {
        test(&[0,1,2,3], 4);
        test(&[0,1,3,2], 3);
        test(&[0,2,1,3], 3);
        test(&[0,2,3,1], 2);
        test(&[0,3,1,2], 2);
        test(&[0,3,2,1], 2);
        test(&[1,0,2,3], 3);
        test(&[1,0,3,2], 2);
        test(&[1,2,0,3], 2);
        test(&[1,2,3,0], 1);
        test(&[1,3,0,2], 1);
        test(&[1,3,2,0], 1);
        test(&[2,0,1,3], 2);
        test(&[2,0,3,1], 1);
        test(&[2,1,0,3], 2);
        test(&[2,1,3,0], 1);
        test(&[2,3,0,1], 1);
        test(&[2,3,1,0], 1);
        test(&[3,0,1,2], 1);
        test(&[3,0,2,1], 1);
        test(&[3,1,0,2], 1);
        test(&[3,1,2,0], 1);
        test(&[3,2,0,1], 1);
        test(&[3,2,1,0], 1);
    }
}
