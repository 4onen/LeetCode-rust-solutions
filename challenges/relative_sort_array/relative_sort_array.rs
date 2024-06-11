// https://leetcode.com/problems/relative-sort-array/

pub struct Solution;

// Braindead sol'n
// impl Solution {
//     pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
//         arr1.sort_unstable_by(|a, b| {
//             match (
//                 arr2.iter().position(|x| x == a),
//                 arr2.iter().position(|x| x == b),
//             ) {
//                 (Some(i), Some(j)) => i.cmp(&j),
//                 (Some(_), None) => std::cmp::Ordering::Less,
//                 (None, Some(_)) => std::cmp::Ordering::Greater,
//                 (None, None) => a.cmp(b),
//             }
//         });
//         arr1
//     }
// }

// Counting sort
impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut count = [0u16; 1001];
        for &x in &arr1 {
            count[x as usize] += 1;
        }
        let mut i = 0;
        for &x in &arr2 {
            arr1[i..i + count[x as usize] as usize].fill(x);
            i += count[x as usize] as usize;
            count[x as usize] = 0;
        }
        for (x, &c) in count.iter().enumerate() {
            arr1[i..i + c as usize].fill(x as i32);
            i += c as usize;
        }
        arr1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr1: &[i32], arr2: &[i32], expected: &[i32]) {
        assert!(arr1.len() > 0);
        assert!(arr2.len() > 0);
        assert!(arr1.len() <= 1000);
        assert!(expected.len() == arr1.len());
        for &x in arr1 {
            assert!(x >= 0);
            assert!(x <= 1000);
        }
        for &x in arr2 {
            assert!(x >= 0);
            assert!(x <= 1000);
        }
        assert_eq!(
            Solution::relative_sort_array(arr1.to_vec(), arr2.to_vec()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(
            &[2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            &[2, 1, 4, 3, 9, 6],
            &[2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19],
        );
    }

    #[test]
    fn ex2() {
        test(
            &[28, 6, 22, 8, 44, 17],
            &[22, 28, 8, 6],
            &[22, 28, 8, 6, 17, 44],
        );
    }

    #[test]
    fn discussion_case1() {
        test(
            &[
                2, 21, 43, 38, 0, 42, 33, 7, 24, 13, 12, 27, 12, 24, 5, 23, 29, 48, 30, 31,
            ],
            &[2, 42, 38, 0, 43, 21],
            &[
                2, 42, 38, 0, 43, 21, 5, 7, 12, 12, 13, 23, 24, 24, 27, 29, 30, 31, 33, 48,
            ],
        );
    }
}
