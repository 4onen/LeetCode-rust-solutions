// https://leetcode.com/problems/reformat-date/

pub struct Solution;

impl Solution {
    pub fn reformat_date(date: String) -> String {
        const MONTHS: [&str; 12] = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let mut string_parts = date.split_whitespace();
        let day: u8 = string_parts
            .next()
            .unwrap()
            .trim_end_matches(|c: char| c.is_ascii_alphabetic())
            .parse()
            .unwrap();
        let month = string_parts.next().unwrap();
        let month_num = MONTHS.iter().position(|&mon| mon == month).unwrap() + 1;
        let year = string_parts.next().unwrap();
        format!("{:04}-{:02}-{:02}", year, month_num, day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(date: &str, expected: &str) {
        assert_eq!(Solution::reformat_date(date.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test("20th Oct 2052", "2052-10-20")
    }

    #[test]
    fn ex2() {
        test("6th Jun 1933", "1933-06-06")
    }

    #[test]
    fn ex3() {
        test("26th May 1960", "1960-05-26")
    }
}
