// https://leetcode.com/problems/closest-prime-numbers-in-range/

pub struct Solution;

// Constant-time prime attempt (Too slow for LeetCode to compile)
// impl Solution {
//     pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
//         type Idx = u32;
//         type Num = i32;
//         const N_PRIMES: Idx = 78_498;
//         const MAX_NUM: Idx = 1_000_000;
//         const PRIMES: [Num; N_PRIMES as usize] = {
//             let predefined_primes = [2, 3, 5, 7, 11, 13];
//             let mut prime_write_head = 1 as Idx;
//             let mut primes = [2i32; N_PRIMES as usize];
//             let mut is_prime = [true; MAX_NUM as usize];
//             while prime_write_head < predefined_primes.len() as Idx {
//                 let prime = predefined_primes[prime_write_head as usize];
//                 primes[prime_write_head as usize] = prime;
//                 prime_write_head += 1;
//                 let prime_step = prime as Idx;
//                 let mut i = prime_step;
//                 while i < MAX_NUM {
//                     is_prime[i as usize] = false;
//                     i += prime_step;
//                 }
//             }
//             let mut i = primes[prime_write_head as usize - 1] as Idx + 2;
//             #[allow(long_running_const_eval)]
//             while i < MAX_NUM {
//                 if is_prime[i as usize] {
//                     primes[prime_write_head as usize] = i as i32;
//                     prime_write_head += 1;
//                     let prime_step = i as Idx;
//                     let mut j = prime_step;
//                     while j < MAX_NUM {
//                         is_prime[j as usize] = false;
//                         j += prime_step;
//                     }
//                 }
//                 i += 2;
//             }
//             primes
//         };
//         if right < 3 {
//             return vec![-1, -1];
//         }
//         // Inclusive indices of primes we want to check
//         let left_prime_index = PRIMES.binary_search(&left).unwrap_or_else(|idx| {
//             if PRIMES[idx] < left {
//                 idx + 1
//             } else {
//                 idx
//             }
//         });
//         let right_prime_index = PRIMES.binary_search(&right).unwrap_or_else(|idx| {
//             if PRIMES[idx] > right {
//                 idx - 1
//             } else {
//                 idx
//             }
//         });
//         PRIMES[left_prime_index..=right_prime_index]
//             .windows(2)
//             .map(|w| [w[0], w[1]])
//             .min_by_key(|w| w[1] - w[0])
//             .unwrap_or([-1, -1])
//             .to_vec()
//     }
// }

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        type Idx = u32;
        type Num = i32;
        fn is_prime(n: Num) -> bool {
            const N_PRIMES: Idx = 168;
            const MAX_PRECOMPUTE_NUM: Num = 1_000;
            const MAX_PRECOMPUTE_NUM_PLUS_1: Num = MAX_PRECOMPUTE_NUM + 1;
            const PRIMES: [Num; N_PRIMES as usize] = {
                let predefined_primes = [2, 3, 5, 7, 11, 13];
                let mut prime_write_head = 1 as Idx;
                let mut primes = [2i32; N_PRIMES as usize];
                let mut is_prime = [true; MAX_PRECOMPUTE_NUM as usize];
                while prime_write_head < predefined_primes.len() as Idx {
                    let prime = predefined_primes[prime_write_head as usize];
                    primes[prime_write_head as usize] = prime;
                    prime_write_head += 1;
                    let prime_step = prime as Idx;
                    let mut i = prime_step;
                    while i < MAX_PRECOMPUTE_NUM as Idx {
                        is_prime[i as usize] = false;
                        i += prime_step;
                    }
                }
                let mut i = primes[prime_write_head as usize - 1] as Idx + 2;
                #[allow(long_running_const_eval)]
                while i < MAX_PRECOMPUTE_NUM as Idx {
                    if is_prime[i as usize] {
                        primes[prime_write_head as usize] = i as i32;
                        prime_write_head += 1;
                        let prime_step = i as Idx;
                        let mut j = prime_step;
                        while j < MAX_PRECOMPUTE_NUM as Idx {
                            is_prime[j as usize] = false;
                            j += prime_step;
                        }
                    }
                    i += 2;
                }
                primes
            };
            match n {
                i32::MIN..=1 => false,
                2..=3 => true,
                4..=MAX_PRECOMPUTE_NUM => PRIMES.binary_search(&n).is_ok(),
                MAX_PRECOMPUTE_NUM_PLUS_1..=i32::MAX => {
                    if n & 1 == 0 {
                        return false;
                    }
                    let mut i = 3;
                    while i * i <= n {
                        if n % i == 0 {
                            return false;
                        }
                        i += 2
                    }
                    true
                }
            }
        }
        if right < 3 {
            return vec![-1, -1];
        }
        if left < 3 {
            // We know right is at least 3,
            // this condition proves left is
            // at least as small as 2,
            // so we can return [2,3]
            // because no other prime pair
            // can beat a diff of 1.
            return vec![2, 3];
        }
        let first_prime = {
            let mut left = left | 1;
            while !is_prime(left) && left < right {
                left += 2;
            }
            if left >= right {
                return vec![-1, -1];
            }
            left
        };
        let mut last_prime = first_prime;
        let mut min_distance = i32::MAX;
        let mut min_pair = [-1, -1];
        loop {
            let next_prime = {
                let mut next = last_prime + 2;
                while !is_prime(next) && next <= right {
                    next += 2;
                }
                if next > right {
                    break min_pair.to_vec();
                }
                next
            };
            if next_prime - last_prime < min_distance {
                min_distance = next_prime - last_prime;
                min_pair = [last_prime, next_prime];
            }
            last_prime = next_prime;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(left: u32, right: u32, expected: Option<[u32; 2]>) {
        assert!(left >= 1);
        assert!(left <= right);
        assert!(right <= 1_000_000);
        let result = Solution::closest_primes(left as i32, right as i32);
        if let Some([a, b]) = expected {
            assert!(left <= a);
            assert!(a < b);
            assert!(b <= right);
            assert_eq!(result, [a as i32, b as i32]);
        } else {
            assert_eq!(result, [-1, -1]);
        }
    }

    #[test]
    fn ex1() {
        test(10, 19, Some([11, 13]))
    }

    #[test]
    fn ex2() {
        test(4, 6, None)
    }

    #[test]
    fn discussion_case1() {
        test(1, 1, None)
    }

    #[test]
    fn discussion_case2() {
        test(1, 2, None)
    }

    #[test]
    fn discussion_case3() {
        test(3, 5, Some([3, 5]))
    }

    #[test]
    fn discussion_case4() {
        test(1, 100000, Some([2, 3]))
    }

    #[test]
    fn discussion_case5() {
        test(1087, 4441, Some([1091, 1093]))
    }

    #[test]
    fn discussion_case6() {
        test(19, 31, Some([29, 31]))
    }

    #[test]
    fn my_extreme_ex1() {
        test(1_000_000, 1_000_000, None)
    }

    #[test]
    fn my_extreme_ex2() {
        test(999_000, 1_000_000, Some([999329, 999331]))
    }
}
