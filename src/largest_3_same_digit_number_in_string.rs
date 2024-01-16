// https://leetcode.com/problems/largest-3-same-digit-number-in-string/

pub struct Solution;

// Iterator solution
// impl Solution {
//     pub fn largest_good_integer(num: String) -> String {
//         num.as_bytes()
//             .windows(3)
//             .fold(None, |acc, x| {
//                 if x[0] == x[1] && x[1] == x[2] {
//                     let x = x[0];
//                     match acc {
//                         Some(y) if y >= x => Some(y),
//                         _ => Some(x),
//                     }
//                 } else {
//                     acc
//                 }
//             })
//             .map(|x| unsafe{String::from_utf8_unchecked(vec![x; 3])})
//             .unwrap_or("".to_string())
//     }
// }

// Raw loop solution
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut max = None;
        let mut i = 0;
        let num = num.as_bytes();
        while i < num.len() - 2 {
            if num[i + 1] != num[i + 2] {
                i += 2;
            } else if num[i] != num[i + 1] {
                i += 1;
            } else {
                let x = num[i];
                match max {
                    Some(y) if y >= x => {}
                    _ => max = Some(x),
                }
                i += 3;
            }
        }
        match max {
            Some(x) => unsafe { String::from_utf8_unchecked(vec![x; 3]) },
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::largest_good_integer("6777133339".to_string()),
            "777".to_string()
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::largest_good_integer("2300019".to_string()),
            "000".to_string()
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::largest_good_integer("42352338".to_string()),
            "".to_string()
        );
    }
}
