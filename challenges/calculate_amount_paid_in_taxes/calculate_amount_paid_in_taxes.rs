// https://leetcode.com/problems/calculate-amount-paid-in-taxes/

pub struct Solution;

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut taxed = 0;
        let mut tax = 0.0f64;
        for bracket_info in brackets {
            let upper_i = bracket_info[0];
            let bracket_top = if income > upper_i { upper_i } else { income };
            let bracket_income = bracket_top - taxed;
            taxed = bracket_top;
            tax += (bracket_income as f64) * (bracket_info[1] as f64);
            if taxed >= income {
                break;
            }
        }
        return tax * 0.01;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(brackets: &[[i32; 2]], income: u16, expected: f64) {
        assert!(brackets.len() >= 1);
        assert!(brackets.len() <= 100);
        let mut last = -1;
        for bracket_info in brackets.iter() {
            assert!(bracket_info[0] >= 1);
            assert!(bracket_info[0] <= 1000);
            assert!(bracket_info[0] > last);
            last = bracket_info[0];
            assert!(bracket_info[1] >= 0);
            assert!(bracket_info[1] <= 100);
        }
        assert!(income <= brackets.last().unwrap()[0] as u16);
        let res = Solution::calculate_tax(
            brackets.iter().map(|&x| x.to_vec()).collect(),
            income as i32,
        );
        assert!(
            (res - expected).abs() < 0.00001,
            "\nExpected: {}\nActual: {}",
            expected,
            res
        );
    }

    #[test]
    fn ex1() {
        test(&[[3, 50], [7, 10], [12, 25]], 10, 2.65000)
    }

    #[test]
    fn ex2() {
        test(&[[1, 0], [4, 25], [5, 50]], 2, 0.25000)
    }

    #[test]
    fn ex3() {
        test(&[[2, 50]], 0, 0.0)
    }
}
