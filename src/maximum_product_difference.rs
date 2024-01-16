// https://leetcode.com/problems/maximum-product-difference-between-two-pairs/

pub struct Solution;

// Easy select_nth linear solution
// impl Solution {
//     pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
//         assert!(nums.len() >= 4);
//
//         let (wslice, x, rest) = nums.select_nth_unstable(1);
//         let (_, y, zslice) = rest.select_nth_unstable(rest.len() - 2);
//         let w = wslice[0];
//         let z = zslice[0];
//
//         (*y * z) - (*x * w)
//     }
// }

// Manual min1, min2, max1, max2 solution
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 4);

        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;
        let mut max1 = i32::MIN;
        let mut max2 = i32::MIN;

        for num in nums {
            if num < min1 {
                min2 = min1;
                min1 = num;
            } else if num < min2 {
                min2 = num;
            }

            if num > max1 {
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max2 = num;
            }
        }

        (max1 * max2) - (min1 * min2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        const NUMS: &[i32] = &[5, 6, 2, 7, 4];

        assert_eq!(Solution::max_product_difference(NUMS.to_vec()), 34);
    }

    #[test]
    fn ex2() {
        const NUMS: &[i32] = &[4, 2, 5, 9, 7, 4, 8];

        assert_eq!(Solution::max_product_difference(NUMS.to_vec()), 64);
    }
}
