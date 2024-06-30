// https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
//         assert!(n >= 1);
//         let n = n as u32;
//         type IndexTyp = u32;
//         #[derive(Clone)]
//         struct MyUnionFind<IndexTyp> {
//             // Yes, this implementation is non-asymptotic, but it is
//             // sufficient for this problem.
//             parent: Vec<IndexTyp>,
//         }
//         impl MyUnionFind<IndexTyp> {
//             fn new(n: IndexTyp) -> Self {
//                 let mut parent = std::vec::Vec::with_capacity(n as usize + 1);
//                 let extra_space_mut = parent.spare_capacity_mut();
//                 for i in 0..=n {
//                     extra_space_mut[i as usize].write(i);
//                 }
//                 unsafe { parent.set_len(n as usize + 1) };
//                 Self { parent }
//             }
//             fn find(&mut self, x: IndexTyp) -> IndexTyp {
//                 let mut curr = x;
//                 let mut parent = self.parent[curr as usize];
//                 while curr != parent {
//                     let grandparent = self.parent[parent as usize];
//                     self.parent[curr as usize] = grandparent;
//                     curr = parent;
//                     parent = grandparent;
//                 }
//                 self.parent[x as usize] = parent;
//                 parent
//             }
//             fn union(&mut self, x: IndexTyp, y: IndexTyp) -> bool {
//                 let x_parent = self.find(x);
//                 let y_parent = self.find(y);
//                 if x_parent == y_parent {
//                     return false;
//                 }
//                 // Again, this is non-asymptotic.
//                 // Again, it's more than fast enough for this problem.
//                 self.parent[x as usize] = y_parent;
//                 self.parent[x_parent as usize] = y_parent;
//                 true
//             }
//             fn is_single_component(&mut self) -> bool {
//                 let root_parent = self.find(1);
//                 for i in 2..self.parent.len() as IndexTyp {
//                     if self.find(i) != root_parent {
//                         return false;
//                     }
//                 }
//                 true
//             }
//         }
//         let mut shared_nodes = MyUnionFind::new(n);
//         let mut kept_edges = 0;
//         // Type 3 pass
//         for edge in edges.iter() {
//             let (edgetype, u, v) = (edge[0], edge[1], edge[2]);
//             match edgetype {
//                 1 | 2 => continue,
//                 3 => {
//                     if shared_nodes.union(u as IndexTyp, v as IndexTyp) {
//                         kept_edges += 1;
//                     }
//                 }
//                 _ => unreachable!(),
//             }
//         }
//         let mut alice_nodes = shared_nodes.clone();
//         let mut bob_nodes = shared_nodes;
//         // Type 1&2 pass
//         for edge in edges.iter() {
//             let (edgetype, u, v) = (edge[0], edge[1], edge[2]);
//             match edgetype {
//                 1 => {
//                     if alice_nodes.union(u as IndexTyp, v as IndexTyp) {
//                         kept_edges += 1;
//                     }
//                 }
//                 2 => {
//                     if bob_nodes.union(u as IndexTyp, v as IndexTyp) {
//                         kept_edges += 1;
//                     }
//                 }
//                 3 => {}
//                 _ => unreachable!(),
//             }
//         }
//         // Now check each entire graph is connected
//         let alice_connected = alice_nodes.is_single_component();
//         let bob_connected = bob_nodes.is_single_component();
//         if !alice_connected || !bob_connected {
//             return -1;
//         }
//         edges.len() as i32 - kept_edges
//     }
// }

// Optimized with partitioning
// impl Solution {
//     pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
//         assert!(n >= 1);
//         let n = n as u32;
//         type IndexTyp = u32;
//         #[derive(Clone)]
//         struct MyUnionFind<IndexTyp> {
//             // Yes, this implementation is non-asymptotic, but it is
//             // sufficient for this problem.
//             parent: Vec<IndexTyp>,
//         }
//         impl MyUnionFind<IndexTyp> {
//             fn new(n: IndexTyp) -> Self {
//                 let mut parent = std::vec::Vec::with_capacity(n as usize + 1);
//                 let extra_space_mut = parent.spare_capacity_mut();
//                 for i in 0..=n {
//                     extra_space_mut[i as usize].write(i);
//                 }
//                 unsafe { parent.set_len(n as usize + 1) };
//                 Self { parent }
//             }
//             fn find(&mut self, x: IndexTyp) -> IndexTyp {
//                 let mut curr = x;
//                 let mut parent = self.parent[curr as usize];
//                 while curr != parent {
//                     let grandparent = self.parent[parent as usize];
//                     self.parent[curr as usize] = grandparent;
//                     curr = parent;
//                     parent = grandparent;
//                 }
//                 self.parent[x as usize] = parent;
//                 parent
//             }
//             fn union(&mut self, x: IndexTyp, y: IndexTyp) -> bool {
//                 let x_parent = self.find(x);
//                 let y_parent = self.find(y);
//                 if x_parent == y_parent {
//                     return false;
//                 }
//                 // Again, this is non-asymptotic.
//                 // Again, it's more than fast enough for this problem.
//                 self.parent[x as usize] = y_parent;
//                 self.parent[x_parent as usize] = y_parent;
//                 true
//             }
//             fn is_single_component(&mut self) -> bool {
//                 let root_parent = self.find(1);
//                 for i in 2..self.parent.len() as IndexTyp {
//                     if self.find(i) != root_parent {
//                         return false;
//                     }
//                 }
//                 true
//             }
//         }
//         let mut kept_edges = 0;
//         // Sort edges by whether they are type 3 or not
//         // All type 3 edges should be at the front,
//         // so we can partition the vector.
//         edges.sort_unstable_by_key(|v| v[0] != 3);
//         let edges_count = edges.len() as i32;
//         let mut edges_iter = edges.into_iter().peekable();
//         // Type 3 pass
//         let mut shared_nodes = MyUnionFind::new(n);
//         while let Some(edge) = edges_iter.next_if(|v| v[0] == 3) {
//             let (u, v) = (edge[1], edge[2]);
//             if shared_nodes.union(u as IndexTyp, v as IndexTyp) {
//                 kept_edges += 1;
//             }
//         }
//         // Type 1&2 pass
//         let mut alice_nodes = shared_nodes.clone();
//         let mut bob_nodes = shared_nodes;
//         for edge in edges_iter {
//             let (edgetype, u, v) = (edge[0], edge[1], edge[2]);
//             match edgetype {
//                 1 => {
//                     if alice_nodes.union(u as IndexTyp, v as IndexTyp) {
//                         kept_edges += 1;
//                     }
//                 }
//                 2 => {
//                     if bob_nodes.union(u as IndexTyp, v as IndexTyp) {
//                         kept_edges += 1;
//                     }
//                 }
//                 _ => unreachable!(),
//             }
//         }
//         // Now check each entire graph is connected
//         let alice_connected = alice_nodes.is_single_component();
//         let bob_connected = bob_nodes.is_single_component();
//         if !alice_connected || !bob_connected {
//             return -1;
//         }
//         edges_count - kept_edges
//     }
// }

// Optimized with partitioning all 3 edge types
impl Solution {
    pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
        assert!(n >= 1);
        let n = n as u32;
        type IndexTyp = u32;
        #[derive(Clone)]
        struct MyUnionFind<IndexTyp> {
            // Yes, this implementation is non-asymptotic, but it is
            // sufficient for this problem.
            parent: Vec<IndexTyp>,
        }
        impl MyUnionFind<IndexTyp> {
            fn new(n: IndexTyp) -> Self {
                let mut parent = std::vec::Vec::with_capacity(n as usize + 1);
                let extra_space_mut = parent.spare_capacity_mut();
                for i in 0..=n {
                    extra_space_mut[i as usize].write(i);
                }
                unsafe { parent.set_len(n as usize + 1) };
                Self { parent }
            }
            fn find(&mut self, x: IndexTyp) -> IndexTyp {
                let mut curr = x;
                let mut parent = self.parent[curr as usize];
                while curr != parent {
                    let grandparent = self.parent[parent as usize];
                    self.parent[curr as usize] = grandparent;
                    curr = parent;
                    parent = grandparent;
                }
                self.parent[x as usize] = parent;
                parent
            }
            fn union(&mut self, x: IndexTyp, y: IndexTyp) -> bool {
                let x_parent = self.find(x);
                let y_parent = self.find(y);
                if x_parent == y_parent {
                    return false;
                }
                // Again, this is non-asymptotic.
                // Again, it's more than fast enough for this problem.
                self.parent[x as usize] = y_parent;
                self.parent[x_parent as usize] = y_parent;
                true
            }
            fn is_single_component(&mut self) -> bool {
                let root_parent = self.find(1);
                for i in 2..self.parent.len() as IndexTyp {
                    if self.find(i) != root_parent {
                        return false;
                    }
                }
                true
            }
        }
        let mut kept_edges = 0;
        // Sort edges by whether they are type 3 or not
        // All type 3 edges should be at the front,
        // so we can partition the vector.
        edges.sort_unstable_by_key(|v| std::cmp::Reverse(v[0]));
        let edges_count = edges.len() as i32;
        let mut edges_iter = edges.into_iter().peekable();
        // Type 3 pass
        let mut shared_nodes = MyUnionFind::new(n);
        while let Some(edge) = edges_iter.next_if(|v| v[0] == 3) {
            let (u, v) = (edge[1], edge[2]);
            if shared_nodes.union(u as IndexTyp, v as IndexTyp) {
                kept_edges += 1;
            }
        }
        // Type 2 pass
        let mut alice_nodes = shared_nodes.clone();
        while let Some(edge) = edges_iter.next_if(|v| v[0] == 2) {
            let (u, v) = (edge[1], edge[2]);
            if alice_nodes.union(u as IndexTyp, v as IndexTyp) {
                kept_edges += 1;
            }
        }
        // Type 1 pass
        let mut bob_nodes = shared_nodes;
        for edge in edges_iter {
            let (u, v) = (edge[1], edge[2]);
            if bob_nodes.union(u as IndexTyp, v as IndexTyp) {
                kept_edges += 1;
            }
        }
        // Now check each entire graph is connected
        let alice_connected = alice_nodes.is_single_component();
        let bob_connected = bob_nodes.is_single_component();
        if !alice_connected || !bob_connected {
            return -1;
        }
        edges_count - kept_edges
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u32, edges: &[[u32; 3]], expect: i32) {
        assert!(n >= 1);
        assert!(n <= 100_000);
        assert!(edges.len() >= 1);
        assert!(edges.len() <= std::cmp::min(100_000, 3 * n as usize * (n as usize - 1) / 2));
        for &[edgetype, u, v] in edges.iter() {
            assert!(edgetype >= 1);
            assert!(edgetype <= 3);
            assert!(u >= 1);
            assert!(u < v);
            assert!(v <= n);
        }
        let edges = edges
            .iter()
            .map(|v| v.into_iter().map(|&x| x as i32).collect())
            .collect();
        assert_eq!(Solution::max_num_edges_to_remove(n as i32, edges), expect);
    }

    #[test]
    fn ex1() {
        test(
            4,
            &[
                [3, 1, 2],
                [3, 2, 3],
                [1, 1, 3],
                [1, 2, 4],
                [1, 1, 2],
                [2, 3, 4],
            ],
            2,
        );
    }

    #[test]
    fn ex2() {
        test(4, &[[3, 1, 2], [3, 2, 3], [1, 1, 4], [2, 1, 4]], 0);
    }

    #[test]
    fn ex3() {
        test(4, &[[3, 2, 3], [1, 1, 2], [2, 3, 4]], -1);
    }
}
