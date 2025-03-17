// https://leetcode.com/problems/divide-array-into-equal-pairs/

pub struct Solution;

// XOR partial sol'n (See myex2 for a failing case. myex1 also fails if the array odd check is dropped.)
// impl Solution {
//     pub fn divide_array(nums: Vec<i32>) -> bool {
//         if nums.len() & 1 != 0 {
//             return false;
//         }
//         let mut acc = 0;
//         for &num in &nums {
//             acc ^= num;
//         }
//         acc == 0
//     }
// }

// Bitset sol'n
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        // The max number is 500, so we can use a bitset of size 500
        type T = u32;
        const BITSET_ELEMENT_COUNT: u16 = (500 / T::BITS as u16) + (500 % T::BITS > 0) as u16;
        let mut bitset: [T; BITSET_ELEMENT_COUNT as usize] = [0; BITSET_ELEMENT_COUNT as usize];
        for &num in &nums {
            bitset[num as usize / T::BITS as usize] ^= 1 << (num as usize % T::BITS as usize);
        }
        for bitset_el in bitset {
            if bitset_el != 0 {
                return false;
            }
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
            assert!(num <= 500);
        }
        assert_eq!(Solution::divide_array(nums.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[3, 2, 3, 2, 2, 2], true)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4], false)
    }

    #[test]
    fn myex1() {
        test(&[1, 2, 3], false)
    }

    #[test]
    fn myex2() {
        test(&[1, 2, 4, 7], false)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                9, 9, 19, 10, 9, 12, 2, 12, 3, 3, 11, 5, 8, 4, 13, 6, 2, 11, 9, 19, 11, 15, 9, 17,
                15, 12, 5, 14, 12, 16, 18, 16, 10, 3, 8, 9, 16, 20, 2, 4, 16, 12, 11, 14, 20, 16,
                2, 18, 17, 20, 3, 13, 16, 17, 1, 1, 11, 20, 20, 4,
            ],
            false,
        )
    }
}
