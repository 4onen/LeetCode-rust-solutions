// https://leetcode.com/problems/height-checker/

pub struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let expected = {
            let mut heights = heights.clone();
            heights.sort_unstable();
            heights
        };
        heights
            .iter()
            .zip(expected.iter())
            .filter(|(h, e)| h != e)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(heights: &[i32], expected: i32) {
        assert!(heights.len() >= 1);
        assert!(heights.len() <= 100);
        for &h in heights {
            assert!(h >= 1);
            assert!(h <= 100);
        }
        assert!(expected >= 0);
        assert!(expected <= 100);
        assert_eq!(Solution::height_checker(heights.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[1, 1, 4, 2, 1, 3], 3);
    }

    #[test]
    fn ex2() {
        test(&[5, 1, 2, 3, 4], 5);
    }

    #[test]
    fn ex3() {
        test(&[1, 2, 3, 4, 5], 0);
    }
}
