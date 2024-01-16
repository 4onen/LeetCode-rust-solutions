// https://leetcode.com/problems/container-with-most-water/

pub struct Solution;

// Initial linear range sol'n
// impl Solution {
//     pub fn max_area(height: Vec<i32>) -> i32 {
//         let mut max = 0;
//         let mut subslice = &height[..];
//         while subslice.len() > 1 {
//             let start = subslice[0];
//             let end = subslice[subslice.len() - 1];
//             let area = (subslice.len() - 1) as i32 * std::cmp::min(start, end);
//             max = std::cmp::max(max, area);
//             if start < end {
//                 subslice = &subslice[1..];
//             } else {
//                 subslice = &subslice[..subslice.len() - 1];
//             }
//         }
//         max
//     }
// }

// Two idx with interval sol'n
// Turns out, interval calculations slow it down. Guess I don't have the free registers.
// impl Solution {
//     pub fn max_area(height: Vec<i32>) -> i32 {
//         let mut max = 0;
//         let mut left = 0;
//         let mut right = height.len() - 1;
//         let mut interval: i32 = (right - left) as i32;
//         while left < right {
//             let start = unsafe { *height.get_unchecked(left) };
//             let end = unsafe { *height.get_unchecked(right) };
//             let area = interval * std::cmp::min(start, end);
//             max = std::cmp::max(max, area);
//             if start < end {
//                 left += 1;
//             } else {
//                 right -= 1;
//             }
//             interval -= 1;
//         }
//         max
//     }
// }

// Plain two idx sol'n
// Somehow still slower than using a subslice?
// impl Solution {
//     pub fn max_area(height: Vec<i32>) -> i32 {
//         let mut max = 0;
//         let mut left = 0;
//         let mut right = height.len() - 1;
//         while left < right {
//             let start = unsafe { *height.get_unchecked(left) };
//             let end = unsafe { *height.get_unchecked(right) };
//             let area = (right - left) as i32 * std::cmp::min(start, end);
//             max = std::cmp::max(max, area);
//             if start < end {
//                 left += 1;
//             } else {
//                 right -= 1;
//             }
//         }
//         max
//     }
// }

// Subslice sol'n with unsafe
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut subslice = &height[..];
        while subslice.len() > 1 {
            let start = unsafe { *subslice.get_unchecked(0) };
            let end = unsafe { *subslice.get_unchecked(subslice.len() - 1) };
            let area = (subslice.len() - 1) as i32 * std::cmp::min(start, end);
            max = std::cmp::max(max, area);
            if start < end {
                subslice = &subslice[1..];
            } else {
                subslice = &subslice[..subslice.len() - 1];
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn myex1() {
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn myex2() {
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }

    #[test]
    fn myex3() {
        assert_eq!(Solution::max_area(vec![1, 2, 4, 3]), 4);
    }
}
