// https://leetcode.com/problems/design-hashmap/

// Hope you're a fan of cheese, because this is a cheesy solution.
// Look at my previous submission for a more serious solution.

pub struct MyHashMap {
    // Negative keys are empty fields
    map: Box<[i32]>,
}

impl MyHashMap {
    pub fn new() -> Self {
        // Can't let it allocate on the stack because it's too big
        let map = vec![-1; 1_000_001].into_boxed_slice();
        Self { map }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        unsafe {
            *self.map.get_unchecked_mut(key as usize) = value;
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        unsafe { *self.map.get_unchecked(key as usize) }
    }

    pub fn remove(&mut self, key: i32) {
        unsafe {
            *self.map.get_unchecked_mut(key as usize) = -1;
        }
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
