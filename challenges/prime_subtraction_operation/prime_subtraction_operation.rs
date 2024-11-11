// https://leetcode.com/problems/prime-subtraction-operation/

pub struct Solution;

impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        type Dtype = i16;
        let mut primes: Vec<Dtype> = vec![0, 2, 3, 5, 7, 11, 13];
        let mut next_num_to_check_prime: Dtype = 17;
        for i in 0..nums.len() {
            // Prime or zero below
            let prev = if i > 0 { nums[i - 1] } else { 0 };
            let tgt = (nums[i] - prev) as Dtype;
            if tgt <= 0 {
                return false;
            }
            'outer: while tgt > *primes.last().unwrap() {
                // Find the next largest prime
                // and append to our list.
                for &prime in &primes[2..] {
                    if next_num_to_check_prime % prime == 0 {
                        next_num_to_check_prime += 2;
                        continue 'outer;
                    }
                    if prime >= tgt / 4 {
                        // Check valid for prime checks > 17,
                        // faster than sqrt, and only incurrs
                        // an 8x perf overhead for number
                        // of iterations vs sqrt at y=1000.
                        break;
                    }
                }
                primes.push(next_num_to_check_prime);
                next_num_to_check_prime += 2;
            }
            let prime_idx = primes.partition_point(|&p| p < tgt);
            if prime_idx < 1 {
                dbg!(i, nums[i], prev, tgt, prime_idx, &primes);
            }
            nums[i] -= primes[prime_idx - 1] as i32;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: bool) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 1000);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 1000);
        }
        assert_eq!(Solution::prime_sub_operation(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[4, 9, 6, 10], true)
    }

    #[test]
    fn ex2() {
        test(&[6, 8, 11, 12], true)
    }

    #[test]
    fn ex3() {
        test(&[5, 8, 3], false)
    }

    #[test]
    fn discussion_case1() {
        test(&[918, 7], false)
    }

    #[test]
    fn discussion_case2() {
        test(&[998, 2], true)
    }

    #[test]
    fn discussion_case3() {
        test(&[17, 2], false)
    }

    #[test]
    fn discussion_case4() {
        test(&[11, 2, 10, 15, 19], false)
    }

    #[test]
    fn discussion_case5() {
        test(&[92, 9, 39, 11, 20, 13], false)
    }

    #[test]
    fn discussion_case6() {
        test(&[48, 2, 88, 78, 18, 71, 86, 92], true)
    }

    #[test]
    fn discussion_case7() {
        test(
            &[
                471, 275, 336, 615, 491, 568, 396, 892, 215, 823, 161, 176, 903, 472, 826, 602,
                216, 1000, 3, 751, 200, 842,
            ],
            false,
        )
    }

    #[test]
    fn discussion_case8() {
        test(&[1000; 169], true)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[1000; 1000], false)
    }

    #[test]
    fn failing_case1() {
        test(&[2, 2], false)
    }

    #[test]
    fn failing_case1_1() {
        test(&[3, 2], true)
    }
}
