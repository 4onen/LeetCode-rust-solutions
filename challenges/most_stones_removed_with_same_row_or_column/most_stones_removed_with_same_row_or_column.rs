// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/

pub struct Solution;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        struct MyUnionFindReimplementation {
            // Because I'm too lazy to find my old implementation
            // Because the stones range is 0..=10000, and we'll
            // represent the second index as just +10000,
            // then the full range is 0..=20000 which safely
            // fits in u16 indices.
            parent: Vec<u16>,
            // No rank nor cardinality here because the problem
            // is sufficiently small. Just bias toward
            // smaller indices.
        }
        impl MyUnionFindReimplementation {
            fn new() -> Self {
                Self {
                    parent: (0..=20001).collect(),
                }
            }
            fn find(&mut self, mut x: u16) -> u16 {
                while self.parent[x as usize] != x {
                    self.parent[x as usize] = self.parent[self.parent[x as usize] as usize];
                    x = self.parent[x as usize];
                }
                x
            }
            fn union(&mut self, x: u16, y: u16) {
                let x = self.find(x);
                let y = self.find(y);
                if x < y {
                    self.parent[y as usize] = x;
                } else {
                    self.parent[x as usize] = y;
                }
            }
        }
        let mut uf = MyUnionFindReimplementation::new();
        let mut seen = std::collections::HashSet::new();
        for stone in stones.iter() {
            uf.union(stone[0] as u16, stone[1] as u16 + 10001);
            seen.insert(uf.find(stone[0] as u16));
        }
        let mut components = 0;
        for coord in seen.into_iter() {
            if coord == uf.find(coord) {
                components += 1;
            }
        }
        // The last stone we can't remove from component is a stone we couldn't
        // remove, so the number of stones we can remove is the total number of
        // stones minus those we can't remove.
        stones.len() as i32 - components
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(stones: &[[u16; 2]], expected: i32) {
        assert!(stones.len() >= 1);
        assert!(stones.len() <= 1000);
        for stone in stones.iter() {
            assert!(stone[0] <= 10000);
            assert!(stone[1] <= 10000);
        }
        let stones = stones
            .iter()
            .map(|v| v.into_iter().map(|&x| x as i32).collect())
            .collect();
        assert_eq!(Solution::remove_stones(stones), expected);
    }

    #[test]
    fn ex1() {
        test(&[[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]], 5)
    }

    #[test]
    fn ex2() {
        test(&[[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]], 3)
    }

    #[test]
    fn ex3() {
        test(&[[0, 0]], 0)
    }

    #[test]
    fn failing_case1() {
        test(&[[0, 0], [2, 2], [10000, 2]], 1)
    }
}
