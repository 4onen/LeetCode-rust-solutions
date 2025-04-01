// https://leetcode.com/problems/solving-questions-with-brainpower/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
//         let mut dp: Vec<_> = vec![0i64; questions.len() + 1];
//         let n = questions.len() as u32;
//         for i in (0..n).rev() {
//             let points = questions[i as usize][0];
//             let brainpower = questions[i as usize][1] as u32;
//             let next = i + brainpower + 1;
//             let skip = dp[i as usize + 1];
//             let do_it = if next < n {
//                 points as i64 + dp[next as usize]
//             } else {
//                 points as i64
//             };
//             dp[i as usize] = std::cmp::max(skip, do_it);
//         }
//         dp[0]
//     }
// }

// Optimization: destructuring syntax, change condition to value ternary (cmov?)
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut dp: Vec<_> = vec![0i64; questions.len() + 1];
        let n = questions.len() as u32;
        for i in (0..n).rev() {
            let [points, brainpower] = questions[i as usize][..] else {
                unreachable!()
            };
            let next = i + brainpower as u32 + 1;
            let skip = dp[i as usize + 1];
            let do_it = points as i64 + if next < n { dp[next as usize] } else { 0 };
            dp[i as usize] = std::cmp::max(skip, do_it);
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(questions: &[[i32; 2]], expected: i64) {
        assert!(questions.len() >= 1);
        assert!(questions.len() <= 100_000);
        for &[points, brainpower] in questions {
            assert!(points >= 1);
            assert!(points <= 100_000);
            assert!(brainpower >= 1);
            assert!(brainpower <= 100_000);
        }
        assert_eq!(
            Solution::most_points(questions.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[[3, 2], [4, 3], [4, 4], [2, 5]], 5)
    }

    #[test]
    fn ex2() {
        test(&[[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]], 7)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                [21, 5],
                [92, 3],
                [74, 2],
                [39, 4],
                [58, 2],
                [5, 5],
                [49, 4],
                [65, 3],
            ],
            157,
        )
    }

    #[test]
    fn discussion_case2() {
        test(&[[100000, 1], [100000, 1], [100000, 1]], 200000)
    }

    #[test]
    fn discussion_case3() {
        test(&[[12, 46], [78, 19], [63, 15], [79, 62], [13, 10]], 79)
    }

    #[test]
    fn discussion_case4() {
        test(
            &[
                [21, 2],
                [1, 2],
                [12, 5],
                [7, 2],
                [35, 3],
                [32, 2],
                [80, 2],
                [91, 5],
                [92, 3],
                [27, 3],
                [19, 1],
                [37, 3],
                [85, 2],
                [33, 4],
                [25, 1],
                [91, 4],
                [44, 3],
                [93, 3],
                [65, 4],
                [82, 3],
                [85, 5],
                [81, 3],
                [29, 2],
                [25, 1],
                [74, 2],
                [58, 1],
            ],
            465,
        )
    }

    #[test]
    fn my_extreme_ex1() {
        test(&[[100_000, 1]; 100_000], 5000000000)
    }
}
