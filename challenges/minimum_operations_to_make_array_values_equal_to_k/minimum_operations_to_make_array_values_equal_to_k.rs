// https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut has = [false; 101];
        for &num in &nums {
            if num < k {
                return -1;
            }
            has[num as usize] = true;
        }
        has[k as usize + 1..=100]
            .iter()
            .map(|&x| x as u8)
            .sum::<u8>() as i32
    }
}

// Bitset sol'n (slower)
// impl Solution {
//     pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
//         let mut has = 0u128;
//         for &num in &nums {
//             if num < k {
//                 return -1;
//             }
//             has |= 1 << num;
//         }
//         (has >> k + 1).count_ones() as i32
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], k: i32, expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100);
        assert!(k >= 1);
        assert!(k <= 100);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 100);
        }
        assert_eq!(
            Solution::min_operations(nums.iter().map(|&x| x as i32).collect(), k as i32),
            expected
        );
    }

    #[test]
    fn ex0() {
        test(&[10, 8, 10, 8], 8, 1)
    }

    #[test]
    fn ex0_1() {
        test(&[10, 8, 10, 8], 7, 2)
    }

    #[test]
    fn ex0_2() {
        test(&[10, 8, 10, 8], 10, -1)
    }

    #[test]
    fn ex0_3() {
        test(&[10, 8, 10, 8], 9, -1)
    }

    #[test]
    fn ex1() {
        test(&[5, 2, 5, 4, 5], 2, 2)
    }

    #[test]
    fn ex2() {
        test(&[2, 1, 2], 2, -1)
    }

    #[test]
    fn ex3() {
        test(&[9, 7, 5, 3], 1, 4)
    }

    #[test]
    fn discussion_case1() {
        test(&[2, 4, 4, 5, 4, 5, 3, 3, 3], 2, 3)
    }
}
