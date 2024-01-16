pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_neg = x.is_negative();
        let x = x
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0);
        if is_neg {
            -x
        } else {
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn run_tests() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}
