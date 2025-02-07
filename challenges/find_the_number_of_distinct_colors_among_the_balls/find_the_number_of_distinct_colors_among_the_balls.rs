// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/

pub struct Solution;

// Failing sol'n 1 (Can't handle 1_000_000_000 limit on LeetCode)
// impl Solution {
//     pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
//         let mut balls = vec![0; limit as usize + 1];
//         let mut counts = std::collections::HashMap::new();
//         queries
//             .into_iter()
//             .map(|query| {
//                 let x = query[0] as usize;
//                 let y = query[1] as i32;
//                 if let Some(old_count) = counts.get_mut(&balls[x]) {
//                     *old_count -= 1;
//                     if *old_count <= 0 {
//                         counts.remove(&balls[x]);
//                     }
//                 }
//                 balls[x as usize] = y;
//                 *counts.entry(y).or_insert(0) += 1;
//                 counts.len() as i32
//             })
//             .collect()
//     }
// }

// Initial sol'n
impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let _ = limit;
        let mut balls = std::collections::HashMap::<i32, i32>::new();
        let mut counts = std::collections::HashMap::new();
        queries
            .into_iter()
            .map(|query| {
                let x = query[0];
                let y = query[1] as i32;
                let old = balls.insert(x, y).unwrap_or(0);
                if let Some(old_count) = counts.get_mut(&old) {
                    *old_count -= 1;
                    if *old_count <= 0 {
                        counts.remove(&old);
                    }
                };
                *counts.entry(y).or_insert(0) += 1;
                counts.len() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(limit: i32, queries: &[[i32; 2]], expected: &[i32]) {
        assert!(limit >= 1);
        assert!(limit <= 1_000_000_000);
        assert!(queries.len() >= 1);
        assert!(queries.len() <= 100_000);
        assert_eq!(expected.len(), queries.len());
        for &query in queries {
            assert!(query[0] >= 0);
            assert!(query[0] <= limit);
            assert!(query[1] >= 1);
            assert!(query[1] <= 1_000_000_000);
        }
        assert_eq!(
            Solution::query_results(limit, queries.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(4, &[[1, 4], [2, 5], [1, 3], [3, 4]], &[1, 2, 2, 3])
    }

    #[test]
    fn ex2() {
        test(
            4,
            &[[0, 1], [1, 2], [2, 2], [3, 4], [4, 5]],
            &[1, 2, 2, 3, 4],
        )
    }

    #[test]
    fn discussion_case1() {
        test(2, &[[0, 1], [1, 2], [2, 3]], &[1, 2, 3])
    }

    #[test]
    fn discussion_case2() {
        test(
            30448,
            &[
                [21364, 240766080],
                [3639, 849423187],
                [22679, 515800713],
                [5380, 599544533],
                [16705, 405453064],
                [9903, 272362238],
                [91, 110564765],
                [18078, 384426512],
                [16927, 85019053],
                [16750, 18013573],
                [7701, 544527527],
                [23462, 902209596],
                [2279, 214701875],
                [10287, 255934382],
                [26040, 608064271],
                [29948, 804699287],
                [23617, 324996467],
                [1189, 965452724],
                [9451, 712927360],
                [18919, 65737311],
                [1993, 724234699],
                [14682, 625754722],
                [7641, 982592243],
                [21063, 145346429],
                [21341, 807430664],
                [21136, 749932148],
                [18209, 66024730],
                [22974, 593655630],
                [3461, 117690992],
                [15782, 858599132],
                [14425, 580363509],
                [2795, 66658868],
            ],
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32,
            ],
        )
    }

    #[test]
    fn my_extreme_ex1() {
        let mut queries = vec![];
        for i in 0..100_000 {
            queries.push([i, 1]);
        }
        test(100_000, &queries, &[1; 100_000]);
    }

    #[test]
    fn my_extreme_ex2() {
        let mut queries = vec![];
        for i in 1..=100_000 {
            queries.push([i, i]);
        }
        let mut expected = vec![];
        for i in 1..=100_000 {
            expected.push(i);
        }
        test(100_000, &queries, &expected);
    }

    #[test]
    fn failing_case1() {
        let queries_str = include_str!("failing_case1_queries.txt");
        let queries = queries_str[2..queries_str.len() - 2]
            .split("],[")
            .map(|x| {
                let mut parts = x.split(",");
                [
                    parts.next().expect("No int1").parse().unwrap(),
                    parts.next().expect("No int2").parse().unwrap(),
                ]
            })
            .collect::<Vec<[i32; 2]>>();
        let expected = vec![1; queries.len()];
        test(1_000_000_000, &queries, &expected);
    }
}
