// https://leetcode.com/problems/evaluate-reverse-polish-notation/

pub struct Solution;

// Braindead sol'n
// impl Solution {
//     pub fn eval_rpn(tokens: Vec<String>) -> i32 {
//         let mut operands = Vec::new();
//         for token in tokens {
//             match token.as_str() {
//                 "+" => {
//                     let a = operands.pop().unwrap();
//                     let b = operands.pop().unwrap();
//                     operands.push(a + b);
//                 }
//                 "-" => {
//                     let a = operands.pop().unwrap();
//                     let b = operands.pop().unwrap();
//                     operands.push(b - a);
//                 }
//                 "*" => {
//                     let a = operands.pop().unwrap();
//                     let b = operands.pop().unwrap();
//                     operands.push(a * b);
//                 }
//                 "/" => {
//                     let a = operands.pop().unwrap();
//                     let b = operands.pop().unwrap();
//                     operands.push(b / a);
//                 }
//                 _ => {
//                     operands.push(token.parse::<i32>().unwrap());
//                 }
//             }
//         }
//         operands.pop().unwrap()
//     }
// }

// Optimized sol'n
// impl Solution {
//     pub fn eval_rpn(tokens: Vec<String>) -> i32 {
//         let mut operands = Vec::new();
//         for token in tokens {
//             let tokstr = token.as_str();
//             let c = if let Ok(c) = tokstr.parse::<i32>() {
//                 c
//             } else {
//                 let a = operands.pop().unwrap();
//                 let b = operands.pop().unwrap();
//                 match tokstr {
//                     "+" => a + b,
//                     "-" => b - a,
//                     "*" => a * b,
//                     "/" => b / a,
//                     _ => unreachable!("Invalid input"),
//                 }
//             };
//             operands.push(c);
//         }
//         operands[0]
//     }
// }

// Galaxy brain sol'n
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut operands = Vec::new();
        for token in tokens {
            let bytes = token.as_bytes();
            let c = match bytes.len() {
                1 => {
                    let byte = bytes[0];
                    match bytes[0] {
                        b'0'..=b'9' => (byte - b'0') as i32,
                        _ => {
                            let a = operands.pop().unwrap();
                            let b = operands.pop().unwrap();
                            match byte {
                                b'+' => a + b,
                                b'-' => b - a,
                                b'*' => a * b,
                                b'/' => b / a,
                                _ => unreachable!("Invalid input"),
                            }
                        }
                    }
                }
                2 => {
                    // This is either
                    // 1. a negative 1 digit number
                    // 2. a 2 digit number
                    if bytes[0] == b'-' {
                        -((bytes[1] - b'0') as i32)
                    } else {
                        ((bytes[0] - b'0') * 10 + (bytes[1] - b'0')) as i32
                    }
                }
                3 => {
                    // This is either
                    // 1. a negative 2 digit number
                    // 2. a 3 digit number
                    if bytes[0] == b'-' {
                        -(((bytes[1] - b'0') * 10 + (bytes[2] - b'0')) as i32)
                    } else {
                        ((bytes[0] - b'0') * 100) as i32
                            + ((bytes[1] - b'0') * 10 + (bytes[2] - b'0')) as i32
                    }
                }
                4 => {
                    // This can only be a negative 3 digit number
                    -(((bytes[1] - b'0') * 100) as i32
                        + ((bytes[2] - b'0') * 10 + (bytes[3] - b'0')) as i32)
                }
                _ => unreachable!("Invalid input"),
            };
            operands.push(c);
        }
        operands[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_to_tokens(input: &[&str]) -> Vec<String> {
        input.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn ex1() {
        const INPUT: &[&str] = &["2", "1", "+", "3", "*"];
        let tokens = input_to_tokens(INPUT);
        assert_eq!(Solution::eval_rpn(tokens), 9);
    }

    #[test]
    fn ex2() {
        const INPUT: &[&str] = &["4", "13", "5", "/", "+"];
        let tokens = input_to_tokens(INPUT);
        assert_eq!(Solution::eval_rpn(tokens), 6);
    }

    #[test]
    fn ex3() {
        const INPUT: &[&str] = &[
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];
        let tokens = input_to_tokens(INPUT);
        assert_eq!(Solution::eval_rpn(tokens), 22);
    }
}
