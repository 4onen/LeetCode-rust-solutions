// https://leetcode.com/problems/find-the-town-judge/

// type TrustTup = (std::num::NonZeroU16, std::num::NonZeroU16);
// fn tupleize_trusts(trusts: Vec<Vec<i32>>) -> Box<[TrustTup]> {
//     // First, convert this vector of vectors to a more useful vector
//     // of tuples. We know each subvector has exactly two elements, so
//     // we can stuff them into tuples in the same memory space as the
//     // original sub-vectors.
//     // Obviously super unsafe if run on a 16-bit system or something,
//     // but this is a toy problem on leetcode, so the environment is
//     // known.
//     let original_len = trusts.len();
//     let trusts_ptr = trusts.leak();
//     let trusts_ptr2: &mut [Vec<i32>] = &mut trusts_ptr[0..original_len];
//     let trusts_tuple_ptr =
//         unsafe { std::mem::transmute::<_, &mut [TrustTup]>(trusts_ptr2) };
//     for i in 0..original_len {
//         // We know all the trusts are 1 to 1000, so we can safely
//         // convert them to NonZeroU16s.
//         let tup = unsafe {
//             (
//                 std::num::NonZeroU16::new_unchecked(trusts_ptr[i][0] as u16),
//                 std::num::NonZeroU16::new_unchecked(trusts_ptr[i][1] as u16),
//             )
//         };
//         trusts_tuple_ptr[i] = tup;
//     }
//     unsafe { std::boxed::Box::from_raw(trusts_tuple_ptr) }
// }

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
//         let n = n as i16;
//         assert!(n > 0, "n must be greater than 0");
//         assert!(n <= 1000, "n must be less than or equal to 1000");
//         let mut trust_count = vec![0i16; n as usize];
//         for t in trust {
//             let a_i = t[0];
//             let b_i = t[1];
//             trust_count[a_i as usize - 1] -= 1;
//             trust_count[b_i as usize - 1] += 1;
//         }
//         for i in 0..n {
//             if trust_count[i as usize] == n - 1 {
//                 return i as i32 + 1;
//             }
//         }
//         -1
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        match n {
            i32::MIN..=0 => unreachable!("n must be greater than 0"),
            1 => return 1,
            1..=1000 => {
                let n = n as u16;
                let mut trust_dec = vec![0u16; n as usize + 1];
                let mut trust_inc = vec![0u16; n as usize + 1];
                for t in trust {
                    let a_i = t[0];
                    let b_i = t[1];
                    trust_dec[a_i as usize] += 1;
                    trust_inc[b_i as usize] += 1;
                }
                for i in 1..=n {
                    if trust_dec[i as usize] == 0 && trust_inc[i as usize] == n as u16 - 1 {
                        return i as i32;
                    }
                }
                -1
            }
            _ => unreachable!("n must be less than or equal to 1000"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1
        );
    }
}
