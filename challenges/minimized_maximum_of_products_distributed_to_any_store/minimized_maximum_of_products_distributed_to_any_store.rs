// https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/

pub struct Solution;

impl Solution {
    pub fn minimized_maximum(n: i32, mut quantities: Vec<i32>) -> i32 {
        fn is_valid(mut n: i32, quantities: &[i32], max: i32) -> bool {
            let mut i = 0;
            while n >= 0 && i < quantities.len() {
                let q = quantities[i];
                n -= (q / max) + (q % max > 0) as i32;
                i += 1;
            }
            n >= 0
        }
        quantities.sort_unstable();
        let mut left = quantities.iter().sum::<i32>() / n;
        let mut right = *quantities.iter().max().unwrap();
        while left < right {
            let mid = left + (right - left) / 2;
            if mid > 0 && is_valid(n, &quantities, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, quantities: &[i32], expected: i32) {
        assert!(n >= 1);
        assert!(n <= 100_000);
        assert!(quantities.len() >= 1);
        assert!(quantities.len() <= 100_000);
        for &q in quantities {
            assert!(q >= 1);
            assert!(q <= 100_000);
        }
        assert_eq!(
            Solution::minimized_maximum(n, quantities.to_vec()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(6, &[11, 6], 3)
    }

    #[test]
    fn ex2() {
        test(7, &[15, 10, 10], 5)
    }

    #[test]
    fn ex3() {
        test(1, &[100_000], 100_000)
    }

    #[test]
    fn discussion_case1() {
        test(2, &[5, 8], 8)
    }

    #[test]
    fn discussion_case2() {
        test(100000, &[1], 1)
    }

    #[test]
    fn discussion_case3() {
        test(15, &[18, 28, 11, 8, 22, 16, 24, 18, 26, 26, 21, 24], 24)
    }

    #[test]
    fn discussion_case4() {
        test(
            100000,
            &[
                4, 5, 4, 2, 1, 1, 4, 5, 2, 5, 3, 1, 2, 5, 2, 4, 2, 2, 2, 3, 1, 4, 1, 3, 3,
            ],
            1,
        )
    }

    #[test]
    fn discussion_case5() {
        test(
            100000,
            &[
                1, 5, 4, 5, 4, 1, 1, 2, 2, 4, 1, 1, 4, 5, 3, 3, 4, 1, 4, 4, 4, 2, 4, 2, 4,
            ],
            1,
        )
    }

    #[test]
    fn discussion_case6() {
        test(
            8,
            &[
                100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000,
            ],
            100_000,
        )
    }

    #[test]
    fn discussion_case7() {
        test(
            100000,
            &[
                100000, 100000, 100000, 100000, 100000, 100000, 100000, 100000,
            ],
            8,
        )
    }
}
