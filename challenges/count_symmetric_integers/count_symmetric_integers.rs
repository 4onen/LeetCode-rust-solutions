// https://leetcode.com/problems/count-symmetric-integers/

pub struct Solution;

// Naive sol'n: Just iterate over the range and count the symmetric ones.
// impl Solution {
//     pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
//         (low as u16..=high as u16)
//             .filter(|&n| {
//                 let digits = n.ilog10();
//                 if digits % 2 == 0 {
//                     return false;
//                 }
//                 let s = n.to_string();
//                 let sb = s.as_bytes();
//                 let res = {
//                     let sum_firstn = sb
//                         .iter()
//                         .take(sb.len() / 2)
//                         .map(|&b| (b - b'0'))
//                         .sum::<u8>();
//                     let sum_lastn = sb
//                         .iter()
//                         .rev()
//                         .take(sb.len() / 2)
//                         .map(|&b| (b - b'0'))
//                         .sum::<u8>();
//                     sum_firstn == sum_lastn
//                 };
//                 res
//             })
//             .count() as i32
//     }
// }

// Optimization based on included ranges
impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let high = std::cmp::min(high, 9999);
        (low as u16..=high as u16)
            .filter(|&n| {
                if n < 100 && n % 11 == 0 {
                    true
                } else if n >= 1000 {
                    let sum_firstn = n / 1000 + (n % 1000) / 100;
                    let sum_lastn = (n / 10) % 10 + n % 10;
                    sum_firstn == sum_lastn
                } else {
                    false
                }
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(low: u16, high: u16, expected: u16) {
        assert!(low >= 1);
        assert!(high >= low);
        assert!(high <= 10_000);
        assert!(expected <= 1 + high - low);
        assert_eq!(
            Solution::count_symmetric_integers(low as i32, high as i32),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test(1, 100, 9)
    }

    #[test]
    fn ex2() {
        test(1200, 1230, 4)
    }

    #[test]
    fn my_extreme_ex1() {
        test(1, 10_000, 624)
    }
}
