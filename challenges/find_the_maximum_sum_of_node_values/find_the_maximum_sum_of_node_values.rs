// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/

pub struct Solution;

// Operation we want to apply to maximize the sum:
// Choose two nums at positions u and v, and update their values as follows:
//     nums[u] = nums[u] XOR k
//     nums[v] = nums[v] XOR k

// Realization 1: We can pick any pair of nodes in the graph to apply the op to,
// not just the ones connected by an edge. This is because if we have a chain
// like
// 0 -> 1 -> 2 -> 3 and k = 4, then our XOR process looks like:
// 4 -> 5 -> 2 -> 3
// 4 -> 1 -> 6 -> 3
// 4 -> 1 -> 2 -> 7
// Which is equivalent to as if we had selected 0 and 3 in the first place.
// Given the above, `edges` is not needed and we can just use the input `nums`.

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let mut nums = nums;
        let mut count_xored = 0;
        for ele in nums.iter_mut() {
            let xored = *ele ^ k;
            if xored > *ele {
                *ele = xored;
                count_xored += 1;
            }
        }
        if count_xored & 1 == 1 {
            // We've XORed one too many times, so we need to find the element
            // whose value is least reduced by the XOR operation and XOR it to
            // get to an even number of XORs applied.
            // We can't just XOR the smallest element. We need to find the
            // element whose value is least reduced by the XOR operation.
            let mut min = 0; // Assume the first element
            let mut damage = (nums[0] ^ k) - nums[0];
            for (i, &ele) in nums.iter().enumerate().skip(1) {
                let new_damage = (ele ^ k) - ele;
                if new_damage > damage {
                    min = i;
                    damage = new_damage;
                }
            }
            nums[min] ^= k;
        }
        nums.iter().map(|&x| x as i64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(nums: &[u32], k: i32, edges: &[[u32; 2]], expected: i64) {
        assert!(nums.len() >= 2);
        assert!(nums.len() <= 20_000);
        assert_eq!(nums.len() - 1, edges.len());
        assert!(k >= 1);
        assert!(k <= 1_000_000_000);
        assert_eq!(
            Solution::maximum_value_sum(
                nums.into_iter().map(|&x| x as i32).collect(),
                k,
                edges
                    .iter()
                    .map(|e| {
                        e.into_iter()
                            .map(|&x| {
                                assert!(x < nums.len() as u32);
                                x as i32
                            })
                            .collect()
                    })
                    .collect()
            ),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(&[1, 2, 1], 3, &[[0, 1], [1, 2]], 6)
    }

    #[test]
    fn ex2() {
        test(&[2, 3], 7, &[[0, 1]], 9)
    }

    #[test]
    fn ex3() {
        test(
            &[7, 7, 7, 7, 7, 7],
            3,
            &[[0, 1], [0, 2], [0, 3], [0, 4], [0, 5]],
            42,
        )
    }

    #[test]
    fn myex3_variation1() {
        test(
            &[7, 7, 7, 7, 7, 7, 4],
            3,
            &[[0, 1], [0, 2], [0, 3], [0, 4], [0, 5], [0, 6]],
            46,
        )
    }

    #[test]
    fn myex3_variation2() {
        test(
            &[7, 7, 7, 7, 7, 7, 8],
            3,
            &[[0, 1], [0, 2], [0, 3], [0, 4], [0, 5], [0, 6]],
            50,
        )
    }

    #[test]
    fn myex3_variation3() {
        test(
            &[7, 7, 7, 7, 7, 7, 9],
            3,
            &[[0, 1], [0, 2], [0, 3], [0, 4], [0, 5], [0, 6]],
            51,
        )
    }

    #[test]
    fn discussion_case1() {
        test(
            &[24, 78, 1, 97, 44],
            6,
            &[[0, 2], [1, 2], [4, 2], [3, 4]],
            260,
        )
    }
}
