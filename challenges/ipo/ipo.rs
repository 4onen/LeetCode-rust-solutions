// https://leetcode.com/problems/ipo/

pub struct Solution;

// Initial sol'n (too slow)
// impl Solution {
//     pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
//         let mut projects: Vec<_> =
//             std::iter::zip(profits.into_iter(), capital.into_iter()).collect();
//         projects.sort_unstable_by_key(|&(p, _)| -p);
//         for _ in 0..k {
//             let mut i = 0;
//             while i < projects.len() && projects[i].1 > w {
//                 i += 1;
//             }
//             if i == projects.len() {
//                 break;
//             }
//             w += projects[i].0;
//             projects[i].1 = std::i32::MAX;
//         }
//         w
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn find_maximized_capital(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        assert!(k > 0);
        assert!(profits.len() == capital.len());
        let mut projects_needing_capital = std::collections::BinaryHeap::new();
        let mut doable_projects = std::collections::BinaryHeap::new();
        let mut i = 0;
        while i < profits.len() {
            if capital[i] <= w {
                doable_projects.push(profits[i]);
            } else {
                projects_needing_capital.push(std::cmp::Reverse((capital[i], i)));
            }
            i += 1;
        }
        loop {
            let Some(profit) = doable_projects.pop() else {
                break;
            };
            w += profit;
            k -= 1;
            if k <= 0 {
                break;
            }
            loop {
                match projects_needing_capital.peek_mut() {
                    Some(pm) if pm.0.0 <= w => {
                        let std::cmp::Reverse((_, i)) = std::collections::binary_heap::PeekMut::pop(pm);
                        doable_projects.push(profits[i]);
                    }
                    _ => {break;}
                }
            }
        }
        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(k: u32, w: u32, profits: &[u16], capital: &[u32], expected: i32) {
        let profits = profits.iter().map(|&x| x as i32).collect();
        let capital = capital.iter().map(|&x| x as i32).collect();
        assert_eq!(
            Solution::find_maximized_capital(k as i32, w as i32, profits, capital),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(2, 0, &[1, 2, 3], &[0, 1, 1], 4)
    }

    #[test]
    fn ex2() {
        test(3, 0, &[1, 2, 3], &[0, 1, 2], 6)
    }

    #[test]
    fn discussion_case1() {
        test(3, 0, &[5, 8, 9, 10, 10], &[0, 2, 2, 3, 4], 25)
    }

    #[test]
    fn discussion_case2() {
        test(1, 2, &[1, 2, 3], &[1, 1, 2], 5)
    }

    #[test]
    fn discussion_case3() {
        test(11, 11, &[1, 2, 3], &[11, 12, 13], 17)
    }

    #[test]
    fn discussion_case4() {
        test(1, 0, &[1, 2, 3], &[1, 1, 2], 0)
    }

    #[test]
    fn discussion_case5() {
        test(1, 2, &[1, 2, 3], &[1, 1, 2], 5)
    }

    #[test]
    fn tle_case1() {
        let capital_str = include_str!("tle_case1.txt");
        let capital = capital_str
            // Then split on commas
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let profit = vec![10000; capital.len()];
        test(100000, 1000000000, &profit, &capital, 2000000000)
    }
}
