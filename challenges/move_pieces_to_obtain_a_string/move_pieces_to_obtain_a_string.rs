// https://leetcode.com/problems/move-pieces-to-obtain-a-string/

pub struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let start = start.as_bytes();
        let target = target.as_bytes();
        let mut start_idx = 0;
        let mut target_idx = 0;
        while start_idx < start.len() && target_idx < target.len() {
            match (start[start_idx], target[target_idx]) {
                (b'L', b'L') => {
                    if target_idx > start_idx {
                        return false;
                    }
                    start_idx += 1;
                    target_idx += 1;
                }
                (b'R', b'R') => {
                    if start_idx > target_idx {
                        return false;
                    }
                    start_idx += 1;
                    target_idx += 1;
                }
                (b'_', b'_') => {
                    start_idx += 1;
                    target_idx += 1;
                }
                (b'R', b'L') | (b'L', b'R') => {
                    return false;
                }
                (b'_', b'L') => {
                    start_idx += 1;
                }
                (b'L', b'_') => {
                    if target_idx < start_idx {
                        target_idx += 1;
                    } else {
                        return false;
                    }
                }
                (b'R', b'_') => {
                    target_idx += 1;
                }
                (b'_', b'R') => {
                    if start_idx < target_idx {
                        start_idx += 1;
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        while target_idx < target.len() {
            if target[target_idx] != b'_' {
                return false;
            }
            target_idx += 1;
        }
        while start_idx < start.len() {
            if start[start_idx] != b'_' {
                return false;
            }
            start_idx += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(start: &str, target: &str, expected: bool) {
        assert!(start.len() == target.len());
        assert!(start.len() >= 1);
        assert!(start.len() <= 100_000);
        for b in start.bytes() {
            assert!(b == b'_' || b == b'L' || b == b'R');
        }
        for b in target.bytes() {
            assert!(b == b'_' || b == b'L' || b == b'R');
        }
        assert_eq!(
            Solution::can_change(start.to_string(), target.to_string()),
            expected
        )
    }

    #[test]
    fn ex1() {
        test("_L__R__R_", "L______RR", true)
    }

    #[test]
    fn ex2() {
        test("R_L_", "__LR", false)
    }

    #[test]
    fn ex3() {
        test("_R", "R_", false)
    }

    #[test]
    fn failing_case1() {
        test("RL_", "_RL", false)
    }

    #[test]
    fn failing_case1_1() {
        test("R_L", "_RL", true)
    }
}
