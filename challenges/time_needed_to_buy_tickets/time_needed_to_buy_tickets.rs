// https://leetcode.com/problems/time-needed-to-buy-tickets/

pub struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        assert!(tickets.len() <= 100);
        assert!(k >= 0);
        assert!(k < tickets.len() as i32);
        let k = k as u8;
        let k_wants = tickets[k as usize];
        let mut time = k_wants;
        for i in 0..k {
            time += std::cmp::min(tickets[i as usize], k_wants);
        }
        let k_wants_sub_1 = k_wants - 1;
        for i in k + 1..tickets.len() as u8 {
            time += std::cmp::min(tickets[i as usize], k_wants_sub_1);
        }
        time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(tickets: &[i32], k: i32, expected: i32) {
        assert_eq!(
            Solution::time_required_to_buy(tickets.to_vec(), k),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[2, 3, 2], 2, 6);
    }

    #[test]
    fn ex2() {
        test(&[5, 1, 1, 1], 0, 8);
    }

    #[test]
    fn ex3() {
        test(&[1, 1], 1, 2);
    }
}
