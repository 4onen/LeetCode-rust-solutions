// https://leetcode.com/problems/gas-station/

pub struct Solution;

// Do the loop sol'n
// impl Solution {
//     pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
//         let mut start = 0;
//         let mut total = 0;
//         let mut tank = 0;
//         for (i, (this_gas, this_cost)) in
//             std::iter::zip(gas.into_iter(), cost.into_iter()).enumerate()
//         {
//             tank += this_gas - this_cost;
//             if tank < 0 {
//                 start = i + 1;
//                 total += tank;
//                 tank = 0;
//             }
//         }
//         if total + tank < 0 {
//             -1
//         } else {
//             start as i32
//         }
//     }
// }

// Early exit
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() {
            return -1;
        }

        let mut start: i32 = 0;
        let mut tank = 0;

        for i in 0..gas.len() {
            tank += gas[i] - cost[i];
            if tank < 0 {
                tank = 0;
                start = i as i32 + 1;
            }
        }
        start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::can_complete_circuit(vec![2], vec![2]), 0);
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(
            Solution::can_complete_circuit(vec![3, 1, 1], vec![1, 2, 2]),
            0
        );
    }
}
