// https://leetcode.com/problems/mean-of-array-after-removing-some-elements/

pub struct Solution;

impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        assert!(arr.len() % 20 == 0);
        // Partition the lower 5% and upper 5% of the array.
        let one_twentieth = arr.len() / 20;
        let (_, _, rest) = arr.select_nth_unstable(one_twentieth - 1);
        let (rest, _, _) = rest.select_nth_unstable(rest.len() - one_twentieth);
        // Sum the remaining elements.
        rest.iter().sum::<i32>() as f64 / rest.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let arr = vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3];
        assert_eq!(Solution::trim_mean(arr), 2.00000);
    }

    #[test]
    fn ex2() {
        let arr = vec![6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0];
        assert_eq!(Solution::trim_mean(arr), 4.00000);
    }

    #[test]
    fn ex3() {
        let arr = vec![
            6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5, 10,
            8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4,
        ];
        assert!((Solution::trim_mean(arr) - 4.77778).abs() < 0.00001);
    }

    #[test]
    fn myex1() {
        let mut arr = vec![500; 1000];
        arr[900..1000].fill(0);
        arr[0..100].fill(10_000);
        assert_eq!(Solution::trim_mean(arr), 1000.0);
    }
}
