// https://leetcode.com/problems/xor-queries-of-a-subarray/description/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
//         let mut prefix_xor = arr;
//         prefix_xor.insert(0,0);
//         for i in 1..prefix_xor.len() {
//             prefix_xor[i] ^= prefix_xor[i - 1];
//         }
//         queries.into_iter().map(|q| prefix_xor[q[0] as usize] ^ prefix_xor[q[1] as usize + 1]).collect()
//     }
// }

// Optimized sol'n (reverse prefix xor to allow push back instead of insert)
impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_xor = arr;
        prefix_xor.push(0);
        for i in (0..prefix_xor.len()-1).rev() {
            prefix_xor[i] ^= prefix_xor[i + 1];
        }
        queries.into_iter().map(|q| prefix_xor[q[0] as usize] ^ prefix_xor[q[1] as usize + 1]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(arr: &[i32], queries: &[[i32; 2]], expected: &[i32]) {
        let arr = arr.to_vec();
        let queries = queries.iter().map(|q| q.to_vec()).collect();
        assert_eq!(Solution::xor_queries(arr, queries), expected);
    }

    #[test]
    fn ex1() {
        test(
            &[1, 3, 4, 8],
            &[[0, 1], [1, 2], [0, 3], [3, 3]],
            &[2, 7, 14, 8],
        )
    }

    #[test]
    fn ex2() {
        test(
            &[4, 8, 2, 10],
            &[[2, 3], [1, 3], [0, 0], [0, 3]],
            &[8, 0, 4, 4],
        )
    }
}
