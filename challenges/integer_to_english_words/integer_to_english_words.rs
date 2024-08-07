// https://leetcode.com/problems/integer-to-english-words/

pub struct Solution;

const LESS_THAN_21: [&str; 21] = [
    "Zero",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
    "Twenty",
];
const TENS: [&str; 10] = [
    "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];
const THOUSANDS: [&str; 4] = ["", "Thousand", "Million", "Billion"];
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num < 0 {
            unreachable!();
        }
        if num == 0 {
            return "Zero".to_string();
        }
        fn convert_hundreds(num: i32) -> Vec<&'static str> {
            match num {
                0 => vec![],
                1..=20 => vec![LESS_THAN_21[num as usize]],
                21..=99 if num % 10 == 0 => vec![TENS[num as usize / 10]],
                21..=99 => vec![TENS[num as usize / 10], LESS_THAN_21[num as usize % 10]],
                100..=999 => vec![LESS_THAN_21[num as usize / 100], "Hundred"]
                    .into_iter()
                    .chain(convert_hundreds(num % 100).into_iter())
                    .collect::<Vec<&str>>(),
                _ => unreachable!(),
            }
        }
        let mut num = num;
        let mut words = std::vec::Vec::new();
        let mut i = 0;
        while num > 0 {
            if num % 1000 != 0 {
                let mut words_part = convert_hundreds(num % 1000);
                if i > 0 {
                    words_part.push(THOUSANDS[i]);
                }
                words.push(words_part);
            }
            num /= 1000;
            i += 1;
        }
        words.into_iter().rev().flatten().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(num: u32, expected: &str) {
        assert!(num <= 0x7FFFFFFF);
        assert_eq!(Solution::number_to_words(num as i32), expected, "num: {}", num)
    }

    #[test]
    fn ex1() {
        test(123, "One Hundred Twenty Three");
    }

    #[test]
    fn ex2() {
        test(12345, "Twelve Thousand Three Hundred Forty Five");
    }

    #[test]
    fn ex3() {
        test(
            1234567,
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven",
        );
    }

    #[test]
    fn myex1() {
        test(1000000, "One Million");
    }

    #[test]
    fn less_than_20() {
        for i in 0..21 {
            test(i, LESS_THAN_21[i as usize]);
        }
    }

    #[test]
    fn tens() {
        for i in (20..100).step_by(10) {
            test(i, TENS[i as usize / 10]);
        }
    }

    #[test]
    fn myex100() {
        test(100, "One Hundred");
    }

    #[test]
    fn myex101() {
        test(101, "One Hundred One");
    }

    #[test]
    fn myex201() {
        test(201, "Two Hundred One");
    }

    #[test]
    fn myex1000() {
        test(1000, "One Thousand");
    }

    #[test]
    fn myex1001() {
        test(1001, "One Thousand One");
    }

    #[test]
    fn myex1000000() {
        test(1000000, "One Million");
    }

    #[test]
    fn myex1000001() {
        test(1000001, "One Million One");
    }

    #[test]
    fn myex1001001() {
        test(1001001, "One Million One Thousand One");
    }

    #[test]
    fn myex1001101() {
        test(1001101, "One Million One Thousand One Hundred One");
    }

    #[test]
    fn myex2000() {
        test(2000, "Two Thousand");
    }

    #[test]
    fn myex2002() {
        test(2002, "Two Thousand Two");
    }

    #[test]
    fn myex2000000() {
        test(2000000, "Two Million");
    }

    #[test]
    fn myex2000002() {
        test(2000002, "Two Million Two");
    }

    #[test]
    fn myex2002002() {
        test(2002002, "Two Million Two Thousand Two");
    }

    #[test]
    fn myex2002202() {
        test(2002202, "Two Million Two Thousand Two Hundred Two");
    }
}
