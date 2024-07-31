// https://leetcode.com/problems/filling-bookcase-shelves/

pub struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        type Cnt = i32;
        let mut dp = vec![0 as Cnt; books.len() + 1];
        for i in 1..=books.len() {
            let mut width = books[i - 1][0];
            let mut height = books[i - 1][1];
            dp[i] = dp[i - 1] + height as Cnt;
            for j in (0..i - 1).rev() {
                width += books[j][0];
                if width > shelf_width {
                    break;
                }
                height = std::cmp::max(height, books[j][1]);
                dp[i] = std::cmp::min(dp[i], dp[j] + height as Cnt);
            }
        }
        dp[books.len()] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(books: &[[i32; 2]], shelf_width: u16, expected: i32) {
        assert!(books.len() >= 1);
        assert!(books.len() <= 1000);
        assert!(shelf_width >= 1);
        assert!(shelf_width <= 1000);
        assert!(expected >= 1);
        for book in books {
            assert!(book[0] >= 1);
            assert!(book[0] <= shelf_width as i32);
            assert!(book[1] >= 1);
            assert!(book[1] <= 1000);
        }
        let books = books.iter().map(|b| b.to_vec()).collect();
        assert_eq!(
            Solution::min_height_shelves(books, shelf_width as i32),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &[[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]],
            4,
            6,
        )
    }

    #[test]
    fn ex2() {
        test(&[[1, 3], [2, 4], [3, 2]], 6, 4)
    }

    #[test]
    fn discussion_case1() {
        test(&[[7, 3], [8, 7], [2, 7], [2, 5]], 10, 15)
    }

    #[test]
    fn discussion_case2() {
        test(&[[1, 1], [2, 10], [2, 3]], 4, 11)
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[[1000, 1000]; 1000], 1000, 1000 * 1000)
    }
}
