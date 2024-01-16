// https://leetcode.com/problems/add-binary/

pub struct Solution;

// Iterator sol'n
// impl Solution {
//     pub fn add_binary(a: String, b: String) -> String {
//         let mut aiter = a.bytes().rev();
//         let mut biter = b.bytes().rev();

//         let mut result = (0..std::cmp::max(a.len(), b.len()) + 1)
//             .scan(false, |carry, _pos| match (aiter.next(), biter.next()) {
//                 (None, None) => {
//                     if *carry {
//                         *carry = false;
//                         Some(b'1')
//                     } else {
//                         None
//                     }
//                 }
//                 (Some(b'1'), Some(b'1')) => {
//                     if *carry {
//                         Some(b'1')
//                     } else {
//                         *carry = true;
//                         Some(b'0')
//                     }
//                 }
//                 (Some(b'1'), _) | (_, Some(b'1')) => {
//                     if *carry {
//                         *carry = true;
//                         Some(b'0')
//                     } else {
//                         Some(b'1')
//                     }
//                 }
//                 (Some(b'0'), _) | (_, Some(b'0')) => {
//                     if *carry {
//                         *carry = false;
//                         Some(b'1')
//                     } else {
//                         Some(b'0')
//                     }
//                 }
//                 _ => unreachable!("Invalid binary string"),
//             })
//             .collect::<Vec<u8>>();

//         result.reverse();

//         unsafe { String::from_utf8_unchecked(result) }
//     }
// }

// Vec sol'n
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();

        let mut result = Vec::with_capacity(std::cmp::max(a.len(), b.len()) + 1);
        let mut carry = false;

        let mut apos = a.len();
        let mut bpos = b.len();

        while apos > 0 || bpos > 0 {
            let mut s = carry as u8;
            if apos > 0 {
                apos -= 1;
                s += a[apos] - b'0';
            }
            if bpos > 0 {
                bpos -= 1;
                s += b[bpos] - b'0';
            }
            result.push((s & 1) + b'0');
            carry = s > 1;
        }

        if carry {
            result.push(b'1');
        }

        result.reverse();

        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100"
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101"
        );
    }

    #[test]
    fn myex3() {
        assert_eq!(
            Solution::add_binary("1111".to_string(), "1111".to_string()),
            "11110"
        );
    }

    #[test]
    fn failing_case1() {
        assert_eq!(
            Solution::add_binary("100".to_string(), "110010".to_string()),
            "110110"
        );
    }
}
