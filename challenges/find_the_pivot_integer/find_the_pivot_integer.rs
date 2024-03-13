// https://leetcode.com/problems/find-the-pivot-integer/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn pivot_integer_v1(n: i32) -> i32 {
//         match n {
//             i32::MIN..=0 => unreachable!("n must be positive"),
//             1 => 1,
//             2..=1000 => {
//                 let prefix_sum: Vec<u16> = (1..=n as u16)
//                     .scan(0u16, |a, b| {
//                         *a += b;
//                         Some(*a)
//                     })
//                     .collect();
//                 for x in 1..n {
//                     let sum_until_x = prefix_sum[x as usize];
//                     let sum_x_to_n =
//                         *(prefix_sum.last().unwrap()) - prefix_sum[x as usize - 1usize];
//                     if sum_until_x == sum_x_to_n {
//                         return (x + 1) as i32;
//                     }
//                 }
//                 -1
//             }
//             _ => unreachable!("n must be less than or equal to 1000"),
//         }
//     }
// }

// Arithmetic sol'n
// impl Solution {
//     pub fn pivot_integer(n: i32) -> i32 {
//         match n {
//             i32::MIN..=0 => unreachable!("n must be positive"),
//             1 => 1,
//             2..=1000 => {
//                 let n = n as u32;
//                 let sum: u32 = n * (n + 1) / 2;
//                 let pivot = (sum as f64).sqrt() as u32;
//                 if pivot * pivot == sum {
//                     pivot as i32
//                 } else {
//                     -1
//                 }
//             }
//             _ => unreachable!("n must be less than or equal to 1000"),
//         }
//     }
// }

// Faster (const) sol'n
// impl Solution {
//     pub const fn pivot_integer(n: i32) -> i32 {
//         match n {
//             i32::MIN..=0 => panic!("n must be positive"),
//             1 => 1,
//             2..=1000 => {
//                 let mut n = n;
//                 let mut sum = n * (n + 1) / 2;
//                 let mut sum_x_to_n = n;
//                 loop {
//                     if n <= 0 {
//                         break -1;
//                     }
//                     if sum == sum_x_to_n {
//                         break n;
//                     }
//                     sum -= n;
//                     n -= 1;
//                     sum_x_to_n += n;
//                 }
//             }
//             _ => panic!("n must be less than or equal to 1000"),
//         }
//     }
// }

// Fastest (precomputed const) sol'n
impl Solution {
    const LUT: [i16; 1001] = {
        let mut lut = [-1i16; 1001];
        let mut i = 1;
        while i < 1001 {
            let mut n = i;
            let mut sum = n * (n + 1) / 2;
            let mut sum_x_to_n = n;
            lut[i] = loop {
                if n <= 0 {
                    break -1;
                }
                if sum == sum_x_to_n {
                    break n as i16;
                }
                sum -= n;
                n -= 1;
                sum_x_to_n += n;
            };
            i += 1;
        }
        lut
    };
    pub const fn pivot_integer(n: i32) -> i32 {
        Self::LUT[n as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::pivot_integer(8), 6)
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::pivot_integer(1), 1)
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::pivot_integer(4), -1)
    }
}
