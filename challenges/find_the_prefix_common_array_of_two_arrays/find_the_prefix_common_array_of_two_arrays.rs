// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
//         let mut seen_state = vec![false; a.len()];
//         let mut common = 0;
//         let mut res = std::vec::Vec::with_capacity(a.len());
//         for i in 0..a.len() as u8 {
//             let idx_a = (a[i as usize] - 1) as usize;
//             common += seen_state[idx_a] as i32;
//             seen_state[idx_a] = true;
//             let idx_b = (b[i as usize] - 1) as usize;
//             common += seen_state[idx_b] as i32;
//             seen_state[idx_b] = true;
//             res.push(common);
//         }
//         res
//     }
// }

// Bitset sol'n
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        assert_eq!(a.len(), b.len());
        assert!(a.len() <= 50);
        let mut seen_state = 0u64;
        let mut common = 0i32;
        let mut res = std::vec::Vec::with_capacity(a.len());
        for i in 0..a.len() as u8 {
            let idx_a = a[i as usize];
            common += ((seen_state >> idx_a as u64) & 1) as i32;
            seen_state |= 1 << idx_a as u64;
            let idx_b = b[i as usize];
            common += ((seen_state >> idx_b as u64) & 1) as i32;
            seen_state |= 1 << idx_b as u64;
            res.push(common);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(a: &[i32], b: &[i32], expected: &[i32]) {
        assert_eq!(a.len(), b.len());
        assert!(a.len() >= 1);
        assert!(a.len() <= 50);
        let mut seen = vec![false; a.len()];
        for &i in a {
            assert!(i >= 1);
            assert!(i <= a.len() as i32);
            seen[(i - 1) as usize] = true;
        }
        assert!(seen.into_iter().all(|x|x));
        let mut seen = vec![false; a.len()];
        for &i in b {
            assert!(i >= 1);
            assert!(i <= a.len() as i32);
            seen[(i - 1) as usize] = true;
        }
        assert!(seen.drain(..).all(|x|x));
        assert_eq!(Solution::find_the_prefix_common_array(a.to_owned(), b.to_owned()), expected);
    }

    #[test]
    fn ex1() {
        test(&[1,3,2,4], &[3,1,2,4], &[0,2,3,4])
    }

    #[test]
    fn ex2() {
        test(&[2,3,1], &[3,1,2], &[0,1,3])
    }
}
