// https://leetcode.com/problems/zigzag-conversion/

pub struct Solution;

// Braindead sol'n
// impl Solution {
//     pub fn convert(s: String, num_rows: i32) -> String {
//         assert!(num_rows > 0);
//         if num_rows <= 1 {
//             return s;
//         }
//         let mut rows: Vec<Vec<u8>> = vec![vec![]; num_rows as usize];
//         let mut row = 0;
//         let mut down = true;
//         for c in s.bytes() {
//             rows[row].push(c);
//             if down {
//                 row += 1;
//             } else {
//                 row -= 1;
//             }
//             if row >= num_rows as usize - 1 {
//                 down = false;
//             } else if row <= 0 {
//                 down = true;
//             }
//         }
//         unsafe { std::string::String::from_utf8_unchecked(rows.into_iter().flatten().collect()) }
//     }
// }

// Faster concat using vec of strings as rows
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        assert!(num_rows > 0);
        if num_rows <= 1 {
            return s;
        }
        let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
        let mut row: i16 = 0;
        let mut down = true;
        for c in s.chars() {
            rows[row as usize].push(c);
            row += if down { 1 } else { -1 };
            if row as i32 >= (num_rows - 1) {
                down = false;
            } else if row <= 0 {
                down = true;
            }
        }
        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
    }

    #[test]
    fn discussion_ex1() {
        assert_eq!(Solution::convert("ABC".to_string(), 2), "ACB".to_string());
    }

    #[test]
    fn myex4() {
        assert_eq!(Solution::convert("AB".to_string(), 1), "AB".to_string());
    }
}
