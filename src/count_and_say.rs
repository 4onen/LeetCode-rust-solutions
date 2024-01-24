// https://leetcode.com/problems/count-and-say/

pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        const fn count_and_say_lut(n: u8) -> Option<&'static str> {
            match n {
                1 => Some("1"),
                2 => Some("11"),
                3 => Some("21"),
                4 => Some("1211"),
                5 => Some("111221"),
                6 => Some("312211"),
                7 => Some("13112221"),
                _ => None,
            }
        }
        if n < 1 {
            return String::new();
        } else if n < 8 {
            return count_and_say_lut(n as u8).unwrap().to_string();
        } else {
            let mut prev = count_and_say_lut(7).unwrap().to_string();
            for _ in 8..=n {
                let mut next = String::with_capacity(prev.len() * 2);
                let mut bytes = prev.bytes();
                let mut prev_char = bytes.next().unwrap();
                let mut count = 1;
                for c in bytes {
                    if c == prev_char {
                        count += 1;
                    } else {
                        next.push_str(&count.to_string());
                        next.push(prev_char as char);
                        prev_char = c;
                        count = 1;
                    }
                }
                next.push_str(&count.to_string());
                next.push(prev_char as char);
                prev = next;
            }
            prev
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::count_and_say(1), "1");
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::count_and_say(4), "1211");
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::count_and_say(2), "11");
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::count_and_say(3), "21");
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::count_and_say(5), "111221");
    }

    #[test]
    fn case6() {
        assert_eq!(Solution::count_and_say(6), "312211");
    }

    #[test]
    fn case7() {
        assert_eq!(Solution::count_and_say(7), "13112221");
    }

    #[test]
    fn case8() {
        assert_eq!(Solution::count_and_say(8), "1113213211");
    }

    #[test]
    fn case9() {
        assert_eq!(Solution::count_and_say(9), "31131211131221");
    }
}
