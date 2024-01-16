// https://leetcode.com/problems/design-hashmap/

use std::{num::NonZeroI32, vec};

// All keys fall in the range 0 to 1_000_000, so we'll store them all +1
// This means we can use them being nonzero to indicate whether the entry
// is empty or not.

// We'll use a NonZeroI32 to tell Rust that zero is not a valid key,
// so it can optimize the memory layout.

fn key_to_nonzero_key(key: i32) -> NonZeroI32 {
    assert!((0..i32::MAX).contains(&key));
    unsafe { NonZeroI32::new_unchecked(key + 1) }
}

#[derive(Debug, Clone)]
pub enum Entry {
    Empty,
    SmallOpt1Entry([(NonZeroI32, i32); 1]),
    SmallOpt2Entry([(NonZeroI32, i32); 2]),
    SmallOpt3Entry([(NonZeroI32, i32); 3]),
    ManyEntries(Vec<(NonZeroI32, i32)>),
}

impl Entry {
    fn new() -> Self {
        Self::Empty
    }

    fn as_slice(&self) -> &[(NonZeroI32, i32)] {
        match self {
            Self::Empty => &[],
            Self::SmallOpt1Entry(entries) => entries,
            Self::SmallOpt2Entry(entries) => entries,
            Self::SmallOpt3Entry(entries) => entries,
            Self::ManyEntries(entries) => entries,
        }
    }

    fn iter<'a>(&'a self) -> core::slice::Iter<'a, (NonZeroI32, i32)> {
        self.as_slice().iter()
    }

    fn put(&mut self, key: NonZeroI32, value: i32) {
        match self {
            Self::Empty => {
                *self = Self::SmallOpt1Entry([(key, value)]);
            }
            Self::SmallOpt1Entry(entries) => {
                if entries[0].0 == key {
                    entries[0].1 = value;
                } else {
                    *self = Self::SmallOpt2Entry([entries[0], (key, value)]);
                }
            }
            Self::SmallOpt2Entry(entries) => {
                if entries[0].0 == key {
                    entries[0].1 = value;
                } else if entries[1].0 == key {
                    entries[1].1 = value;
                } else {
                    *self = Self::SmallOpt3Entry([entries[0], entries[1], (key, value)]);
                }
            }
            Self::SmallOpt3Entry(entries) => {
                if entries[0].0 == key {
                    entries[0].1 = value;
                } else if entries[1].0 == key {
                    entries[1].1 = value;
                } else if entries[2].0 == key {
                    entries[2].1 = value;
                } else {
                    *self =
                        Self::ManyEntries(vec![entries[0], entries[1], entries[2], (key, value)]);
                }
            }
            Self::ManyEntries(entries) => {
                for entry in entries.iter_mut() {
                    if entry.0 == key {
                        entry.1 = value;
                        return;
                    }
                }
                entries.push((key, value));
            }
        }
    }

    fn get(&self, key: NonZeroI32) -> i32 {
        self.iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or(-1)
    }

    fn remove(&mut self, key: NonZeroI32) {
        match self {
            Self::Empty => {}
            Self::SmallOpt1Entry(entries) => {
                if entries[0].0 == key {
                    *self = Self::Empty;
                }
            }
            Self::SmallOpt2Entry(entries) => {
                if entries[0].0 == key {
                    *self = Self::SmallOpt1Entry([entries[1]]);
                } else if entries[1].0 == key {
                    *self = Self::SmallOpt1Entry([entries[0]]);
                }
            }
            Self::SmallOpt3Entry(entries) => {
                if entries[0].0 == key {
                    *self = Self::SmallOpt2Entry([entries[1], entries[2]]);
                } else if entries[1].0 == key {
                    *self = Self::SmallOpt2Entry([entries[0], entries[2]]);
                } else if entries[2].0 == key {
                    *self = Self::SmallOpt2Entry([entries[0], entries[1]]);
                }
            }
            Self::ManyEntries(entries) => {
                let index = entries.iter().position(|(k, _)| *k == key);
                if let Some(index) = index {
                    entries.swap_remove(index);
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }
}

pub struct MyHashMap {
    // Negative keys are empty fields
    map: Vec<Entry>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    pub fn new() -> Self {
        Self {
            map: vec![Entry::new(); 4091],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let key = key_to_nonzero_key(key);
        let index = key.get() as usize % self.map.len();
        self.map[index].put(key, value);
    }

    pub fn get(&self, key: i32) -> i32 {
        let key = key_to_nonzero_key(key);
        let index = key.get() as usize % self.map.len();
        self.map[index].get(key)
    }

    pub fn remove(&mut self, key: i32) {
        let key = key_to_nonzero_key(key);
        let index = key.get() as usize % self.map.len();
        self.map[index].remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut obj = MyHashMap::new();
        obj.put(1, 1); // The map is now [[1,1]]
        obj.put(2, 2); // The map is now [[1,1], [2,2]]
        assert_eq!(obj.get(1), 1); // return 1, The map is now [[1,1], [2,2]]
        assert_eq!(obj.get(3), -1); // return -1 (i.e., not found), The map is now [[1,1], [2,2]]
        obj.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the existing value)
        assert_eq!(obj.get(2), 1); // return 1, The map is now [[1,1], [2,1]]
        obj.remove(2); // remove the mapping for 2, The map is now [[1,1]]
        assert_eq!(obj.get(2), -1); // return -1 (i.e., not found), The map is now [[1,1]]
    }

    #[test]
    fn myex1() {
        let mut obj = MyHashMap::new();
        for i in 0..10_000 {
            obj.put(i, 1024 - i);
            assert_eq!(obj.get(i), 1024 - i);
        }

        for i in 0..10_000 {
            assert_eq!(obj.get(i), 1024 - i);
            obj.remove(i);
            assert_eq!(obj.get(i), -1);
        }
    }
}
