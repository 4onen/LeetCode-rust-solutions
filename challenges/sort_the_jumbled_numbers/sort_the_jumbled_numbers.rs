// https://leetcode.com/problems/sort-the-jumbled-numbers/

pub struct Solution;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|&x| {
            let mut corrected = 0;
            let mut num = x;
            let mut multiplier = 1;
            if num == 0 {
                mapping[0] as u32
            } else {
                while num > 0 {
                    let digit = num % 10;
                    num /= 10;
                    corrected += mapping[digit as usize] as u32 * multiplier;
                    multiplier *= 10;
                }
                corrected
            }
        });
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(mapping: [u8; 10], nums: &[u32], expected: &[i32]) {
        assert!(nums.len() == expected.len());
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 30_000);
        for &x in nums {
            assert!(x < 1_000_000_000);
        }
        for i in 0..10 {
            assert_eq!(mapping.contains(&(i as u8)), true);
        }
        let mapping_vec = mapping.iter().map(|&x| x as i32).collect();
        let nums_vec: Vec<_> = nums.iter().map(|&x| x as i32).collect();
        for &x in expected {
            assert!(nums_vec.contains(&x));
        }
        assert_eq!(Solution::sort_jumbled(mapping_vec, nums_vec), expected)
    }

    #[test]
    fn ex1() {
        test(
            [8, 9, 4, 0, 2, 1, 3, 5, 7, 6],
            &[991, 338, 38],
            &[338, 38, 991],
        )
    }

    #[test]
    fn ex2() {
        test(
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            &[789, 456, 123],
            &[123, 456, 789],
        )
    }

    #[test]
    fn myex2_1() {
        test(
            [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            &[789, 456, 123],
            &[789, 456, 123],
        )
    }

    #[test]
    fn myex2_2() {
        test(
            [0, 1, 2, 3, 7, 8, 9, 4, 5, 6],
            &[789, 456, 123],
            &[123, 789, 456],
        )
    }

    #[test]
    fn failing_case1() {
        test(
            [5, 6, 8, 7, 4, 0, 3, 1, 9, 2],
            &[7686, 97012948, 84234023, 2212638, 99],
            &[99, 7686, 2212638, 97012948, 84234023],
        )
    }

    #[test]
    fn failing_case2() {
        // Lol, forgot the obvious test case of 0
        test(
            [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        )
    }
}
