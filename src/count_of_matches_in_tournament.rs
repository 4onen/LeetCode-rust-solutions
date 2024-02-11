// https://leetcode.com/problems/count-of-matches-in-tournament/

pub struct Solution;

impl Solution {
    pub const fn number_of_matches(n: i32) -> i32 {
        let halved = n >> 1;
        if n <= 1 {
            0
        } else {
            halved + Self::number_of_matches(halved + ((n & 1 == 1) as i32))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::number_of_matches(7), 6);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::number_of_matches(14), 13);
    }

    #[test]
    fn champions() {
        assert_eq!(Solution::number_of_matches(1), 0);
    }

    #[test]
    fn duel() {
        assert_eq!(Solution::number_of_matches(2), 1);
    }

    #[test]
    fn cheaters() {
        assert_eq!(Solution::number_of_matches(3), 2);
    }

    #[test]
    fn doubles() {
        assert_eq!(Solution::number_of_matches(4), 3);
    }
}
