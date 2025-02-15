// https://leetcode.com/problems/find-the-punishment-number-of-an-integer/

pub struct Solution;

// Naive linear time
// type NType = u32;
// impl Solution {
//     const fn is_punishment_number(n: NType) -> bool {
//         const fn backtrack(
//             current_sum: NType,
//             current_partition: NType,
//             current_partition_scale: NType,
//             num: NType,
//             target: NType,
//         ) -> bool {
//             if num == 0 {
//                 return current_sum + current_partition == target;
//             } else if current_sum + current_partition > target {
//                 return false;
//             }
//             let digit = num % 10;
//             let rest = num / 10;
//             // Two different paths:
//             // 1. Add the current digit to the current partition
//             // 2. Start a new partition with the current digit
//             if backtrack(
//                 current_sum,
//                 current_partition + digit * current_partition_scale,
//                 current_partition_scale * 10,
//                 rest,
//                 target,
//             ) {
//                 return true;
//             } else if backtrack(current_sum + current_partition, digit, 10, rest, target) {
//                 return true;
//             }
//             false
//         }
//         backtrack(0, 0, 1, n * n, n)
//     }
//     pub fn punishment_number(n: i32) -> i32 {
//         let n = n as NType;
//         let mut result = 0;
//         for i in 1..=n {
//             if Solution::is_punishment_number(i) {
//                 result += i * i;
//             }
//         }
//         result as i32
//     }
// }

// Using discarding of nines
// type NType = u32;
// impl Solution {
//     const fn is_punishment_number(n: NType) -> bool {
//         const fn backtrack(
//             current_sum: NType,
//             current_partition: NType,
//             current_partition_scale: NType,
//             num: NType,
//             target: NType,
//         ) -> bool {
//             if num == 0 {
//                 return current_sum + current_partition == target;
//             } else if current_sum + current_partition > target {
//                 return false;
//             }
//             let digit = num % 10;
//             let rest = num / 10;
//             // Two different paths:
//             // 1. Add the current digit to the current partition
//             // 2. Start a new partition with the current digit
//             if backtrack(
//                 current_sum,
//                 current_partition + digit * current_partition_scale,
//                 current_partition_scale * 10,
//                 rest,
//                 target,
//             ) {
//                 return true;
//             } else if backtrack(current_sum + current_partition, digit, 10, rest, target) {
//                 return true;
//             }
//             false
//         }
//         backtrack(0, 0, 1, n * n, n)
//     }
//     pub fn punishment_number(n: i32) -> i32 {
//         let n = n as NType;
//         let mut result = 0;
//         for i in (0..=n).step_by(9) {
//             let num = i;
//             if Solution::is_punishment_number(i) {
//                 result += i * i;
//             }
//             let num = i + 1;
//             if num <= n && Solution::is_punishment_number(num) {
//                 result += num * num;
//             }
//         }
//         result as i32
//     }
// }

// Constant-time lookup sol'n
type NType = u32;
impl Solution {
    const fn is_punishment_number(n: NType) -> bool {
        const fn backtrack(
            current_sum: NType,
            current_partition: NType,
            current_partition_scale: NType,
            num: NType,
            target: NType,
        ) -> bool {
            if num == 0 {
                return current_sum + current_partition == target;
            } else if current_sum + current_partition > target {
                return false;
            }
            let digit = num % 10;
            let rest = num / 10;
            // Two different paths:
            // 1. Add the current digit to the current partition
            // 2. Start a new partition with the current digit
            if backtrack(
                current_sum,
                current_partition + digit * current_partition_scale,
                current_partition_scale * 10,
                rest,
                target,
            ) {
                return true;
            } else if backtrack(current_sum + current_partition, digit, 10, rest, target) {
                return true;
            }
            false
        }
        backtrack(0, 0, 1, n * n, n)
    }
    pub const fn punishment_number(n: i32) -> i32 {
        let n = n as NType;
        assert!(n <= 1000);
        const NUM_NUMS: usize = 224;
        const NUMS: [NType; NUM_NUMS] = {
            let mut nums = [0; NUM_NUMS];
            let mut prev = 0;
            let mut i = 0;
            while i <= 1000 {
                let idx = i / 9;
                if Solution::is_punishment_number(i) {
                    prev += i * i;
                }
                nums[2 * idx as usize] = prev;
                if Solution::is_punishment_number(i + 1) {
                    prev += (i + 1) * (i + 1);
                };
                nums[2 * idx as usize + 1] = prev;
                i += 9;
            }
            nums
        };
        let idx = n / 9;
        let mut rem = n % 9;
        if rem > 1 {
            rem = 1;
        }
        NUMS[idx as usize * 2 + rem as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u16, expected: i32) {
        assert!(n >= 1);
        assert!(n <= 1000);
        assert_eq!(
            Solution::punishment_number(n as i32),
            expected,
            "Evaluating n={}",
            n
        );
    }

    #[test]
    fn is_punishment_number_myex1() {
        assert!(Solution::is_punishment_number(1));
        assert!(Solution::is_punishment_number(9));
        assert!(Solution::is_punishment_number(10));
    }

    #[test]
    fn ex1() {
        test(10, 182)
    }

    #[test]
    fn ex2() {
        test(37, 1478)
    }

    #[test]
    fn discussion_case_hardcode() {
        // if (n<36) return 182;
        // if (n<45) return 1478;
        // if (n<55) return 3503;
        // if (n<82) return 6528;
        // if (n<91) return 13252;
        // if (n<99) return 21533;
        // if (n<100) return 31334;
        // if (n<235) return 41334;
        // if (n<297) return 96559;
        // if (n<369) return 184768;
        // if (n<370) return 320929;
        // if (n<379) return 457829;
        // if (n<414) return 601470;
        // if (n<657) return 772866;
        // if (n<675) return 1204515;
        // if (n<703) return 1660140;
        // if (n<756) return 2154349;
        // if (n<792) return 2725885;
        // if (n<909) return 3353149;
        // if (n<918) return 4179430;
        // if (n<945) return 5022154;
        // if (n<964) return 5915179;
        // if (n<990) return 6844475;
        // if (n<991) return 7824575;
        // if (n<999) return 8806656;
        // if (n<1000) return 9804657;
        // return 10804657;
        test(1, 1);
        test(2, 1);
        test(8, 1);
        test(9, 82);
        test(10, 182);
        test(35, 182);
        test(36, 1478);
        test(44, 1478);
        test(45, 3503);
        test(54, 3503);
        test(55, 6528);
        test(81, 6528);
        test(82, 13252);
        test(90, 13252);
        test(91, 21533);
        test(98, 21533);
        test(99, 31334);
        test(100, 41334);
        test(234, 41334);
        test(235, 96559);
        test(296, 96559);
        test(368, 184768);
        test(369, 320929);
        test(370, 457829);
        test(378, 457829);
        test(379, 601470);
        test(413, 601470);
        test(414, 772866);
        test(656, 772866);
        test(657, 1204515);
        test(674, 1204515);
        test(675, 1660140);
        test(702, 1660140);
        test(755, 2154349);
        test(756, 2725885);
        test(791, 2725885);
        test(792, 3353149);
        test(908, 3353149);
        test(909, 4179430);
        test(917, 4179430);
        test(918, 5022154);
        test(944, 5022154);
        test(945, 5915179);
        test(963, 5915179);
        test(964, 6844475);
        test(989, 6844475);
        test(990, 7824575);
        test(991, 8806656);
        test(998, 8806656);
        test(999, 9804657);
        test(1000, 10804657);
    }
}
