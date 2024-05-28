// https://leetcode.com/problems/get-equal-substrings-within-budget/

pub struct Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        assert!(max_cost >= 0);
        let max_cost = max_cost as u32;
        fn cost(c1: u8, c2: u8) -> u8 {
            (c1 as i8 - c2 as i8).abs() as u8
        }
        let costs: Vec<_> = std::iter::zip(s.bytes(), t.bytes())
            .map(|(c1, c2)| cost(c1, c2))
            .collect();
        let mut max_len = 0;
        let mut cost: u32 = 0;
        let mut i = 0;
        let mut j = 0;
        while j < s.len() {
            cost += costs[j] as u32;
            while i < j && cost > max_cost {
                cost -= costs[i] as u32;
                i += 1;
            }
            if cost <= max_cost {
                max_len = std::cmp::max(max_len, j - i + 1);
            }
            j += 1;
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(s: &str, t: &str, max_cost: i32, expected: i32) {
        assert!(max_cost >= 0);
        assert!(max_cost <= 100_000);
        assert_eq!(
            Solution::equal_substring(s.to_string(), t.to_string(), max_cost),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("abcd", "bcdf", 3, 3);
    }

    #[test]
    fn ex2() {
        test("abcd", "cdef", 3, 1);
    }

    #[test]
    fn ex3() {
        test("abcd", "acde", 0, 1);
    }

    #[test]
    fn myex1() {
        test("zbcd", "acde", 0, 0);
    }

    #[test]
    fn discussion_case1() {
        test("krrgw", "zjxss", 19, 2);
    }
}
