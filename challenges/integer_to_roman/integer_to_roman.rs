// https://leetcode.com/problems/integer-to-roman/

pub struct Solution;

// Braindead solution
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut roman = String::new();

        loop {
            if num >= 1000 {
                roman.push_str("M");
                num -= 1000;
            } else if num >= 900 {
                roman.push_str("CM");
                num -= 900;
            } else if num >= 500 {
                roman.push_str("D");
                num -= 500;
            } else if num >= 400 {
                roman.push_str("CD");
                num -= 400;
            } else if num >= 100 {
                roman.push_str("C");
                num -= 100;
            } else if num >= 90 {
                roman.push_str("XC");
                num -= 90;
            } else if num >= 50 {
                roman.push_str("L");
                num -= 50;
            } else if num >= 40 {
                roman.push_str("XL");
                num -= 40;
            } else if num >= 10 {
                roman.push_str("X");
                num -= 10;
            } else if num >= 9 {
                roman.push_str("IX");
                num -= 9;
            } else if num >= 5 {
                roman.push_str("V");
                num -= 5;
            } else if num >= 4 {
                roman.push_str("IV");
                num -= 4;
            } else if num >= 1 {
                roman.push_str("I");
                num -= 1;
            } else {
                break;
            }
        }
        roman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }

    #[test]
    fn discussion_example_set() {
        let romans = [
            ("I", 1),
            ("IV", 4),
            ("V", 5),
            ("IX", 9),
            ("X", 10),
            ("XL", 40),
            ("L", 50),
            ("XC", 90),
            ("C", 100),
            ("CD", 400),
            ("D", 500),
            ("CM", 900),
            ("M", 1000),
        ];

        for (roman, integer) in romans.iter() {
            assert_eq!(
                Solution::int_to_roman(*integer),
                roman.to_string(),
                "{}",
                integer
            );
        }
    }

    #[test]
    fn myex1() {
        // Extrema case
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX".to_string());
    }
}
