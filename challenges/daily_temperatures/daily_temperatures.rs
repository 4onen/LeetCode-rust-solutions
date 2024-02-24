// https://leetcode.com/problems/daily-temperatures/

pub struct Solution;

// Simple stack-based sol'n
// impl Solution {
//     pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//         let mut stack: Vec<u32> = vec![0];
//         let mut result = vec![0; temperatures.len()];
//         for i in 1..temperatures.len() as u32 {
//             while !stack.is_empty()
//                 && temperatures[i as usize] > temperatures[*stack.last().unwrap() as usize]
//             {
//                 let idx = stack.pop().unwrap();
//                 result[idx as usize] = (i - idx) as i32;
//             }
//             stack.push(i as u32);
//         }
//         result
//     }
// }

// Lookup-avoidant stack-based sol'n
// impl Solution {
//     pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//         let mut stack: Vec<(u32, u8)> = vec![];
//         let mut result = vec![0; temperatures.len()];
//         for i in 0..temperatures.len() as u32 {
//             let this_temp = temperatures[i as usize] as u8;
//             while !stack.is_empty() && this_temp > stack.last().unwrap().1 {
//                 let (idx, _) = stack.pop().unwrap();
//                 result[idx as usize] = (i - idx) as i32;
//             }
//             stack.push((i, this_temp));
//         }
//         result
//     }
// }

// Further optimized sol'n
// impl Solution {
//     pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//         let mut stack: Vec<(u32, u8)> = vec![];
//         let mut result = vec![0; temperatures.len()];
//         for i in 0..temperatures.len() as u32 {
//             let this_temp = temperatures[i as usize] as u8;
//             if this_temp >= 100 {
//                 for (idx, _) in stack.drain(..) {
//                     result[idx as usize] = (i - idx) as i32;
//                 }
//                 // i remains at 0 because no day is hotter than 100
//             } else {
//                 let mut j = stack.len();
//                 while j > 0 && this_temp > stack[j - 1].1 {
//                     let idx = stack[j - 1].0;
//                     result[idx as usize] = (i - idx) as i32;
//                     j -= 1;
//                 }
//                 stack.truncate(j);
//                 stack.push((i, this_temp));
//             }
//         }
//         result
//     }
// }

// Reverse iteration sol'n
// impl Solution {
//     pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//         let mut stack: Vec<u32> = vec![];
//         let mut result: Vec<i32> = vec![0; temperatures.len()];
//         for i in (0..temperatures.len() as u32).rev() {
//             let this_temp = temperatures[i as usize];
//             let mut j = stack.len() as u32;
//             while j > 0 && this_temp >= temperatures[stack[(j - 1) as usize] as usize] {
//                 j -= 1;
//             }
//             stack.truncate(j as usize);
//             result[i as usize] = if let Some(&idx) = stack.last() {
//                 (idx - i) as i32
//             } else {
//                 0
//             };
//             stack.push(i);
//         }
//         result
//     }
// }

// Reverse iteration lookup avoidant sol'n
// impl Solution {
//     pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//         let mut stack: Vec<(u32, u8)> = vec![];
//         let mut result: Vec<i32> = vec![0; temperatures.len()];
//         for i in (0..temperatures.len() as u32).rev() {
//             let this_temp = temperatures[i as usize] as u8;
//             let mut j = stack.len() as u32;
//             while j > 0 && this_temp >= stack[(j - 1) as usize].1 {
//                 j -= 1;
//             }
//             stack.truncate(j as usize);
//             result[i as usize] = if let Some(&(idx, _)) = stack.last() {
//                 (idx - i) as i32
//             } else {
//                 0
//             };
//             stack.push((i, this_temp));
//         }
//         result
//     }
// }

// Reverse iteration bit packed sol'n
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        #[derive(Debug, Clone, Copy)]
        struct StackEntry {
            pack: u32, // idx: u24 upper, temp: u8 lower
        }
        impl StackEntry {
            fn new(idx: u32, temp: u8) -> Self {
                Self {
                    pack: (idx << 8) | temp as u32,
                }
            }
            fn idx(&self) -> u32 {
                self.pack >> 8
            }
            fn temp(&self) -> u8 {
                self.pack as u8
            }
        }
        let mut stack: Vec<StackEntry> = vec![];
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        for i in (0..temperatures.len() as u32).rev() {
            let this_temp = temperatures[i as usize] as u8;
            let mut j = stack.len() as u32;
            while j > 0 && this_temp >= stack[(j - 1) as usize].temp() {
                j -= 1;
            }
            stack.truncate(j as usize);
            result[i as usize] = if let Some(&entry) = stack.last() {
                (entry.idx() - i) as i32
            } else {
                0
            };
            stack.push(StackEntry::new(i, this_temp));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(
            Solution::daily_temperatures(vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70]),
            vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(
            Solution::daily_temperatures(vec![77, 77, 77, 77, 77, 41, 77, 41, 41, 77]),
            vec![0, 0, 0, 0, 0, 1, 0, 2, 1, 0]
        )
    }

    #[test]
    fn failing_case1() {
        const INPUT: &str = include_str!("daily_temperatures_failing_case1.txt");
        let mut lines = INPUT.lines();
        let input: Vec<i32> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let expected: Vec<i32> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let result = Solution::daily_temperatures(input);

        // We need a custom array comparison because the
        // expected result is too large to be printed in the
        // test output.
        assert_eq!(result.len(), expected.len());
        let mut differing_index_ranges = vec![];
        for i in 0..result.len() {
            if result[i] != expected[i] {
                match differing_index_ranges.last_mut() {
                    Some((_, end)) if *end == i - 1 => *end = i,
                    _ => differing_index_ranges.push((i, i)),
                }
            }
        }
        assert!(
            differing_index_ranges.is_empty(),
            "Differing indices: {:?}",
            differing_index_ranges
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 30, 30, 31, 31, 32, 32]),
            vec![3, 2, 1, 2, 1, 0, 0]
        );
    }
}
