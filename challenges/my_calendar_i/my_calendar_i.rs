// https://leetcode.com/problems/my-calendar-i/

// Initial sol'n
pub struct MyCalendar {
    // Start-sorted list of bookings
    bookings: Vec<(i32, i32)>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    pub fn new() -> Self {
        MyCalendar {
            bookings: Vec::new(),
        }
    }
    pub fn book(&mut self, start: i32, end: i32) -> bool {
        let mut i = 0;
        while i < self.bookings.len() {
            let (s,e) = self.bookings[i];
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
        // Insert the booking
        self.bookings.insert(i, (start, end));
        true
    }
}

// Writing my own vec for tiny gains
// const MAX_BOOKINGS: usize = 1000;
// pub struct MyCalendar {
//     starts: [i32; MAX_BOOKINGS],
//     ends: [i32; MAX_BOOKINGS],
//     length: u16,
// }
// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl MyCalendar {
//     pub fn new() -> Self {
//         // Ignore the warning about using MaybeUninit to leave it actually uninit
//         let mut res = unsafe { std::mem::MaybeUninit::<Self>::uninit().assume_init() };
//         res.length = 0;
//         res
//     }
//     pub fn book(&mut self, start: i32, end: i32) -> bool {
//         // Binary search for the insertion point
//         let pos = match self.starts[..self.length as usize].binary_search(&start) {
//             Ok(_) => return false,
//             Err(pos) => pos as u16,
//         };
//         // Check for overlap
//         // Check if the previous booking ends after the new booking starts
//         if pos > 0 && self.ends[pos as usize - 1] > start {
//             return false;
//         }
//         // Check if the next booking starts before the new booking ends
//         if pos < self.length && self.starts[pos as usize] < end {
//             return false;
//         }
//         // Insert the booking
//         self.starts
//             .copy_within(pos as usize..self.length as usize, pos as usize + 1);
//         self.ends
//             .copy_within(pos as usize..self.length as usize, pos as usize + 1);
//         self.starts[pos as usize] = start;
//         self.ends[pos as usize] = end;
//         self.length += 1;
//         true
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(bookings: &[(i32, i32)], expected: &[bool]) {
        /*
         * Your MyCalendar object will be instantiated and called as such:
         * let obj = MyCalendar::new();
         * let ret_1: bool = obj.book(start, end);
         */
        let mut obj = MyCalendar::new();
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
        test(&[(10, 20), (15, 25), (20, 30)], &[true, false, true])
    }

    #[test]
    fn ex2() {
        test(
            &[
                (20, 29),
                (13, 22),
                (44, 50),
                (1, 7),
                (2, 10),
                (14, 20),
                (19, 25),
                (36, 42),
                (45, 50),
                (47, 50),
                (39, 45),
                (44, 50),
                (16, 25),
                (45, 50),
                (45, 50),
                (12, 20),
                (21, 29),
                (11, 20),
                (12, 17),
                (34, 40),
                (10, 18),
                (38, 44),
                (23, 32),
                (38, 44),
                (15, 20),
                (27, 33),
                (34, 42),
                (44, 50),
                (35, 40),
                (24, 31),
            ],
            &[
                true, false, true, true, false, true, false, true, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false,
            ],
        )
    }
}
