// https://leetcode.com/problems/the-number-of-the-smallest-unoccupied-chair/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn smallest_chair(mut times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
//         assert!(target_friend >= 0);
//         assert!(target_friend as usize <= times.len());
//         let target_friend_arrival = times[target_friend as usize][0];
//         let mut i = 0;
//         while i < times.len() {
//             if times[i][0] > target_friend_arrival {
//                 times.swap_remove(i);
//                 continue;
//             }
//             i += 1;
//         }
//         times.sort_unstable();
//         let mut open_chairs = std::collections::BinaryHeap::new();
//         let mut free_times = std::collections::BinaryHeap::new();
//         let mut next_chair_to_consider = 0;
//         for time in times {
//             let current_time = time[0];
//             while free_times
//                 .peek()
//                 .map(|&std::cmp::Reverse((time_free, _))| time_free <= current_time)
//                 .unwrap_or(false)
//             {
//                 let freed_chair = free_times.pop().unwrap().0 .1;
//                 if next_chair_to_consider == freed_chair + 1 {
//                     next_chair_to_consider -= 1;
//                 } else {
//                     open_chairs.push(std::cmp::Reverse(freed_chair));
//                 }
//             }
//             let chair_to_sit_in = open_chairs
//                 .pop()
//                 .unwrap_or_else(|| {
//                     let res = next_chair_to_consider;
//                     next_chair_to_consider += 1;
//                     std::cmp::Reverse(res)
//                 })
//                 .0;
//             if current_time == target_friend_arrival {
//                 return chair_to_sit_in;
//             }
//             free_times.push(std::cmp::Reverse((time[1], chair_to_sit_in)));
//         }
//         // Now the target friend is arriving:
//         unreachable!();
//         // open_chairs
//         //     .pop()
//         //     .unwrap_or(std::cmp::Reverse(next_chair_to_consider))
//         //     .0
//     }
// }

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
impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut times = tupleize_events(times);
        assert!(target_friend >= 0);
        assert!(target_friend as usize <= times.len());
        let target_friend_arrival = times[target_friend as usize].0;
        times.sort_unstable();
        let mut open_chairs = std::collections::BinaryHeap::new();
        let mut free_times = std::collections::BinaryHeap::new();
        let mut next_chair_to_consider = 0;
        for time in times.into_iter() {
            let current_time = time.0;
            while free_times
                .peek()
                .map(|&std::cmp::Reverse((time_free, _))| time_free <= current_time)
                .unwrap_or(false)
            {
                let freed_chair = free_times.pop().unwrap().0 .1;
                if next_chair_to_consider == freed_chair + 1 {
                    next_chair_to_consider -= 1;
                } else {
                    open_chairs.push(std::cmp::Reverse(freed_chair));
                }
            }
            let chair_to_sit_in = open_chairs
                .pop()
                .unwrap_or_else(|| {
                    let res = next_chair_to_consider;
                    next_chair_to_consider += 1;
                    std::cmp::Reverse(res)
                })
                .0;
            if current_time == target_friend_arrival {
                return chair_to_sit_in;
            }
            free_times.push(std::cmp::Reverse((time.1, chair_to_sit_in)));
        }
        // Now the target friend is arriving:
        unreachable!();
        // open_chairs
        //     .pop()
        //     .unwrap_or(std::cmp::Reverse(next_chair_to_consider))
        //     .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(times: &[[i32; 2]], target_friend: i32, expected: i32) {
        assert!(times.len() >= 2);
        assert!(times.len() <= 10_000);
        assert!(target_friend >= 0);
        assert!(target_friend <= times.len() as i32 - 1);
        let mut arrivals = std::collections::HashSet::new();
        for time in times {
            assert!(time[0] >= 0);
            assert!(time[0] <= time[1]);
            assert!(time[1] <= 100_000);
            assert!(time[0] < time[1]);
            assert!(arrivals.insert(time[0]));
        }
        assert_eq!(
            Solution::smallest_chair(
                times.into_iter().map(|&x| x.to_vec()).collect(),
                target_friend
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[[1, 4], [2, 3], [4, 6]], 1, 1)
    }

    #[test]
    fn ex2() {
        test(&[[3, 10], [1, 5], [2, 6]], 0, 2)
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                [33, 35],
                [26, 29],
                [9, 28],
                [4, 31],
                [8, 10],
                [32, 34],
                [15, 24],
                [27, 39],
                [14, 36],
                [1, 14],
                [25, 39],
                [5, 27],
                [6, 15],
                [2, 38],
                [19, 36],
                [24, 34],
                [3, 26],
            ],
            0,
            3,
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                [1, 2],
                [2, 10],
                [3, 10],
                [4, 10],
                [5, 10],
                [6, 10],
                [7, 10],
                [8, 10],
                [9, 10],
                [10, 11],
            ],
            8,
            7,
        )
    }

    #[test]
    fn discussion_case3() {
        test(&[[1, 2], [2, 3]], 1, 0)
    }

    #[test]
    fn discussion_case3_1() {
        test(&[[1, 2], [2, 3]], 0, 0)
    }

    #[test]
    fn failing_case1() {
        test(
            &[
                [33889, 98676],
                [80071, 89737],
                [44118, 52565],
                [52992, 84310],
                [78492, 88209],
                [21695, 67063],
                [84622, 95452],
                [98048, 98856],
                [98411, 99433],
                [55333, 56548],
                [65375, 88566],
                [55011, 62821],
                [48548, 48656],
                [87396, 94825],
                [55273, 81868],
                [75629, 91467],
            ],
            6,
            2,
        )
    }
}
