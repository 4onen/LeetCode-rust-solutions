// https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct/

pub struct Solution;

// Initial sol'n (failed)
// impl Solution {
//     pub fn minimum_operations(nums: Vec<i32>) -> i32 {
//         const MAX_ELS: u8 = 100;
//         let mut has = [false; (MAX_ELS + 1) as usize];
//         let mut delete_head = 0u8;
//         let mut read_head = 0u8;
//         while read_head < nums.len() as u8 {
//             let num = nums[read_head as usize];
//             read_head += 1;
//             if has[num as usize] {
//                 for _ in 0..3 {
//                     if delete_head < nums.len() as u8 {
//                         has[nums[delete_head as usize] as usize] = false;
//                     }
//                     delete_head += 1;
//                 }
//                 if delete_head >= read_head {
//                     read_head = delete_head;
//                     continue;
//                 }
//             }
//             has[num as usize] = true;
//         }
//         (delete_head / 3) as i32
//     }
// }

// Naive sol'n (recount after every op)
// impl Solution {
//     pub fn minimum_operations(nums: Vec<i32>) -> i32 {
//         const MAX_ELS: u8 = 100;
//         let mut has = [false; (MAX_ELS + 1) as usize];
//         let mut delete_head = 0u8;
//         'outer: loop {
//             if delete_head >= nums.len() as u8 {
//                 break;
//             }
//             for &num in &nums[delete_head as usize..] {
//                 if has[num as usize] {
//                     has.fill(false);
//                     delete_head += 3;
//                     continue 'outer;
//                 }
//                 has[num as usize] = true;
//             }
//             break;
//         }
//         (delete_head / 3) as i32
//     }
// }

// Fixed my mistake (finish the !has number case completely first)
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        const MAX_ELS: u8 = 100;
        let mut has = [false; (MAX_ELS + 1) as usize];
        let mut delete_head = 0u8;
        let mut read_head = 0u8;
        while read_head < nums.len() as u8 {
            let num = nums[read_head as usize];
            if !has[num as usize] {
                has[num as usize] = true;
                read_head += 1;
                continue;
            }
            for _ in 0..3 {
                if delete_head < nums.len() as u8 {
                    has[nums[delete_head as usize] as usize] = false;
                }
                delete_head += 1;
            }
            if delete_head > read_head {
                read_head = delete_head;
            }
        }
        (delete_head / 3) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u8], expected: i32) {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 100);
        for &num in nums {
            assert!(num >= 1);
            assert!(num <= 100);
        }
        assert_eq!(
            Solution::minimum_operations(nums.iter().map(|&x| x as i32).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3, 4, 2, 3, 3, 5, 7], 2)
    }

    #[test]
    fn ex2() {
        test(&[4, 5, 6, 4, 4], 2)
    }

    #[test]
    fn ex3() {
        test(&[6, 7, 8, 9], 0)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[100; 100], 33)
    }

    #[test]
    fn discussion_case1() {
        test(&[2], 0)
    }

    #[test]
    fn discussion_case2() {
        test(&[5, 5], 1)
    }

    #[test]
    fn discussion_case3() {
        test(&[4, 5], 0)
    }

    #[test]
    fn discussion_case4() {
        test(&[2, 7, 15, 1, 15], 1)
    }

    #[test]
    fn discussion_case5() {
        test(&[5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 3)
    }

    #[test]
    fn discussion_case6() {
        test(&[1, 1, 1], 1)
    }

    #[test]
    fn discussion_case7() {
        test(&[1, 2, 2, 3, 3, 4], 2)
    }

    #[test]
    fn discussion_case7_1() {
        test(&[1, 2, 2, 3, 3], 2)
    }

    #[test]
    fn discussion_case8() {
        test(&[1, 2, 3], 0)
    }

    #[test]
    fn discussion_case9() {
        test(&[1, 2, 3, 4, 4, 4, 4, 4, 4], 3)
    }

    #[test]
    fn discussion_case10() {
        test(&[10, 10, 10, 10, 10], 2)
    }

    #[test]
    fn discussion_case11() {
        test(&[1, 2, 1, 2, 1, 2, 1, 2], 2)
    }

    #[test]
    fn failing_case1() {
        test(&[5, 7, 11, 12, 12], 2)
    }

    #[test]
    fn failing_case1_1() {
        test(&[11, 7, 11, 12, 12], 2)
    }
}
