// https://leetcode.com/problems/insert-delete-getrandom-o1/

// Roll-my-own random sol'n, because I didn't know leetcode accepted rand
// use std::{
//     collections::HashMap,
//     hash::{BuildHasher, Hasher},
// };
// pub struct RandomizedSet {
//     values: Vec<i32>,
//     indexes: HashMap<i32, usize>,
//     random_state: u64,
// }
// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl RandomizedSet {
//     pub fn new() -> Self {
//         Self {
//             values: Vec::new(),
//             indexes: HashMap::new(),
//             random_state: std::collections::hash_map::RandomState::new()
//                 .build_hasher()
//                 .finish(),
//         }
//     }
//     pub fn insert(&mut self, val: i32) -> bool {
//         if self.indexes.contains_key(&val) {
//             return false;
//         }
//         self.values.push(val);
//         self.indexes.insert(val, self.values.len() - 1);
//         true
//     }
//     pub fn remove(&mut self, val: i32) -> bool {
//         // If we don't have the value, we can't remove it
//         if !self.indexes.contains_key(&val) {
//             return false;
//         }
//         // Get the index of the value
//         let index = self.indexes.get(&val).unwrap().clone();
//         // Get the last value in the vector
//         let last = self.values.last().unwrap().clone();
//         // Swap the value at the index with the last value
//         self.values.swap_remove(index);
//         // Update the index of the last value, unless it was the value we removed
//         if last != val {
//             self.indexes.insert(last, index);
//         }
//         // Remove the value from the index map
//         self.indexes.remove(&val);
//         true
//     }
//     pub fn get_random(&mut self) -> i32 {
//         let mut next_state = self.random_state;
//         // Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
//         // https://github.com/rust-lang/rust/blob/1.55.0/library/core/src/slice/sort.rs#L559-L573
//         // Copied from https://blog.orhun.dev/zero-deps-random-in-rust/
//         next_state ^= next_state << 13;
//         next_state ^= next_state >> 17;
//         next_state ^= next_state << 5;
//         self.random_state = next_state;
//         let index = (next_state as usize) % self.values.len();
//         self.values[index]
//     }
// }

// Use rand crate
use std::collections::HashMap;
type IndexType = u32;
pub struct RandomizedSet {
    values: Vec<i32>,
    indexes: HashMap<i32, IndexType>,
    rng: rand::rngs::ThreadRng,
}
impl RandomizedSet {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            indexes: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
    pub fn insert(&mut self, val: i32) -> bool {
        if self.indexes.contains_key(&val) {
            return false;
        }
        self.values.push(val);
        self.indexes
            .insert(val, (self.values.len() - 1) as IndexType);
        true
    }
    pub fn remove(&mut self, val: i32) -> bool {
        // If we don't have the value, we can't remove it
        if !self.indexes.contains_key(&val) {
            return false;
        }
        // Get the index of the value
        let index = self.indexes.get(&val).unwrap().clone();
        // Get the last value in the vector
        let last = self.values.last().unwrap().clone();
        // Swap the value at the index with the last value
        self.values.swap_remove(index as usize);
        // Update the index of the last value, unless it was the value we removed
        if last != val {
            self.indexes.insert(last, index);
        }
        // Remove the value from the index map
        self.indexes.remove(&val);
        true
    }
    pub fn get_random(&mut self) -> i32 {
        use rand::seq::SliceRandom;
        *self.values.choose(&mut self.rng).unwrap()
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut obj = RandomizedSet::new();
        assert_eq!(obj.insert(1), true);
        assert_eq!(obj.remove(2), false);
        assert_eq!(obj.insert(2), true);
        let random = obj.get_random();
        assert!(random == 1 || random == 2);
        assert_eq!(obj.remove(1), true);
        assert_eq!(obj.insert(2), false);
        assert_eq!(obj.get_random(), 2);
    }
}
