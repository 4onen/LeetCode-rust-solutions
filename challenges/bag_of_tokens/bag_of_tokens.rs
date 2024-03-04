// https://leetcode.com/problems/bag-of-tokens/

pub struct Solution;

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, power: i32) -> i32 {
        assert!(power >= 0);
        assert!(power <= 10_000);
        let mut power = power as u16;
        tokens.sort_unstable();
        let mut score: i16 = 0;
        let mut max_score: i16 = 0;
        let mut tokens_iter = tokens.drain(..).peekable();
        loop {
            let Some(&token) = tokens_iter.peek() else {
                break;
            };
            if power >= token as u16 {
                power -= tokens_iter.next().unwrap() as u16;
                score += 1;
                max_score = std::cmp::max(max_score, score);
            } else if score > 0 {
                power += tokens_iter.next_back().unwrap() as u16;
                score -= 1;
            } else {
                break;
            }
        }
        max_score as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100, 200], 150), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200),
            2
        );
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::bag_of_tokens_score(vec![71, 55, 82], 54), 0);
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(Solution::bag_of_tokens_score(vec![], 7), 0);
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(Solution::bag_of_tokens_score(vec![100, 1, 1, 1], 5), 3);
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(Solution::bag_of_tokens_score(vec![2, 3, 4], 1), 0);
    }

    #[test]
    fn discussion_case5() {
        assert_eq!(Solution::bag_of_tokens_score(vec![0], 0), 1);
    }

    #[test]
    fn discussion_case6() {
        assert_eq!(Solution::bag_of_tokens_score(vec![0, 1], 1), 2);
    }
}
