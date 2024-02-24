// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/

pub struct Solution;

fn tupleize_events(events: Vec<Vec<i32>>) -> Box<[(i32, i32)]> {
    // First, convert this vector of vectors to a more useful vector
    // of tuples. We know each subvector has exactly two elements, so
    // we can stuff them into tuples in the same memory space as the
    // original sub-vectors.
    // Obviously super unsafe if run on a 32-bit system, but this is
    // a toy problem on leetcode, so the environment is known.

    let original_len = events.len();
    let events_ptr = events.leak();
    let events_ptr2: &mut [Vec<i32>] = &mut events_ptr[0..original_len];
    let events_tuple_ptr = unsafe { std::mem::transmute::<_, &mut [(i32, i32)]>(events_ptr2) };

    for i in 0..original_len {
        let tup = (events_ptr[i][0], events_ptr[i][1]);
        events_tuple_ptr[i] = tup;
    }

    unsafe { std::boxed::Box::from_raw(events_tuple_ptr) }
}

// TLE Bool array impl
// impl Solution {
//     pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
//         let mut events = tupleize_events(events);
//         events.sort_by_key(|t| t.1);
//         let max_end = events.last().unwrap().1;
//         let mut days = vec![false; max_end as usize + 1];
//         let mut count = 0;
//         for &(start, end) in events.into_iter() {
//             for day in start..=end {
//                 if !days[day as usize] {
//                     days[day as usize] = true;
//                     count += 1;
//                     break;
//                 }
//             }
//         }
//         count
//     }
// }

// Incorrect Linear impl
// impl Solution {
//     pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
//         let mut events = tupleize_events(events);
//         events.sort_by_key(|&t| (t.1, t.0));
//         dbg!(events.clone());
//         let mut count = 0;
//         let mut next_free_day = 0;
//         for &(start, end) in events.iter() {
//             if (start..=end).contains(&next_free_day) {
//                 count += 1;
//                 next_free_day += 1;
//             } else if next_free_day < start {
//                 count += 1;
//                 next_free_day = start + 1;
//             }
//         }
//         count
//     }
// }

// Correct minheap impl
// impl Solution {
//     pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
//         let mut events = tupleize_events(events);
//         events.sort_unstable();
//         let mut count = 0;
//         let mut next_free_day = 0;
//         let mut idx = 0;
//         let mut heap = std::collections::BinaryHeap::new();
//         while idx < events.len() || !heap.is_empty() {
//             if heap.is_empty() {
//                 next_free_day = events[idx].0;
//             }
//             while idx < events.len() && events[idx].0 <= next_free_day {
//                 heap.push(std::cmp::Reverse(events[idx].1));
//                 idx += 1;
//             }
//             count += 1;
//             next_free_day += 1;
//             heap.pop();
//             loop {
//                 match heap.peek() {
//                     Some(&std::cmp::Reverse(end)) if end < next_free_day => {
//                         heap.pop();
//                     }
//                     _ => break,
//                 }
//             }
//         }
//         count
//     }
// }

// Correct linear scan impl
impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = tupleize_events(events);
        events.sort_unstable_by_key(|&t| t.1);

        let mut min_day = vec![0; events.last().unwrap().1 as usize + 2];
        for i in 0..min_day.len() as i32 {
            min_day[i as usize] = i;
        }

        let mut count = 0;
        for &(start, end) in events.iter() {
            let mut day = start;
            while min_day[day as usize] != day {
                day = min_day[day as usize];
            }

            if day <= end {
                count += 1;
                min_day[day as usize] += 1;
                min_day[start as usize] = min_day[day as usize];
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
        let events = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::max_events(events), 3);
    }

    #[test]
    fn ex2() {
        let events = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]];
        assert_eq!(Solution::max_events(events), 4);
    }

    #[test]
    fn discussion_case1() {
        let events = vec![vec![1, 2], vec![1, 2], vec![3, 3], vec![1, 5], vec![1, 5]];
        assert_eq!(Solution::max_events(events), 5);
    }

    #[test]
    fn discussion_case2() {
        let events = vec![vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2]];
        assert_eq!(Solution::max_events(events), 2);
    }

    #[test]
    fn discussion_case3() {
        let events = vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![1, 5],
            vec![1, 6],
            vec![1, 7],
        ];
        assert_eq!(Solution::max_events(events), 7);
    }

    #[test]
    fn failing_case1() {
        let events = vec![vec![1, 5], vec![1, 5], vec![1, 5], vec![2, 3], vec![2, 3]];
        assert_eq!(Solution::max_events(events), 5);
    }

    #[test]
    fn failing_case2() {
        let events = (1..=100000).into_iter().map(|v| vec![1, v]).collect();
        assert_eq!(Solution::max_events(events), 100000);
    }
}
