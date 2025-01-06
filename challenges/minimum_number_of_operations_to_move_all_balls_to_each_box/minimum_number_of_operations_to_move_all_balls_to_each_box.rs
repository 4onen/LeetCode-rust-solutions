// https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn min_operations(boxes: String) -> Vec<i32> {
//         let boxes = boxes.as_bytes();
//         let mut res = vec![0i32; boxes.len()];
//         let mut seen_right = 0;
//         let mut move_to = 0;
//         for &b in boxes.iter().rev() {
//             move_to += seen_right;
//             seen_right += (b == b'1') as i32;
//         }
//         res[0] = move_to;
//         let total = seen_right;
//         let mut seen_left = (boxes[0] == b'1') as i32;
//         for i in 1..boxes.len() {
//             let here = (boxes[i] == b'1') as i32;
//             move_to += 2 * seen_left - total;
//             seen_left += here;
//             res[i] = move_to;
//         }
//         res
//     }
// }

// Clean up loop logic, make loops both index-based
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes.as_bytes();
        let mut res = vec![0i32; boxes.len()];
        let mut seen_right = 0;
        let mut move_to = 0;
        for i in (0..boxes.len()).rev() {
            move_to += seen_right;
            seen_right += (boxes[i] == b'1') as i32;
        }
        res[0] = move_to;
        let total = seen_right;
        let mut seen_left = 0;
        let mut i = 0;
        loop {
            seen_left += (boxes[i] == b'1') as i32;
            i += 1;
            if i >= boxes.len() {
                break;
            }
            res[i] = res[i - 1] + 2 * seen_left - total;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(boxes: &str, expected: &[i32]) {
        assert!(boxes.len() >= 1);
        assert!(boxes.len() <= 2000);
        for &b in boxes.as_bytes() {
            assert!(b == b'0' || b == b'1');
        }
        assert_eq!(Solution::min_operations(boxes.to_owned()), expected);
        let reversed_boxes = {
            let mut r = boxes.as_bytes().to_owned();
            r.reverse();
            unsafe { std::string::String::from_utf8_unchecked(r) }
        };
        assert_eq!(
            Solution::min_operations(reversed_boxes),
            expected.iter().copied().rev().collect::<Vec<_>>()
        );
    }

    #[test]
    fn ex1() {
        test("110", &[1, 1, 3])
    }

    #[test]
    fn ex2() {
        test("001011", &[11, 8, 5, 4, 3, 4])
    }
}
