// https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/

pub struct Solution;

// Naive DFS
impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        fn dfs(favorite: &[i32], seated: &mut [bool], i: i32, stays: bool) -> (i32, bool) {
            let mut best = 0;
            let next = favorite[i as usize];
            if !stays && !seated[next as usize] {
                seated[i as usize] = true;
                let (sub_best, stayed) = dfs(favorite, seated, next, false);
                seated[i as usize] = false;
                if !stayed {
                    return (0, false);
                } else {
                    return (sub_best + 1, true);
                }
            }
            for j in 0..favorite.len() as i32 {
                if !seated[j as usize] && favorite[j as usize] == i {
                    seated[i as usize] = true;
                    let (sub_best, _) = dfs(favorite, seated, j, true);
                    seated[i as usize] = false;
                    if sub_best > best {
                        best = sub_best;
                    }
                }
            }
            (best + 1, true)
        }
        let mut seated = vec![false; favorite.len()];
        let mut best = 0;
        for i in 0..favorite.len() as i32 {
            if !seated[i as usize] {
                let (sub_best, stayed) = dfs(&favorite, &mut seated, i, false);
                if !stayed {
                    continue;
                }
                if sub_best > best {
                    best = sub_best;
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(favorite: &[i32], expected: i32) {
        assert!(favorite.len() >= 2);
        assert!(favorite.len() <= 100_000);
        for (i, &x) in favorite.iter().enumerate() {
            assert!(x >= 0);
            assert!(x <= favorite.len() as i32);
            assert!(x != i as i32);
        }
        assert!(expected >= 0);
        assert!(expected <= favorite.len() as i32);
        assert_eq!(Solution::maximum_invitations(favorite.to_vec()), expected);
    }

    #[test]
    fn ex1() {
        test(&[2, 2, 1, 2], 3)
    }

    #[test]
    fn ex2() {
        test(&[1, 2, 0], 3)
    }

    #[test]
    fn ex3() {
        test(&[3, 0, 1, 4, 1], 4)
    }

    #[test]
    fn discussion_case1() {
        test(&[1, 0], 2)
    }

    #[test]
    fn discussion_case2() {
        test(&[1, 2, 3, 4, 5, 0], 6)
    }

    #[test]
    fn discussion_case3() {
        test(
            &[6, 4, 4, 5, 0, 3, 3],
            // 1 -> 4 -> 0 -> 6 -> 3 -> 5
            6,
        )
    }

    #[test]
    fn discussion_case4() {
        test(
            &[1, 0, 0, 2, 1, 4, 7, 8, 9, 6, 7, 10, 8],
            // 5 -> 4 -> 1 -> 0 -> 2 -> 3
            6,
        )
    }

    #[test]
    fn discussion_case5() {
        test(&[1, 0, 3, 2, 5, 6, 7, 4, 9, 8, 11, 10, 11, 12, 10], 11)
    }
}
