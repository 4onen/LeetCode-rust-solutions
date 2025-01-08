// https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
//         let mut students = 0;
//         for (start, end) in std::iter::zip(start_time, end_time) {
//             students += ((query_time >= start) && (query_time <= end)) as u8;
//         }
//         students as i32
//     }
// }

// Constant sol'n
// impl Solution {
//     pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
//         const fn busy_student(start_time: &[i32], end_time: &[i32], query_time: i32) -> i32 {
//             //assert!(start_time.len() == end_time.len());
//             let mut i = 0;
//             let mut students = 0;
//             while i < start_time.len() as u8 {
//                 students += ((query_time >= start_time[i as usize])
//                     && (query_time <= end_time[i as usize])) as u8;
//                 i += 1;
//             }
//             students as i32
//         }
//         busy_student(&start_time, &end_time, query_time)
//     }
// }

// Removing as many asserts as possible
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        const fn busy_student(start_time: &[i32], end_time: &[i32], query_time: i32) -> i32 {
            //assert!(start_time.len() == end_time.len());
            let mut i = 0;
            let mut students = 0;
            while i < start_time.len() {
                students += ((query_time >= start_time[i as usize])
                    && (query_time <= end_time[i as usize])) as u8;
                i += 1;
            }
            students as i32
        }
        busy_student(&start_time, &end_time, query_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(start_time: &[i32], end_time: &[i32], query_time: i32, expected: i32) {
        assert_eq!(start_time.len(), end_time.len());
        assert!(start_time.len() >= 1);
        assert!(start_time.len() <= 100);
        for (&start, &end) in std::iter::zip(start_time, end_time) {
            assert!(start >= 1);
            assert!(end >= start);
            assert!(end <= 1000);
        }
        assert!(query_time >= 1);
        assert!(query_time <= 1000);
        assert_eq!(
            Solution::busy_student(start_time.to_vec(), end_time.to_vec(), query_time),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 3], &[3, 2, 7], 4, 1)
    }

    #[test]
    fn ex2() {
        test(&[4], &[4], 4, 1)
    }
}
