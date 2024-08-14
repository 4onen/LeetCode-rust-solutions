// https://leetcode.com/problems/find-k-th-smallest-pair-distance/

pub struct Solution;

// Binary search best sol'n
impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        assert!(nums.len() >= 2);
        assert!(nums.len() <= 10_000);
        if nums.len() == 2 {
            return nums[1] - nums[0];
        }
        let mut low = 0;
        let mut high = nums[nums.len() - 1] - nums[0];
        while low < high {
            let mid = low + (high - low) / 2;
            let mut count = 0i32;
            let mut left = 0u16;
            for right in 0..nums.len() as u16 {
                while nums[right as usize] - nums[left as usize] > mid {
                    left += 1;
                }
                count += (right - left) as i32;
            }
            if (count as i32) < k {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: u32, expected: u32) {
        assert!(nums.len() >= 2);
        assert!(nums.len() <= 10_000);
        for num in nums {
            assert!(*num >= 0);
            assert!(*num <= 1_000_000);
        }
        assert!(k >= 1);
        assert!(k <= nums.len() as u32 * (nums.len() as u32 - 1) / 2);
        assert_eq!(
            Solution::smallest_distance_pair(nums.to_vec(), k as i32),
            expected as i32
        )
    }

    fn test_seq(input: &[i32], expected: &[u32]) {
        assert_eq!(expected.len(), input.len() * (input.len() - 1) / 2);
        for (i, num) in expected.iter().enumerate() {
            test(input, i as u32 + 1, *num);
        }
    }

    #[test]
    fn ex1() {
        test(&[1, 3, 1], 1, 0);
    }

    #[test]
    fn ex2() {
        test(&[1, 1, 1], 2, 0);
    }

    #[test]
    fn ex3() {
        test(&[1, 6, 1], 3, 5);
    }

    #[test]
    fn myex12() {
        let input = [1,2];
        let results = [1];
        test_seq(&input, &results);
    }

    #[test]
    fn myex123() {
        let input = [1,2,3];
        let results = [1,1,2];
        test_seq(&input, &results);
    }

    #[test]
    fn myex1234() {
        let input = [1,2,3,4];
        let results = [1,1,1,2,2,3];
        test_seq(&input, &results);
    }

    #[test]
    fn myex12345() {
        let input = [1,2,3,4,5];
        let results = [1,1,1,1,2,2,2,3,3,4];
        test_seq(&input, &results);
    }

    #[test]
    fn myex123456() {
        let input = [1,2,3,4,5,6];
        let results = [1,1,1,1,1,2,2,2,2,3,3,3,4,4,5];
        test_seq(&input, &results);
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[0, 1_000_000], 1, 1_000_000);
    }

    #[test]
    fn my_extreme_ex2() {
        let input = [0, 500_000, 1_000_000];
        let results = [500_000, 500_000, 1_000_000];
        test_seq(&input, &results);
    }

    #[test]
    fn my_extreme_ex3() {
        let input = [0, 500_000, 500_000, 1_000_000];
        let results = [0, 500_000, 500_000, 500_000, 500_000, 1_000_000];
        test_seq(&input, &results);
    }

    #[test]
    fn my_extreme_ex4() {
        let mut input = [0; 10_000];
        for i in 0..10_000 {
            input[i] = i as i32;
        }
        test(&input, 1, 1);
        test(&input, (input.len() * (input.len() - 1) / 2) as u32, 9_999);
    }

    #[test]
    fn my_extreme_ex5() {
        let mut input = [0; 10_000];
        for i in 0..10_000 {
            input[i] = 10*i as i32;
        }
        test(&input, 1, 10);
        test(&input, (input.len() * (input.len() - 1) / 2) as u32, 99_990);
    }

    #[test]
    fn my_extreme_ex6() {
        let mut input = [0; 10_000];
        for i in 0..10_000 {
            input[i] = 100*i as i32;
        }
        test(&input, 1, 100);
        test(&input, (input.len() * (input.len() - 1) / 2) as u32, 999_900);
    }

    #[test]
    fn my_extreme_ex7() {
        let mut input = [0; 10_000];
        for i in 0..10_000 {
            input[i] = std::cmp::min(1_000_000,1_000*i as i32);
        }
        test(&input, 1, 0);
        test(&input, (input.len() * (input.len() - 1) / 2) as u32, 1_000_000);
    }
}
