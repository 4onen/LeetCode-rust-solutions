// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_max_k(nums: Vec<i32>) -> i32 {
//         let mut negatives = std::collections::HashSet::<i32>::new();
//         let mut positives = std::collections::HashSet::<i32>::new();
//         let mut maxk = -1;
//         for num in nums {
//             if num == 0 {
//                 unreachable!();
//             } else if num < 0 {
//                 if num == i32::MIN {
//                     unreachable!();
//                 }
//                 let num = num.abs();
//                 negatives.insert(num);
//                 if positives.contains(&num) {
//                     if num > maxk {
//                         maxk = num;
//                     }
//                 }
//             } else {
//                 // num > 0
//                 positives.insert(num.abs());
//                 if negatives.contains(&num) {
//                     if num > maxk {
//                         maxk = num;
//                     }
//                 }
//             }
//         }
//         maxk
//     }
// }

// Bitset sol'n
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut found = [0u8; 1001];
        let mut maxk = -1;
        for num in nums {
            if num == 0 {
                unreachable!();
            } else if num < 0 {
                // Rust has base two numbers on x86(_64)
                // So given a negative number,
                // the following is equivalent to
                let num = num.abs();
                found[num as usize] |= 1;
                if found[num as usize] == 3 {
                    if num > maxk {
                        maxk = num;
                    }
                }
            } else {
                // num > 0
                found[num as usize] |= 2;
                if found[num as usize] == 3 {
                    if num > maxk {
                        maxk = num;
                    }
                }
            }
        }
        maxk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[i32], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 1000);
        assert!(nums.iter().all(|&n| n <= 1000));
        assert!(nums.iter().all(|&n| n != 0));
        assert!(nums.iter().all(|&n| n >= -1000));
        let result = Solution::find_max_k(nums.to_vec());
        assert!(result > 0 || result == -1);
        assert_eq!(result, expected)
    }

    #[test]
    fn ex1() {
        test(&[-1, 2, -3, 3], 3)
    }

    #[test]
    fn ex2() {
        test(&[-1, 10, 6, 7, -7, 1], 7)
    }

    #[test]
    fn ex3() {
        test(&[-10, 8, 6, 7, -2, -3], -1)
    }
}
