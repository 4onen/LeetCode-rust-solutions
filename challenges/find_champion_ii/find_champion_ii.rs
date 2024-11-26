// https://leetcode.com/problems/find-champion-ii/

pub struct Solution;

// Initial sol'n
impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut is_champion = vec![true; n as usize];
        for edge in edges {
            let v = edge[1] as usize;
            if is_champion[v] {
                is_champion[v] = false;
            }
        }
        let mut champ = -1;
        for i in 0..n as usize {
            if is_champion[i] {
                if champ == -1 {
                    champ = i as i32;
                } else {
                    return -1;
                }
            }
        }
        champ
    }
}

// Optimized sol'n (Somehow slower)
// impl Solution {
//     pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
//         let mut is_champion = vec![true; n as usize];
//         for edge in edges {
//             is_champion[edge[1] as usize] = false;
//         }
//         let mut champ = -1;
//         for i in 0..n as usize {
//             if is_champion[i] {
//                 if champ != -1 {
//                     return -1;
//                 }
//                 champ = i as i32;
//             }
//         }
//         champ
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u8, edges: &[[i32; 2]], expected: i32) {
        assert!(n >= 1);
        assert!(n <= 100);
        assert!(edges.len() <= n as usize * (n as usize - 1) / 2);
        for edge in edges {
            assert!(edge[0] >= 0);
            assert!(edge[0] < n as i32);
            assert!(edge[1] >= 0);
            assert!(edge[1] < n as i32);
            assert_ne!(edge[0], edge[1]);
        }
        assert!(expected >= -1);
        assert!(expected < n as i32);
        assert_eq!(
            Solution::find_champion(n as i32, edges.iter().map(|&x| x.to_vec()).collect()),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(3, &[[0, 1], [1, 2]], 0)
    }

    #[test]
    fn ex2() {
        test(4, &[[0, 2], [1, 3], [1, 2]], -1)
    }

    #[test]
    fn myex0() {
        test(3, &[[2, 1], [0, 1]], -1)
    }

    #[test]
    fn discussion_case1() {
        test(1, &[], 0)
    }

    #[test]
    fn discussion_case2() {
        test(5, &[], -1)
    }
}
