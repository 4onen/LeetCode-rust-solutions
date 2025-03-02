// https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/

pub struct Solution;

impl Solution {
    pub fn merge_arrays(mut nums1: Vec<Vec<i32>>, mut nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = std::vec::Vec::with_capacity(nums1.len() + nums2.len());
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < nums1.len() && i2 < nums2.len() {
            let id1 = nums1[i1][0];
            let id2 = nums2[i2][0];
            if id1 == id2 {
                let mut res = std::mem::take(&mut nums1[i1]);
                res[1] += nums2[i2][1];
                result.push(res);
                i1 += 1;
                i2 += 1;
            } else if id1 < id2 {
                result.push(std::mem::take(&mut nums1[i1]));
                i1 += 1;
            } else {
                result.push(std::mem::take(&mut nums2[i2]));
                i2 += 1;
            }
        }
        while i1 < nums1.len() {
            result.push(std::mem::take(&mut nums1[i1]));
            i1 += 1;
        }
        while i2 < nums2.len() {
            result.push(std::mem::take(&mut nums2[i2]));
            i2 += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums1: &[[u16; 2]], nums2: &[[u16; 2]], expected: &[[i32; 2]]) {
        assert!(nums1.len() >= 1);
        assert!(nums1.len() <= 200);
        assert!(nums2.len() >= 1);
        assert!(nums2.len() <= 200);
        let mut seen1 = std::collections::HashMap::with_capacity(nums1.len());
        let mut last = 0;
        for &[id, val] in nums1 {
            assert!(id >= 1);
            assert!(id <= 1000);
            assert!(val >= 1);
            assert!(val <= 1000);
            assert!(seen1.insert(id, val).is_none());
            assert!(id > last);
            last = id;
        }
        let mut seen2 = std::collections::HashMap::with_capacity(nums2.len());
        let mut last = 0;
        for &[id, val] in nums2 {
            assert!(id >= 1);
            assert!(id <= 1000);
            assert!(val >= 1);
            assert!(val <= 1000);
            assert!(seen2.insert(id, val).is_none());
            assert!(id > last);
            last = id;
        }
        for &[id, val] in expected {
            assert!(id >= 1);
            assert!(id <= 1000);
            assert!(val >= 1);
            assert!(val <= 1000);
            let expected_val = seen1.get(&(id as u16)).copied().unwrap_or(0)
                + seen2.get(&(id as u16)).copied().unwrap_or(0);
            assert_eq!(val, expected_val as i32, "Mismatched value for id {}", id);
        }
        assert_eq!(
            Solution::merge_arrays(
                nums1
                    .iter()
                    .map(|&x| x.iter().map(|&x| x as i32).collect())
                    .collect(),
                nums2
                    .iter()
                    .map(|&x| x.iter().map(|&x| x as i32).collect())
                    .collect(),
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &[[1, 2], [2, 3], [4, 5]],
            &[[1, 4], [3, 2], [4, 1]],
            &[[1, 6], [2, 3], [3, 2], [4, 6]],
        )
    }

    #[test]
    fn ex2() {
        test(
            &[[2, 4], [3, 6], [5, 5]],
            &[[1, 3], [4, 3]],
            &[[1, 3], [2, 4], [3, 6], [4, 3], [5, 5]],
        )
    }
}
