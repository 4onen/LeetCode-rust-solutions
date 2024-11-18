// https://leetcode.com/problems/defuse-the-bomb/

pub struct Solution;

// Excessive alloc+iter sol'n
// impl Solution {
//     pub fn decrypt(mut code: Vec<i32>, k: i32) -> Vec<i32> {
//         match k.cmp(&0) {
//             std::cmp::Ordering::Equal => {
//                 code.fill(0);
//                 code
//             }
//             std::cmp::Ordering::Greater => {
//                 let code_len = code.len();
//                 let mut extended_code = std::vec::Vec::with_capacity(code_len * 2);
//                 extended_code.extend(code.iter().copied());
//                 extended_code.extend(code.into_iter());
//                 extended_code
//                     .windows(k as usize)
//                     .inspect(|&x| {dbg!(x);})
//                     .skip(1)
//                     .take(code_len)
//                     .map(|x| x.iter().sum())
//                     .collect()
//             }
//             std::cmp::Ordering::Less => {
//                 let code_len = code.len();
//                 let mut extended_code = std::vec::Vec::with_capacity(code_len * 2);
//                 extended_code.extend(code.iter().copied());
//                 extended_code.extend(code.into_iter());
//                 extended_code.reverse();
//                 let mut res: Vec<i32> = extended_code
//                     .windows((-k) as usize)
//                     .inspect(|&x| {dbg!(x);})
//                     .skip(1)
//                     .take(code_len)
//                     .map(|x| x.iter().sum())
//                     .collect();
//                 res.reverse();
//                 res
//             }
//         }
//     }
// }

// Modulo sol'n
impl Solution {
    pub fn decrypt(mut code: Vec<i32>, k: i32) -> Vec<i32> {
        match k.cmp(&0) {
            std::cmp::Ordering::Equal => {
                code.fill(0);
                code
            }
            std::cmp::Ordering::Greater => {
                (0..code.len()).into_iter().map(|i| {
                    (i+1..=i+k as usize).into_iter().map(|j| code[j % code.len()]).sum()
                }).collect()
            }
            std::cmp::Ordering::Less => {
                (0..code.len()).into_iter().map(|i| {
                    let start = i+code.len().wrapping_add(k as usize);
                    let end = i+code.len();
                    (start..end).into_iter().map(|j| code[j % code.len()]).sum()
                }).collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(code: &[u8], k: i8, expected: &[i32]) {
        assert_eq!(code.len(), expected.len());
        assert!(code.len() >= 1);
        assert!(code.len() <= 100);
        for &pt in code {
            assert!(pt >= 1);
            assert!(pt <= 100);
        }
        if k > 0 {
            assert!(k < code.len() as i8);
        } else if k < 0 {
            assert!(k > -(code.len() as i8))
        }
        assert_eq!(
            Solution::decrypt(code.into_iter().map(|&x| x as i32).collect(), k as i32),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[5, 7, 1, 4], 3, &[12, 10, 16, 13])
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4], 0, &[0; 4])
    }

    #[test]
    fn ex3() {
        test(&[2, 4, 9, 3], -2, &[12, 5, 6, 13])
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[100;100],99,&[9900;100])
    }

    #[test]
    fn my_extreme_ex2() {
        test(&[100;100],-99,&[9900;100])
    }

    #[test]
    fn my_extreme_ex3() {
        test(&[100;100],0,&[0;100])
    }
}
