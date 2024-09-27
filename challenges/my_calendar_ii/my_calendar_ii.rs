// https://leetcode.com/problems/my-calendar-ii/

// Initial, complicated line sweep handcrafted vec sol'n
// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// struct BookingEdge(i32);
// impl BookingEdge {
//     const fn from_start(start: i32) -> Self {
//         BookingEdge(start << 1)
//     }
//     const fn from_end(end: i32) -> Self {
//         BookingEdge((end << 1) | 1)
//     }
//     const fn is_start(&self) -> bool {
//         self.0 & 1 == 0
//     }
// }
// impl std::fmt::Debug for BookingEdge {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if self.is_start() {
//             write!(f, "BookingStart({})", self.0 >> 1)
//         } else {
//             write!(f, "BookingEnd({})", self.0 >> 1)
//         }
//     }
// }
// const MAX_BOOKINGS: u16 = 1000;
// const MAX_BOOKING_EDGES: u16 = MAX_BOOKINGS * 2;
// pub struct MyCalendarTwo {
//     bookings: [BookingEdge; MAX_BOOKING_EDGES as usize],
//     length: u16,
// }
// impl MyCalendarTwo {
//     pub fn new() -> Self {
//         MyCalendarTwo {
//             #[allow(invalid_value)]
//             // Safety: "Length" controls the values we read from the array
//             // so we can safely ignore the warning about leaving the array
//             // uninitialized.
//             bookings: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
//             length: 0,
//         }
//     }
//     pub fn book(&mut self, start: i32, end: i32) -> bool {
//         let start = BookingEdge::from_start(start);
//         let end = BookingEdge::from_end(end);
//         dbg!(&self.bookings[..self.length as usize], start, end);
//         let mut count: u8 = 0;
//         let mut i: u16 = 0;
//         let mut start_idx = MAX_BOOKING_EDGES;
//         loop {
//             if i >= self.length {
//                 break;
//             }
//             let edge = self.bookings[i as usize];
//             if edge.0 > end.0 {
//                 break;
//             }
//             if edge.is_start() {
//                 count += 1;
//             } else {
//                 count -= 1;
//             }
//             if count >= 2 - ((edge > start) as u8) {
//                 return false;
//             }
//             if edge > start && start_idx >= MAX_BOOKING_EDGES {
//                 start_idx = i;
//             }
//             i += 1;
//         }
//         // Insert the booking
//         if start_idx >= MAX_BOOKINGS {
//             self.bookings[self.length as usize] = start;
//             self.length += 1;
//             self.bookings[self.length as usize] = end;
//             self.length += 1;
//         } else {
//             let mut i = self.length;
//             while i > start_idx {
//                 if self.bookings[i as usize] < end {
//                     self.bookings[i as usize + 2] = end;
//                     break;
//                 }
//                 self.bookings[i as usize + 2] = self.bookings[i as usize];
//                 i -= 1;
//             }
//             while i > start_idx {
//                 self.bookings[i as usize + 1] = self.bookings[i as usize];
//                 i -= 1;
//             }
//             self.bookings[start_idx as usize] = start;
//             self.length += 2;
//         }
//         true
//     }
// }

// Prefix sum sol'n (Still too complicated)
// const MAX_BOOKINGS: u16 = 1000;
// const MAX_BOOKING_EDGES: u16 = MAX_BOOKINGS * 2;
// #[derive(Debug)]
// pub struct MyCalendarTwo {
//     edges: std::vec::Vec<i32>,
//     counts: std::vec::Vec<u8>,
// }
// const fn unwrap_either<T>(res: &Result<T, T>) -> T
// where
//     T: Copy,
// {
//     match res {
//         &Ok(val) => val,
//         &Err(val) => val,
//     }
// }
// impl MyCalendarTwo {
//     pub fn new() -> Self {
//         MyCalendarTwo {
//             edges: std::vec::Vec::with_capacity(MAX_BOOKING_EDGES as usize),
//             counts: std::vec::Vec::with_capacity(MAX_BOOKING_EDGES as usize),
//         }
//     }
//     pub fn book(&mut self, start: i32, end: i32) -> bool {
//         let start_idx = match self.edges.binary_search(&start) {
//             Err(pos) if pos >= self.edges.len() => {
//                 self.edges.extend_from_slice(&[start, end]);
//                 self.counts.extend_from_slice(&[1, 0]);
//                 return true;
//             }
//             otherwise => otherwise,
//         };
//         let end_idx = match self.edges.binary_search(&end) {
//             Ok(0) => {
//                 self.edges.splice(0..0, [start, end].into_iter());
//                 self.counts.splice(0..0, [0, 1].into_iter());
//                 return true;
//             }
//             Err(0) => {
//                 self.edges.splice(0..0, [start, end].into_iter());
//                 self.counts.splice(0..0, [0, 1].into_iter());
//                 return true;
//             }
//             otherwise => otherwise,
//         };
//         // First, do a path to check if there will be more than 2 overlaps
//         // If there are, return false
//         dbg!(&self, &start_idx, &end_idx);
//         if self.counts[unwrap_either(&start_idx)..unwrap_either(&end_idx)]
//             .iter()
//             .any(|&count| count >= 2)
//         {
//             return false;
//         }
//         // Next, update the counts over this range.
//         // Insert the start and end edges if they don't exist.
//         let mut end_idx_num = match end_idx {
//             Ok(pos) => pos,
//             Err(pos) => {
//                 self.edges.insert(pos, end);
//                 self.counts.insert(pos, self.counts[pos - 1]);
//                 pos
//             }
//         };
//         let start_idx_num = match start_idx {
//             Ok(pos) => pos,
//             Err(pos) => {
//                 self.edges.insert(pos, start);
//                 self.counts.insert(pos, self.counts[pos - 1]);
//                 end_idx_num += 1; // We just shifted the end right by 1
//                 pos
//             }
//         };
//         for i in start_idx_num..end_idx_num {
//             self.counts[i] += 1;
//         }
//         true
//     }
// }

// Bookings and overlaps like last assignment sol'n
const MAX_BOOKINGS: u16 = 1000;
pub struct MyCalendarTwo {
    bookings: std::vec::Vec<(i32, i32)>,
    overlaps: std::vec::Vec<(i32, i32)>,
}
impl MyCalendarTwo {
    pub fn new() -> Self {
        MyCalendarTwo {
            bookings: std::vec::Vec::with_capacity(MAX_BOOKINGS as usize),
            overlaps: std::vec::Vec::with_capacity(MAX_BOOKINGS as usize),
        }
    }
    pub fn book(&mut self, start: i32, end: i32) -> bool {
        let mut i = 0;
        while i < self.overlaps.len() {
            let (s, e) = self.overlaps[i];
            if end <= s {
                // No overlap
                break;
            } else if start >= e {
                // No overlap
                i += 1;
            } else {
                // Overlap
                return false;
            }
        }
        // Check for overlaps with existing bookings
        i = 0;
        while i < self.bookings.len() {
            let (s, e) = self.bookings[i];
            if end <= s {
                // No overlap
                i += 1;
            } else if start > e {
                // No overlap
                i += 1;
            } else {
                i += 1;
                // Find the right position to insert the overlap
                // We already know it can't overlap with any existing overlaps
                // because we would have returned false above.
                let overlap = (std::cmp::max(start, s), std::cmp::min(end, e));
                if overlap.0 >= overlap.1 {
                    // No overlap -- False alarm
                    continue;
                }
                let pos = self
                    .overlaps
                    .binary_search_by_key(&overlap, |&(s, e)| (s, e))
                    .unwrap_or_else(|x| x);
                self.overlaps.insert(pos, overlap);
            }
        }
        // Insert the booking (Gave up on sorting)
        self.bookings.push((start, end));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(bookings: &[(i32, i32)], expected: &[bool]) {
        assert_eq!(bookings.len(), expected.len());
        let mut obj = MyCalendarTwo::new();
        for (i, ((start, end), &exp)) in
            std::iter::Iterator::zip(bookings.iter(), expected.iter()).enumerate()
        {
            assert_eq!(
                obj.book(*start, *end),
                exp,
                "Test failed at index: {} with start: {} and end: {}",
                i,
                start,
                end
            );
        }
    }

    #[test]
    fn ex1() {
        test(
            &[(10, 20), (50, 60), (10, 40), (5, 15), (5, 10), (25, 55)],
            &[true, true, true, false, true, true],
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            &[(26, 35), (26, 32), (25, 32), (18, 26), (19, 26)],
            &[true, true, false, true, true],
        )
    }

    #[test]
    fn discussion_case2() {
        test(
            &[
                (28, 46),
                (9, 21),
                (21, 39),
                (37, 48),
                (38, 50),
                (22, 39),
                (45, 50),
                (1, 12),
                (40, 50),
                (31, 44),
            ],
            &[
                true, true, true, false, false, false, true, true, false, false,
            ],
        )
    }

    #[test]
    fn discussion_case3() {
        test(
            &[
                (24, 40),
                (43, 50),
                (27, 43),
                (5, 21),
                (30, 40),
                (14, 29),
                (3, 19),
                (3, 14),
                (25, 39),
                (6, 19),
            ],
            &[
                true, true, true, true, false, false, true, false, false, false,
            ],
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[(10, 20), (25, 30), (35, 40), (13, 37), (22, 24), (25, 55)],
            &[true, true, true, true, true, false],
        )
    }

    #[test]
    fn failing_case1() {
        test(
            &[
                (22, 29),
                (12, 17),
                (20, 27),
                (27, 36),
                (24, 31),
                (23, 28),
                (47, 50),
                (23, 30),
                (24, 29),
                (19, 25),
                (19, 27),
                (3, 9),
                (34, 41),
                (22, 27),
                (3, 9),
                (29, 38),
                (34, 40),
                (49, 50),
                (42, 48),
                (43, 50),
                (39, 44),
                (30, 38),
                (42, 50),
                (31, 39),
                (9, 16),
                (10, 18),
                (31, 39),
                (30, 39),
                (48, 50),
                (36, 42),
            ],
            &[
                true, true, true, true, false, false, true, false, false, false, false, true, true,
                false, true, false, false, true, true, false, true, false, false, false, true,
                false, false, false, false, false,
            ],
        )
    }

    #[test]
    fn failing_case1_trim7() {
        test(
            &[
                (22, 29),
                (20, 27),
                (27, 36),
                // g1
                (34, 41),
                (29, 38),
            ],
            &[
                true, true, true, // g1
                true, false,
            ],
        )
    }

    #[test]
    fn failing_case1_trim8() {
        test(
            &[
                (20, 27),
                (27, 36),
                // g1
                (34, 41),
                (29, 38),
            ],
            &[
                true, true, // g1
                true, false,
            ],
        )
    }

    #[test]
    fn failing_case1_trim9() {
        test(
            &[
                (27, 36),
                // g1
                (34, 41),
                (29, 38),
            ],
            &[
                true, // g1
                true, false,
            ],
        )
    }

    #[test]
    fn failing_case2() {
        test(
            &[
                (99, 100),
                (11, 19),
                (86, 92),
                (9, 14),
                (70, 76),
                (55, 64),
                (48, 55),
                (39, 47),
                (21, 26),
                (54, 60),
                (92, 97),
                (21, 28),
                (20, 25),
                (24, 33),
                (36, 44),
                (79, 88),
                (15, 22),
                (89, 94),
                (15, 22),
                (39, 44),
                (49, 55),
                (36, 45),
                (79, 86),
                (4, 10),
                (48, 56),
                (8, 15),
                (98, 100),
                (54, 60),
                (76, 85),
                (24, 31),
                (75, 80),
                (91, 100),
                (74, 80),
                (3, 11),
                (37, 43),
                (56, 63),
                (35, 41),
                (61, 67),
                (7, 16),
                (83, 91),
                (47, 52),
                (65, 72),
                (88, 96),
                (49, 58),
                (49, 54),
                (18, 25),
                (66, 75),
                (44, 49),
                (75, 83),
                (94, 100),
                (5, 14),
                (46, 54),
                (95, 100),
                (12, 19),
                (17, 22),
                (16, 22),
                (38, 44),
                (83, 91),
                (53, 61),
                (99, 100),
                (77, 83),
                (27, 35),
                (95, 100),
                (77, 84),
                (63, 70),
                (18, 26),
                (52, 60),
                (51, 57),
                (12, 18),
                (96, 100),
                (56, 61),
                (2, 9),
                (35, 44),
                (62, 70),
                (30, 36),
                (41, 48),
                (92, 100),
                (35, 41),
                (61, 69),
                (3, 11),
            ],
            &[
                true, true, true, true, true, true, true, true, true, true, true, true, false,
                false, true, true, false, true, false, false, false, false, true, true, false,
                false, true, false, false, false, false, false, false, false, false, false, false,
                true, false, false, true, true, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, false, false, false, false, false, false, false, false, true, false,
                false, true, false, false, false, false, false,
            ],
        )
    }

    #[test]
    fn failing_case2_trim1() {
        test(
            &[
                (99, 100),
                (11, 19),
                (86, 92),
                (9, 14),
                (70, 76),
                (55, 64),
                (48, 55),
                (39, 47),
                (21, 26),
                (54, 60),
                (92, 97),
                (21, 28),
                (20, 25),
                (24, 33),
                (36, 44),
                (79, 88),
                (15, 22),
                (89, 94),
            ],
            &[
                true, true, true, true, true, true, true, true, true, true, true, true, false,
                false, true, true, false, true,
            ],
        )
    }

    #[test]
    fn failing_case2_trim2() {
        test(
            &[
                (99, 100),
                (11, 19),
                (86, 92),
                // g1
                (9, 14),
                (70, 76),
                (55, 64),
                // g2
                (48, 55),
                (39, 47),
                (21, 26),
                // g3
                (54, 60),
                (92, 97),
                (21, 28),
                // g4
                (36, 44),
                // g5
                (79, 88),
                (89, 94),
            ],
            &[
                true, true, true, // g1
                true, true, true, // g2
                true, true, true, // g3
                true, true, true, // g4
                true, // g5
                true, true,
            ],
        )
    }

    #[test]
    fn failing_case2_trim3() {
        test(
            &[(86, 92), (92, 97), (79, 88), (89, 94)],
            &[true, true, true, true],
        )
    }
}
