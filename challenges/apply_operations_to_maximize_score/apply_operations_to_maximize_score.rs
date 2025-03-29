// https://leetcode.com/problems/apply-operations-to-maximize-score/

pub struct Solution;

// Naive sol'n (recompute sieve per number, compute all subarrays)
// impl Solution {
//     pub fn prime_score(num: i32) -> u16 {
//         // The prime score of a number is the prime omega of the number, i.e.
//         // the number of distinct prime factors. For example, the prime score
//         // of 12 is 2 (2 * 2 * 3), and the prime score of 18 is 2 (2 * 3 * 3).
//         // This function returns the prime score of a number. It uses the
//         // Sieve of Eratosthenes to find all prime numbers up to sqrt(num)
//         // skipping over 2 which is better handled by bit manipulation.
//         if num < 2 {
//             return 0;
//         }
//         let mut prime_score = (num & 1 == 0) as u16;
//         let oddity = num >> num.trailing_zeros() as u32;
//         if oddity == 1 {
//             return 1;
//         }
//         let mut sieve = vec![true; oddity as usize];
//         let mut p = 3;
//         while p <= oddity {
//             if sieve[(p - 3) as usize] {
//                 let mut i = p * p;
//                 while i <= oddity {
//                     sieve[(i - 3) as usize] = false;
//                     i += 2 * p;
//                 }
//                 if oddity % p == 0 {
//                     prime_score += 1;
//                 }
//             }
//             p += 2;
//         }
//         prime_score
//     }
//     pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
//         const MOD: u64 = 1_000_000_007;
//         let k = k as u32;
//         let prime_scores = nums.iter().map(|&num| Solution::prime_score(num)).collect::<Vec<u16>>();
//         let mut mults = std::collections::BinaryHeap::with_capacity(k as usize+1);
//         for i in 0..nums.len() {
//             let mut top_subarray_score = prime_scores[i as usize];
//             let mut top_subarray_index = i;
//             for j in i..nums.len() {
//                 if prime_scores[j as usize] > top_subarray_score {
//                     top_subarray_score = prime_scores[j as usize];
//                     top_subarray_index = j;
//                 }
//                 mults.push(std::cmp::Reverse(nums[top_subarray_index as usize] as u32));
//                 if mults.len() > k as usize {
//                     mults.pop();
//                 }
//             }
//         }
//         let mut score = 1;
//         while let Some(std::cmp::Reverse(mult)) = mults.pop() {
//             score = ((score as u64*mult as u64) % MOD) as u32;
//         }
//         score as i32
//     }
// }

// Optimized sol'n (precompute sieve, reuse for all numbers)
// (Barely faster than the naive sol'n)
// type SieveResult = Vec<bool>;
// impl Solution {
//     pub fn sieve(limit: i32) -> SieveResult {
//         // THIS SIEVE IMPL IGNORES EVEN NUMBERS
//         if limit <= 3 {
//             return vec![true; limit as usize];
//         }
//         let mut sieve = vec![true; limit as usize];
//         let mut p = 3;
//         while p <= limit {
//             if sieve[(p - 3) as usize] {
//                 let mut i = 3 * p;
//                 while i <= limit {
//                     sieve[(i - 3) as usize] = false;
//                     i += p;
//                 }
//             }
//             p += 2;
//         }
//         sieve
//     }
//     pub fn prime_score(num: i32) -> u16 {
//         let sieve = Solution::sieve(num >> num.trailing_zeros() as i32);
//         Solution::prime_score_with_sieve(num, &sieve)
//     }
//     pub fn prime_score_with_sieve(num: i32, sieve: &SieveResult) -> u16 {
//         if num < 2 {
//             return 0;
//         }
//         let mut prime_score = (num & 1 == 0) as u16;
//         let oddity = num >> num.trailing_zeros() as u32;
//         if oddity == 1 {
//             return 1;
//         }
//         let mut p = 3;
//         while p <= oddity {
//             if sieve[(p - 3) as usize] && oddity % p == 0 {
//                 prime_score += 1;
//             }
//             p += 2;
//         }
//         prime_score
//     }
//     pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
//         const MOD: u64 = 1_000_000_007;
//         let max_num = nums.iter().copied().max().unwrap();
//         let sieve = Solution::sieve(max_num);
//         let prime_scores = nums
//             .iter()
//             .map(|&num| Solution::prime_score_with_sieve(num, &sieve))
//             .collect::<Vec<u16>>();
//         let mut mults = std::collections::BinaryHeap::with_capacity(k as usize + 1);
//         for i in 0..nums.len() {
//             let mut top_subarray_score = prime_scores[i as usize];
//             let mut top_subarray_index = i;
//             for j in i..nums.len() {
//                 if prime_scores[j as usize] > top_subarray_score {
//                     top_subarray_score = prime_scores[j as usize];
//                     top_subarray_index = j;
//                 }
//                 mults.push(std::cmp::Reverse(nums[top_subarray_index as usize] as u32));
//                 if mults.len() > k as usize {
//                     mults.pop();
//                 }
//             }
//         }
//         let mut score = 1;
//         while let Some(std::cmp::Reverse(mult)) = mults.pop() {
//             score = ((score as u64 * mult as u64) % MOD) as u32;
//         }
//         score as i32
//     }
// }

// Optimized sol'n (heap on value indices, monotonic stack for subarrays)
// (Took too long to run)
// type SieveResult = Vec<bool>;
// impl Solution {
//     pub fn sieve(limit: i32) -> SieveResult {
//         // THIS SIEVE IMPL IGNORES EVEN NUMBERS
//         if limit <= 3 {
//             return vec![true; limit as usize];
//         }
//         let mut sieve = vec![true; limit as usize];
//         let mut p = 3;
//         while p <= limit {
//             if sieve[(p - 3) as usize] {
//                 let mut i = 3 * p;
//                 while i <= limit {
//                     sieve[(i - 3) as usize] = false;
//                     i += p;
//                 }
//             }
//             p += 2;
//         }
//         sieve
//     }
//     pub fn prime_score(num: i32) -> u16 {
//         let sieve = Solution::sieve(num >> num.trailing_zeros() as i32);
//         Solution::prime_score_with_sieve(num, &sieve)
//     }
//     pub fn prime_score_with_sieve(num: i32, sieve: &SieveResult) -> u16 {
//         if num < 2 {
//             return 0;
//         }
//         let mut prime_score = (num & 1 == 0) as u16;
//         let oddity = num >> num.trailing_zeros() as u32;
//         if oddity == 1 {
//             return 1;
//         }
//         let mut p = 3;
//         while p <= oddity {
//             if sieve[(p - 3) as usize] && oddity % p == 0 {
//                 prime_score += 1;
//             }
//             p += 2;
//         }
//         prime_score
//     }
//     pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
//         const MOD: u64 = 1_000_000_007;
//         let k = k as u32;
//         let max_num = nums.iter().copied().max().unwrap();
//         let sieve = Solution::sieve(max_num);
//         let prime_scores = nums
//             .iter()
//             .map(|&num| Solution::prime_score_with_sieve(num, &sieve))
//             .collect::<Vec<u16>>();
//         let mut left_boundaries = vec![-1; nums.len()];
//         let mut right_boundaries = vec![nums.len() as i32; nums.len()];
//         let mut monotonic_stack = Vec::with_capacity(nums.len());
//         for i in 0..prime_scores.len() as i32 {
//             while let Some(&j) = monotonic_stack.last() {
//                 if prime_scores[i as usize] > prime_scores[j as usize] {
//                     right_boundaries[j as usize] = i;
//                     monotonic_stack.pop();
//                 } else {
//                     left_boundaries[i as usize] = j;
//                     break;
//                 }
//             }
//             monotonic_stack.push(i);
//         }
//         let mut mults: std::collections::BinaryHeap<(i32, u32)> = nums
//             .into_iter()
//             .enumerate()
//             .map(|(i, num)| (num, i as u32))
//             .collect();
//         let mut k = k;
//         let mut score = 1;
//         while k > 0 {
//             let Some((num, i)) = mults.pop() else {
//                 break;
//             };
//             let mut possible = std::cmp::min(
//                 k,
//                 (right_boundaries[i as usize] - i as i32) as u32
//                     * (i as i32 - left_boundaries[i as usize]) as u32,
//             );
//             k -= possible;
//             while possible > 0 {
//                 score = ((score as u64 * num as u64) % MOD) as u32;
//                 possible -= 1;
//             }
//         }
//         score as i32
//     }
// }

// Optimized sol'n (exponential multiplication)
// (Still not fast enough; lots of slowdown in prime score calc)
// type SieveResult = Vec<bool>;
// impl Solution {
//     pub fn sieve(limit: i32) -> SieveResult {
//         // THIS SIEVE IMPL IGNORES EVEN NUMBERS
//         if limit <= 3 {
//             return vec![true; limit as usize];
//         }
//         let mut sieve = vec![true; limit as usize];
//         let mut p = 3;
//         while p <= limit {
//             if sieve[(p - 3) as usize] {
//                 let mut i = 3 * p;
//                 while i <= limit {
//                     sieve[(i - 3) as usize] = false;
//                     i += p;
//                 }
//             }
//             p += 2;
//         }
//         sieve
//     }
//     pub fn prime_score(num: i32) -> u16 {
//         let sieve = Solution::sieve(num >> num.trailing_zeros() as i32);
//         Solution::prime_score_with_sieve(num, &sieve)
//     }
//     pub fn prime_score_with_sieve(num: i32, sieve: &SieveResult) -> u16 {
//         if num < 2 {
//             return 0;
//         }
//         let mut prime_score = (num & 1 == 0) as u16;
//         let oddity = num >> num.trailing_zeros() as u32;
//         if oddity == 1 {
//             return 1;
//         }
//         let mut p = 3;
//         while p*p <= oddity {
//             if sieve[(p - 3) as usize] && oddity % p == 0 {
//                 prime_score += 1;
//             }
//             p += 2;
//         }
//         prime_score
//     }
//     pub const fn exponentiate(num: u64, mut exp: u64, mod_val: u64) -> u64 {
//         if exp < 2 {
//             return num;
//         }
//         let mut result = 1;
//         let mut base = num;
//         while exp > 0 {
//             if exp & 1 == 1 {
//                 result = (result as u64 * base as u64) % mod_val as u64;
//             }
//             base = (base as u64 * base as u64) % mod_val as u64;
//             exp >>= 1;
//         }
//         result
//     }
// 
//     pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
//         const MOD: u32 = 1_000_000_007;
//         let k = k as u32;
//         let max_num = nums.iter().copied().max().unwrap();
//         let sieve = Solution::sieve(max_num);
//         let prime_scores = nums
//             .iter()
//             .map(|&num| Solution::prime_score_with_sieve(num, &sieve))
//             .collect::<Vec<u16>>();
//         let mut left_boundaries = vec![-1; nums.len()];
//         let mut right_boundaries = vec![nums.len() as i32; nums.len()];
//         let mut monotonic_stack = Vec::with_capacity(nums.len());
//         for i in 0..prime_scores.len() as i32 {
//             while let Some(&j) = monotonic_stack.last() {
//                 if prime_scores[i as usize] > prime_scores[j as usize] {
//                     right_boundaries[j as usize] = i;
//                     monotonic_stack.pop();
//                 } else {
//                     left_boundaries[i as usize] = j;
//                     break;
//                 }
//             }
//             monotonic_stack.push(i);
//         }
//         let mut mults: std::collections::BinaryHeap<(i32, u32)> = nums
//             .into_iter()
//             .enumerate()
//             .map(|(i, num)| (num, i as u32))
//             .collect();
//         let mut k = k;
//         let mut score = 1;
//         while k > 0 {
//             let Some((num, i)) = mults.pop() else {
//                 break;
//             };
//             let possible = std::cmp::min(
//                 k,
//                 (right_boundaries[i as usize] - i as i32) as u32
//                     * (i as i32 - left_boundaries[i as usize]) as u32,
//             );
//             k -= possible;
//             // Use the exponential multiplication method
//             score = (score as u64 * Solution::exponentiate(num as u64, possible as u64, MOD as u64)
//                 % MOD as u64) as u32;
//         }
//         score as i32
//     }
// }

// Try making prime score calc stupid simple non-sieve (Works ridiculously well?)
impl Solution {
    pub const fn prime_score(num: i32) -> u16 {
        if num < 2 {
            return 0;
        }
        let mut prime_score = (num & 1 == 0) as u16;
        let mut oddity = num >> num.trailing_zeros() as u32;
        if oddity == 1 {
            return 1;
        }
        let mut p = 3;
        while p*p <= oddity {
            if oddity % p == 0 {
                prime_score += 1;
                while oddity % p == 0 {
                    oddity /= p;
                }
            }
            p += 2;
        }
        prime_score + (oddity > 1) as u16
    }
    pub const fn exponentiate(num: u64, mut exp: u64, mod_val: u64) -> u64 {
        if exp < 2 {
            return num;
        }
        let mut result = 1;
        let mut base = num;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result as u64 * base as u64) % mod_val as u64;
            }
            base = (base as u64 * base as u64) % mod_val as u64;
            exp >>= 1;
        }
        result
    }

    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: u32 = 1_000_000_007;
        let k = k as u32;
        let prime_scores = nums
            .iter()
            .map(|&num| Solution::prime_score(num))
            .collect::<Vec<u16>>();
        let mut left_boundaries = vec![-1; nums.len()];
        let mut right_boundaries = vec![nums.len() as i32; nums.len()];
        let mut monotonic_stack = Vec::with_capacity(nums.len());
        for i in 0..prime_scores.len() as i32 {
            while let Some(&j) = monotonic_stack.last() {
                if prime_scores[i as usize] > prime_scores[j as usize] {
                    right_boundaries[j as usize] = i;
                    monotonic_stack.pop();
                } else {
                    left_boundaries[i as usize] = j;
                    break;
                }
            }
            monotonic_stack.push(i);
        }
        let mut mults: std::collections::BinaryHeap<(i32, u32)> = nums
            .into_iter()
            .enumerate()
            .map(|(i, num)| (num, i as u32))
            .collect();
        let mut k = k;
        let mut score = 1;
        while k > 0 {
            let Some((num, i)) = mults.pop() else {
                break;
            };
            let possible = std::cmp::min(
                k,
                (right_boundaries[i as usize] - i as i32) as u32
                    * (i as i32 - left_boundaries[i as usize]) as u32,
            );
            k -= possible;
            // Use the exponential multiplication method
            score = (score as u64 * Solution::exponentiate(num as u64, possible as u64, MOD as u64)
                % MOD as u64) as u32;
        }
        score as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ps(num: i32, expected: u16) {
        assert_eq!(
            Solution::prime_score(num),
            expected,
            "prime_score({}) != {}",
            num,
            expected
        );
    }

    #[test]
    fn ps1() {
        // Prime numbers list: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
        // 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107.
        let tests = [
            (1, 0),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 2),
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 2),
            (11, 1),
            (12, 2),
            (13, 1),
            (14, 2),
            (15, 2),
            (16, 1),
            (17, 1),
            (18, 2),
            (19, 1),
            (20, 2),
            (21, 2),
            (22, 2),
            (23, 1),
            (24, 2),
            (25, 1),
            (26, 2),
            (27, 1),
            (28, 2),
            (29, 1),
            (30, 3),
            (31, 1),
            (32, 1),
            (33, 2),
            (34, 2),
            (35, 2),
            (36, 2),
            (37, 1),
            (38, 2),
            (39, 2),
            (40, 2),
            (41, 1),
            (42, 3),
            (43, 1),
            (44, 2),
            (45, 2),
            (46, 2),
            (47, 1),
            (48, 2),
            (49, 1),
            (50, 2),
            (51, 2),
            (52, 2),
            (53, 1),
            // Just primes from here up
            (59, 1),
            (61, 1),
            (67, 1),
            (71, 1),
            (73, 1),
            (79, 1),
            (83, 1),
            (89, 1),
            (97, 1),
            (101, 1),
            (103, 1),
            (107, 1),
        ];
        for (num, expected) in tests {
            test_ps(num, expected);
        }
    }

    #[test]
    fn ps10s() {
        let tests = [
            (1, 0),
            (10, 2),
            (100, 2),
            (1_000, 2),
            (10_000, 2),
            (100_000, 2),
        ];
        for (num, expected) in tests {
            test_ps(num, expected);
        }
    }

    // #[test]
    // fn ps100_000() {
    //     let tests = &[(100_000, 2); 100_000];
    //     let sieve = Solution::sieve(tests[0].0);
    //     for &(num, expected) in tests {
    //         // test_ps(num, expected);
    //         assert_eq!(
    //             Solution::prime_score_with_sieve(num, &sieve),
    //             expected,
    //             "prime_score({}) != {}",
    //             num,
    //             expected
    //         );
    //     }
    // }

    fn test(nums: &[u32], k: u32, expected: u32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100_000);
        assert!(k >= 1);
        assert!(k <= 1_000_000_000);
        let possible_subarrays = (nums.len() * (nums.len() + 1) / 2) as usize;
        assert!(
            k as usize <= possible_subarrays,
            "Only {} subarrays are possible, but {} were requested",
            possible_subarrays,
            k
        );
        assert!(expected <= 1_000_000_007);
        let result = Solution::maximum_score(nums.iter().map(|&x| x as i32).collect(), k as i32);
        assert!(result >= 0);
        assert!(result <= 1_000_000_007);
        assert_eq!(result, expected as i32);
    }

    #[test]
    fn ex1() {
        test(&[8, 3, 9, 3, 8], 2, 81)
    }

    #[test]
    fn ex2() {
        test(&[19, 12, 14, 6, 10, 18], 3, 4788)
    }

    #[test]
    fn discussion_case0() {
        test(&[1], 1, 1);
    }

    #[test]
    fn discussion_case_one() {
        test(&[19, 12, 14, 6, 10, 18], 3, 4788)
    }

    #[test]
    fn discussion_case_eno() {
        test(&[18, 10, 6, 14, 12, 19], 3, 6156)
    }

    #[test]
    fn discussion_case2() {
        // Prime score of 1 is 0
        test(&[1, 7, 11, 1, 5], 14, 823751938)
    }

    #[test]
    fn discussion_case3() {
        test(&[3289, 2832, 14858, 22011], 6, 256720975)
    }

    #[test]
    fn discussion_case4() {
        test(&[6, 1, 13, 10, 1, 17, 6], 27, 630596200)
    }

    #[test]
    fn my_extreme_ex10() {
        test(&[100_000; 10], 10, 319300014)
    }

    #[test]
    fn my_extreme_ex100() {
        test(&[100_000; 100], 100, 587199608)
    }

    #[test]
    fn my_extreme_ex1_000() {
        test(&[100_000; 1_000], 1_000, 456683348)
    }

    #[test]
    fn my_extreme_ex10_000() {
        test(&[100_000; 10_000], 10_000, 411081351)
    }

    #[test]
    fn my_extreme_ex100_000() {
        test(&[100_000; 100_000], 100_000, 513479976)
    }

    #[test]
    fn my_extreme_ex100_000_1nomod() {
        let mut input = vec![1; 100_000];
        for i in 0..input.len() {
            input[i] = i as u32 + 1;
        }
        test(&input, 1_000_000_000, 560184304)
    }

    #[test]
    fn my_extreme_ex100_000_1mod10() {
        let mut input = vec![1; 100_000];
        for i in 0..input.len() {
            input[i] = (i as u32 + 1) % 10;
        }
        test(&input, 1_000_000_000, 516200418)
    }

    #[test]
    fn my_extreme_ex100_000_1mod1_000() {
        let mut input = vec![1; 100_000];
        for i in 0..input.len() {
            input[i] = (i as u32 + 1) % 1_000;
        }
        test(&input, 1_000_000_000, 728446350)
    }

    #[test]
    fn my_extreme_ex100_000_1mod10_000() {
        let mut input = vec![1; 100_000];
        for i in 0..input.len() {
            input[i] = (i as u32 + 1) % 10_000;
        }
        test(&input, 1_000_000_000, 511779289)
    }

    #[test]
    fn tle_case_1() {
        test(
            &[
                331, 51480, 10715, 1, 92013, 65506, 95369, 66990, 4590, 53130, 74071, 18826, 1, 1,
                56994, 67390, 15456, 76597, 1, 1, 33507, 9235, 26044, 94295, 36804, 1, 62790,
                85425, 62790, 260, 65096, 83089, 21535, 31796, 80652, 14711, 15830, 92820, 48885,
                68628, 19320, 67830, 64770, 92820, 67714, 48326, 92493, 1, 72823, 75283, 1, 84546,
                6871, 56402, 95359, 66990, 98826, 98670, 46620, 38583, 1, 40097, 87711, 90836,
                57338, 50320, 61469, 36993, 53512, 1, 1993, 75530, 47190, 66264, 30268, 98227,
                50689, 1, 75670, 81498, 1, 47186, 78660, 1, 84630, 19965, 91220, 1, 86689, 98406,
                51388, 76338, 36337, 18099, 1, 84252, 39714, 32573, 90045, 42185, 1, 91433, 1, 1,
                85224, 96190, 40123, 43395, 46410, 78540, 1, 43554, 17652, 47422, 16786, 71743,
                83265, 15664, 13402, 5838, 72105, 41695, 43089, 1, 57498, 68844, 20040, 72105,
                62220, 62790, 50389, 92602, 78294, 23380, 32072, 99331, 1, 1, 11766, 55316, 1,
                19889, 63588, 79678, 60060, 79170, 90090, 2311, 99527, 1, 54367, 39786, 53130,
                66470, 69559, 19890, 26910, 78812, 32970, 82110, 63910, 43890, 75157, 1, 83720,
                75980, 56191, 91770, 46662, 46530, 69020, 5071, 1, 22620, 91458, 27180, 96348,
                98670, 78540, 41538, 14535, 53471, 67303, 56977, 1, 82110, 38441, 91560, 36984,
                19609, 6996, 94710, 1, 95906, 1, 6914, 78540, 1, 81510, 82110, 53130, 66919, 90023,
                64941, 81001, 33976, 50502, 75347, 30030, 76223, 64636, 1, 4410, 25476, 99330,
                98355, 46410, 1, 17556, 17387, 96141, 34216, 79170, 83490, 79730, 99330, 39270,
                74518, 44310, 89468, 57018, 1, 81998, 1, 36120, 39060, 60378, 34604, 71190, 85470,
                1, 23718, 79170, 1, 12090, 1, 67830, 71610, 1, 78815, 1, 87589, 1, 63070, 84630,
                75740, 43554, 1, 76738, 30119, 1, 72031, 90090, 1, 74671, 75020, 79170, 50986,
                43032, 91950, 79068, 38874, 81060, 12829, 16971, 56406, 15990, 3510, 81774, 72197,
                67830, 25573, 1, 99955, 1, 1, 1, 1, 1, 53130, 1, 1, 3603, 18220, 87690, 67536,
                60191, 82797, 98670, 1, 87705, 64717, 1, 45750, 68897, 24780, 1, 51870, 35389,
                38360, 75790, 1, 81510, 43460, 87934, 16035, 66292, 8050, 18346, 92820, 55139,
                55977, 65416, 85028, 44415, 87582, 1, 1, 1, 17663, 63968, 92761, 66990, 79170, 1,
                91770, 1, 59663, 53823, 74949, 17188, 92923, 93815, 1, 98670, 61468, 94710, 44801,
                44898, 1, 58086, 66990, 66113, 1, 6127, 92579, 66958, 33096, 1, 63574, 33705,
                90090, 10111, 37070, 92820, 42900, 65497, 64280, 11970, 72410, 78991, 32604, 85470,
                1, 31122, 21543, 31395, 52757, 37157, 51170, 67452, 66990, 38154, 73709, 46177,
                67298, 60841, 94710, 51870, 62787, 27241, 71610, 41869, 1, 16330, 95532, 18714, 1,
                9057, 1, 44574, 52355, 39424, 21767, 1, 3230, 73661, 29347, 90706, 29116, 1, 39008,
                1, 81592, 97005, 67830, 8616, 92560, 90090, 69006, 35028, 42090, 1, 57652, 60483,
                32377, 59262, 69568, 4075, 16871, 55796, 3643, 1, 13090, 1, 10337, 25878, 78468,
                42075, 84630, 74248, 45388, 46410, 28047, 9442, 60342, 1, 1, 46410, 1, 1, 76712, 1,
                50405, 45906, 85470, 45671, 93943, 94651, 55965, 80220, 1, 51679, 60060, 33983, 1,
                42788, 68266, 22330, 16166, 43650, 70980, 81246, 1, 7722, 92993, 1, 56166, 98662,
                39468, 38582, 91770, 55132, 9150, 25530, 71210, 46199, 28920, 28065, 61894, 46410,
                70117, 76618, 73776, 1, 39270, 96488, 71610, 313, 54530, 1, 1, 63859, 1, 37790, 1,
                65862, 39600, 42874, 37633, 1, 39270, 93032, 63146, 71655, 1, 92834, 98670, 12249,
                87780, 24229, 79134, 23010, 49290, 45958, 90060, 72105, 89667, 51988, 91316, 20171,
                1, 26323, 81510, 95416, 1, 53130, 46206, 1, 99330, 1, 94827, 53613, 40413, 60060,
                53130, 28492, 30668, 3238, 79571, 85690, 30210, 1, 87706, 69480, 59166, 68178, 1,
                87780, 40939, 20660, 85260, 52374, 86070, 1, 68257, 64769, 84630, 61001, 26844,
                6636, 1, 81435, 66990, 39270, 72930, 15964, 60763, 68538, 59141, 1, 79641, 30030,
                77215, 1, 1, 1, 25662, 1, 61989, 45100, 1, 70680, 1, 76905, 62205, 71610, 71754,
                90307, 48888, 62894, 17327, 79516, 80857, 1, 1, 69174, 70525, 26270, 53130, 57018,
                76380, 2632, 1, 44520, 35462, 14957, 29410, 4279, 1, 8453, 2636, 18995, 52374,
                5242, 21233, 34467, 1, 80730, 1, 1, 26371, 42218, 37871, 49019, 16275, 87780,
                70070, 78280, 47563, 66612, 41050, 2262, 7590, 79752, 1, 66990, 94146, 72930, 1,
                41778, 22245, 1, 87780, 73290, 66916, 91047, 14190, 9231, 60060, 72732, 69695,
                86150, 53074, 31507, 96768, 32119, 52512, 1876, 94253, 1, 70350, 1, 99712, 95430,
                83022, 61809, 1, 74934, 28974, 32470, 91626, 12918, 98670, 60354, 15084, 53569, 1,
                1, 52745, 24258, 46601, 30497, 92820, 70455, 21875, 94409, 27748, 94710, 43890, 1,
                98430, 72930, 91455, 1, 94435, 6910, 21090, 1, 12683, 46479, 6238, 81180, 82110,
                42282, 89546, 85571, 30030, 23197, 64153, 48361, 39270, 50581, 1, 30660, 75884,
                75276, 62244, 58839, 98670, 81534, 55139, 62790, 1, 30030, 47058, 93099, 61488,
                19203, 87780, 62041, 53634, 49830, 84606, 27104, 86936, 36373, 90677, 4371, 95940,
                68370, 1, 20344, 5827, 50671, 53657, 23209, 40404, 1789, 1, 44331, 44962, 79461,
                76249, 12741, 80643, 23528, 44171, 1, 51826, 1, 94954, 30214, 99529, 1, 40387,
                32340, 182, 98670, 61631, 84112, 85470, 72930, 87290, 14044, 33655, 1, 91770,
                53130, 62790, 27042, 85794, 43890, 33528, 39911, 83370, 63331, 1, 22111, 46126,
                76099, 72013, 58676, 93563, 95131, 7097, 28225, 91770, 92820, 99762, 64006, 92820,
                14114, 88920, 64439, 40031, 54775, 1, 25212, 48188, 31249, 66576, 96558, 1, 82110,
                15071, 92744, 85561, 95552, 1, 55195, 1, 1, 74293, 98628, 1, 19582, 60060, 1,
                11831, 76070, 50820, 79695, 19001, 45264, 53125, 16614, 62471, 1, 86001, 59482,
                83086, 70610, 43890, 34017, 1, 81510, 4211, 79004, 66495, 61778, 40754, 1, 91542,
                1, 46410, 30601, 34899, 90090, 72660, 37710, 97738, 93621, 1115, 74630, 2267,
                35340, 5485, 81900, 1, 1, 45474, 64333, 62790, 84546, 65476, 78540, 94781, 77840,
                1, 23379, 76284, 75709, 85469, 72990, 70327, 18770, 98841, 78109, 93380, 26427,
                92820, 82110, 80850, 49170, 99330, 1025, 69855, 1, 35729, 12090, 74028, 96587,
                96262, 89110, 1, 89524, 71168, 46767, 1, 11481, 39070, 59513, 33829, 91770, 71289,
                74613, 60380, 46410, 91770, 1, 67830, 62790, 82488, 15708, 41003, 92208, 84630,
                76103, 73140, 83622, 14083, 72534, 40394, 1, 4254, 65880, 42294, 65356, 1, 1,
                25090, 99330, 60373, 74709, 1, 91335, 46410, 26950, 94273, 85333, 27988, 24513,
                76801, 48930, 84991, 33899, 58611, 58383, 64730, 30030, 40559, 45711, 72930, 15860,
                74370, 55146, 72930, 1, 2299, 68969, 76893, 7714, 80031, 87405, 1, 89760, 35831,
                15555, 41845, 11680, 50208, 94710, 45377, 46046, 80317, 1, 67830, 78921, 1, 94710,
                53196, 11326, 37674, 67830, 92820, 7457, 11934, 43890, 89684, 1, 87702, 60060, 1,
                59058, 33110, 46009, 87909, 1, 86243, 40227, 49351, 60060, 79170, 62790, 88970,
                40248, 47865, 59184, 90867, 66990, 55770, 67501, 90090, 1, 31920, 1, 10910, 20280,
                85470, 1, 71349, 1091, 938, 19530, 95319, 1, 94710, 74460, 99330, 77227, 11073,
                49233, 73291, 1, 22585, 84630, 1, 76350, 84191, 30181, 60060, 46610, 44785, 71610,
                8431, 1, 77430, 61194, 41314, 22961, 75390, 1, 33023, 1, 60214, 3828, 60060, 92460,
                1, 33384, 25439, 32509, 82110, 23712, 82110, 1, 66674, 1, 73052, 57720, 62842,
                37312, 39905, 45195, 55752, 91080, 1, 3845, 49896, 89191, 59259, 1, 23365, 87570,
                52902, 20763, 40110, 79926, 30030, 1, 62790, 10865, 39270, 62699, 46410, 61660,
                95858, 79170, 29610, 1, 44130, 94869, 66990, 3909, 38238, 90090, 26895, 93716, 1,
                1, 10515, 72930, 9798, 46410, 88610, 91770, 43890, 94710, 28270, 65605, 53130,
                55201, 92650, 84630, 4219, 42262, 30156, 72534, 63336, 46872, 89615, 1, 62987,
                98670, 1, 66828, 1, 62533, 80271, 33767, 1, 14980, 52951, 70686, 34261, 73920, 1,
                1, 58337, 95770, 90887, 74370, 29546, 84201, 88700, 46410, 31403, 33911, 20050,
                27791, 21177, 92430, 65406, 72930, 84748, 95865, 79657, 81510, 98580, 22655, 15510,
                57273, 59708, 38514, 71045, 16997, 53130, 1, 60575, 3001, 1, 88749, 42082, 4987,
                5003, 44220, 35310, 70914, 46530, 18330, 1, 22615, 65033, 30030, 17927, 99637,
                42966, 6744, 72116, 83160, 1, 7927, 54786, 76681, 1, 4416, 79170, 7328, 66091,
                71610, 84829, 84102, 16939, 96330, 95460, 9894, 34554, 78540, 58379, 2820, 99131,
                79374, 53130, 84944, 30492, 1, 39270, 46793, 32957, 87371, 71071, 42321, 1, 89154,
                1, 8637, 52470, 45214, 59351, 98516, 78260, 90706, 92820, 84490, 53421, 86415,
                39270, 99929, 53130, 69149, 19265, 39727, 1, 26828, 17922, 46967, 59645, 53130,
                99330, 99330, 89982, 12655, 1, 1, 97290, 44390, 427, 53130, 8513, 85147, 53130,
                49812, 24, 85470, 1, 53728, 33961, 1, 54740, 98277, 99254, 14323, 86020, 99330,
                83969, 61050, 62790, 95460, 76632, 75411, 76160, 94986, 70392, 91770, 40722, 1,
                92820, 82110, 47963, 1, 59430, 80703, 80892, 33770, 60970, 72930, 37171, 99330,
                85754, 68432, 41895, 1553, 42978, 36190, 44742, 43990, 99960, 1, 89430, 19553,
                60060, 761, 81354, 70810, 91770, 38693, 1, 67724, 23029, 51870, 50, 46410, 62961,
                97157, 78540, 1, 22369, 23454, 60060, 64170, 93030, 1, 1, 91770, 61381, 88296,
                51870, 19621, 3690, 12491, 99563, 9221, 24749, 34713, 1, 85422, 593, 33169, 60060,
                79170, 41893, 72870, 5743, 1, 33863, 16852, 39159, 87780, 1, 18565, 80663, 78302,
                1, 1, 68037, 1, 74760, 90040, 31391, 90741, 53130, 78540, 52149, 40145, 43800,
                12109, 70121, 8629, 92820, 69495, 66571, 35561, 3705, 1, 1, 1, 1, 37013, 71610, 1,
                7210, 98488, 1, 28549, 13723, 44687, 82837, 1, 72419, 44113, 83980, 70969, 65076,
                26790, 81026, 68186, 81510, 63570, 17138, 79170, 1, 54103, 45319, 26959, 75387,
                41580, 62790, 21133, 87780, 8614, 65254, 67297, 37863, 20991, 2125, 82110, 17160,
                1, 72520, 17826, 48358, 66990, 2189, 48393, 1, 91341, 87780, 66660, 71610, 67830,
                55064, 33113, 36188, 83049, 9661, 85470, 87780, 27720, 5191, 55068, 34356, 51414,
                56704, 79275, 1, 83639, 1511, 7590, 64946, 59797, 88807, 56406, 43680, 39270, 6166,
                319, 97257, 29247, 92808, 60060, 49062, 19158, 59607, 98507, 59209, 96460, 50584,
                63618, 32945, 80070, 99330, 47370, 68409, 48570, 1, 60060, 1, 70095, 1, 1, 79310,
                84805, 14070, 53130, 59671, 44730, 85386, 77588, 60690, 97054, 46410, 46664, 5460,
                24167, 3241, 95151, 91770, 87780, 83622, 60060, 18308, 1, 48025, 43425, 32099,
                68880, 43035, 30630, 4670, 39060, 38010, 1, 20970, 94710, 73710, 93567, 7389,
                78540, 91770, 81510, 15791, 1997, 12038, 31692, 18282, 53130, 42644, 1, 94600, 1,
                69169, 81510, 39270, 29525, 87780, 43793, 86265, 12455, 18537, 1, 89261, 3301, 1,
                21450, 42198, 65843, 98670, 61226, 93401, 39270, 11732, 1, 55667, 72852, 53130,
                34086, 18367, 1, 76695, 31080, 67932, 99745, 13232, 1, 15990, 1, 32993, 50085, 1,
                25788, 35838, 86428, 10010, 1, 46410, 23247, 1, 86580, 23801, 6097, 13110, 1,
                62790, 75514, 35201, 65142, 53130, 33468, 76699, 802, 84183, 68059, 63440, 62790,
                74106, 70455, 1, 51310, 76591, 66990, 91770, 1, 67522, 29006, 1, 11621, 64699,
                39270, 79687, 65450, 66990, 69143, 81366, 38384, 13012, 41507, 70286, 84315, 25179,
                51870, 1, 30030, 87610, 95479, 50830, 56210, 49842, 70490, 41036, 1, 1, 72930, 1,
                1, 30030, 81795, 62790, 91910, 72508, 6006, 50694, 84870, 90856, 91770, 45465,
                82110, 26993, 6353, 2210, 77816, 88903, 1, 29370, 19469, 92820, 82110, 58406, 1,
                20963, 44691, 59113, 1, 1, 39711, 67830, 87906, 63527, 1, 63810, 48555, 84509,
                76206, 90390, 1, 39270, 84630, 46411, 45966, 94400, 17031, 18841, 40056, 59890,
                10212, 1, 6132, 1, 8377, 49893, 75289, 22737, 82643, 1, 85470, 14702, 43547, 27060,
                1, 35233, 77700, 18002, 40194, 88259, 1, 61484, 78995, 51870, 1, 36441, 82485,
                67993, 39208, 35323, 2635, 85470, 33490, 26645, 3217, 66990, 98547, 69592, 17004,
                91141, 56527, 84630, 2170, 46229, 45564, 85870, 14997, 45218, 1, 67523, 1, 74123,
                71114, 1, 93058, 50409, 37947, 44207, 43615, 10230, 88338, 1, 91770, 45045, 86901,
                38556, 1, 43890, 43830, 21347, 91806, 92742, 73830, 79451, 72114, 62270, 32148,
                51875, 37039, 1, 62790, 35341, 82110, 94710, 43890, 1, 1, 1, 1, 96570, 56633, 96,
                33272, 1, 39140, 21488, 36582, 66675, 71610, 43890, 50287, 9211, 97021, 34403,
                60647, 75735, 50672, 1, 1, 62790, 39404, 31267, 1, 43890, 15700, 28123, 43615,
                46991, 74474, 96018, 8385, 98670, 82971, 2346, 1, 5280, 84630, 28578, 67830, 91770,
                39041, 71492, 1, 54120, 96667, 7926, 7064, 87769, 20693, 28622, 3448, 1, 53106,
                60002, 44953, 74289, 32749, 1, 20153, 73130, 1, 82486, 59669, 87780, 65740, 17895,
                71610, 82940, 73886, 87780, 19236, 79465, 42060, 1, 68145, 54269, 47955, 52140,
                82110, 49807, 1, 90090, 56651, 86563, 1, 1, 83103, 1, 51870, 53130, 46410, 68571,
                1, 88098, 1, 9067, 74891, 67214, 94710, 56406, 57456, 1, 43466, 40292, 90600,
                54529, 1, 90090, 1, 32844, 43890, 69650, 58590, 1, 74426, 1, 29252, 89250, 1,
                22393, 61017, 1, 20315, 66674, 36609, 3393, 80917, 4262, 90804, 46410, 3900, 63769,
                76332, 49263, 1, 27531, 72930, 22481, 1, 56179, 83188, 24486, 62790, 75029, 89930,
                33173, 53709, 75191, 29485, 89499, 24640, 45923, 94794, 2273, 43890, 1, 1, 84930,
                1, 67219, 82830, 1, 83850, 1, 2873, 27749, 49590, 43568, 20183, 1, 56486, 18424,
                406, 56709, 1, 13228, 35700, 63756, 78540, 45635, 1, 1, 53447, 63577, 1, 68996,
                79170, 66780, 83559, 29992, 53429, 21575, 19110, 1, 47730, 87780, 36341, 96600,
                43780, 82264, 90090, 73140, 97577, 73361, 57770, 31347, 54180, 89119, 67913, 1,
                99330, 15828, 4099, 62328, 23997, 8678, 1, 37872, 31467, 58485, 59220, 6061, 41465,
                1, 67830, 68145, 15473, 78161, 95700, 76284, 69154, 1, 57940, 84873, 42323, 1,
                16897, 94306, 39094, 92880, 1, 64553, 30977, 53130, 69327, 19759, 54151, 1629,
                36811, 91517, 38851, 52042, 71610, 83201, 22069, 39231, 54138, 85852, 99359, 31489,
                58640, 58495, 25619, 43890, 55043, 74220, 45566, 10650, 76341, 90090, 6441, 73790,
                8792, 77280, 54180, 65809, 57057, 71370, 51222, 94913, 37091, 57992, 24646, 49882,
                1, 59594, 1, 87668, 44730, 51870, 98720, 19758, 31541, 40020, 94962, 1, 20408,
                54264, 58200, 8043, 94444, 71610, 74202, 18980, 90090, 55390, 31891, 1, 53453,
                87780, 72726, 1, 66990, 37635, 14630, 90568, 56116, 12911, 89916, 61490, 72869,
                54948, 68060, 21147, 67933, 12012, 77550, 82264, 53624, 34615, 1, 51240, 91801,
                82110, 72930, 89096, 53130, 83226, 73907, 2258, 1, 91770, 1, 61065, 80509, 83380,
                1, 1, 90831, 61948, 2849, 64554, 71610, 30030, 30030, 1, 65688, 62054, 889, 36599,
                1, 1, 48100, 95962, 74357, 45466, 35587, 40883, 1, 7993, 1, 53913, 86868, 91770,
                57172, 90090, 54422, 30200, 97955, 81449, 19443, 36809, 68034, 94015, 39270, 99462,
                1, 91770, 9787, 1, 29225, 81888, 92644, 76362, 57822, 62632, 67795, 26067, 43890,
                18884, 1, 86649, 30291, 35098, 84630, 76106, 76570, 1, 59210, 21252, 1, 45491,
                45697, 79170, 40314, 15299, 44034, 68234, 75400, 93521, 1, 1, 55290, 87780, 1,
                47970, 72594, 1, 30307, 1, 51875, 66705, 71305, 24631, 83838, 449, 71498, 75593,
                29040, 14761, 56720, 36660, 72450, 33987, 9187, 17589, 94770, 66487, 85429, 29210,
                56623, 26670, 6299, 51398, 71233, 27065, 77638, 86020, 66990, 99500, 64841, 13884,
                20202, 43752, 51599, 89586, 12628, 1, 49232, 65940, 60060, 1, 86057, 77377, 1,
                8818, 1, 51987, 59582, 98011, 18927, 450, 46410, 1, 20590, 53130, 38586, 17509,
                3344, 39371, 44963, 8887, 54319, 17883, 55083, 35742, 8139, 66990, 88762, 11717,
                75658, 35717, 70644, 84895, 1710, 3320, 96851, 45942, 46410, 61129, 26417, 31119,
                1, 1, 90090, 90885, 98136, 20625, 1, 90610, 78535, 86268, 42765, 36700, 50898, 1,
                14767, 77032, 55829, 80170, 1, 50808, 1, 1, 48032, 22260, 81510, 1, 71610, 46468,
                38543, 90713, 67830, 42821, 11588, 59067, 25119, 80184, 97170, 73479, 7140, 57150,
                60961, 1568, 38527, 1, 72390, 84322, 32964, 28770, 53719, 23370, 18611, 1, 88766,
                1, 63726, 51870, 41203, 26376, 29586, 46410, 1, 91770, 46221, 38542, 41479, 12634,
                78540, 41998, 42456, 1, 56965, 71610, 55502, 46620, 92235, 87197, 4472, 1, 75041,
                24337, 51913, 14979, 1, 85888, 73351, 29261, 34320, 47797, 75240, 50764, 61194,
                14867, 63030, 9291, 84126, 80776, 98396, 99330, 59737, 51870, 54980, 93501, 78810,
                28058, 94710, 18573, 1, 66990, 2113, 23666, 70177, 4580, 81331, 71610, 85470,
                19236, 31543, 84630, 15907, 53808, 29863, 47681, 31224, 1, 47145, 98190, 57839,
                9006, 42433, 46046, 83720, 74256, 87393, 1, 53634, 26565, 46904, 98210, 71563,
                18237, 45420, 52721, 1, 86364, 90090, 91289, 39270, 38766, 40722, 90306, 34191,
                85470, 63179, 56089, 1, 42814, 78540, 45150, 39270, 7161, 12152, 22594, 41171,
                81510, 18018, 64516, 94093, 33134, 59282, 81043, 53130, 32395, 36558, 92820, 30237,
                23650, 85173, 82745, 63294, 78950, 1, 20549, 51700, 41519, 50658, 98670, 13477, 1,
                50929, 71305, 19117, 56810, 78090, 32027, 1, 63420, 94352, 95262, 2, 16120, 64655,
                75012, 18312, 2044, 1, 82443, 65699, 21054, 93222, 44275, 53940, 1, 68730, 84420,
                69250, 74046, 1, 4123, 42837, 72930, 1, 1, 10122, 1, 81510, 75990, 25988, 16832,
                64310, 35598, 21143, 31884, 1, 85528, 81191, 16729, 74663, 99407, 1, 93467, 99498,
                1, 63383, 63360, 53130, 37758, 11113, 9629, 40612, 43890, 68908, 1, 1, 1, 13267,
                10277, 18942, 81294, 15075, 57713, 71247, 46655, 60407, 45944, 84695, 96058, 15233,
                66990, 81510, 92898, 39270, 30030, 67218, 89273, 28573, 47493, 52182, 41039, 64972,
                74287, 70224, 46410, 1, 43890, 53007, 94580, 48049, 85470, 1, 32797, 94710, 30517,
                1, 90298, 60690, 82894, 28600, 34993, 51529, 51870, 52845, 62790, 68172, 52104,
                59709, 46768, 81510, 81510, 39270, 86295, 76320, 90643, 1, 13488, 9570, 1, 82110,
                1, 91770, 82526, 34188, 94470, 491, 37914, 77666, 43890, 95940, 88983, 69600,
                74136, 66130, 79170, 91406, 8844, 14005, 34591, 7824, 75825, 89148, 17583, 53130,
                70843, 54985, 80535, 73864, 30741, 85691, 34180, 67672, 6768, 90090, 30030, 52156,
                51198, 58241, 52470, 14266, 74550, 42923, 59340, 1, 85819, 1, 46167, 25763, 1, 1,
                90480, 87151, 1, 6213, 60060, 26334, 62016, 930, 1, 1, 91770, 3931, 62790, 21759,
                74903, 47717, 26255, 71105, 1, 42002, 1, 59430, 71610, 89182, 1, 28449, 26842, 1,
                65650, 1, 1, 88638, 50312, 42017, 98670, 54713, 84630, 67830, 1, 95117, 416, 91770,
                10434, 1, 79170, 40939, 1, 69603, 54257, 88170, 99551, 90390, 50484, 31416, 85470,
                1, 95370, 1, 62790, 72534, 23874, 1, 97890, 81033, 87780, 1, 30030, 1, 72930,
                61295, 1, 35149, 2550, 26740, 11454, 43860, 69030, 52279, 16951, 16000, 1, 67830,
                81972, 63597, 1, 82604, 50140, 22530, 61838, 1, 23741, 84630, 53686, 44823, 41563,
                94512, 84167, 413, 6367, 13660, 68390, 43019, 1, 14614, 91740, 49008, 99190, 79025,
                43890, 1, 1, 53130, 96179, 1, 53130, 26059, 52272, 98929, 92970, 52143, 64362,
                67980, 25271, 1, 64460, 1, 38610, 68353, 70964, 40859, 71868, 16306, 53419, 95280,
                61258, 95602, 7922, 36678, 46410, 17347, 53822, 18959, 47681, 1, 58110, 72096, 1,
                5658, 99330, 69700, 25970, 1, 84630, 59892, 43541, 68799, 21977, 72755, 53130,
                59981, 25747, 1, 60091, 457, 81990, 56099, 39270, 86140, 89404, 15041, 1, 38794,
                62644, 1, 36290, 42373, 1, 1, 81510, 94544, 25445, 1, 81510, 79170, 58410, 92347,
                81847, 56106, 84630, 1, 77556, 1, 62497, 95191, 98670, 97328, 82110, 46841, 95238,
                95553, 82110, 6571, 1, 51240, 1, 49020, 9796, 52189, 79170, 33330, 40774, 56933,
                79170, 92488, 85429, 94021, 63751, 90480, 80934, 25530, 1, 1, 51870, 88242, 17118,
                99330, 20424, 20370, 1, 87780, 91804, 64792, 95363, 77380, 50540, 53508, 63558,
                62790, 98670, 67830, 66990, 91290, 47124, 1, 90090, 30030, 67442, 29041, 73738, 1,
                1, 69190, 1, 1, 1, 54510, 61979, 1, 43771, 43890, 25834, 9690, 26008, 63070, 68767,
                24407, 82290, 71610, 39234, 91770, 40141, 75149, 19950, 67148, 24889, 1, 11087,
                82076, 1, 1, 30091, 1, 1, 81291, 66990, 90233, 17467, 90821, 13384, 30030, 62790,
                51176, 20413, 92820, 95352, 23079, 61091, 75600, 90090, 82062, 12367, 71671, 60060,
                99330, 79172, 28119, 6821, 76472, 10624, 87087, 5776, 74191, 91790, 89166, 1,
                91426, 95532, 64740, 84047, 35135, 53591, 55575, 64125, 41051, 40810, 92442, 80430,
                12480, 36563, 69927, 43890, 81510, 1, 1, 62790, 51870, 30030, 33726, 27109, 69889,
                20403, 46410, 1, 94501, 1, 84630, 90090, 61886, 86814, 37451, 2025, 62790, 86147,
                69510, 59655, 69355, 81510, 1, 67830, 53130, 82110, 51870, 44730, 1, 27918, 4178,
                1, 97748, 42613, 62790, 49714, 77805, 1, 11628, 81510, 99330, 84411, 59110, 99595,
                93478, 92460, 17480, 75843, 1, 73907, 58445, 76278, 83399, 36158, 1, 74778, 73441,
                37567, 89880, 1, 36497, 76110, 67607, 65831, 6449, 1, 84971, 30030, 99330, 13020,
                69930, 45894, 42120, 84660, 86928, 71910, 1, 4159, 60610, 89142, 71054, 62730,
                44350, 88019, 128, 46395, 90481, 24354, 1, 54644, 34060, 9285, 46410, 25818, 99330,
                50455, 51396, 81070, 79379, 47005, 75144, 21420, 96360, 1013, 71610, 48935, 64410,
                1, 1, 84630, 1, 48553, 24652, 67153, 25842, 1154, 54095, 15239, 39904, 88816, 631,
                90630, 40213, 15642, 82110, 83299, 1, 65068, 4326, 92820, 41291, 13058, 83226,
                10768, 87780, 70512, 1, 1, 18964, 98670, 21856, 1, 1828, 66919, 47867, 30976,
                46569, 21641, 34428, 44173, 87780, 68997, 59570, 64744, 31362, 51339, 80790, 77502,
                79765, 21425, 23016, 82633, 14154, 99484, 51142, 27202, 45381, 36989, 13047, 1,
                43662, 51870, 29239, 62790, 52728, 30030, 7410, 44370, 78540, 68970, 34440, 76908,
                66428, 22565, 1, 1, 84588, 63884, 38504, 84390, 93853, 46860, 96149, 84630, 14630,
                99620, 1, 53466, 57807, 79170, 64554, 46150, 39270, 9997, 98795, 55998, 45234,
                24781, 82236, 4770, 68340, 87780, 91938, 1, 92689, 90090, 1, 82246, 45474, 82110,
                1, 89561, 29830, 1, 42406, 75990, 83513, 95340, 36442, 1, 70862, 1, 89760, 78697,
                1, 1, 35162, 89899, 17878, 72673, 76507, 1841, 99330, 69439, 81480, 18869, 97060,
                72778, 55965, 87780, 948, 7068, 85470, 67116, 38682, 94134, 81052, 1, 66990, 1,
                82500, 1, 50739, 11545, 33377, 13456, 35468, 72420, 23178, 76525, 1, 1, 61566,
                23520, 87780, 61640, 71162, 11592, 51129, 60088, 90090, 26194, 94710, 67830, 1,
                7573, 99556, 24496, 48685, 56420, 78259, 1117, 57815, 1, 1, 26947, 44315, 89870,
                41387, 53130, 37117, 7682, 25017, 30030, 76913, 69467, 1, 31769, 68813, 61390,
                70290, 1, 11896, 1, 97890, 59369, 17538, 1, 92472, 58196, 76975, 7294, 67830,
                44839, 1, 43186, 21277, 1, 1, 1, 62790, 84590, 59627, 96872, 13557, 1, 98175,
                87780, 84991, 79799, 1, 83158, 12279, 2100, 91873, 86590, 11910, 63570, 87961,
                69746, 78875, 99330, 84630, 67217, 66570, 66990, 1, 6006, 82446, 19565, 56442, 1,
                14831, 98670, 1, 58271, 11284, 18618, 66990, 63491, 62039, 1, 62790, 30030, 1,
                55931, 57190, 12739, 22179, 72930, 1, 59508, 18729, 64079, 94710, 18965, 9331,
                8551, 2775, 79241, 62790, 68487, 63671, 12289, 80340, 76597, 76362, 40282, 67130,
                12468, 71482, 71148, 60060, 27751, 54340, 1, 34277, 21879, 37230, 39270, 72660,
                2595, 45259, 55333, 44108, 62790, 91770, 81599, 1, 90090, 73320, 97790, 78540,
                67830, 90555, 93450, 1, 60830, 91060, 48723, 88110, 68709, 1, 41556, 93702, 53130,
                33563, 7325, 62790, 68393, 89069, 60937, 12831, 33335, 84483, 88683, 60530, 13544,
                42699, 35939, 80544, 96731, 40420, 1, 84630, 33402, 4475, 79170, 25752, 65139,
                72930, 30030, 35594, 15351, 84461, 14954, 62266, 86449, 36592, 27694, 3950, 88578,
                21275, 49983, 84630, 14671, 28219, 76830, 63838, 32424, 1, 71610, 61889, 1, 72618,
                77176, 66990, 14518, 61404, 42509, 23354, 53657, 1, 41933, 1, 21141, 98670, 22712,
                83640, 1610, 1, 30328, 91630, 64668, 1, 32579, 21798, 41006, 32062, 59640, 1,
                11182, 13697, 87846, 78850, 94380, 15372, 56238, 13380, 43890, 1, 89384, 88093,
                16325, 66149, 16363, 98470, 5434, 8001, 81510, 11059, 1, 1, 1, 47969, 84630, 55829,
                1, 67301, 1481, 37380, 69389, 52183, 46410, 1, 63030, 81158, 888, 1, 79170, 96226,
                29580, 1, 96450, 45659, 63288, 98823, 93614, 1, 10025, 99330, 15009, 78540, 13356,
                18330, 78540, 88349, 69166, 43890, 22076, 1, 27527, 99330, 99672, 98670, 81345,
                6006, 1, 11898, 1, 53130, 20418, 71610, 67830, 10480, 76837, 54751, 31741, 1, 1,
                79170, 31634, 66300, 21300, 71011, 51708, 9571, 91770, 22040, 79170, 30030, 92598,
                56170, 1, 2602, 83132, 27651, 59294, 1, 80293, 16861, 41280, 31057, 43758, 1, 7358,
                57974, 69700, 33539, 78518, 71450, 1, 15904, 1, 77557, 71610, 39390, 56951, 78636,
                1423, 70680, 52515, 1, 1, 73939, 1, 37555, 24677, 98670, 81730, 1, 97215, 76231,
                77686, 89880, 53019, 70936, 98670, 27634, 99257, 92430, 1, 92914, 10651, 38786,
                94871, 28380, 1, 5043, 62790, 24543, 78540, 91410, 1, 10836, 9341, 79170, 79943,
                66240, 56779, 97650, 46293, 22190, 87703, 72951, 96135, 15073, 42552, 17741, 46410,
                57570, 90060, 81510, 16529, 40211, 64330, 6221, 53130, 39886, 64038, 21668, 99330,
                2221, 1, 47310, 34132, 90452, 55498, 1, 1, 91770, 67061, 15878, 24279, 17977, 4225,
                99991, 48033, 81390, 84285, 71575, 61189, 17217, 79519, 83946, 14430, 25107, 85470,
                11265, 45490, 39270, 35731, 37620, 32629, 30030, 1, 62790, 84092, 36366, 1, 95880,
                1, 58389, 99330, 96460, 55335, 92683, 28066, 1, 1, 24475, 48180, 79760, 92820,
                93763, 85875, 1, 49142, 69630, 15513, 2233, 1, 60802, 27809, 14684, 16417, 53928,
                35490, 1, 59220, 57964, 50660, 88103, 46768, 1, 81136, 60060, 78659, 89410, 55062,
                80548, 5544, 73308, 76400, 23436, 19720, 53010, 49946, 95582, 22659, 1, 92107,
                71610, 32717, 1, 84042, 63419, 23142, 56070, 59010, 162, 88543, 53130, 99330,
                72511, 1, 26938, 1, 88436, 5772, 95337, 40187, 7767, 11651, 27947, 80897, 94920,
                1889, 41719, 56069, 92415, 78372, 1, 70118, 42102, 86081, 90099, 76103, 67830,
                46061, 93996, 42966, 58631, 67169, 78598, 1, 95254, 85331, 41185, 1, 1, 42510,
                96720, 86765, 35308, 45330, 68257, 1103, 1, 1, 1, 56531, 60213, 60918, 4970, 66632,
                94620, 45439, 84630, 70620, 31189, 80040, 6562, 99330, 45681, 46046, 3459, 72380,
                59718, 91770, 8863, 89670, 35002, 66894, 90634, 1, 55881, 94458, 15289, 90090,
                71610, 93317, 98262, 52321, 12595, 90457, 31991, 17430, 70345, 1, 55000, 64524,
                32886, 36498, 46410, 41867, 11299, 16003, 32807, 73416, 81510, 85470, 9314, 1,
                71866, 84630, 18690, 95422, 38135, 52963, 92361, 92486, 20697, 87780, 1, 3430,
                46410, 1, 46798, 1, 92153, 1, 53090, 98026, 53057, 9368, 50790, 15376, 1, 1, 1,
                19053, 9811, 99330, 22198, 1, 59173, 90427, 87782, 81349, 65919, 57373, 17519,
                9859, 71610, 22134, 10237, 1, 84630, 48429, 82110, 94581, 27611, 74597, 98670, 1,
                67830, 12730, 51330, 32797, 94710, 1, 20947, 84630, 43890, 65919, 1, 62896, 28860,
                59646, 85213, 66931, 95840, 63420, 47482, 39390, 79693, 48994, 84987, 41843, 96710,
                61961, 93895, 12927, 81906, 37562, 22380, 1, 66820, 82110, 35530, 59207, 35983,
                77744, 73242, 35140, 1, 93436, 18344, 43890, 9775, 1, 64884, 72667, 67494, 36678,
                72150, 2687, 25530, 56113, 8281, 1, 66810, 17264, 1, 49041, 14328, 8193, 43997,
                67189, 8801, 37599, 66499, 54973, 86588, 95153, 458, 97927, 1, 31588, 92888, 74521,
                94020, 79170, 69630, 59762, 10623, 48172, 74289, 57184, 22383, 41230, 77918, 78540,
                43890, 28151, 37574, 78540, 24880, 84930, 1283, 12608, 33462, 82110, 1, 4969, 289,
                82183, 34305, 84336, 1, 47726, 71610, 65436, 1596, 88410, 1, 93338, 18618, 78540,
                30650, 32526, 92820, 20023, 62827, 42743, 29463, 75050, 36161, 1, 16697, 41983,
                50728, 61907, 56704, 75270, 95478, 1, 46746, 78540, 84630, 46352, 78540, 67158, 1,
                20173, 92820, 92820, 1, 1, 92237, 85140, 22366, 99330, 81510, 66990, 1, 85680, 1,
                47854, 31663, 94710, 1, 94710, 38087, 1, 62790, 15941, 88854, 69230, 87603, 92976,
                29000, 1, 24043, 87629, 43890, 22207, 83644, 8512, 58283, 35971, 30030, 77588,
                66990, 45724, 90090, 86677, 1, 73818, 99330, 99420, 25116, 30030, 1, 1, 96399,
                58058, 89040, 70380, 1, 86250, 1, 93030, 67575, 1, 56197, 786, 74099, 81750, 86166,
                6581, 1, 85085, 1, 93223, 64997, 72930, 50261, 10260, 1, 98280, 80084, 68442,
                26220, 17538, 76574, 45249, 65480, 79241, 45186, 12477, 96039, 9204, 26017, 88088,
                29314, 4110, 29358, 62407, 11444, 17794, 45829, 77036, 41670, 47599, 91575, 39270,
                51051, 22578, 90090, 88242, 84105, 644, 1, 71610, 1, 16981, 2287, 35250, 33128, 1,
                1, 97371, 66990, 23742, 46275, 92820, 8899, 1, 14546, 1, 8190, 92820, 37851, 23319,
                1, 78540, 37674, 71890, 73470, 24180, 41340, 94430, 90090, 4045, 11682, 35081, 1,
                23940, 53130, 9907, 66990, 1, 81510, 43890, 92820, 3001, 59617, 72072, 1, 1, 59364,
                48174, 1, 10830, 79170, 9955, 62790, 1, 75786, 72930, 87780, 1, 57829, 1, 1, 47718,
                46990, 69258, 80520, 39169, 92820, 35558, 19139, 62482, 71610, 1, 21450, 1, 1,
                49434, 28160, 49811, 83791, 1, 32445, 15695, 1, 98280, 65589, 69426, 83938, 17170,
                1, 30467, 6882, 10201, 77090, 1, 1, 91378, 81343, 54028, 38403, 34183, 54869,
                98457, 72560, 1, 60322, 28428, 1, 40093, 91290, 50778, 53552, 73150, 8580, 94710,
                85470, 48651, 45890, 43953, 89430, 93324, 72930, 62790, 79156, 97524, 11890, 74827,
                60060, 30341, 78540, 25339, 27390, 79170, 79170, 93949, 91770, 57086, 58466, 12558,
                70289, 20972, 95030, 73303, 46571, 26102, 5698, 38760, 63687, 94823, 25207, 42666,
                82008, 26130, 27006, 1, 55073, 63492, 2275, 78540, 1, 65983, 1, 96872, 61747, 1853,
                78858, 39270, 1, 90930, 71167, 86901, 44502, 25950, 1, 64064, 83689, 95599, 70350,
                90090, 1, 42031, 1123, 15042, 81978, 44202, 51146, 63618, 7730, 33229, 1, 1, 57144,
                66495, 1092, 74453, 6612, 52546, 51714, 46960, 41230, 58421, 97110, 20930, 46410,
                1, 84272, 72930, 35715, 57467, 3697, 99864, 54284, 1, 1, 26208, 82954, 94710,
                58786, 83248, 56560, 47818, 21253, 46410, 45971, 57151, 27250, 1, 67963, 34188,
                36506, 46410, 47390, 69552, 1, 18096, 81066, 64075, 71400, 50563, 79245, 42504,
                66220, 70849, 8355, 1, 91080, 1, 54373, 51870, 23558, 24222, 39270, 20280, 20120,
                71610, 60942, 19, 64423, 2017, 89830, 58933, 77131, 82446, 1, 78339, 50958, 1,
                79373, 82110, 12906, 19228, 1, 88093, 52668, 1, 34740, 46027, 54727, 26731, 90780,
                40993, 67834, 72590, 3187, 53238, 58710, 26133, 80251, 62790, 1, 82110, 84186,
                90075, 32093, 59453, 2387, 40086, 1, 1, 63059, 84630, 1, 85470, 71610, 19942,
                92092, 11845, 2539, 1, 65962, 45290, 43207, 22278, 1, 65379, 20179, 69318, 76020,
                46444, 29800, 1, 66990, 98670, 81919, 1, 61432, 40709, 1, 46860, 1, 69510, 23730,
                66045, 91770, 30688, 1, 51925, 37230, 1, 1, 92598, 99330, 89113, 90866, 68150,
                90090, 89550, 83345, 18049, 72603, 43890, 91770, 50880, 52459, 58898, 44784, 85470,
                66660, 1, 6068, 74214, 7183, 1, 47860, 12963, 81199, 75826, 79937, 74784, 69069,
                85470, 1, 74463, 62790, 95101, 49801, 1, 90090, 46410, 65063, 29961, 85470, 64591,
                54651, 62790, 95217, 66990, 1, 65452, 59051, 69774, 27644, 2039, 62401, 30702,
                56957, 6490, 87353, 1583, 36570, 1, 81428, 5184, 20010, 4110, 5028, 16698, 49020,
                51870, 98670, 1, 1, 67830, 50622, 21502, 1, 1, 30030, 1, 1, 63382, 85470, 66332,
                78259, 32130, 9690, 20290, 68148, 89427, 50909, 30192, 86777, 89880, 42372, 87978,
                51051, 96547, 27758, 54774, 94710, 91770, 95684, 67092, 62790, 29166, 1, 86892,
                31135, 29402, 97812, 57783, 25385, 53550, 94710, 7273, 91305, 12527, 1, 87182,
                97687, 73315, 1, 40967, 63720, 92910, 56316, 1, 90015, 55818, 22612, 72868, 80468,
                71610, 49590, 86753, 1, 88578, 67830, 1, 56829, 43890, 66990, 1, 78540, 56490, 1,
                79170, 88320, 34763, 56459, 98670, 1, 85470, 91558, 27720, 1, 1, 1, 91153, 54103,
                38766, 27813, 65417, 50622, 3517, 4674, 1, 23238, 39199, 63962, 45341, 1, 40260,
                82968, 28476, 56712, 63765, 2559, 32751, 1, 30013, 75372, 13947, 29526, 93310,
                85549, 41316, 11938, 64902, 53130, 82110, 70683, 1279, 45350, 92820, 75484, 20123,
                86117, 87780, 48960, 43890, 59052, 75347, 34285, 50820, 90364, 54757, 19592, 8623,
                1, 1, 1, 53130, 53130, 85037, 85803, 1, 92820, 29488, 69833, 61700, 69426, 1,
                85470, 55770, 74646, 48720, 85785, 56760, 96570, 41296, 91443, 91770, 1, 85410,
                40750, 46957, 40694, 90623, 1, 90327, 92820, 1228, 5180, 98670, 79458, 1, 71610,
                1228, 66396, 91488, 52642, 66990, 79679, 1, 79581, 68474, 1, 51870, 1, 60661,
                17107, 1, 13, 90662, 6906, 1, 80990, 80309, 1, 80565, 1, 84252, 54989, 1, 1, 46410,
                1, 71713, 1, 59239, 35452, 64460, 57100, 81510, 91740, 78540, 57840, 31620, 1,
                91870, 41687, 41340, 1381, 90860, 45780, 87720, 57710, 1, 4357, 45390, 53130,
                58530, 1, 57928, 13530, 87179, 1, 70246, 1, 93153, 70388, 99330, 46410, 1, 3224,
                40260, 74547, 84630, 41879, 30030, 92467, 85470, 1, 51795, 1, 1, 97614, 85470,
                57548, 85470, 85215, 68748, 69263, 82590, 86144, 66167, 84424, 46410, 36566, 73370,
                65996, 66030, 83720, 69300, 56996, 1, 42861, 83896, 65010, 83003, 71761, 26034,
                71610, 90090, 51180, 98264, 85283, 82514, 30030, 23029, 30030, 8602, 43890, 65579,
                22230, 74910, 82469, 1, 1, 33859, 97410, 58305, 6371, 34209, 1, 97881, 38993,
                27090, 71204, 21089, 16531, 4546, 74074, 74172, 66306, 86339, 19798, 98610, 1,
                70380, 78540, 90090, 1, 1, 1, 18666, 90684, 86982, 65525, 60060, 84490, 1, 24121,
                1, 84893, 1, 44382, 75360, 43890, 82790, 62444, 1, 25194, 14639, 30421, 76986,
                49405, 43050, 99330, 6859, 96370, 29014, 59708, 25155, 61788, 62297, 61970, 9473,
                31890, 23562, 1, 73416, 6701, 1, 15761, 78540, 71610, 55377, 36960, 41519, 39270,
                45433, 88343, 72345, 48445, 26043, 57498, 66092, 30030, 66819, 78540, 1, 48326,
                17262, 1, 2065, 65514, 1, 1, 28644, 34315, 97645, 45838, 75263, 27240, 1, 84509,
                50786, 37740, 47988, 25769, 1, 43890, 1, 14040, 53323, 23395, 59809, 44582, 34310,
                1, 79662, 97790, 1, 69737, 79646, 80899, 91707, 90090, 44847, 1, 98670, 39195,
                10027, 28490, 53130, 65730, 47658, 88536, 77422, 90407, 39548, 92990, 16296, 18480,
                9, 75030, 53590, 1, 1, 72930, 18798, 85470, 14595, 315, 29741, 1, 22412, 72930,
                58170, 40720, 47638, 58710, 84533, 90090, 40356, 90210, 51315, 62597, 90090, 54531,
                1, 18689, 12266, 22878, 1, 26300, 53130, 19320, 1, 1, 25185, 24575, 50530, 1,
                70325, 16223, 1, 85800, 42186, 56694, 81130, 58144, 62790, 13093, 51373, 30911,
                66496, 13020, 59233, 32850, 53592, 1, 79170, 33137, 96307, 87016, 1, 1, 72577,
                45526, 98487, 38633, 56024, 15844, 43195, 67830, 71610, 58043, 25426, 91102, 79170,
                43544, 78474, 85470, 81510, 28810, 1, 98610, 75559, 79042, 25663, 58997, 64139,
                5874, 80768, 90610, 69790, 37841, 91770, 1, 1, 28413, 95380, 46410, 10544, 10798,
                65725, 78982, 15408, 1, 10825, 19273, 27968, 1, 22447, 78540, 14015, 97463, 1,
                69738, 24369, 1, 79967, 79170, 32766, 86269, 96720, 74263, 1, 64218, 66990, 99809,
                58494, 1, 31630, 49620, 52630, 66980, 87414, 1574, 42157, 41748, 22004, 1, 51870,
                50466, 50280, 26871, 25806, 33136, 1, 45150, 99330, 1, 96098, 77747, 78690, 46410,
                91860, 85551, 69420, 47112, 1, 51376, 18439, 90255, 1, 2540, 9446, 81571, 91921, 1,
                8499, 70746, 12239, 97384, 68889, 47310, 88142, 90972, 60060, 6473, 28842, 11832,
                79170, 90090, 17159, 21031, 1, 96472, 57162, 42019, 75128, 17622, 2087, 34230, 1,
                50736, 45500, 72307, 37714, 1, 54419, 83936, 1, 81427, 92130, 60606, 97614, 64303,
                98670, 1, 1, 53130, 78580, 79261, 74214, 33427, 95676, 78540, 1, 18659, 98670, 1,
                1, 58824, 46410, 43890, 51870, 52898, 56760, 40277, 1, 1, 35747, 181, 42937, 65478,
                86293, 65450, 43890, 32833, 27642, 77220, 29717, 48620, 7643, 1, 26813, 40729, 1,
                1, 91390, 69420, 12497, 59957, 98210, 63987, 43890, 22576, 78540, 62790, 60060, 1,
                98405, 35913, 73491, 98670, 92877, 4749, 97531, 1926, 99750, 97615, 26110, 1,
                67393, 78760, 1, 68188, 41433, 75323, 91770, 1, 51721, 1, 39280, 71928, 85470,
                33150, 25391, 58056, 58830, 81510, 99057, 72930, 1, 71101, 8873, 26683, 32490,
                60060, 1, 24160, 78540, 58912, 34410, 30463, 29831, 90132, 21217, 26388, 1, 82110,
                11950, 41694, 43890, 65024, 22145, 55193, 87906, 20621, 45886, 70488, 20856, 86190,
                1485, 37652, 78307, 49201, 54570, 87990, 89838, 34771, 14790, 8799, 92841, 90090,
                61215, 95970, 77532, 47901, 44298, 15368, 24077, 90132, 3198, 42169, 64187, 19635,
                63716, 90090, 35723, 3833, 18235, 1, 62497, 96078, 87778, 84630, 68810, 79216, 1,
                1, 90067, 75572, 42812, 82110, 95069, 86058, 82110, 99330, 66990, 22890, 6813,
                89270, 98210, 43890, 84630, 15249, 91698, 78479, 1057, 1, 1, 14790, 43890, 76984,
                57260, 82301, 76903, 91770, 89166, 35266, 38451, 70434, 12045, 87538, 72526, 21323,
                62100, 58788, 99371, 1, 43649, 89876, 54349, 1, 84863, 81039, 40290, 84582, 41580,
                40156, 28119, 69006, 34234, 1, 93090, 84630, 1, 9516, 70956, 45240, 4118, 34210, 1,
                90646, 99330, 76908, 67260, 63149, 13566, 42790, 13740, 7715, 64302, 69960, 56347,
                32672, 60060, 48544, 99252, 809, 6393, 53873, 88269, 17190, 36403, 1, 21661, 1,
                91770, 71610, 5961, 78330, 58698, 88536, 69054, 52910, 1, 91770, 60420, 86186,
                67411, 23430, 19541, 11226, 41603, 25303, 1, 11970, 50666, 82110, 38431, 24088,
                70541, 68489, 11310, 62415, 51644, 37240, 53130, 91770, 1914, 7880, 80175, 1,
                15362, 96860, 1, 1, 81510, 14561, 70737, 89399, 39648, 1, 4392, 1, 51870, 94710, 1,
                1, 6188, 74516, 72930, 80831, 58759, 91161, 78540, 72930, 9630, 3834, 46410, 43890,
                2144, 72930, 82236, 62622, 1, 49800, 1, 24862, 81700, 25902, 59083, 22200, 73780,
                66990, 1, 82771, 21896, 14509, 65065, 46410, 90967, 9534, 2805, 40524, 1, 53215,
                46349, 19518, 44384, 11060, 92650, 42945, 90893, 39270, 89745, 87780, 75375, 92027,
                1, 94804, 63555, 17238, 11570, 26130, 79170, 99819, 5083, 76977, 44492, 78540,
                35175, 44748, 12933, 1, 53130, 82110, 21317, 60060, 5562, 47656, 21450, 15335,
                58926, 44460, 38142, 40938, 67606, 54421, 70589, 45305, 71610, 72469, 45654, 58188,
                1, 33765, 68145, 48953, 69384, 49941, 1, 71610, 1, 94710, 30755, 82530, 87780,
                85470, 70554, 88140, 92083, 85470, 13862, 50101, 1713, 46970, 85470, 1, 37320,
                11788, 67129, 1, 98670, 87780, 1, 72534, 1, 11273, 1, 1, 99653, 91967, 20594,
                72623, 90678, 41601, 39429, 94710, 1, 1, 79365, 24066, 98910, 94185, 77509, 13629,
                58290, 81510, 56423, 41930, 10973, 4790, 1, 79009, 1, 9654, 49684, 54331, 57625,
                11052, 54357, 41137, 10453, 50294, 40641, 1, 7639, 37570, 41688, 92820, 1, 8719,
                87326, 90636, 1, 1, 1, 71496, 93310, 83076, 1, 6659, 66585, 13831, 94710, 1, 27659,
                97549, 50539, 12113, 1, 57900, 47670, 86870, 40609, 91770, 1, 98670, 68963, 71249,
                14283, 87060, 45875, 47378, 66990, 32802, 1, 72930, 73920, 66822, 84654, 1, 72930,
                47399, 43890, 94552, 88515, 92085, 1, 1, 65968, 1, 81840, 1, 41040, 98672, 93900,
                1, 39270, 55927, 80220, 57628, 1, 33150, 1, 32969, 1, 86611, 52780, 73164, 22364,
                46410, 1, 46410, 9300, 98770, 77117, 1, 34028, 18077, 98670, 44041, 94270, 62449,
                30133, 80153, 80306, 89915, 40658, 87959, 20930, 92820, 65658, 1, 72150, 60060,
                63259, 48337, 38280, 35273, 90090, 21318, 80427, 46410, 86597, 98210, 98670, 62731,
                16882, 43395, 61469, 1, 1, 6431, 80671, 75721, 13616, 35335, 1, 43230, 80351,
                74200, 44202, 77491, 50610, 80630, 31732, 39585, 69630, 1, 12012, 1, 1, 12642, 1,
                47085, 1, 1, 15452, 1, 71610, 16926, 28470, 61845, 85874, 76314, 78540, 51870,
                88578, 32498, 80431, 38570, 58357, 87780, 76581, 78967, 27973, 56837, 40979, 39267,
                1, 43890, 60507, 49455, 1, 1, 2502, 47260, 27176, 46530, 42384, 73896, 42228,
                30509, 92809, 73952, 91529, 52077, 76286, 75683, 77430, 1, 43890, 64020, 16312, 1,
                66178, 1, 85470, 1, 55586, 72930, 74821, 51136, 31627, 94710, 90090, 54133, 79170,
                88110, 1, 1, 1, 83761, 1, 61195, 1482, 81090, 1, 31052, 1, 80598, 70247, 84630,
                89310, 51870, 50093, 87780, 17923, 1, 1, 76610, 10940, 81510, 1, 36621, 45540,
                22170, 1, 72930, 14430, 1, 24528, 82995, 1, 75511, 89115, 1, 82069, 79561, 78540,
                23539, 22763, 66990, 63611, 70980, 66990, 85984, 51870, 84630, 30030, 1, 52976,
                90651, 16866, 13485, 5964, 1, 94392, 1, 1, 7630, 7312, 1, 62985, 94710, 1, 50178,
                68039, 54086, 54193, 38447, 98975, 47955, 70110, 85470, 91770, 34909, 68266, 1,
                53917, 30827, 77323, 1, 1, 1, 80990, 47096, 42001, 91770, 1, 39270, 84420, 96497,
                67499, 70122, 67830, 7041, 66462, 17209, 20476, 27390, 51870, 15834, 67582, 83220,
                78540, 64260, 62204, 45276, 1, 38346, 41345, 29156, 61908, 74650, 64110, 26970,
                7770, 13267, 67773, 81510, 20345, 61983, 34782, 53269, 83930, 94380, 1, 90821,
                97812, 78787, 96327, 70308, 77688, 62790, 20435, 43050, 98655, 26136, 35223, 98810,
                71190, 82915, 57594, 1, 1, 40387, 21854, 1, 36582, 99251, 66612, 31080, 93849,
                75871, 25051, 92342, 86389, 60679, 47193, 1, 27647, 1, 39841, 32150, 33814, 73909,
                9326, 98070, 19523, 1, 26691, 1, 38220, 1, 60390, 1, 17089, 31746, 85470, 40343,
                60060, 95790, 51590, 98670, 22421, 28777, 57032, 52338, 1, 3923, 77282, 20767,
                80496, 90177, 34860, 1, 33403, 83131, 99330, 27531, 46457, 78540, 72930, 85272,
                48393, 1, 82030, 89040, 79170, 84647, 43890, 20568, 82110, 60060, 12413, 28931,
                27030, 66801, 11071, 31746, 1621, 73783, 97854, 17047, 20239, 67830, 93756, 51870,
                1, 53465, 70986, 38842, 56171, 61902, 44476, 54250, 93702, 55650, 35094, 94710,
                55660, 99330, 151, 57855, 83675, 93179, 79170, 88578, 43890, 66990, 45723, 1,
                70962, 65411, 61446, 30809, 90090, 84630, 229, 34855, 18181, 93849, 49388, 75685,
                8283, 34057, 39371, 17183, 35196, 9816, 43890, 61668, 8241, 78166, 23925, 57573,
                82137, 76963, 82110, 56267, 40400, 75486, 98898, 18438, 1, 46840, 55146, 79742,
                52051, 1, 37681, 70165, 44855, 39270, 61493, 31746, 64498, 1, 87780, 59514, 53340,
                51971, 50881, 98784, 67962, 99789, 65812, 47430, 19552, 30649, 91770, 79732, 37037,
                10710, 78919, 13182, 54366, 1, 95164, 4176, 27615, 1, 98670, 62985, 18870, 6786,
                22899, 90090, 40404, 98670, 51612, 1330, 1, 89383, 39229, 87780, 93889, 77209,
                42093, 36936, 63393, 95597, 1, 12367, 54730, 67619, 62960, 82673, 34086, 89700,
                55391, 30660, 8262, 43890, 38083, 66030, 1, 1, 90507, 66990, 68382, 81510, 1,
                29914, 1, 57849, 73458, 9399, 30030, 45351, 47569, 10303, 81510, 4603, 1, 40378,
                81327, 51034, 68558, 45695, 7378, 86898, 1, 42435, 75010, 25861, 68713, 30352,
                57884, 53130, 90090, 1, 92924, 94333, 1, 1, 67830, 63799, 91827, 60332, 59850,
                71308, 73114, 1, 1, 57863, 30203, 5861, 43322, 91770, 1, 1, 22450, 78540, 1, 1002,
                24253, 33189, 19699, 1, 43038, 63367, 1, 53235, 91290, 72013, 98430, 60766, 23131,
                1, 6435, 7058, 223, 92651, 51283, 18510, 82110, 1, 65663, 98670, 78183, 70181,
                8501, 46581, 1, 39270, 26119, 33343, 71346, 1, 72390, 1039, 7590, 36525, 1, 7630,
                74070, 1, 94710, 1, 1, 61915, 23851, 99330, 46410, 19094, 55968, 90090, 60060,
                22780, 95370, 24461, 32606, 79180, 25489, 96418, 92820, 75834, 85531, 9750, 1, 823,
                43890, 98670, 4093, 1, 18893, 1, 83476, 90356, 74879, 74220, 14060, 72335, 34577,
                49955, 1, 56867, 51870, 47711, 73320, 94710, 15031, 68442, 13903, 1, 62118, 11196,
                48930, 31849, 86273, 69454, 72579, 50982, 50887, 86580, 50460, 1, 37568, 47562,
                9097, 13116, 28664, 9858, 66360, 57853, 38922, 93330, 84630, 39270, 1, 1, 43761,
                83622, 24940, 6952, 19668, 85022, 1, 1, 67452, 77137, 83661, 73554, 92461, 66990,
                67830, 82110, 50034, 86900, 48545, 79170, 83930, 82110, 25349, 90821, 67853, 21649,
                1, 25428, 60298, 54601, 1, 53130, 1, 85470, 52375, 31310, 29420, 68325, 46410,
                57364, 39270, 1, 68514, 84630, 64691, 1, 47609, 97650, 28721, 66990, 1, 2418, 1,
                74046, 83267, 74587, 76099, 1135, 1, 61967, 1, 62129, 35401, 79170, 80712, 1,
                91770, 1, 1, 76054, 4764, 30445, 85470, 23970, 1, 67260, 72930, 8980, 50166, 51995,
                88410, 1, 47283, 28961, 85260, 1, 1, 66430, 67830, 59753, 90655, 72618, 1, 44856,
                94605, 1, 45570, 78737, 1, 44831, 1, 73900, 83833, 82110, 92220, 40398, 90930,
                62790, 6529, 1, 20022, 62790, 68273, 88933, 1, 3323, 28037, 1020, 13667, 71706,
                4260, 15031, 67830, 254, 67461, 63492, 83980, 15330, 36296, 41, 21635, 12456, 1,
                76496, 72550, 60060, 3348, 42302, 27230, 84857, 1, 98670, 1, 86249, 1, 90090, 1,
                33180, 59581, 6660, 30030, 13957, 96773, 1, 1, 81892, 72998, 45402, 92796, 96186,
                53070, 20354, 87829, 60330, 66990, 62328, 65963, 4196, 46148, 87739, 22890, 50705,
                51870, 56514, 11970, 93704, 33547, 53130, 98070, 28837, 83903, 26742, 1, 54907,
                41495, 33613, 8233, 26993, 92383, 9205, 84693, 1045, 66990, 83122, 19392, 57690,
                43890, 84630, 66990, 30030, 27619, 32413, 26535, 83050, 58475, 89017, 53149, 89299,
                39270, 92621, 27027, 98213, 1, 51870, 73451, 25278, 24107, 67830, 63761, 1, 1,
                92820, 50121, 18881, 3372, 1, 43890, 27737, 1, 39270, 49872, 1707, 49393, 73349,
                94575, 68653, 1, 21557, 47355, 83477, 1, 40755, 1, 28907, 32116, 1, 69337, 94246,
                1321, 86658, 53130, 41538, 1, 92823, 38333, 239, 51973, 66300, 97894, 35062, 1,
                87780, 69762, 50781, 1534, 59598, 54881, 60851, 67289, 65222, 57440, 1, 50302, 1,
                12236, 81420, 45289, 1, 1, 88665, 97546, 28860, 98670, 72930, 15127, 88740, 1,
                41496, 73122, 46410, 1, 89842, 78540, 1, 31694, 57210, 88150, 1, 24123, 62790,
                95810, 67830, 77027, 92820, 38610, 80322, 17210, 10133, 55737, 87734, 43069, 1,
                4818, 9129, 47082, 26130, 15470, 57750, 90090, 26632, 36022, 35702, 66148, 9862,
                49742, 37633, 99330, 53389, 39137, 49770, 92110, 1, 41665, 93290, 80341, 87780,
                69690, 85470, 18354, 1, 30139,
            ],
            20749735,
            390154823,
        )
    }
}
