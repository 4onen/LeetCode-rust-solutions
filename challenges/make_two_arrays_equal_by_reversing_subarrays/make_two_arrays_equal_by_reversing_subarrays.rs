// https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/

pub struct Solution;

// Sort sol'n
// impl Solution {
//     pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
//         target.sort_unstable();
//         arr.sort_unstable();
//         target == arr
//     }
// }

// Counting sol'n
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut count = [0i16; 1001];
        for i in 0..target.len() {
            count[target[i] as usize] += 1;
            count[arr[i] as usize] -= 1;
        }
        count.iter().all(|&x| x == 0)
    }
}

// Sort sol'n with early return
// impl Solution {
//     pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
//         if target.len() != arr.len() {
//             return false;
//         }
//         target.sort_unstable();
//         arr.sort_unstable();
//         target == arr
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(target: &[i32], arr: &[i32]) {
        let mut sorted_target = target.to_vec();
        sorted_target.sort_unstable();
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort_unstable();
        let expected = sorted_target == sorted_arr;
        assert_eq!(
            Solution::can_be_equal(target.to_vec(), arr.to_vec()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3, 4], &[2, 4, 1, 3]);
    }

    #[test]
    fn ex2() {
        test(&[7], &[7]);
    }

    #[test]
    fn ex3() {
        test(&[3, 7, 9], &[3, 7, 11]);
    }
}
