// https://leetcode.com/problems/minimum-cost-for-tickets/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn mincost_tickets(mut days: Vec<i32>, costs: Vec<i32>) -> i32 {
//         let max_day = *days.last().unwrap() as u16;
//         let min_day = *days.first().unwrap() as u16;
//         let mut dp = vec![0; (max_day + 1) as usize];
//         for day_year_idx in (min_day..max_day + 1).rev() {
//             let last_day_unbought = days.last().copied().unwrap();
//             let res = if day_year_idx as i32 == last_day_unbought {
//                 let buythirty =
//                     costs[2] + dp.get((day_year_idx + 30) as usize).copied().unwrap_or(0);
//                 let buyseven = costs[1] + dp.get((day_year_idx + 7) as usize).copied().unwrap_or(0);
//                 let buyone = costs[0] + dp.get((day_year_idx + 1) as usize).copied().unwrap_or(0);
//                 days.pop();
//                 std::cmp::min(buyone, std::cmp::min(buyseven, buythirty))
//             } else {
//                 dp[(day_year_idx + 1) as usize]
//             };
//             dbg!((day_year_idx, last_day_unbought, res));
//             dp[day_year_idx as usize] = res;
//         }
//         dp[min_day as usize]
//     }
// }

// Remember to take out the dbg op
impl Solution {
    pub fn mincost_tickets(mut days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let max_day = *days.last().unwrap() as u16;
        let min_day = *days.first().unwrap() as u16;
        let mut dp = vec![0; (max_day + 1) as usize];
        for day_year_idx in (min_day..max_day + 1).rev() {
            let last_day_unbought = days.last().copied().unwrap();
            let res = if day_year_idx as i32 == last_day_unbought {
                let buythirty =
                    costs[2] + dp.get((day_year_idx + 30) as usize).copied().unwrap_or(0);
                let buyseven = costs[1] + dp.get((day_year_idx + 7) as usize).copied().unwrap_or(0);
                let buyone = costs[0] + dp.get((day_year_idx + 1) as usize).copied().unwrap_or(0);
                days.pop();
                std::cmp::min(buyone, std::cmp::min(buyseven, buythirty))
            } else {
                dp[(day_year_idx + 1) as usize]
            };
            dp[day_year_idx as usize] = res;
        }
        dp[min_day as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(days: &[i32], costs: [i32; 3], expected: i32) {
        assert!(days.len() >= 1);
        assert!(days.len() <= 365);
        let mut last = 0;
        for &day in days {
            assert!(day >= 1);
            assert!(day <= 365);
            assert!(day >= last);
            last = day;
        }
        for &cost in &costs {
            assert!(cost >= 1);
            assert!(cost <= 1000);
        }
        assert_eq!(
            Solution::mincost_tickets(days.to_vec(), costs.to_vec()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 4, 6, 7, 8, 20], [2, 7, 15], 11)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], [2, 7, 15], 17)
    }
}
