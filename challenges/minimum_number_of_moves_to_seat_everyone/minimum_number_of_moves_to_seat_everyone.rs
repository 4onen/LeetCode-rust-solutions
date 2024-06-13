// https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
//         assert!(seats.len() == students.len());
//         assert!(seats.len() >= 1);
//         assert!(seats.len() <= 100);
//         seats.sort_unstable();
//         students.sort_unstable();
//         let mut moves = 0;
//         for i in 0..seats.len() {
//             moves += (seats[i] - students[i]).abs();
//         }
//         moves
//     }
// }

// Zip iter
impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        assert!(seats.len() == students.len());
        assert!(seats.len() >= 1);
        assert!(seats.len() <= 100);
        seats.sort_unstable();
        students.sort_unstable();
        let mut moves = 0;
        for (s, st) in std::iter::zip(seats.into_iter(), students.into_iter()) {
            moves += (s - st).abs();
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(seats: &[i32], students: &[i32], expected: i32) {
        assert!(seats.len() == students.len());
        assert!(seats.len() >= 1);
        assert!(seats.len() <= 100);
        for &s in seats {
            assert!(s >= 1);
            assert!(s <= 100);
        }
        for &s in students {
            assert!(s >= 1);
            assert!(s <= 100);
        }
        assert_eq!(
            Solution::min_moves_to_seat(seats.to_vec(), students.to_vec()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[3, 1, 5], &[2, 7, 4], 4);
    }

    #[test]
    fn ex2() {
        test(&[4, 1, 5, 9], &[1, 3, 2, 6], 7);
    }

    #[test]
    fn ex3() {
        test(&[2, 2, 6, 6], &[1, 3, 2, 6], 4);
    }
}
