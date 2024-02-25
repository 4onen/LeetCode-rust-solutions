// https://leetcode.com/problems/greatest-common-divisor-traversal/

pub struct Solution;

// Initial O(n^2) solution
// impl Solution {
//     pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
//         type UnionFindItem = u32;
//         struct UnionFind {
//             parent: Vec<UnionFindItem>,
//             // No rank -- we only merge with people in the same set as 0
//         }
//         impl UnionFind {
//             fn new(n: u32) -> Self {
//                 Self {
//                     parent: (0..n).collect(),
//                 }
//             }
//             fn find(&mut self, mut x: UnionFindItem) -> UnionFindItem {
//                 while self.parent[x as usize] != x {
//                     self.parent[x as usize] = self.parent[self.parent[x as usize] as usize];
//                     x = self.parent[x as usize];
//                 }
//                 x
//             }
//             fn union(&mut self, x: UnionFindItem, y: UnionFindItem) {
//                 let x_root = self.find(x);
//                 let y_root = self.find(y);
//                 if x_root == y_root {
//                     return;
//                 }
//                 let lesser_root = std::cmp::min(x_root, y_root);
//                 self.parent[x_root as usize] = lesser_root;
//                 self.parent[y_root as usize] = lesser_root;
//             }
//             fn connected(&mut self, x: UnionFindItem, y: UnionFindItem) -> bool {
//                 self.find(x) == self.find(y)
//             }
//             fn all_connected(&mut self) -> bool {
//                 let root = self.find(0);
//                 for i in 1..self.parent.len() {
//                     if self.find(i as UnionFindItem) != root {
//                         return false;
//                     }
//                 }
//                 true
//             }
//         }
//         type GCDType = u32;
//         const fn gcd_fn(mut a: GCDType, mut b: GCDType) -> GCDType {
//             while b != 0 {
//                 let t = b;
//                 b = a % b;
//                 a = t;
//             }
//             a
//         }
//         // Safety: Lol, I'm lazy and want the minute speedup
//         // We know all numbers are in [1,100_000] so this should be fine.
//         let nums: Vec<u32> = unsafe { std::mem::transmute(nums) };
//         if nums.len() < 2 {
//             return true;
//         }
//         if nums.iter().any(|&x| x == 1) {
//             return false;
//         }
//         let mut uf = UnionFind::new(nums.len() as u32);
//         for i in 0..nums.len() as u32 {
//             for j in i + 1..nums.len() as u32 {
//                 dbg!(i, j);
//                 if uf.connected(i, j) {
//                     continue;
//                 }
//                 if gcd_fn(nums[i as usize], nums[j as usize]) != 1 {
//                     uf.union(i, j)
//                 }
//             }
//         }
//         uf.all_connected()
//     }
// }

// Two loop prime factorization sol'n
// const PRIME_COUNT: u32 = 5133;
// const MAX_PRIME: u32 = 49999;
// const PRIMES: [u32; PRIME_COUNT as usize] = {
//     let mut primes = [0u32; PRIME_COUNT as usize];
//     primes[0] = 2;
//     let mut p = 1;
//     let mut nums = [true; MAX_PRIME as usize + 1];
//     // First, disable all even numbers
//     let mut i = 0;
//     while i < MAX_PRIME {
//         nums[i as usize] = false;
//         i += 2;
//     }
//     // Disable 1
//     nums[1] = false;
//     // Sieve of Eratosthenes
//     let mut i: u32 = 3;
//     while i <= MAX_PRIME && p < 9592 {
//         if nums[i as usize] {
//             primes[p] = i;
//             p += 1;
//             let mut j = i * i;
//             while j < MAX_PRIME {
//                 nums[j as usize] = false;
//                 j += i;
//             }
//         }
//         i += 2;
//     }
//     primes
// };
// fn prime_factors_of(n: u32) -> Vec<u32> {
//     PRIMES.iter().copied().filter(|&p| n % p == 0).collect()
// }
// impl Solution {
//     pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
//         type UnionFindItem = u32;
//         struct UnionFind {
//             parent: std::collections::HashMap<UnionFindItem, UnionFindItem>,
//             // No rank -- we only merge with people in the same set as 0
//         }
//         impl UnionFind {
//             fn new() -> Self {
//                 Self {
//                     parent: std::collections::HashMap::new(),
//                 }
//             }
//             fn parent(&mut self, x: UnionFindItem) -> UnionFindItem {
//                 *self.parent.entry(x).or_insert(x)
//             }
//             fn find(&mut self, mut x: UnionFindItem) -> UnionFindItem {
//                 let mut parent = self.parent(x);
//                 while x != parent {
//                     let old_x = x;
//                     x = parent;
//                     parent = self.parent(parent);
//                     self.parent.insert(old_x, parent);
//                 }
//                 x
//             }
//             fn union(&mut self, x: UnionFindItem, y: UnionFindItem) {
//                 let x_root = self.find(x);
//                 let y_root = self.find(y);
//                 if x_root == y_root {
//                     return;
//                 }
//                 let lesser_root = std::cmp::min(x_root, y_root);
//                 self.parent.insert(x_root, lesser_root);
//                 self.parent.insert(y_root, lesser_root);
//             }
//             fn all_connected(&mut self) -> bool {
//                 let mut values = self
//                     .parent
//                     .values()
//                     .copied()
//                     .collect::<Vec<_>>()
//                     .into_iter();
//                 let first = values.next().unwrap();
//                 let first_parent = self.find(first);
//                 values.all(|x| self.find(x) == first_parent)
//             }
//         }
//         // Safety: Lol, I'm lazy and want the minute speedup
//         // We know all numbers are in [1,100_000] so this should be fine.
//         let nums: Vec<u32> = unsafe { std::mem::transmute(nums) };
//         if nums.len() < 2 {
//             return true;
//         }
//         if nums.iter().any(|&x| x == 1) {
//             return false;
//         }
//         let mut factor_map: std::collections::HashMap<u32, Vec<_>> =
//             std::collections::HashMap::new();
//         for n in nums {
//             let factors = prime_factors_of(n);
//             for f in factors {
//                 factor_map.entry(f).or_default().push(n);
//             }
//         }
//         let mut uf = UnionFind::new();
//         for nums in factor_map.into_values() {
//             match nums.len() {
//                 0 => unreachable!("No numbers had a factor, yet the factor was in the map."),
//                 1 => {
//                     // Insert the number as its own union if not yet present
//                     uf.parent(nums[0]);
//                 }
//                 _ => {
//                     let first = nums[0];
//                     for &n in &nums[1..] {
//                         uf.union(first, n);
//                     }
//                 }
//             }
//         }
//         uf.all_connected()
//     }
// }

// One pass, one sieve + hash union find solution
// impl Solution {
//     pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
//         type UnionFindItem = u32;
//         struct UnionFind {
//             parent: std::collections::HashMap<UnionFindItem, UnionFindItem>,
//             // No rank -- we only merge with people in the same set as 0
//         }
//         impl UnionFind {
//             fn new() -> Self {
//                 Self {
//                     parent: std::collections::HashMap::new(),
//                 }
//             }
//             fn parent(&mut self, x: UnionFindItem) -> UnionFindItem {
//                 *self.parent.entry(x).or_insert(x)
//             }
//             fn find(&mut self, mut x: UnionFindItem) -> UnionFindItem {
//                 let mut parent = self.parent(x);
//                 while x != parent {
//                     let old_x = x;
//                     x = parent;
//                     parent = self.parent(parent);
//                     self.parent.insert(old_x, parent);
//                 }
//                 x
//             }
//             fn union(&mut self, x: UnionFindItem, y: UnionFindItem) {
//                 let x_root = self.find(x);
//                 let y_root = self.find(y);
//                 if x_root == y_root {
//                     return;
//                 }
//                 let lesser_root = std::cmp::min(x_root, y_root);
//                 self.parent.insert(x_root, lesser_root);
//                 self.parent.insert(y_root, lesser_root);
//             }
//             fn all_connected(&mut self) -> bool {
//                 let mut values = self
//                     .parent
//                     .values()
//                     .copied()
//                     .collect::<Vec<_>>()
//                     .into_iter();
//                 let first = values.next().unwrap();
//                 let first_parent = self.find(first);
//                 values.all(|x| self.find(x) == first_parent)
//             }
//         }
//         if nums.len() < 2 {
//             return true;
//         }
//         let mut max = 0;
//         let mut num_set = std::collections::HashSet::with_capacity(nums.len());
//         let mut uf = UnionFind::new();
//         for n in nums {
//             if n < 2 {
//                 return false;
//             }
//             let n = n as u32;
//             if n > max {
//                 max = n;
//             }
//             uf.parent(n);
//             num_set.insert(n);
//         }
//         let mut num_prime = vec![true; (max + 1) as usize];
//         num_prime[0] = false;
//         num_prime[1] = false;
//         for n in 2..=max {
//             if num_prime[n as usize] {
//                 for i in (2 * n..=max).step_by(n as usize) {
//                     num_prime[i as usize] = false;
//                     if num_set.contains(&i) {
//                         uf.union(n, i);
//                     }
//                 }
//             }
//         }
//         uf.all_connected()
//     }
// }

// One pass, one sieve + vec union find solution
impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        type UnionFindItem = u32;
        struct UnionFind {
            parent: Vec<UnionFindItem>,
            // No rank -- we only merge with people in the same set as 0
        }
        impl UnionFind {
            fn with_capacity(n: u32) -> Self {
                Self {
                    parent: (0..n).collect(),
                }
            }
            fn find(&mut self, mut x: UnionFindItem) -> UnionFindItem {
                let mut parent = self.parent[x as usize];
                while x != parent {
                    let old_x = x;
                    x = parent;
                    parent = self.parent[parent as usize];
                    self.parent[old_x as usize] = parent;
                }
                x
            }
            fn union(&mut self, x: UnionFindItem, y: UnionFindItem) {
                let x_root = self.find(x);
                let y_root = self.find(y);
                if x_root == y_root {
                    return;
                }
                let lesser_root = std::cmp::min(x_root, y_root);
                self.parent[x_root as usize] = lesser_root;
                self.parent[y_root as usize] = lesser_root;
            }
        }
        if nums.len() < 2 {
            return true;
        }
        let mut max = 0;
        let mut num_set = std::collections::HashSet::with_capacity(nums.len());
        for n in nums {
            if n < 2 {
                return false;
            }
            let n = n as u32;
            if n > max {
                max = n;
            }
            num_set.insert(n);
        }
        let mut uf = UnionFind::with_capacity(max + 1);
        let mut num_prime = vec![true; (max + 1) as usize];
        num_prime[0] = false;
        num_prime[1] = false;
        for n in 2..=max {
            if num_prime[n as usize] {
                for i in (2 * n..=max).step_by(n as usize) {
                    num_prime[i as usize] = false;
                    if num_set.contains(&i) {
                        uf.union(n, i);
                    }
                }
            }
        }

        let mut nums = num_set.into_iter();
        let first = nums.next().unwrap();
        let first_parent = uf.find(first);
        nums.all(|x| uf.find(x as u32) == first_parent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let nums = vec![2, 3, 6];
        assert_eq!(Solution::can_traverse_all_pairs(nums), true);
    }

    #[test]
    fn ex2() {
        let nums = vec![3, 5, 9];
        assert_eq!(Solution::can_traverse_all_pairs(nums), false);
    }

    #[test]
    fn ex3() {
        let nums = vec![4, 3, 12, 8];
        assert_eq!(Solution::can_traverse_all_pairs(nums), true);
    }

    #[test]
    fn discussion_case1() {
        let nums = vec![1];
        assert_eq!(Solution::can_traverse_all_pairs(nums), true);
    }

    #[test]
    fn discussion_case2() {
        let nums = vec![1, 1];
        assert_eq!(Solution::can_traverse_all_pairs(nums), false);
    }

    #[test]
    fn discussion_case3() {
        let nums = vec![30, 30];
        assert_eq!(Solution::can_traverse_all_pairs(nums), true);
    }

    #[test]
    fn discussion_case4() {
        let nums = vec![21, 88, 75];
        assert_eq!(Solution::can_traverse_all_pairs(nums), false);
    }

    #[test]
    fn discussion_case5() {
        let nums = vec![3, 4, 8, 9];
        assert_eq!(Solution::can_traverse_all_pairs(nums), false);
    }

    #[test]
    fn my_half_extreme_example() {
        let nums = (2..=1000).step_by(2).collect();
        assert_eq!(Solution::can_traverse_all_pairs(nums), true);
    }

    #[test]
    fn my_extreme_example1() {
        let nums = (2..=100_000).step_by(2).collect();
        assert_eq!(Solution::can_traverse_all_pairs(nums), true);
    }

    // #[test]
    // fn prime_factors_of_1() {
    //     assert_eq!(prime_factors_of(1), vec![]);
    // }

    // #[test]
    // fn prime_factors_of_primes() {
    //     for p in PRIMES {
    //         assert_eq!(prime_factors_of(p), vec![p]);
    //     }
    // }
}
