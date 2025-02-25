// https://leetcode.com/problems/find-if-path-exists-in-graph/

pub struct Solution;

// Initial sol'n - Union-Find but not doing smart things like path compression
// impl Solution {
//     pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
//         type UnionFindItem = u32;
//         struct UnionFind {
//             parent: Vec<UnionFindItem>,
//             // No rank -- It's already fast enough
//         }
//         impl UnionFind {
//             fn new(n: UnionFindItem) -> Self {
//                 UnionFind {
//                     parent: (0..n).collect(),
//                 }
//             }
//             fn find(&mut self, mut x: UnionFindItem) -> UnionFindItem {
//                 while x != self.parent[x as usize] {
//                     self.parent[x as usize] = self.parent[self.parent[x as usize] as usize];
//                     x = self.parent[x as usize];
//                 }
//                 x
//             }
//             fn union(&mut self, x: UnionFindItem, y: UnionFindItem) {
//                 let root_x = self.find(x);
//                 let root_y = self.find(y);
//                 if root_x != root_y {
//                     self.parent[root_x as usize] = root_y;
//                 }
//             }
//         }
//         assert!(n > 0);
//         assert!(n <= 2 * (10i32.pow(5)));
//         assert!(source >= 0);
//         assert!(source < n);
//         assert!(destination >= 0);
//         assert!(destination < n);
//         let n = n as UnionFindItem;
//         let source = source as UnionFindItem;
//         let destination = destination as UnionFindItem;
//         let mut uf = UnionFind::new(n);
//         for edge in edges {
//             let u = edge[0];
//             if u < 0 {
//                 unreachable!("Invalid negative edge: {:?}", edge);
//             }
//             let u = u as UnionFindItem;
//             let v = edge[1];
//             if v < 0 {
//                 unreachable!("Invalid negative edge: {:?}", edge);
//             }
//             let v = v as UnionFindItem;
//             uf.union(u, v);
//         }
//         uf.find(source) == uf.find(destination)
//     }
// }

// Optimized sol'n
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        type UnionFindItem = u32;
        // Can be const in rustc 1.84.0 (9fc6b4312 2025-01-07)
        const fn find(uf: &mut [UnionFindItem], mut x: UnionFindItem) -> UnionFindItem {
            while uf[x as usize] != x as UnionFindItem {
                let new = uf[x as usize];
                let new_new = uf[new as usize];
                uf[x as usize] = new_new;
                x = new_new;
            }
            x
        }
        assert!(n > 0);
        assert!(n <= 2 * (10i32.pow(5)));
        assert!(source >= 0);
        assert!(source < n);
        assert!(destination >= 0);
        assert!(destination < n);
        let n = n as UnionFindItem;
        let mut uf = (0..n).collect::<Vec<UnionFindItem>>();
        for edge in edges {
            let a_start = edge[0] as UnionFindItem;
            let a_end = find(&mut uf, a_start);
            let b_start = edge[1] as UnionFindItem;
            let b_end = find(&mut uf, b_start);
            uf[a_start as usize] = b_end;
            uf[a_end as usize] = b_end;
        }
        find(&mut uf, source as UnionFindItem) == find(&mut uf, destination as UnionFindItem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: i32, edges: &[&[i32]], source: i32, destination: i32, expected: bool) {
        let edges = edges.iter().map(|edge| edge.to_vec()).collect();
        assert_eq!(
            Solution::valid_path(n, edges, source, destination),
            expected
        );
    }

    #[test]
    fn ex1() {
        test(3, &[&[0, 1], &[1, 2], &[2, 0]], 0, 2, true)
    }

    #[test]
    fn ex2() {
        test(
            6,
            &[&[0, 1], &[0, 2], &[3, 5], &[5, 4], &[4, 3]],
            0,
            5,
            false,
        )
    }

    #[test]
    fn discussion_case0() {
        test(1, &[], 0, 0, true)
    }

    #[test]
    fn discussion_case1() {
        test(2, &[], 0, 1, false)
    }

    #[test]
    fn discussion_case2() {
        test(
            9,
            &[
                &[0, 7],
                &[0, 8],
                &[6, 1],
                &[2, 0],
                &[0, 4],
                &[5, 8],
                &[4, 7],
                &[1, 3],
                &[3, 5],
                &[6, 5],
            ],
            7,
            5,
            true,
        )
    }

    #[test]
    fn failing_case1() {
        let input = include_str!("failing_case1.txt");
        // Each line has the form 123,345 where 123 is the source and 345 is the destination
        let edges = input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                let source: i32 = parts.next().unwrap().parse().unwrap();
                let destination: i32 = parts.next().unwrap().parse().unwrap();
                vec![source, destination]
            })
            .collect::<Vec<_>>();
        let edges = edges.iter().map(|edge| edge.as_slice()).collect::<Vec<_>>();
        test(200000, &edges, 62749, 104478, true)
    }

    #[test]
    fn failing_case1_my_variant() {
        let input = include_str!("failing_case1.txt");
        // Each line has the form 123,345 where 123 is the source and 345 is the destination
        let edges = input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                let source: i32 = parts.next().unwrap().parse().unwrap();
                let destination: i32 = parts.next().unwrap().parse().unwrap();
                vec![source, destination]
            })
            .collect::<Vec<_>>();
        let edges = edges.iter().map(|edge| edge.as_slice()).collect::<Vec<_>>();
        test(200000, &edges, 0, 199999, true)
    }
}
