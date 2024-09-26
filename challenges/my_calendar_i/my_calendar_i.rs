// https://leetcode.com/problems/my-calendar-i/

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
        for (i,((start, end), &exp)) in std::iter::Iterator::zip(bookings.iter(), expected.iter()).enumerate() {
            assert_eq!(obj.book(*start, *end), exp, "Test failed at index: {} with start: {} and end: {}", i, start, end);
        }
    }

    #[test]
    fn ex1() {
        test(&[(10, 20), (15, 25), (20, 30)], &[true, false, true]);
    }
}
