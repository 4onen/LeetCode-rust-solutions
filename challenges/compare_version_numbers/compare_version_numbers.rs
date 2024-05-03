// https://leetcode.com/problems/compare-version-numbers/

pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut iter1 = version1.split('.').map(|s| s.parse::<u32>().unwrap());
        let mut iter2 = version2.split('.').map(|s| s.parse::<u32>().unwrap());
        loop {
            match (iter1.next(), iter2.next()) {
                (None, None) => break 0,
                (None, Some(0)) => (),
                (Some(0), None) => (),
                (None, Some(_)) => break -1,
                (Some(_), None) => break 1,
                (Some(a), Some(b)) => {
                    if a < b {
                        break -1;
                    } else if a > b {
                        break 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cmp::Ordering;
    use Ordering::*;

    fn valid_version_number(v: &str) -> bool {
        assert!(v.len() > 0);
        assert!(v.len() <= 500);
        assert!(v.chars().all(|c| c.is_ascii_digit() || c == '.'));
        for part in v.split('.') {
            if part.parse::<i32>().is_err() {
                return false;
            }
        }
        true
    }

    fn test(v1: &str, v2: &str, output: Ordering) {
        assert!(valid_version_number(v1));
        assert!(valid_version_number(v2));
        let result = Solution::compare_version(v1.to_string(), v2.to_string());
        let result = match result {
            -1 => Less,
            0 => Equal,
            1 => Greater,
            n => panic!("Invalid output: {}", n),
        };
        assert_eq!(result, output)
    }

    #[test]
    fn ex1() {
        test("1.01", "1.001", Equal)
    }

    #[test]
    fn ex2() {
        test("1.0", "1.0.0", Equal)
    }

    #[test]
    fn ex3() {
        test("0.1", "1.1", Less)
    }

    #[test]
    fn myex1() {
        test("1.0.1", "1", Greater)
    }

    #[test]
    fn myex2() {
        test("1.0.1", "1.0.1", Equal)
    }

    #[test]
    fn myex3() {
        test("1.0.1", "1.0.2", Less)
    }

    #[test]
    fn myex4() {
        test("1.0.1", "1.0.0", Greater)
    }

    #[test]
    fn discussion_case1() {
        test("1.2", "1.10", Less)
    }

    #[test]
    fn discussion_case2() {
        test("1", "0", Greater)
    }

    #[test]
    fn discussion_case_big() {
        test(
            "19.8.3.17.5.01.0.0.4.0.0.0.0.0.0.0.0.0.0.0.0.0.00.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.000000.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.000000", 
            "19.8.3.17.5.01.0.0.4.0.0.0.0.0.0.0.0.0.0.0.0.0.00.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.000000.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.000001",
            Less
        )
    }
}
