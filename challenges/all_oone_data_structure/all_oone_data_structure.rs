// https://leetcode.com/problems/all-oone-data-structure/

// ==== Initial sol'n ====
// This solution fails due to the linear search to find the new min_count
// also because it is way, way too complicated and slow for large inputs.
//
// #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// struct Key([u8; 10]);
// impl std::fmt::Debug for Key {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:?}", unsafe { std::str::from_utf8_unchecked(&self.0) })
//     }
// }
// const fn key_from_string(s: &str) -> Key {
//     let s = s.as_bytes();
//     let mut key = [0; 10];
//     let mut i = 0;
//     while i < s.len() {
//         key[i] = s[i];
//         i += 1;
//     }
//     Key(key)
// }
// fn string_from_key(key: &Key) -> String {
//     let mut res = std::vec::Vec::with_capacity(10);
//     for &c in key.0.iter() {
//         if c == 0 {
//             break;
//         }
//         res.push(c);
//     }
//     unsafe { std::string::String::from_utf8_unchecked(res) }
// }
// #[derive(Debug)]
// pub struct AllOne {
//     // key: count
//     key_count: std::collections::HashMap<Key, u16>,
//     // count: keys
//     count_keys: std::collections::HashMap<u16, std::collections::HashSet<Key>>,
//     // max count
//     max_count: u16,
//     // min count
//     min_count: u16,
// }
// impl AllOne {
//     pub fn new() -> Self {
//         AllOne {
//             key_count: std::collections::HashMap::new(),
//             count_keys: std::collections::HashMap::new(),
//             max_count: 0,
//             min_count: 0,
//         }
//     }
//     pub fn inc(&mut self, key: String) {
//         let key = key_from_string(&key);
//         let count = self.key_count.get(&key).map(|&x| x).unwrap_or(0);
//         let new_count = count + 1;
//         self.key_count.insert(key, new_count);
//         self.count_keys
//             .entry(new_count)
//             .or_insert_with(std::collections::HashSet::new)
//             .insert(key);
//         if let Some(en) = self.count_keys.get_mut(&count) {
//             en.remove(&key);
//             if en.is_empty() && count <= self.min_count {
//                 self.min_count = new_count;
//             }
//         } else {
//             // Only true if incrementing from 0
//             assert_eq!(count, 0);
//             self.min_count = 1;
//         }
//         if count >= self.max_count {
//             self.max_count = new_count;
//         }
//     }
//     pub fn dec(&mut self, key: String) {
//         let key = key_from_string(&key);
//         let count = *self.key_count.get(&key).unwrap();
//         let new_count = count - 1;
//         if new_count > 0 {
//             self.key_count.insert(key, new_count);
//             self.count_keys
//                 .entry(new_count)
//                 .or_insert_with(std::collections::HashSet::new)
//                 .insert(key);
//         } else {
//             // new_count == 0
//             self.key_count.remove(&key);
//         }
//         let en = self.count_keys.get_mut(&count).unwrap();
//         en.remove(&key);
//         if count >= self.max_count && en.is_empty() {
//             self.max_count = new_count;
//         }
//         if count <= self.min_count && new_count > 0 && en.is_empty() {
//             self.min_count = new_count;
//         } else if count <= self.min_count && en.is_empty() {
//             // WRONG: Fails here due to linear search to find new min_count
//             let mut i = 1;
//             loop {
//                 if i>self.max_count {
//                     self.min_count = 0;
//                     break;
//                 }
//                 if self.count_keys.get(&i).unwrap().len() > 0 {
//                     self.min_count = i;
//                     break;
//                 }
//                 i += 1;
//             }
//         }
//     }
//     pub fn get_max_key(&self) -> String {
//         if self.max_count == 0 {
//             return "".to_string();
//         }
//         let key = self
//             .count_keys
//             .get(&self.max_count)
//             .unwrap()
//             .iter()
//             .next()
//             .unwrap();
//         string_from_key(key)
//     }
//     pub fn get_min_key(&self) -> String {
//         if self.min_count == 0 {
//             return "".to_string();
//         }
//         let key = self
//             .count_keys
//             .get(&self.min_count)
//             .unwrap()
//             .iter()
//             .next()
//             .unwrap();
//         string_from_key(key)
//     }
// }

// ==== Doubly linked list hashmap sol'n ====
// This solution implements a "doubly linked list"-like structure by storing
// the next-largest and previous-largest counts in the count_keys hashmap
// along with the keys. This allows for O(1) increment and decrement operations
// while solving the search for the new min_count problem.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Key([u8; 10]);
impl Key {
    const fn from_string(s: &str) -> Self {
        let s = s.as_bytes();
        let mut key = [0; 10];
        let mut i = 0;
        while i < s.len() {
            key[i] = s[i];
            i += 1;
        }
        Key(key)
    }
}
impl ToString for Key {
    fn to_string(&self) -> String {
        let mut res = std::vec::Vec::with_capacity(10);
        for &c in self.0.iter() {
            if c == 0 {
                break;
            }
            res.push(c);
        }
        unsafe { std::string::String::from_utf8_unchecked(res) }
    }
}
impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", unsafe { std::str::from_utf8_unchecked(&self.0) })
    }
}
#[derive(Debug)]
struct CountKeysEntry {
    keys: std::collections::HashSet<Key>,
    next: u16, // 0 is empty (but Option<NonZeroU16> is slow)
    prev: u16, // 0 is empty (but Option<NonZeroU16> is slow)
}
#[derive(Debug)]
pub struct AllOne {
    // key: count
    key_count: std::collections::HashMap<Key, u16>,
    // count: keys
    count_keys: std::collections::HashMap<u16, CountKeysEntry>,
    // max count
    max_count: u16,
    // min count
    min_count: u16,
}
impl AllOne {
    pub fn new() -> Self {
        AllOne {
            key_count: std::collections::HashMap::new(),
            count_keys: std::collections::HashMap::new(),
            max_count: 0,
            min_count: 0,
        }
    }
    pub fn inc(&mut self, key: String) {
        let key = Key::from_string(&key);
        let new_count = match self.key_count.entry(key) {
            std::collections::hash_map::Entry::Vacant(key_count_entry) => {
                //Insert new entry at 1
                self.count_keys
                    .entry(1)
                    .or_insert_with(|| {
                        let mut keys = std::collections::HashSet::new();
                        keys.insert(key);
                        CountKeysEntry {
                            keys,
                            next: self.min_count,
                            prev: 0,
                        }
                    })
                    .keys
                    .insert(key);
                if self.min_count > 1 {
                    self.count_keys.get_mut(&self.min_count).unwrap().prev = 1;
                }
                self.min_count = 1;
                key_count_entry.insert(1);
                1
            }
            std::collections::hash_map::Entry::Occupied(mut key_count_entry) => {
                let count = *key_count_entry.get();
                let new_count = count + 1;
                let (prev, next) = match self.count_keys.entry(count) {
                    std::collections::hash_map::Entry::Vacant(_) => {
                        unreachable!(
                            "count_keys entry should always exist if key_count references it"
                        )
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        entry.get_mut().keys.remove(&key);
                        let next = entry.get().next;
                        let prev = entry.get().prev;
                        if entry.get().keys.is_empty() {
                            entry.remove();
                            if next > 0 {
                                self.count_keys.get_mut(&next).unwrap().prev = new_count;
                            }
                            if prev > 0 {
                                self.count_keys.get_mut(&prev).unwrap().next = next;
                            }
                            if count == self.min_count {
                                self.min_count = new_count;
                            }
                            (prev, next)
                        } else {
                            entry.get_mut().next = new_count;
                            (count, next)
                        }
                    }
                };
                match self.count_keys.entry(new_count) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        let mut keys = std::collections::HashSet::new();
                        keys.insert(key);
                        entry.insert(CountKeysEntry { keys, next, prev });
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        entry.get_mut().keys.insert(key);
                        entry.get_mut().prev = prev;
                    }
                }
                key_count_entry.insert(new_count);
                new_count
            }
        };
        if new_count > self.max_count {
            self.max_count = new_count;
        }
    }
    pub fn dec(&mut self, key: String) {
        let key = Key::from_string(&key);
        match self.key_count.entry(key) {
            std::collections::hash_map::Entry::Vacant(_) => {
                unreachable!("Key not found");
            }
            std::collections::hash_map::Entry::Occupied(mut key_count_entry) => {
                let count = *key_count_entry.get();
                let new_count = count - 1;
                let (prev, next) = match self.count_keys.entry(count) {
                    std::collections::hash_map::Entry::Vacant(_) => {
                        unreachable!(
                            "count_keys entry should always exist if key_count references it"
                        )
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        entry.get_mut().keys.remove(&key);
                        let next = entry.get().next;
                        let prev = entry.get().prev;
                        if entry.get().keys.is_empty() {
                            entry.remove();
                            if next > 0 {
                                self.count_keys.get_mut(&next).unwrap().prev =
                                    if new_count > 0 { new_count } else { prev };
                            }
                            if prev > 0 {
                                self.count_keys.get_mut(&prev).unwrap().next = new_count;
                            }
                            if count == self.max_count {
                                self.max_count = new_count;
                            }
                            (prev, next)
                        } else {
                            entry.get_mut().prev = new_count;
                            (prev, count)
                        }
                    }
                };
                if new_count <= 0 {
                    key_count_entry.remove();
                    self.min_count = next;
                } else {
                    key_count_entry.insert(new_count);
                    match self.count_keys.entry(new_count) {
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            let mut keys = std::collections::HashSet::new();
                            keys.insert(key);
                            entry.insert(CountKeysEntry { keys, next, prev });
                        }
                        std::collections::hash_map::Entry::Occupied(mut entry) => {
                            entry.get_mut().keys.insert(key);
                            entry.get_mut().next = next;
                        }
                    }
                    if new_count < self.min_count {
                        self.min_count = new_count;
                    }
                }
            }
        };
    }
    pub fn get_max_key(&self) -> String {
        if self.max_count <= 0 {
            return "".to_string();
        }
        self.count_keys
            .get(&self.max_count)
            .unwrap()
            .keys
            .iter()
            .next()
            .unwrap()
            .to_string()
    }
    pub fn get_min_key(&self) -> String {
        if self.min_count <= 0 {
            return "".to_string();
        }
        self.count_keys
            .get(&self.min_count)
            .unwrap()
            .keys
            .iter()
            .next()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum TestStep<'a> {
        New,                         // Null result
        Inc(&'a str),                // Null result
        Dec(&'a str),                // Null result
        GetMaxKey(&'a str),          // String result
        GetMinKey(&'a str),          // String result
        GetMaxKeyAny(&'a [&'a str]), // Any-string result
        GetMinKeyAny(&'a [&'a str]), // Any-string result
    }
    use TestStep::*;

    fn test(steps: &[TestStep]) {
        let mut obj = None;
        //let step_count = steps.len();
        for (i, step) in steps.into_iter().copied().enumerate() {
            // if step_count - i < 100 {
            //     dbg!(&obj);
            //     println!("Step {}: {:?}", i, step);
            // }
            match (step, &mut obj) {
                (New, obj) => {
                    if obj.is_some() {
                        println!("Step {}: Warning: New() called twice", i);
                    }
                    *obj = Some(AllOne::new());
                }
                (Inc(key), Some(obj)) => {
                    obj.inc(key.to_string());
                }
                (Dec(key), Some(obj)) => {
                    obj.dec(key.to_string());
                }
                (GetMaxKey(expected), Some(obj)) => {
                    assert_eq!(obj.get_max_key(), expected, "Step {}", i);
                }
                (GetMinKey(expected), Some(obj)) => {
                    assert_eq!(obj.get_min_key(), expected, "Step {}", i);
                }
                (GetMaxKeyAny(expected), Some(obj)) => {
                    let result = obj.get_max_key();
                    assert!(
                        expected.contains(&result.as_str()),
                        "Step {} could not find returned max {:?} among {:?}",
                        i,
                        result,
                        expected
                    );
                }
                (GetMinKeyAny(expected), Some(obj)) => {
                    let result = obj.get_min_key();
                    assert!(
                        expected.contains(&result.as_str()),
                        "Step {} could not find returned min {:?} among {:?}",
                        i,
                        result,
                        expected
                    );
                }
                (_, None) => {
                    println!("Step {}: Warning: Object not initialized", i);
                }
            }
        }
    }

    #[test]
    fn ex1() {
        test(&[
            New,
            Inc("hello"),
            Inc("hello"),
            GetMaxKey("hello"),
            GetMinKey("hello"),
            Inc("leet"),
            GetMaxKey("hello"),
            GetMinKey("leet"),
        ])
    }

    #[test]
    fn myex0() {
        test(&[
            New,
            GetMaxKey(""),
            GetMinKey(""),
            Inc("a"),
            Dec("a"),
            GetMaxKey(""),
            GetMinKey(""),
            Inc("a"),
            Dec("a"),
            GetMaxKey(""),
            GetMinKey(""),
            Inc("b"),
            Dec("b"),
            GetMaxKey(""),
            GetMinKey(""),
            Inc("a"),
            Inc("b"),
            Dec("a"),
            Dec("b"),
            GetMaxKey(""),
            GetMinKey(""),
        ])
    }

    #[test]
    fn myex0_1() {
        test(&[
            New,
            GetMaxKey(""),
            GetMinKey(""),
            Inc("a"),
            Inc("b"),
            Dec("b"),
            Dec("a"),
            GetMaxKey(""),
            GetMinKey(""),
        ])
    }

    #[test]
    fn myex1() {
        test(&[
            New,
            GetMaxKey(""),
            GetMinKey(""),
            Inc("hello"),
            Inc("hello"),
            GetMaxKey("hello"),
            GetMinKey("hello"),
            Inc("leet"),
            GetMaxKey("hello"),
            GetMinKey("leet"),
            Dec("hello"),
            GetMaxKeyAny(&["hello", "leet"]),
            GetMinKeyAny(&["hello", "leet"]),
            Dec("hello"),
            GetMaxKey("leet"),
            GetMinKey("leet"),
            Dec("leet"),
            GetMaxKey(""),
            GetMinKey(""),
        ])
    }

    #[test]
    fn myex2() {
        test(&[
            New,
            Inc("mary"),
            Inc("mary"),
            Inc("mary"),
            Inc("had"),
            Inc("a"),
            Inc("little"),
            Inc("lamb"),
            GetMaxKey("mary"),
            GetMinKeyAny(&["had", "a", "little", "lamb"]),
            Dec("mary"),
            GetMaxKey("mary"),
            GetMinKeyAny(&["had", "a", "little", "lamb"]),
            Dec("a"),
            GetMaxKey("mary"),
            GetMinKeyAny(&["had", "little", "lamb"]),
            Dec("mary"),
            GetMaxKeyAny(&["mary", "had", "little", "lamb"]),
            GetMinKeyAny(&["mary", "had", "little", "lamb"]),
            Dec("mary"),
            GetMaxKeyAny(&["had", "little", "lamb"]),
            GetMinKeyAny(&["had", "little", "lamb"]),
            Dec("little"),
            GetMaxKeyAny(&["had", "lamb"]),
            GetMinKeyAny(&["had", "lamb"]),
            Inc("had"),
            GetMaxKey("had"),
            GetMinKey("lamb"),
            Inc("lamb"),
            GetMaxKeyAny(&["had", "lamb"]),
            GetMinKeyAny(&["had", "lamb"]),
            Inc("lamb"),
            GetMaxKey("lamb"),
            GetMinKey("had"),
            Dec("had"),
            GetMaxKey("lamb"),
            GetMinKey("had"),
            Dec("had"),
            GetMaxKey("lamb"),
            GetMinKey("lamb"),
            Dec("lamb"),
            GetMaxKey("lamb"),
            GetMinKey("lamb"),
        ])
    }

    #[test]
    fn my_extreme_ex1() {
        let mut test_data = vec![New];
        test_data.extend_from_slice(&[Inc("a"); 49980]);
        test_data.extend_from_slice(&[
            Inc("b"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Dec("b"),
            GetMaxKey("a"),
            GetMinKey("a"),
            Dec("a"),
            GetMaxKey("a"),
            GetMinKey("a"),
            Inc("b"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Dec("a"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Dec("b"),
            GetMaxKey("a"),
            GetMinKey("a"),
        ]);
        test(&test_data);
    }

    #[test]
    fn myex7() {
        let mut test_data = vec![New];
        test_data.extend_from_slice(&[Inc("a"); 7]);
        test_data.extend_from_slice(&[
            Inc("b"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Inc("b"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Dec("b"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Dec("a"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Inc("b"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Dec("a"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Dec("b"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Dec("b"),
            GetMaxKey("a"),
            GetMinKey("a"),
        ]);
        test(&test_data);
    }

    #[test]
    fn myex_sweep() {
        let mut test_data = vec![New];
        test_data.extend_from_slice(&[Inc("a"); 7]);
        test_data.extend_from_slice(&[GetMaxKey("a"), GetMinKey("a")]);
        test_data.extend_from_slice(&[Inc("b"); 3]);
        test_data.extend_from_slice(&[GetMaxKey("a"), GetMinKey("b")]);
        test_data.extend_from_slice(&[
            Inc("c"),
            GetMaxKey("a"),
            GetMinKey("c"),
            Inc("c"),
            GetMaxKey("a"),
            GetMinKey("c"),
            Inc("c"),
            GetMaxKey("a"),
            GetMinKeyAny(&["b","c"]),
            Inc("c"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Inc("c"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Inc("c"),
            GetMaxKey("a"),
            GetMinKey("b"),
            Inc("c"),
            GetMaxKeyAny(&["a","c"]),
            GetMinKey("b"),
            Inc("c"),
            GetMaxKey("c"),
            GetMinKey("b"),
        ]);
    }

    #[test]
    fn discussion_case0() {
        test(&[New, GetMaxKey(""), GetMinKey("")])
    }
}
