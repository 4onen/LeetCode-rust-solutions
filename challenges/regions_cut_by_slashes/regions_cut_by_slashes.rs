// https://leetcode.com/problems/regions-cut-by-slashes/

pub struct Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        type UnionIdxType = u16;
        struct MyUnionFind {
            parent: Vec<UnionIdxType>,
        }
        impl MyUnionFind {
            fn new(n: UnionIdxType) -> Self {
                let mut parent = Vec::with_capacity(n as usize);
                let cap = parent.spare_capacity_mut();
                for i in 0..n {
                    cap[i as usize].write(i);
                }
                unsafe {
                    parent.set_len(n as usize);
                }
                Self { parent }
            }
            fn find(&mut self, mut x: UnionIdxType) -> UnionIdxType {
                while x != self.parent[x as usize] {
                    self.parent[x as usize] = self.parent[self.parent[x as usize] as usize];
                    x = self.parent[x as usize];
                }
                x
            }
            fn union(&mut self, x: UnionIdxType, y: UnionIdxType) {
                let root_x = self.find(x);
                let root_y = self.find(y);
                if root_x != root_y {
                    self.parent[root_x as usize] = root_y;
                }
            }
            fn count(&self) -> UnionIdxType {
                (0..self.parent.len() as UnionIdxType)
                    .into_iter()
                    .filter(|&i| i == self.parent[i as usize])
                    .count() as UnionIdxType
            }
        }
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 30);
        let mut uf = MyUnionFind::new((2 * grid.len() * grid.len()) as UnionIdxType);
        for i in 0..grid.len() as UnionIdxType {
            let row = grid[i as usize].as_bytes();
            for j in 0..row.len() as UnionIdxType {
                let idx = 2 * (i * grid.len() as UnionIdxType + j);
                // At each point, imagine a 1 col, 2 row grid
                //
                // blank:
                //   0
                //
                //   1
                //
                // /:
                //   0/
                //   /
                //  /1
                //
                // \:
                //  \0
                //   \
                //   1\
                if i > 0 {
                    uf.union(idx, idx + 1 - 2 * grid.len() as UnionIdxType);
                }
                if row[j as usize] == b' ' {
                    uf.union(idx, idx + 1);
                }
                if j > 0 {
                    uf.union(
                        idx + if row[j as usize] == b'\\' { 1 } else { 0 },
                        idx - if row[j as usize - 1] == b'/' { 1 } else { 2 },
                    );
                }
            }
        }
        uf.count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(grid: &[&str], expected: u16) {
        assert!(grid.len() >= 1);
        assert!(grid.len() <= 30);
        assert!(expected >= 1);
        assert!(expected <= 30 * 30 * 2);
        assert_eq!(
            Solution::regions_by_slashes(grid.iter().map(|s| s.to_string()).collect()),
            expected as i32
        );
    }

    #[test]
    fn ex1() {
        test(&[" /", "/ "], 2);
    }

    #[test]
    fn ex2() {
        test(&[" /", "  "], 1);
    }

    #[test]
    fn ex3() {
        test(&["/\\", "\\/"], 5);
    }

    #[test]
    fn my_ex3() {
        test(&["\\/", "/\\"], 4);
    }

    #[test]
    fn ex4() {
        test(&[" /\\", " \\/", "\\  "], 4);
    }

    #[test]
    fn my_trivial0() {
        test(&[" "], 1);
    }

    #[test]
    fn my_trivial1() {
        test(&["/"], 2);
    }

    #[test]
    fn my_trivial2() {
        test(&["\\"], 2);
    }

    #[test]
    fn my_extreme_ex1() {
        const N: usize = 30;
        let strings = [unsafe{ std::str::from_utf8_unchecked(&[b'/';N])};N];
        test(&strings, 2*N as u16);
    }

    #[test]
    fn my_extreme_ex2() {
        const N: usize = 30;
        let strings = [unsafe{ std::str::from_utf8_unchecked(&[b'\\';N])};N];
        test(&strings, 2*N as u16);
    }

    #[test]
    fn my_extreme_ex3() {
        const N: usize = 30;
        let strings = [unsafe{ std::str::from_utf8_unchecked(&[b' ';N])};N];
        test(&strings, 1);
    }
}
