// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/

pub struct Solution;

// Initial brute-force sol'n
impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len() as u16;
        for i in 0..n {
            let mut j = i + 1;
            let discount = loop {
                if j >= n {
                    break 0;
                }
                if prices[j as usize] <= prices[i as usize] {
                    break prices[j as usize];
                }
                j += 1;
            };
            prices[i as usize] -= discount;
        }
        prices
    }
}

// Monotonic stack sol'n
// impl Solution {
//     pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
//         let n = prices.len() as u16;
//         let mut monotonic_stack = std::vec::Vec::with_capacity(n as usize);
//         for i in (0..n).rev() {
//             let el = prices[i as usize];
//             let discount = loop {
//                 if monotonic_stack.len() < 1 {
//                     break 0;
//                 }
//                 let top = *monotonic_stack.last().unwrap();
//                 if top > el {
//                     _ = monotonic_stack.pop();
//                 } else {
//                     break top;
//                 }
//             };
//             monotonic_stack.push(el);
//             prices[i as usize] = el - discount;
//         }
//         prices
//     }
// }

// Monotonic stack sol'n with compressed dtypes
// impl Solution {
//     pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
//         let n = prices.len() as u16;
//         assert!(n <= 500);
//         let mut monotonic_stack = std::vec::Vec::with_capacity(n as usize);
//         for i in (0..n).rev() {
//             let el = prices[i as usize];
//             assert!(el > 0);
//             assert!(el <= 1000);
//             let el = el as u16;
//             let discount = loop {
//                 if monotonic_stack.len() < 1 {
//                     break 0;
//                 }
//                 let top = *monotonic_stack.last().unwrap();
//                 if top > el {
//                     _ = monotonic_stack.pop();
//                 } else {
//                     break top;
//                 }
//             };
//             monotonic_stack.push(el);
//             prices[i as usize] = (el - discount) as i32;
//         }
//         prices
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(prices: &[i32], expected: &[i32]) {
        assert!(prices.len() >= 1);
        assert!(prices.len() <= 500);
        for &price in prices {
            assert!(price >= 1);
            assert!(price <= 1000);
        }
        assert_eq!(Solution::final_prices(prices.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[8, 4, 6, 2, 3], &[4, 2, 4, 2, 3])
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5])
    }

    #[test]
    fn ex3() {
        test(&[10, 1, 1, 6], &[9, 0, 1, 6])
    }
}
