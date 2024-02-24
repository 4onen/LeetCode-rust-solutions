// https://leetcode.com/problems/meeting-rooms-iii/

pub struct Solution;

// Raw meeting array sol'n with tuple input support
// impl Solution {
//     pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
//         /// Convert a vector of length-2 vectors of i32 to a boxed slice of
//         /// length-2 tuples of u32.
//         ///
//         /// Abuses the fact that the memory layout of a Vec<i32> is at least
//         /// as large as (u32, u32), so we can safely transmute the provided
//         /// memory to a slice of tuples.
//         fn tupleize_events(events: Vec<Vec<i32>>) -> Box<[(u32, u32)]> {
//             // First, convert this vector of vectors to a more useful vector
//             // of tuples. We know each subvector has exactly two elements, so
//             // we can stuff them into tuples in the same memory space as the
//             // original sub-vectors.
//             // Obviously super unsafe if run on a 32-bit system, but this is
//             // a toy problem on leetcode, so the environment is known.
//             // While we're at it, we know all meetings are non-negative, so
//             // we can use u32 instead of i32.

//             let original_len = events.len();
//             let events_ptr = events.leak();
//             let events_ptr2: &mut [Vec<i32>] = &mut events_ptr[0..original_len];
//             let events_tuple_ptr =
//                 unsafe { std::mem::transmute::<_, &mut [(u32, u32)]>(events_ptr2) };

//             for i in 0..original_len {
//                 let tup = (events_ptr[i][0] as u32, events_ptr[i][1] as u32);
//                 events_tuple_ptr[i] = tup;
//             }

//             unsafe { std::boxed::Box::from_raw(events_tuple_ptr) }
//         }
//         let n = match n {
//             i32::MIN..=0 => panic!("n must be positive"),
//             1 => return 0,
//             2..=100 => n as u8,
//             _ => panic!("n must be at most 100"),
//         };
//         let mut events = tupleize_events(meetings);
//         Self::most_booked_tuple_input(n, &mut events) as i32
//     }

//     // Raw meeting array sol'n
//     pub fn most_booked_tuple_input(n: u8, meetings: &mut [(u32, u32)]) -> u8 {
//         /// Find the index of the maximum element in a slice.
//         ///
//         /// If there are multiple maximum elements, the *first* one is returned.
//         ///
//         /// The slice must be non-empty, or zero will be returned.
//         ///
//         /// If the slice length exceeds 255, the result is an unspecified u8.
//         fn max_index<T>(eles: &[T]) -> u8
//         where
//             T: Ord,
//         {
//             let mut max_index = 0u8;
//             for i in 1..eles.len() as u8 {
//                 if eles[i as usize] > eles[max_index as usize] {
//                     max_index = i;
//                 }
//             }
//             max_index
//         }
//         /// Find the index of the minimum element in a slice.
//         ///
//         /// If there are multiple minimum elements, the *first* one is returned.
//         ///
//         /// The slice must be non-empty, or zero will be returned.
//         ///
//         /// If the slice length exceeds 255, the result is an unspecified u8.
//         fn min_index<T>(eles: &[T]) -> u8
//         where
//             T: Ord,
//         {
//             let mut min_index = 0u8;
//             for i in 1..eles.len() as u8 {
//                 if eles[i as usize] < eles[min_index as usize] {
//                     min_index = i;
//                 }
//             }
//             min_index
//         }
//         meetings.sort_unstable();
//         let mut room_free_at = vec![0u32; n as usize];
//         let mut room_bookings = vec![0u32; n as usize];
//         'outer: for (start, end) in meetings {
//             for i in 0..n {
//                 if *start >= room_free_at[i as usize] {
//                     // println!("Booking room {} for {}-{}", i, start, end);
//                     room_free_at[i as usize] = *end;
//                     room_bookings[i as usize] += 1;
//                     continue 'outer;
//                 }
//             }
//             let meeting_length = *end - *start;
//             let earliest = min_index(&room_free_at);
//             let earliest_free = room_free_at[earliest as usize];
//             // println!(
//             //     "Booking room {} for {}-{} (delayed from {} to {})",
//             //     earliest,
//             //     earliest_free,
//             //     earliest_free + meeting_length,
//             //     start,
//             //     end,
//             // );
//             room_free_at[earliest as usize] = earliest_free + meeting_length;
//             room_bookings[earliest as usize] += 1;
//         }
//         // println!("Final room bookings: {:?}", &room_bookings);
//         max_index(&room_bookings)
//     }
// }

// Raw meeting array sol'n without tuple input support
// impl Solution {
//     pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
//         let n = match n {
//             i32::MIN..=0 => panic!("n must be positive"),
//             1 => return 0,
//             2..=100 => n as u8,
//             _ => panic!("n must be at most 100"),
//         };
//         let meetings = {
//             let mut meetings = meetings;
//             meetings.sort_unstable_by_key(|x| x[0]);
//             meetings
//         };
//         let mut room_free_at = vec![0u32; n as usize];
//         let mut room_bookings = vec![0u32; n as usize];
//         'outer: for meeting in meetings {
//             let start = meeting[0] as u32;
//             let end = meeting[1] as u32;
//             let mut earliest = 0;
//             for i in 0..n {
//                 if start >= room_free_at[i as usize] {
//                     room_free_at[i as usize] = end;
//                     room_bookings[i as usize] += 1;
//                     continue 'outer;
//                 }
//                 if room_free_at[i as usize] < room_free_at[earliest as usize] {
//                     earliest = i;
//                 }
//             }
//             let meeting_length = end - start;
//             let earliest_free = room_free_at[earliest as usize];
//             room_free_at[earliest as usize] = earliest_free + meeting_length;
//             room_bookings[earliest as usize] += 1;
//         }
//         let mut result: u8 = 0;
//         for i in 1..room_bookings.len() as u8 {
//             if room_bookings[i as usize] > room_bookings[result as usize] {
//                 result = i;
//             }
//         }
//         result as i32
//     }
// }

// Reimplement tupleization
// impl Solution {
//     pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
//         /// Convert a vector of length-2 vectors of i32 to a boxed slice of
//         /// length-2 tuples of u32.
//         ///
//         /// Abuses the fact that the memory layout of a Vec<i32> is at least
//         /// as large as (u32, u32), so we can safely transmute the provided
//         /// memory to a slice of tuples.
//         fn tupleize_events(events: Vec<Vec<i32>>) -> Box<[(u32, u32)]> {
//             // First, convert this vector of vectors to a more useful vector
//             // of tuples. We know each subvector has exactly two elements, so
//             // we can stuff them into tuples in the same memory space as the
//             // original sub-vectors.
//             // Obviously super unsafe if run on a 32-bit system, but this is
//             // a toy problem on leetcode, so the environment is known.
//             // While we're at it, we know all meetings are non-negative, so
//             // we can use u32 instead of i32.
//             let original_len = events.len();
//             let events_ptr = events.leak();
//             let events_ptr2: &mut [Vec<i32>] = &mut events_ptr[0..original_len];
//             let events_tuple_ptr =
//                 unsafe { std::mem::transmute::<_, &mut [(u32, u32)]>(events_ptr2) };
//             for i in 0..original_len {
//                 let tup = (events_ptr[i][0] as u32, events_ptr[i][1] as u32);
//                 events_tuple_ptr[i] = tup;
//             }
//             unsafe { std::boxed::Box::from_raw(events_tuple_ptr) }
//         }
//         let n = match n {
//             i32::MIN..=0 => panic!("n must be positive"),
//             1 => return 0,
//             2..=100 => n as u8,
//             _ => panic!("n must be at most 100"),
//         };
//         let meetings = {
//             let mut meetings = tupleize_events(meetings);
//             meetings.sort_unstable_by_key(|x| x.0);
//             meetings
//         };
//         let mut room_free_at = vec![0u32; n as usize];
//         let mut room_bookings = vec![0u32; n as usize];
//         'outer: for &(start, end) in meetings.into_iter() {
//             let mut earliest = 0;
//             for i in 0..n {
//                 if start >= room_free_at[i as usize] {
//                     room_free_at[i as usize] = end;
//                     room_bookings[i as usize] += 1;
//                     continue 'outer;
//                 }
//                 if room_free_at[i as usize] < room_free_at[earliest as usize] {
//                     earliest = i;
//                 }
//             }
//             let meeting_length = end - start;
//             let earliest_free = room_free_at[earliest as usize];
//             room_free_at[earliest as usize] = earliest_free + meeting_length;
//             room_bookings[earliest as usize] += 1;
//         }
//         let mut result: u8 = 0;
//         for i in 1..room_bookings.len() as u8 {
//             if room_bookings[i as usize] > room_bookings[result as usize] {
//                 result = i;
//             }
//         }
//         result as i32
//     }
// }

// Tupleized Heap sol'n
// impl Solution {
//     pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
//         /// Convert a vector of length-2 vectors of i32 to a boxed slice of
//         /// length-2 tuples of u32.
//         ///
//         /// Abuses the fact that the memory layout of a Vec<i32> is at least
//         /// as large as (u32, u32), so we can safely transmute the provided
//         /// memory to a slice of tuples.
//         fn tupleize_events(events: Vec<Vec<i32>>) -> Box<[(u32, u32)]> {
//             // First, convert this vector of vectors to a more useful vector
//             // of tuples. We know each subvector has exactly two elements, so
//             // we can stuff them into tuples in the same memory space as the
//             // original sub-vectors.
//             // Obviously super unsafe if run on a 32-bit system, but this is
//             // a toy problem on leetcode, so the environment is known.
//             // While we're at it, we know all meetings are non-negative, so
//             // we can use u32 instead of i32.
//             let original_len = events.len();
//             let events_ptr = events.leak();
//             let events_ptr2: &mut [Vec<i32>] = &mut events_ptr[0..original_len];
//             let events_tuple_ptr =
//                 unsafe { std::mem::transmute::<_, &mut [(u32, u32)]>(events_ptr2) };
//             for i in 0..original_len {
//                 let tup = (events_ptr[i][0] as u32, events_ptr[i][1] as u32);
//                 events_tuple_ptr[i] = tup;
//             }
//             unsafe { std::boxed::Box::from_raw(events_tuple_ptr) }
//         }
//         let n = match n {
//             i32::MIN..=0 => panic!("n must be positive"),
//             1 => return 0,
//             2..=100 => n as u8,
//             _ => panic!("n must be at most 100"),
//         };
//         let meetings = {
//             let mut meetings = tupleize_events(meetings);
//             meetings.sort_unstable_by_key(|x| x.0);
//             meetings
//         };
//         let mut room_bookings = vec![0u32; n as usize];
//         let mut next_free = (0..n)
//             .map(|room_num| std::cmp::Reverse(room_num))
//             .collect::<std::collections::BinaryHeap<std::cmp::Reverse<u8>>>();
//         let mut next_busy = std::collections::BinaryHeap::with_capacity(n as usize);
//         for &(start, end) in meetings.into_iter() {
//             loop {
//                 match next_busy.peek() {
//                     Some(&std::cmp::Reverse((end, room))) if end <= start => {
//                         next_busy.pop();
//                         next_free.push(std::cmp::Reverse(room));
//                     }
//                     _ => break,
//                 }
//             }
//             let room = if let Some(std::cmp::Reverse(room)) = next_free.pop() {
//                 next_busy.push(std::cmp::Reverse((end, room)));
//                 room
//             } else {
//                 let meeting_length = end - start;
//                 let std::cmp::Reverse((earliest_end, room)) = &mut *next_busy.peek_mut().unwrap();
//                 *earliest_end += meeting_length;
//                 *room
//             };
//             room_bookings[room as usize] += 1;
//         }
//         let mut result: u8 = 0;
//         for i in 1..room_bookings.len() as u8 {
//             if room_bookings[i as usize] > room_bookings[result as usize] {
//                 result = i;
//             }
//         }
//         result as i32
//     }
// }

// Untupleized heap sol'n
impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let n = match n {
            i32::MIN..=0 => panic!("n must be positive"),
            1 => return 0,
            2..=100 => n as u8,
            _ => panic!("n must be at most 100"),
        };
        meetings.sort_unstable_by_key(|x| x[0]);
        let mut room_bookings = vec![0u32; n as usize];
        let mut next_free = (0..n)
            .map(|room_num| std::cmp::Reverse(room_num))
            .collect::<std::collections::BinaryHeap<std::cmp::Reverse<u8>>>();
        let mut next_busy = std::collections::BinaryHeap::with_capacity(n as usize);
        for meeting in meetings {
            let start = meeting[0] as u32;
            let end = meeting[1] as u32;
            loop {
                match next_busy.peek() {
                    Some(&std::cmp::Reverse((end, room))) if end <= start => {
                        next_busy.pop();
                        next_free.push(std::cmp::Reverse(room));
                    }
                    _ => break,
                }
            }
            let room = if let Some(std::cmp::Reverse(room)) = next_free.pop() {
                next_busy.push(std::cmp::Reverse((end, room)));
                room
            } else {
                let meeting_length = end - start;
                let std::cmp::Reverse((earliest_end, room)) = &mut *next_busy.peek_mut().unwrap();
                *earliest_end += meeting_length;
                *room
            };
            room_bookings[room as usize] += 1;
        }
        let mut result: u8 = 0;
        for i in 1..room_bookings.len() as u8 {
            if room_bookings[i as usize] > room_bookings[result as usize] {
                result = i;
            }
        }
        result as i32
    }
}

// Const array tupleization
// impl Solution {
//     pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
//         /// Convert a vector of length-2 vectors of i32 to a boxed slice of
//         /// length-2 tuples of u32.
//         ///
//         /// Abuses the fact that the memory layout of a Vec<i32> is at least
//         /// as large as (u32, u32), so we can safely transmute the provided
//         /// memory to a slice of tuples.
//         fn tupleize_events(events: Vec<Vec<i32>>) -> Box<[(u32, u32)]> {
//             // First, convert this vector of vectors to a more useful vector
//             // of tuples. We know each subvector has exactly two elements, so
//             // we can stuff them into tuples in the same memory space as the
//             // original sub-vectors.
//             // Obviously super unsafe if run on a 32-bit system, but this is
//             // a toy problem on leetcode, so the environment is known.
//             // While we're at it, we know all meetings are non-negative, so
//             // we can use u32 instead of i32.
//             let original_len = events.len();
//             let events_ptr = events.leak();
//             let events_ptr2: &mut [Vec<i32>] = &mut events_ptr[0..original_len];
//             let events_tuple_ptr =
//                 unsafe { std::mem::transmute::<_, &mut [(u32, u32)]>(events_ptr2) };
//             for i in 0..original_len {
//                 let tup = (events_ptr[i][0] as u32, events_ptr[i][1] as u32);
//                 events_tuple_ptr[i] = tup;
//             }
//             unsafe { std::boxed::Box::from_raw(events_tuple_ptr) }
//         }
//         let n = match n {
//             i32::MIN..=0 => panic!("n must be positive"),
//             1 => return 0,
//             2..=100 => n as u8,
//             _ => panic!("n must be at most 100"),
//         };
//         let meetings = {
//             let mut meetings = tupleize_events(meetings);
//             meetings.sort_unstable_by_key(|x| x.0);
//             meetings
//         };
//         Self::most_booked_sorted_tuple_input(n, &meetings) as i32
//     }
//     pub const fn most_booked_sorted_tuple_input(n: u8, meetings: &[(u32, u32)]) -> u8 {
//         const MAX_N: u8 = 100;
//         assert!(n > 0);
//         assert!(n <= MAX_N);
//         let mut room_free_at = [0u32; MAX_N as usize];
//         let mut room_bookings = [0u32; MAX_N as usize];
//         {
//             let mut meeting_idx = 0u32;
//             'outer: while meeting_idx < meetings.len() as u32 {
//                 let (start, end) = meetings[meeting_idx as usize];
//                 let mut earliest_room = 0;
//                 {
//                     let mut room_idx = 0u8;
//                     while room_idx < n {
//                         if start >= room_free_at[room_idx as usize] {
//                             // println!(
//                             //     "Booking room {} for {}-{} (last booking ended {})",
//                             //     room_idx, start, end, room_free_at[room_idx as usize]
//                             // );
//                             room_free_at[room_idx as usize] = end;
//                             room_bookings[room_idx as usize] += 1;
//                             meeting_idx += 1;
//                             continue 'outer;
//                         }
//                         if room_free_at[room_idx as usize] < room_free_at[earliest_room as usize] {
//                             earliest_room = room_idx;
//                         }
//                         room_idx += 1;
//                     }
//                 }
//                 // println!(
//                 //     "Booking room {} for {}-{} (delayed from {} to {})",
//                 //     earliest_room,
//                 //     room_free_at[earliest_room as usize],
//                 //     room_free_at[earliest_room as usize] + (end - start),
//                 //     start,
//                 //     end,
//                 // );
//                 room_free_at[earliest_room as usize] += end - start;
//                 room_bookings[earliest_room as usize] += 1;
//                 meeting_idx += 1;
//             }
//         }
//         let mut result: u8 = 0;
//         let mut room_idx: u8 = 1;
//         while room_idx < n {
//             if room_bookings[room_idx as usize] > room_bookings[result as usize] {
//                 result = room_idx;
//             }
//             room_idx += 1;
//         }
//         result
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]),
            0
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::most_booked(
                3,
                vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]]
            ),
            1
        )
    }

    #[test]
    fn discussion_case1() {
        assert_eq!(
            Solution::most_booked(
                3,
                vec![
                    vec![18, 19],
                    vec![3, 12],
                    vec![17, 19],
                    vec![2, 13],
                    vec![7, 10]
                ]
            ),
            0
        )
    }

    #[test]
    fn discussion_case2() {
        assert_eq!(
            Solution::most_booked(
                3,
                vec![vec![0, 10], vec![1, 9], vec![2, 8], vec![3, 7], vec![4, 6],]
            ),
            1
        )
    }

    #[test]
    fn discussion_case3() {
        assert_eq!(
            Solution::most_booked(
                3,
                vec![
                    vec![1, 27],
                    vec![29, 49],
                    vec![47, 49],
                    vec![41, 43],
                    vec![15, 36],
                    vec![11, 15],
                ]
            ),
            1
        )
    }

    #[test]
    fn discussion_case4() {
        assert_eq!(
            Solution::most_booked(
                4,
                vec![
                    vec![12, 44],
                    vec![27, 37],
                    vec![48, 49],
                    vec![46, 49],
                    vec![24, 44],
                    vec![32, 38],
                    vec![21, 49],
                    vec![13, 30],
                ]
            ),
            1
        )
    }

    #[test]
    fn discussion_case5() {
        assert_eq!(
            Solution::most_booked(
                2,
                vec![
                    vec![10, 11],
                    vec![2, 10],
                    vec![1, 17],
                    vec![9, 13],
                    vec![18, 20]
                ]
            ),
            1
        )
    }

    #[test]
    fn my_min_ex1() {
        assert_eq!(Solution::most_booked(1, vec![vec![0, 1]]), 0)
    }

    #[test]
    fn my_min_ex2() {
        assert_eq!(Solution::most_booked(1, vec![vec![0, 1], vec![1, 2]]), 0)
    }

    #[test]
    fn my_min_ex3() {
        assert_eq!(
            Solution::most_booked(2, vec![vec![0, 3], vec![1, 2], vec![2, 3]]),
            1
        )
    }

    #[test]
    fn my_extreme_ex1() {
        assert_eq!(
            Solution::most_booked(
                2,
                vec![
                    vec![0, 500_000],
                    vec![1, 500_000],
                    vec![2, 500_000],
                    vec![3, 500_000],
                    vec![4, 500_000]
                ]
            ),
            1
        )
    }

    #[test]
    fn my_extreme_ex2() {
        assert_eq!(
            Solution::most_booked(
                2,
                vec![
                    vec![0, 500_000],
                    vec![1, 500_000],
                    vec![2, 499_999],
                    vec![3, 500_000],
                    vec![4, 500_000]
                ]
            ),
            0
        )
    }

    #[test]
    fn my_extreme_ex3() {
        assert_eq!(
            Solution::most_booked(
                2,
                vec![
                    vec![0, 500_000],
                    vec![1, 500_000],
                    vec![2, 499_998],
                    vec![3, 500_000],
                    vec![4, 500_000]
                ]
            ),
            0
        )
    }
}
