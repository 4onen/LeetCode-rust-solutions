// https://leetcode.com/problems/distribute-candies-to-people/

pub struct Solution;

impl Solution {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let num_people = num_people as u16;
        let mut res = vec![0; num_people as usize];
        let mut i = 0;
        while candies > i + 1 {
            res[(i as u16 % num_people) as usize] += i + 1;
            candies -= i + 1;
            i += 1;
        }
        res[(i as u16 % num_people) as usize] += candies;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(candies: u32, num_people: u16, expected: &[i32]) {
        assert!(candies >= 1);
        assert!(candies <= 1_000_000_000);
        assert!(num_people >= 1);
        assert!(num_people <= 1000);
        assert_eq!(
            Solution::distribute_candies(candies as i32, num_people as i32),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(7, 4, &[1, 2, 3, 1])
    }

    #[test]
    fn ex2() {
        test(10, 3, &[5, 2, 3])
    }

    #[test]
    fn discussion_case1() {
        test(60, 4, &[15, 18, 15, 12])
    }

    #[test]
    fn my_extreme_ex1() {
        // Don't-crash test
        Solution::distribute_candies(1_000_000_000,1000);
    }

    #[test]
    fn my_extreme_ex2() {
        // Don't-crash test
        Solution::distribute_candies(1_000_000_000,1);
    }
}
