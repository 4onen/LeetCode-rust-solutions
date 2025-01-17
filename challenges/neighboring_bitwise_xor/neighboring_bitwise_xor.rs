// https://leetcode.com/problems/neighboring-bitwise-xor/

pub struct Solution;

// Naive sol'n: Try all possible original arrays
// impl Solution {
//     pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
//         if derived.iter().all(|&x| x == 0) {
//             return true;
//         }
//         assert!(derived.len() < 16);
//         let mut original = vec![1;derived.len()];
//         while original.iter().any(|&b| b > 0) {
//             let mut new_derived = original.clone();
//             new_derived.rotate_left(1);
//             for i in 0..new_derived.len() {
//                 new_derived[i] ^= original[i];
//             }
//             if new_derived == derived {
//                 return true;
//             }
//             let mut carry = 1;
//             for i in 0..original.len() {
//                 original[i] ^= carry;
//                 carry = (original[i] == 1) as i32;
//                 if carry == 0 {
//                     break;
//                 }
//             }
//         }
//         false
//     }
// }

// Linear sol'n: Op creates array with xor that must be 0
impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.into_iter().reduce(|a,b| a ^ b) == Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(derived: &[i32], expected: bool) {
        assert!(derived.len() >= 1);
        assert!(derived.len() <= 100_000);
        for &b in derived {
            assert!(b == 0 || b == 1);
        }
        assert_eq!(Solution::does_valid_array_exist(derived.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test(&[1,1,0],true)
    }

    #[test]
    fn ex2() {
        test(&[1,1], true)
    }

    #[test]
    fn ex3() {
        test(&[1,0],false)
    }

    #[test]
    fn myex1_0() {
        test(&[0],true);
    }

    #[test]
    fn myex1_1() {
        test(&[1],false);
    }

    #[test]
    fn myex2() {
        test(&[0,0],true);
        test(&[0,1],false);
        test(&[1,0],false);
        test(&[1,1],true);
    }

    #[test]
    fn myex3() {
        test(&[0,0,0],true);
        test(&[1,0,0],false);
        test(&[0,1,0],false);
        test(&[1,1,0],true);
        test(&[0,0,1],false);
        test(&[1,0,1],true);
        test(&[0,1,1],true);
        test(&[1,1,1],false);
    }

    #[test]
    fn myex4_xx00() {
        test(&[0,0,0,0],true);
        test(&[1,0,0,0],false);
        test(&[0,1,0,0],false);
        test(&[1,1,0,0],true);
    }

    #[test]
    fn myex4_x010() {
        test(&[0,0,1,0],false);
        test(&[1,0,1,0],true);
    }

    #[test]
    fn myex4_x110() {
        test(&[0,1,1,0],true);
        test(&[1,1,1,0],false);
    }

    #[test]
    fn myex4_x001() {
        test(&[0,0,0,1],false);
        test(&[1,0,0,1],true);
    }

    #[test]
    fn myex4_x101() {
        test(&[0,1,0,1],true);
        test(&[1,1,0,1],false);
    }

    #[test]
    fn myex4_xx11() {
        test(&[0,0,1,1],true);
        test(&[1,0,1,1],false);
        test(&[0,1,1,1],false);
        test(&[1,1,1,1],true);
    }

    #[test]
    fn myex5() {
        test(&[0,0,0,0,0], true);
        test(&[1,0,0,0,0], false);
        test(&[0,1,0,0,0], false);
        test(&[1,1,0,0,0], true);
        test(&[0,0,1,0,0], false);
        test(&[1,0,1,0,0], true);
        test(&[0,1,1,0,0], true);
        test(&[1,1,1,0,0], false);
        test(&[0,0,0,1,0], false);
        test(&[1,0,0,1,0], true);
        test(&[0,1,0,1,0], true);
        test(&[1,1,0,1,0], false);
        test(&[0,0,1,1,0], true);
        test(&[1,0,1,1,0], false);
        test(&[0,1,1,1,0], false);
        test(&[1,1,1,1,0], true);
        test(&[0,0,0,0,1], false);
        test(&[1,0,0,0,1], true);
        test(&[0,1,0,0,1], true);
        test(&[1,1,0,0,1], false);
        test(&[0,0,1,0,1], true);
        test(&[1,0,1,0,1], false);
        test(&[0,1,1,0,1], false);
        test(&[1,1,1,0,1], true);
        test(&[0,0,0,1,1], true);
        test(&[1,0,0,1,1], false);
        test(&[0,1,0,1,1], false);
        test(&[1,1,0,1,1], true);
        test(&[0,0,1,1,1], false);
        test(&[1,0,1,1,1], true);
        test(&[0,1,1,1,1], true);
        test(&[1,1,1,1,1], false);
    }

    // start: [a,b,c]
    //    op: [a ^ b, b ^ c, c ^ a]
    // xorred: a ^ b ^ b ^ c ^ c ^ a = 0

    // 1      start: [a,b,c,d,e]
    // 2         op: [a ^ b, b ^ c, c ^ d, d ^ e, e ^ a]
    // 3       op^2: [a ^ b ^ b ^ c, b ^ c ^ c ^ d, c ^ d ^ d ^ e, d ^ e ^ e ^ a, e ^ a ^ a ^ b]
    // 4      simpl: [a ^ c, b ^ d, c ^ a, d ^ a, e ^ b]
    // 5 2->op*op-1: [a ^ b ^ e ^ a, b ^ c ^ a ^ b, c ^ d ^ b ^ c, d ^ e ^ c ^ d, e ^ a ^ d ^ e]
    // 6      simpl: [b ^ e, c ^ a, d ^ b, e ^ c, a ^ d]
}
