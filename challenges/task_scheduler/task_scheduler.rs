// https://leetcode.com/problems/task-scheduler/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
//         match n {
//             0 => tasks.len() as i32,
//             n if n < 0 => panic!("n must be non-negative"),
//             n if n > 100 => panic!("n must be less than or equal to 100"),
//             n => {
//                 let n = n as u8;
//                 assert!(tasks.len() > 0, "tasks length must be greater than 0");
//                 assert!(
//                     tasks.len() <= 10000,
//                     "tasks length must be less than or equal to 10000"
//                 );
//                 let num_tasks = tasks.len() as i32;
//                 let mut task_count = [0u16; 26];
//                 for task in tasks {
//                     task_count[(task as u8 - b'A') as usize] += 1;
//                 }
//                 task_count.sort_unstable();
//                 let max_val = (task_count[25] - 1) as u16;
//                 let mut idle_slots = max_val as u32 * n as u32;
//                 for i in 0..25 {
//                     match idle_slots.checked_sub(std::cmp::min(task_count[i], max_val) as u32) {
//                         Some(val) => idle_slots = val,
//                         None => return num_tasks,
//                     }
//                 }
//                 num_tasks + idle_slots as i32
//             }
//         }
//     }
// }

// Incorporating math trick from other sol'ns
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        match n {
            0 => tasks.len() as i32,
            n if n < 0 => panic!("n must be non-negative"),
            n if n > 100 => panic!("n must be less than or equal to 100"),
            n => {
                let n = n as u8;
                assert!(tasks.len() > 0, "tasks length must be greater than 0");
                assert!(
                    tasks.len() <= 10000,
                    "tasks length must be less than or equal to 10000"
                );
                let num_tasks = tasks.len() as i32;
                let mut task_count = [0u16; 26];
                for task in tasks {
                    task_count[(task as u8 - b'A') as usize] += 1;
                }
                task_count.sort_unstable();
                let max_val = (task_count[25]) as u16;
                let result = task_count.into_iter().filter(|&x| x == max_val).count() as i32
                    + (max_val as i32 - 1) * (n as i32 + 1);
                std::cmp::max(num_tasks, result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'C', 'A', 'B', 'D', 'B'], 1),
            6
        );
    }

    #[test]
    fn ex3() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 3),
            10
        );
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(Solution::least_interval(vec!['A', 'A', 'A'], 2), 7);
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'C', 'A', 'B', 'A', 'B', 'B'], 2),
            8
        );
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(
            Solution::least_interval(
                vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'D', 'D', 'E'],
                2
            ),
            11
        );
    }

    #[test]
    fn my_extreme_case1() {
        assert_eq!(
            Solution::least_interval(vec!['A'; 10_000], 100),
            10_000 * 101 - 101 + 1
        );
    }
}
