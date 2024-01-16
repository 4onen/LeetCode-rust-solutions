// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/

pub struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.into_iter()
            .map(|row| row.bytes().filter(|&b| b == b'1').count() as u16)
            .filter(|&ones| ones > 0)
            .scan(0u16, |last_row, this_row| {
                let result = this_row as i32 * *last_row as i32;
                *last_row = this_row;
                Some(result)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::number_of_beams(vec![
                "011001".to_string(),
                "000000".to_string(),
                "010100".to_string(),
                "001000".to_string(),
                "000000".to_string(),
            ]),
            8
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::number_of_beams(vec![
                "000".to_string(),
                "111".to_string(),
                "000".to_string(),
            ]),
            0
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::number_of_beams(vec![
                "000000".to_string(),
                "000000".to_string(),
                "000000".to_string(),
                "100000".to_string(),
                "000000".to_string(),
            ]),
            0
        );
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::number_of_beams(vec![
                "000001".to_string(),
                "000000".to_string(),
                "000000".to_string(),
                "100000".to_string(),
                "000000".to_string(),
                "000001".to_string(),
            ]),
            2
        );
    }
}
