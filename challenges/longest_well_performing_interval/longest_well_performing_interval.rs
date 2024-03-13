// https://leetcode.com/problems/longest-well-performing-interval/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn longest_wpi(hours: Vec<i32>) -> i32 {
//         let efforts = hours
//             .into_iter()
//             .map(|hour| if hour > 8 { 1i16 } else { -1i16 });
//         let mut sum = 0;
//         let mut max_len = 0;
//         let mut map = std::collections::HashMap::new();
//         for (i, day_effort) in efforts.enumerate() {
//             sum += day_effort;
//             if sum > 0 {
//                 max_len = i + 1;
//             } else {
//                 map.entry(sum).or_insert(i);
//                 if let Some(&j) = map.get(&(sum - 1)) {
//                     max_len = max_len.max(i - j);
//                 }
//             }
//         }
//         max_len as i32
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut sum: i16 = 0;
        let mut max_len: u16 = 0;
        let mut map = std::collections::HashMap::new();
        for i in 0..hours.len() as u16 {
            let day_effort = if hours[i as usize] > 8 { 1i8 } else { -1i8 };
            sum += day_effort as i16;
            map.entry(sum).or_insert(i);
            if sum > 0 {
                max_len = i + 1;
            } else {
                let key = sum - 1;
                if map.contains_key(&key) {
                    let len = i - map.get(&key).unwrap();
                    if len > max_len {
                        max_len = len;
                    }
                }
            }
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = vec![9, 9, 6, 0, 6, 6, 9];
        let expected = 3;
        assert_eq!(Solution::longest_wpi(input), expected)
    }

    #[test]
    fn ex2() {
        let input = vec![6, 6, 6];
        let expected = 0;
        assert_eq!(Solution::longest_wpi(input), expected)
    }
}
