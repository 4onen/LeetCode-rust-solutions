// https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/

pub struct Solution;

// Naive sol'n (Obviously not fast enough)
// impl Solution {
//     pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
//         fn backtrack(n: u8, i: u8, seq: &mut [i32], uses: u64) -> bool {
//             if i == seq.len() as u8 {
//                 return true;
//             }
//             for num in (1..=n).rev() {
//                 let num_uses = ((uses >> (num << 1)) & 3) as u8;
//                 if num == 1 {
//                     if num_uses > 0 {
//                         continue;
//                     }
//                 } else {
//                     match num_uses {
//                         0 => {}
//                         1 => {
//                             if num > i || seq[(i - num) as usize] != num as i32 {
//                                 continue;
//                             }
//                         }
//                         _ => continue,
//                     }
//                 }
//                 seq[i as usize] = num as i32;
//                 if backtrack(n, i + 1, seq, uses | ((num_uses + 1) as u64) << (num << 1)) {
//                     return true;
//                 }
//             }
//             false
//         }
//         let mut seq = vec![0; n as usize * 2 - 1];
//         seq[0] = n as i32;
//         backtrack(n as u8, 1, &mut seq, 1 << (n << 1));
//         seq
//     }
// }

// Number pre-placement branch culling
impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        fn backtrack(n: u8, i: u8, seq: &mut [i32], uses: u32) -> bool {
            if i == seq.len() as u8 {
                return true;
            }
            if seq[i as usize] != 0 {
                return backtrack(n, i + 1, seq, uses);
            }
            for num in (1..=n).rev() {
                let num_uses = ((uses >> num) & 1) as u8;
                if num_uses > 0 {
                    continue;
                }
                if num == 1 {
                    seq[i as usize] = num as i32;
                    if backtrack(n, i + 1, seq, uses | (1 << 1)) {
                        return true;
                    }
                    seq[i as usize] = 0;
                } else {
                    if i + num >= 2 * n - 1 {
                        continue;
                    }
                    if seq[(i + num) as usize] != 0 {
                        continue;
                    }
                    seq[i as usize] = num as i32;
                    seq[(i + num) as usize] = num as i32;
                    if backtrack(n, i + 1, seq, uses | (1 << num)) {
                        return true;
                    }
                    seq[(i + num) as usize] = 0;
                    seq[i as usize] = 0;
                }
            }
            false
        }
        let mut seq = vec![0; n as usize * 2 - 1];
        seq[0] = n as i32;
        if n > 1 {
            seq[n as usize] = n as i32;
        }
        backtrack(n as u8, 1, &mut seq, 1 << n);
        seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u8, expected: &[i32]) {
        assert!(n >= 1);
        assert!(n <= 20);
        assert!(expected.len() == n as usize * 2 - 1);
        assert_eq!(Solution::construct_distanced_sequence(n as i32), expected);
    }

    #[test]
    fn discussion_case1() {
        test(1, &[1])
    }

    #[test]
    fn discussion_case2() {
        test(2, &[2, 1, 2])
    }

    #[test]
    fn ex1() {
        test(3, &[3, 1, 2, 3, 2])
    }

    #[test]
    fn discussion_case4() {
        test(4, &[4, 2, 3, 2, 4, 3, 1])
    }

    #[test]
    fn ex2() {
        test(5, &[5, 3, 1, 4, 3, 5, 2, 4, 2])
    }

    #[test]
    fn discussion_case6() {
        test(6, &[6, 4, 2, 5, 2, 4, 6, 3, 5, 1, 3])
    }

    #[test]
    fn discussion_case7() {
        test(7, &[7, 5, 3, 6, 4, 3, 5, 7, 4, 6, 2, 1, 2])
    }

    #[test]
    fn discussion_case8() {
        test(8, &[8, 6, 4, 2, 7, 2, 4, 6, 8, 5, 3, 7, 1, 3, 5])
    }

    #[test]
    fn discussion_case18() {
        test(
            18,
            &[
                18, 16, 17, 13, 11, 15, 8, 14, 4, 2, 12, 2, 4, 10, 8, 11, 13, 16, 18, 17, 15, 14,
                12, 10, 9, 7, 5, 3, 6, 1, 3, 5, 7, 9, 6,
            ],
        )
    }

    #[test]
    fn discussion_case19() {
        test(
            19,
            &[
                19, 17, 18, 14, 12, 16, 9, 15, 6, 3, 13, 1, 3, 11, 6, 9, 12, 14, 17, 19, 18, 16,
                15, 13, 11, 10, 8, 4, 5, 7, 2, 4, 2, 5, 8, 10, 7,
            ],
        )
    }

    #[test]
    fn discussion_case20() {
        test(
            20,
            &[
                20, 18, 19, 15, 13, 17, 10, 16, 7, 5, 3, 14, 12, 3, 5, 7, 10, 13, 15, 18, 20, 19,
                17, 16, 12, 14, 11, 9, 4, 6, 8, 2, 4, 2, 1, 6, 9, 11, 8,
            ],
        )
    }
}
