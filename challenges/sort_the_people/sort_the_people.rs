// https://leetcode.com/problems/sort-the-people/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn sort_people(mut names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        assert_eq!(names.len(), heights.len());
        assert!(names.len() <= 1000);
        let mut permutation_vec = (0..names.len() as u16).collect::<Vec<_>>();
        permutation_vec.sort_unstable_by_key(|&i| -heights[i as usize]);
        // Rearrange in-place using https://devblogs.microsoft.com/oldnewthing/20170102-00/?p=95095
        // to avoid allocating any more new vectors.
        for i in 0..names.len() as u16 {
            let mut current = i;
            while i != permutation_vec[current as usize] {
                let next = permutation_vec[current as usize];
                names.swap(current as usize, next as usize);
                permutation_vec[current as usize] = current;
                current = next;
            }
            permutation_vec[current as usize] = current;
        }
        names
    }
}

// Lazy realloc sol'n
// impl Solution {
//     pub fn sort_people(mut names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
//         assert_eq!(names.len(), heights.len());
//         assert!(names.len() <= 1000);
//         let mut permutation_vec = (0..names.len() as u16).collect::<Vec<_>>();
//         permutation_vec.sort_unstable_by_key(|&i| -heights[i as usize]);
//         let mut result = Vec::with_capacity(names.len());
//         let result_space = result.spare_capacity_mut();
//         for i in 0..names.len() as u16 {
//             result_space[i as usize].write(std::mem::replace(&mut names[permutation_vec[i as usize] as usize], String::new()));
//         }
//         unsafe { result.set_len(names.len()) } // Safe because we just set them
//                                                // with the permutation vec
//         unsafe { names.set_len(0) } // Safe b/c we just moved out all strings
//         result
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(names: &[&str], heights: &[i32], expected: &[&str]) {
        assert_eq!(names.len(), heights.len());
        assert_eq!(names.len(), expected.len());
        assert!(names.len() >= 1);
        assert!(names.len() <= 1000);
        for &name in names.iter() {
            assert!(name.len() >= 1);
            assert!(name.len() <= 20);
            assert!(name.bytes().all(|b| b.is_ascii_alphabetic()));
        }
        for &height in heights.iter() {
            assert!(height >= 1);
            assert!(height <= 100_000);
        }
        let names = names.iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::sort_people(names, heights.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(
            &["Mary", "John", "Emma"],
            &[180, 165, 170],
            &["Mary", "Emma", "John"],
        );
    }

    #[test]
    fn ex2() {
        test(
            &["Alice", "Bob", "Bob"],
            &[155, 185, 150],
            &["Bob", "Alice", "Bob"],
        );
    }
}
