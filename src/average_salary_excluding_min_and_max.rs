// https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/

pub struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let num_salaries = salary.len();
        let mut min = salary[0];
        let mut max = salary[0];
        let mut sum = 0;
        for s in salary {
            sum += s;
            min = min.min(s);
            max = max.max(s);
        }
        (sum - min - max) as f64 / (num_salaries - 2) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        const INPUT: &[i32] = &[4000, 3000, 1000, 2000];
        assert_eq!(Solution::average(INPUT.to_vec()), 2500.00000);
    }

    #[test]
    fn ex2() {
        const INPUT: &[i32] = &[1000, 2000, 3000];
        assert_eq!(Solution::average(INPUT.to_vec()), 2000.00000);
    }

    #[test]
    fn myex1() {
        const INPUT: &[i32] = &[1000000; 100];
        assert_eq!(Solution::average(INPUT.to_vec()), 1000000.00000);
    }
}
