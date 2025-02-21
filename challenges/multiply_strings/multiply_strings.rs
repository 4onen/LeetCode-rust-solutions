// https://leetcode.com/problems/multiply-strings/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn multiply(num1: String, num2: String) -> String {
//         let num1 = {
//             let mut num1 = num1.into_bytes();
//             for b in &mut num1 {
//                 *b -= b'0';
//             }
//             num1
//         };
//         let mut res = vec![0u8; num1.len() + num2.len() + 1];
//         for (place, b2) in num2.bytes().rev().enumerate() {
//             let digit2 = b2 - b'0';
//             let mut carry = 0;
//             if digit2 == 0 {
//                 continue;
//             }
//             for (i1, digit1) in num1.iter().rev().copied().enumerate() {
//                 let acc = res.get(i1 + place).copied().unwrap_or(0);
//                 let product = digit2 * digit1 + carry + acc;
//                 let product_ones = product % 10;
//                 carry = product / 10;
//                 res[i1 + place] = product_ones;
//             }
//             if carry > 0 {
//                 res[num1.len() + place] = carry;
//             }
//         }
//         while let Some(0) = res.last().copied() {
//             res.pop();
//         }
//         if res.len() == 0 {
//             res.push(0);
//         }
//         for b in &mut res {
//             *b += b'0';
//         }
//         res.reverse();
//         unsafe { std::string::String::from_utf8_unchecked(res) }
//     }
// }

// Cheat for speed on small numbers (much higher memory use, practically same speed.)
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        // Cheat
        if num1.len() <= 37 && num2.len() <= 37 {
            if let Some(res) = num1.parse::<u128>().ok().and_then(|num1| {
                num2.parse::<u128>()
                    .ok()
                    .and_then(|num2| u128::checked_mul(num1, num2))
            }) {
                return res.to_string();
            }
        }
        // Proper
        let num1 = {
            let mut num1 = num1.into_bytes();
            for b in &mut num1 {
                *b -= b'0';
            }
            num1
        };
        let mut res = vec![0u8; num1.len() + num2.len() + 1];
        for (place, b2) in num2.bytes().rev().enumerate() {
            let digit2 = b2 - b'0';
            let mut carry = 0;
            if digit2 == 0 {
                continue;
            }
            for (i1, digit1) in num1.iter().rev().copied().enumerate() {
                let acc = res.get(i1 + place).copied().unwrap_or(0);
                let product = digit2 * digit1 + carry + acc;
                let product_ones = product % 10;
                carry = product / 10;
                res[i1 + place] = product_ones;
            }
            if carry > 0 {
                res[num1.len() + place] = carry;
            }
        }
        while let Some(0) = res.last().copied() {
            res.pop();
        }
        if res.len() == 0 {
            res.push(0);
        }
        for b in &mut res {
            *b += b'0';
        }
        res.reverse();
        unsafe { std::string::String::from_utf8_unchecked(res) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(num1: &str, num2: &str, expected: &str) {
        assert_eq!(
            Solution::multiply(num1.to_owned(), num2.to_owned()),
            expected,
            "Trying {} x {}",
            num1,
            num2
        );
    }

    #[test]
    fn ex1() {
        test("2", "3", "6")
    }

    #[test]
    fn ex2() {
        test("123", "456", "56088")
    }

    #[test]
    fn my_times_table() {
        for i in 0..=100 {
            for j in i..=1000 {
                test(&i.to_string(), &j.to_string(), &(i * j).to_string());
                test(&j.to_string(), &i.to_string(), &(i * j).to_string());
            }
        }
    }

    #[test]
    fn myex1() {
        test("123456789", "456789123", "56393718375706047")
    }
}
