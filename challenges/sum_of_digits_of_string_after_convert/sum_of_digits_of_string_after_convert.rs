// https://leetcode.com/problems/sum-of-digits-of-string-after-convert/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn get_lucky(s: String, k: i32) -> i32 {
//         let mut nprev = 0;
//         for b in s.bytes() {
//             match b - b'a' + 1 {
//                 n @ (1..=9) => nprev += n as i32,
//                 n @ (10..=19) => nprev += 1 + n as i32 - 10,
//                 n @ (20..=26) => nprev += 2 + n as i32 - 20,
//                 0 => unreachable!(),
//                 27.. => unreachable!(),
//             }
//         }
//         for _ in 1..k {
//             let mut nnext = 0;
//             while nprev > 0 {
//                 nnext += nprev % 10;
//                 nprev /= 10;
//             }
//             nprev = nnext;
//         }
//         nprev
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn get_lucky(s: String, mut k: i32) -> i32 {
        let mut nprev = 0;
        for b in s.bytes() {
            match b - b'a' + 1 {
                n @ (1..=9) => nprev += n as i32,
                n @ (10..=19) => nprev += -9 + n as i32,
                n @ (20..=26) => nprev += -18 + n as i32,
                _ => unreachable!(),
            }
        }
        while k > 1 {
            k -= 1;
            let mut nnext = 0;
            while nprev > 0 {
                let div = nprev / 10;
                let rem = nprev % 10;
                nnext += rem;
                nprev = div;
            }
            nprev = nnext;
        }
        nprev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, k: i32, expected: i32) {
        assert_eq!(Solution::get_lucky(s.to_owned(), k), expected);
    }

    #[test]
    fn ex1() {
        test("iiii", 1, 36)
    }

    #[test]
    fn ex2() {
        test("leetcode", 2, 6)
    }

    #[test]
    fn ex3() {
        test("zbax", 2, 8)
    }
}
