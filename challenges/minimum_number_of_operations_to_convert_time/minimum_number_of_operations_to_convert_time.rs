// https://leetcode.com/problems/minimum-number-of-operations-to-convert-time/

pub struct Solution;

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        const fn cvt_time_mins(time: &str) -> u16 {
            let time = time.as_bytes();
            let hour = (time[0] - b'0') * 10 + (time[1] - b'0');
            let min = (time[3] - b'0') * 10 + (time[4] - b'0');
            60 * hour as u16 + min as u16
        }
        const INTERVALS: [u16; 4] = [60, 15, 5, 1];
        let mut diff_mins = cvt_time_mins(&correct) - cvt_time_mins(&current);
        let mut interval = 0u8;
        let mut count = 0;
        while interval < INTERVALS.len() as u8 {
            count += diff_mins / INTERVALS[interval as usize];
            diff_mins = diff_mins % INTERVALS[interval as usize];
            interval += 1;
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(current: &str, correct: &str, expected: i32) {
        fn is_timestr(st: &str) -> bool {
            let s = st.as_bytes();
            assert!(s.len() == 5);
            assert!(s[0].is_ascii_digit());
            assert!(s[1].is_ascii_digit());
            assert!(s[2] == b':');
            assert!(s[3].is_ascii_digit());
            assert!(s[4].is_ascii_digit());
            assert!(st >= "00:00");
            assert!(st <= "23:59");
            return true;
        }
        assert!(is_timestr(current));
        assert!(is_timestr(correct));
        assert!(current <= correct);
        assert_eq!(
            Solution::convert_time(current.to_string(), correct.to_string()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test("02:30", "04:35", 3)
    }

    #[test]
    fn ex2() {
        test("11:00", "11:01", 1)
    }

    #[test]
    fn myex1() {
        test("00:00", "23:59", 23 + 3 + 2 + 4)
    }
}
