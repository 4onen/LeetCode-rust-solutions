// https://leetcode.com/problems/average-waiting-time/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
//         let mut customers_waited: i64 = 0;
//         let mut current_time = 0;
//         let customer_count = customers.len();
//         for customer in customers.iter() {
//             let arrival = customer[0];
//             if current_time < arrival {
//                 current_time = arrival;
//             }
//             current_time += customer[1];
//             customers_waited += (current_time - arrival) as i64;
//         }
//         customers_waited as f64 / customer_count as f64
//     }
// }

// Faster addition via branch separation
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut customers_waited: i64 = 0;
        let mut current_time = 0;
        let customer_count = customers.len();
        for customer in customers.iter() {
            let arrival = customer[0];
            if current_time < arrival {
                current_time = arrival + customer[1];
                customers_waited += customer[1] as i64;
            } else {
                current_time += customer[1];
                customers_waited += (current_time - arrival) as i64;
            }
        }
        customers_waited as f64 / customer_count as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(customers: &[[i32;2]], expected: f64) {
        assert!(customers.len() >= 1);
        assert!(customers.len() <= 100_000);
        for &[arrival, time] in customers.iter() {
            assert!(arrival >= 1);
            assert!(arrival <= 10_000);
            assert!(time >= 1);
            assert!(time <= 10_000);
        }
        let mut prev = 0;
        for i in 0..customers.len() {
            let [arrival, _] = customers[i];
            assert!(arrival >= prev);
            prev = arrival;
        }
        let result = Solution::average_waiting_time(customers.iter().map(|c| c.to_vec()).collect());
        assert!(result > 0.0);
        assert!((result - expected).abs() < 1e-5, "{} != {}", result, expected);
    }

    #[test]
    fn ex1() {
        test(&[[1,2],[2,5],[4,3]], 5.00000);
    }

    #[test]
    fn ex2() {
        test(&[[5,2],[5,4],[10,3],[20,1]], 3.25000);
    }

    #[test]
    fn failing_case1() {
        let content = include_str!("failing_case1.txt");
        let inbounds = content[2..content.len()-3].split("],[");
        let customers: Vec<[i32;2]> = inbounds.map(|s| {
            println!("{}", s);
            let mut parts = s.split(",");
            let arrival = parts.next().unwrap().parse().unwrap();
            let time = parts.next().unwrap().parse().unwrap();
            [arrival, time]
        }).collect();
        test(&customers, 24162470.50191);
    }
}
