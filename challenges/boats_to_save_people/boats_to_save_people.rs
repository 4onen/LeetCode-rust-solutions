// https://leetcode.com/problems/boats-to-save-people/

pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();
        let mut lightest_left = 0u16;
        let mut heaviest_left = people.len() as u16 - 1;
        let mut boats = 0;
        while lightest_left <= heaviest_left {
            boats += 1;
            if people[lightest_left as usize] + people[heaviest_left as usize] <= limit {
                lightest_left += 1;
            }
            match heaviest_left.checked_sub(1) {
                None => break,
                Some(next_heaviest) => heaviest_left = next_heaviest,
            }
        }
        boats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(people: &[u16], limit: u16, expected: u16) {
        assert!(people.len() > 0);
        assert!(people.len() < 50_000);
        assert!(limit > 0);
        assert!(limit < 30_000);
        assert!(people.iter().all(|&x| x >= 1));
        assert!(people.iter().all(|&x| x <= limit));
        let result = Solution::num_rescue_boats(
            people.into_iter().map(|&x| x as i32).collect(),
            limit as i32,
        );
        assert_eq!(result, expected as i32);
    }

    #[test]
    fn ex1() {
        test(&[1, 2], 3, 1)
    }

    #[test]
    fn ex2() {
        test(&[3, 2, 2, 1], 3, 3)
    }

    #[test]
    fn ex3() {
        test(&[3, 5, 3, 4], 5, 4)
    }

    #[test]
    fn discussion_case1() {
        test(&[2, 3, 7, 8], 10, 2)
    }

    #[test]
    fn discussion_case2() {
        test(&[5, 1, 4, 2], 6, 2)
    }

    #[test]
    fn discussion_case3() {
        // Testcase 71/78
        test(
            &[
                2, 49, 10, 7, 11, 41, 47, 2, 22, 6, 13, 12, 33, 18, 10, 26, 2, 6, 50, 10,
            ],
            50,
            11,
        )
    }

    #[test]
    fn discussion_case4() {
        test(&[1, 1, 1, 1, 1], 3, 3)
    }

    #[test]
    fn discussion_case5() {
        test(&[3, 2, 3, 2, 2], 6, 3)
    }

    #[test]
    fn discussion_case6() {
        test(&[1, 1, 1, 1, 2, 3, 3, 4], 4, 5)
    }

    #[test]
    fn discusssion_case7() {
        test(&[1, 2, 2, 2, 2, 2], 6, 3)
    }
}
