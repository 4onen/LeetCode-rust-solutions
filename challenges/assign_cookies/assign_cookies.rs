// https://leetcode.com/problems/assign-cookies/

pub struct Solution;

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut greed_factors = g;
        let mut cookie_sizes = s;
        greed_factors.sort_unstable();
        cookie_sizes.sort_unstable();

        let mut cookie_iter = cookie_sizes.into_iter();

        let mut count = 0;
        for greed in greed_factors {
            while let Some(cookie) = cookie_iter.next() {
                if cookie >= greed {
                    count += 1;
                    break;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }

    #[test]
    fn myex1() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 2, 3]),
            3
        );
    }

    #[test]
    fn myex2() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![3, 2, 1]),
            3
        );
    }

    #[test]
    fn myex3() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 2, 3, 4]),
            3
        );
    }

    #[test]
    fn myex4() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3, 4], vec![1, 2, 3]),
            3
        );
    }

    #[test]
    fn myex5() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3, 4], vec![1, 2, 3, 4]),
            4
        );
    }
}
