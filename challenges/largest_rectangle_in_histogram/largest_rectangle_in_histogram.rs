// https://leetcode.com/problems/largest-rectangle-in-histogram/

pub struct Solution;

// Brute force sol'n
// impl Solution {
//     pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
//         let mut max_area = 0;
//         for i in 0..heights.len() {
//             let mut min_height = i32::MAX;
//             for j in i..heights.len() {
//                 min_height = std::cmp::min(min_height,heights[j]);
//                 max_area = std::cmp::max(max_area, min_height * (j - i + 1) as i32);
//             }
//         }
//         max_area
//     }
// }

// Left-scan brute force sol'n
// impl Solution {
//     pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
//         let mut max_area = 0;
//         for i in 0..heights.len() {
//             let mut min_height = i32::MAX;
//             for j in (0..=i).rev() {
//                 min_height = std::cmp::min(min_height,heights[j]);
//                 max_area = std::cmp::max(max_area, min_height * (i - j + 1) as i32);
//             }
//         }
//         max_area
//     }
// }

// a[i]<a[i-1] left-scan only sol'n
// impl Solution {
//     pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
//         let mut max_area = 0;
//         for i in 0..heights.len() - 1 {
//             if !(heights[i + 1] < heights[i]) {
//                 continue;
//             }
//             let mut min_height = heights[i];
//             for j in (0..=i).rev() {
//                 min_height = std::cmp::min(min_height, heights[j]);
//                 max_area = std::cmp::max(max_area, min_height * (i - j + 1) as i32);
//             }
//         }
//         let mut min_height = *heights.last().unwrap();
//         for j in (0..heights.len()).rev() {
//             min_height = std::cmp::min(min_height, heights[j]);
//             max_area = std::cmp::max(max_area, min_height * (heights.len() - j) as i32);
//         }
//         max_area
//     }
// }

// Next-smaller-el + prev-smaller-el sol'n
// type IdxT = i32;
// impl Solution {
//     fn next_smaller_el(els: &[i32]) -> Vec<IdxT> {
//         let mut result: Vec<IdxT> = Vec::with_capacity(els.len());
//         let next_smaller = result.spare_capacity_mut();
//         let mut stack: Vec<IdxT> = Vec::new();
//         for i in 0..els.len() as IdxT {
//             loop {
//                 match stack.last() {
//                     Some(&idx) if els[idx as usize] > els[i as usize] => {
//                         next_smaller[idx as usize].write(i);
//                         stack.pop();
//                     }
//                     _ => break,
//                 }
//             }
//             stack.push(i);
//         }
//         for idx in stack {
//             next_smaller[idx as usize].write(els.len() as IdxT);
//         }
//         unsafe { result.set_len(els.len()) };
//         result
//     }
//     fn prev_smaller_el(els: &[i32]) -> Vec<IdxT> {
//         let mut result: Vec<IdxT> = Vec::with_capacity(els.len());
//         let prev_smaller = result.spare_capacity_mut();
//         let mut stack: Vec<IdxT> = Vec::new();
//         for i in (0..els.len() as IdxT).rev() {
//             loop {
//                 match stack.last() {
//                     Some(&idx) if els[idx as usize] > els[i as usize] => {
//                         prev_smaller[idx as usize].write(i as IdxT);
//                         stack.pop();
//                     }
//                     _ => break,
//                 }
//             }
//             stack.push(i);
//         }
//         for idx in stack {
//             prev_smaller[idx as usize].write(-1 as IdxT);
//         }
//         unsafe { result.set_len(els.len()) };
//         result
//     }
//     pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
//         let mut max_area = 0;
//         let next_smaller = Self::next_smaller_el(&heights);
//         let prev_smaller = Self::prev_smaller_el(&heights);
//         for i in 0..heights.len() {
//             let area = (next_smaller[i] - prev_smaller[i] - 1) as i32 * heights[i];
//             max_area = std::cmp::max(max_area, area);
//         }
//         max_area
//     }
// }

// Next-smaller-el + prev-smaller-el sol'n with merged next pass (+shared stack)
// type IdxT = i32;
// impl Solution {
//     pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
//         let mut max_area = 0;
//         let mut stack: Vec<IdxT> = Vec::new();
//         let prev_smaller = {
//             let mut result: Vec<IdxT> = Vec::with_capacity(heights.len());
//             let prev_smaller = result.spare_capacity_mut();
//             for i in (0..heights.len() as IdxT).rev() {
//                 loop {
//                     match stack.last() {
//                         Some(&idx) if heights[idx as usize] > heights[i as usize] => {
//                             prev_smaller[idx as usize].write(i as IdxT);
//                             stack.pop();
//                         }
//                         _ => break,
//                     }
//                 }
//                 stack.push(i);
//             }
//             for &idx in stack.iter() {
//                 prev_smaller[idx as usize].write(-1 as IdxT);
//             }
//             unsafe { result.set_len(heights.len()) };
//             result
//         };
//         stack.clear();
//         for i in 0..heights.len() as i32 {
//             loop {
//                 match stack.last() {
//                     Some(&idx) if heights[idx as usize] > heights[i as usize] => {
//                         let next_smaller_idx = i;
//                         stack.pop();
//                         let area = (next_smaller_idx - prev_smaller[idx as usize] - 1) as i32
//                             * heights[idx as usize];
//                         max_area = std::cmp::max(max_area, area);
//                     }
//                     _ => break,
//                 }
//             }
//             stack.push(i)
//         }
//         let next_smaller_idx = heights.len() as i32;
//         for idx in stack {
//             let area =
//                 (next_smaller_idx - prev_smaller[idx as usize] - 1) as i32 * heights[idx as usize];
//             max_area = std::cmp::max(max_area, area);
//         }
//         max_area
//     }
// }

// Just-mutate-the-input monotonic stack sol'n
// impl Solution {
//     pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
//         let heights = {
//             heights.reserve_exact(2);
//             heights.insert(0, 0);
//             heights.push(0);
//             heights
//         };
//         let mut max_area = 0;
//         let mut stack = vec![0];
//         for i in 0..heights.len() as i32 {
//             while heights[*stack.last().unwrap() as usize] > heights[i as usize] {
//                 let rectangle_height = heights[stack.pop().unwrap() as usize];
//                 let area = (i - *stack.last().unwrap() - 1) as i32 * rectangle_height;
//                 max_area = std::cmp::max(max_area, area);
//             }
//             stack.push(i);
//         }
//         max_area
//     }
// }

// Optimized
impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let heights = {
            heights.reserve_exact(2);
            heights.insert(0, 0);
            heights.push(0);
            heights
        };
        let mut max_area = 0;
        let mut stack = vec![0];
        for i in 0..heights.len() as u32 {
            let mut rectangle_start = *stack.last().unwrap();
            while heights[rectangle_start as usize] > heights[i as usize] {
                let rectangle_height = heights[stack.pop().unwrap() as usize];
                rectangle_start = *stack.last().unwrap();
                let area = (i - rectangle_start - 1) as i32 * rectangle_height;
                max_area = std::cmp::max(max_area, area);
            }
            stack.push(i);
        }
        max_area
    }
}

// Conditional instead of insert(0,0) (wowza that's slower)
// impl Solution {
//     pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
//         heights.push(0);
//         let mut max_area = 0;
//         let mut stack = Vec::new();
//         for i in 0..heights.len() as i32 {
//             while !stack.is_empty()
//                 && heights[*stack.last().unwrap() as usize] > heights[i as usize]
//             {
//                 let rectangle_height = heights[stack.pop().unwrap() as usize];
//                 let rectangle_width = if stack.is_empty() {
//                     i
//                 } else {
//                     i - stack.last().unwrap() - 1
//                 };
//                 max_area = std::cmp::max(max_area, rectangle_height * rectangle_width);
//             }
//             stack.push(i);
//         }
//         max_area
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(heights: &[i32], expected: i32) {
        assert_eq!(Solution::largest_rectangle_area(heights.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[2, 1, 5, 6, 2, 3], 10)
    }

    #[test]
    fn ex2() {
        test(&[2, 4], 4)
    }

    #[test]
    fn discussion_case1() {
        test(&[4, 3, 5, 5, 9, 2, 8, 4, 7, 2, 3, 8, 3, 5, 4, 7, 9], 34)
    }

    #[test]
    fn discussion_case2() {
        test(&[2, 1, 2], 3)
    }

    #[test]
    fn discussion_case3() {
        test(&[3, 6, 5, 7, 4, 8, 1, 0], 20)
    }

    #[test]
    fn myex2vee() {
        test(&[2, 1, 2, 0], 3)
    }

    #[test]
    fn myex2_flip() {
        test(&[4, 2], 4)
    }

    #[test]
    fn myex9() {
        test(&[4, 2, 4, 2], 8)
    }

    #[test]
    fn myex0() {
        test(&[0], 0)
    }

    #[test]
    fn myex0_plus() {
        test(&[0, 1], 1)
    }

    #[test]
    fn myex1() {
        test(&[1], 1)
    }

    #[test]
    fn myex2_1() {
        test(&[1, 1], 2)
    }

    #[test]
    fn myex2_2() {
        test(&[2], 2)
    }

    #[test]
    fn my_extreme_ex1() {
        test(
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].repeat(1000).as_slice(),
            10000,
        )
    }

    #[test]
    fn my_extreme_ex1_rev() {
        test(
            [10, 9, 8, 7, 6, 5, 4, 3, 2, 1].repeat(1000).as_slice(),
            10000,
        )
    }

    #[test]
    fn my_extreme_ex2() {
        test((0..10_000).collect::<Vec<_>>().as_slice(), 25_000_000)
    }

    #[test]
    fn my_extreme_ex2_rev() {
        test((0..10_000).rev().collect::<Vec<_>>().as_slice(), 25_000_000)
    }
}
