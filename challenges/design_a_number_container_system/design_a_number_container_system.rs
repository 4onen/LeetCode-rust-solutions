// https://leetcode.com/problems/design-a-number-container-system/

// Naive map implementation
// type Idx = i32;
// type Num = i32;
// pub struct NumberContainers {
//     indices: std::collections::BTreeMap<Idx, Num>,
//     number_min_indices: std::collections::HashMap<Num, Idx>,
// }
// impl NumberContainers {
//     pub fn new() -> Self {
//         Self {
//             indices: std::collections::BTreeMap::new(),
//             number_min_indices: std::collections::HashMap::new(),
//         }
//     }
//     pub fn change(&mut self, index: i32, number: i32) {
//         if let Some(old_number) = self.indices.get(&index).copied() {
//             let old_index = self.number_min_indices[&old_number];
//             if old_index == index {
//                 // We need to find the next smallest index containing the old number, if any
//                 if let Some(next_index) = self
//                     .indices
//                     .range(index + 1..)
//                     .filter(|(_, &n)| n == old_number)
//                     .map(|(&i, _)| i)
//                     .next()
//                 {
//                     self.number_min_indices.insert(old_number, next_index);
//                 } else {
//                     self.number_min_indices.remove(&old_number);
//                 }
//             }
//         }
//         self.indices.insert(index, number);
//         self.number_min_indices
//             .entry(number)
//             .and_modify(|e| *e = i32::min(*e, index))
//             .or_insert(index);
//     }
//     pub fn find(&self, number: i32) -> i32 {
//         self.number_min_indices.get(&number).copied().unwrap_or(-1)
//     }
// }

// Amortized O(1) find implementation (initial sol'n)
// type Idx = i32;
// type Num = i32;
// pub struct NumberContainers {
//     indices: std::collections::HashMap<Idx, Num>,
//     number_min_indices: std::collections::HashMap<Num, Vec<Idx>>,
// }
// impl NumberContainers {
//     pub fn new() -> Self {
//         Self {
//             indices: std::collections::HashMap::new(),
//             number_min_indices: std::collections::HashMap::new(),
//         }
//     }
//     pub fn change(&mut self, index: i32, number: i32) {
//         self.indices.insert(index, number);
//         let entry = self.number_min_indices.entry(number).or_default();
//         // Insert value in reverse sorted order, unless it's already there
//         if let Err(idx) =
//             entry.binary_search_by_key(&std::cmp::Reverse(index), |&x| std::cmp::Reverse(x))
//         {
//             entry.insert(idx, index);
//         }
//     }
//     pub fn find(&mut self, number: i32) -> i32 {
//         let Some(trial_indices) = self.number_min_indices.get_mut(&number) else {
//             return -1;
//         };
//         while let Some(&idx) = trial_indices.last() {
//             if self.indices.get(&idx) == Some(&number) {
//                 return idx;
//             }
//             trial_indices.pop();
//         }
//         -1
//     }
// }

// BTreeSet Amortized O(1) find implementation
type Idx = i32;
type Num = i32;
pub struct NumberContainers {
    indices: std::collections::HashMap<Idx, Num>,
    number_min_indices: std::collections::HashMap<Num, std::collections::BTreeSet<Idx>>,
}
impl NumberContainers {
    pub fn new() -> Self {
        Self {
            indices: std::collections::HashMap::new(),
            number_min_indices: std::collections::HashMap::new(),
        }
    }
    pub fn change(&mut self, index: i32, number: i32) {
        self.number_min_indices
            .entry(number)
            .or_default()
            .insert(index);
        self.indices.insert(index, number);
    }
    pub fn find(&mut self, number: i32) -> i32 {
        let Some(trial_indices) = self.number_min_indices.get_mut(&number) else {
            return -1;
        };
        while let Some(&idx) = trial_indices.first() {
            if self.indices.get(&idx) == Some(&number) {
                return idx;
            }
            trial_indices.pop_first();
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    enum Action {
        Find(i32),
        Change(i32, i32),
    }

    fn test(actions: &[Action], expected: &[i32]) {
        assert!(actions.len() <= 100_000);
        let mut obj = NumberContainers::new();
        for (action, expected) in
            std::iter::zip(actions.into_iter().copied(), expected.into_iter().copied())
        {
            match (action, expected) {
                (Action::Find(number), expected) => {
                    assert!(number >= 1);
                    assert!(number <= 1_000_000_000);
                    assert_eq!(obj.find(number), expected);
                }
                (Action::Change(index, number), 0) => {
                    assert!(index >= 1);
                    assert!(index <= 1_000_000_000);
                    assert!(number >= 1);
                    assert!(number <= 1_000_000_000);
                    obj.change(index, number);
                }
                (Action::Change(_, _), expected) => {
                    unreachable!("Non-output call with output {}", expected);
                }
            }
        }
    }

    #[test]
    fn ex1() {
        // ["NumberContainers", "find", "change", "change", "change", "change", "find", "change", "find"]
        // [[], [10], [2, 10], [1, 10], [3, 10], [5, 10], [10], [1, 20], [10]]
        test(
            &[
                Action::Find(10),
                Action::Change(2, 10),
                Action::Change(1, 10),
                Action::Change(3, 10),
                Action::Change(5, 10),
                Action::Find(10),
                Action::Change(1, 20),
                Action::Find(10),
            ],
            &[-1, 0, 0, 0, 0, 1, 0, 2],
        )
    }

    #[test]
    fn discussion_case1() {
        // ["NumberContainers","find","change","change","find"]
        // [[],[5],[1,10],[2,10],[10]]
        test(
            &[
                Action::Find(5),
                Action::Change(1, 10),
                Action::Change(2, 10),
                Action::Find(10),
            ],
            &[-1, 0, 0, 1],
        )
    }

    #[test]
    fn discussion_case2() {
        // ["NumberContainers","change","find","change","change","find","change","find"]
        // [[],[1,15],[15],[2,20],[3,15],[15],[2,10],[15]]
        test(
            &[
                Action::Change(1, 15),
                Action::Find(15),
                Action::Change(2, 20),
                Action::Change(3, 15),
                Action::Find(15),
                Action::Change(2, 10),
                Action::Find(15),
            ],
            &[0, 1, 0, 0, 1, 0, 1],
        )
    }

    #[test]
    fn discussion_case3() {
        // ["NumberContainers","change","change","change","find","change","find"]
        // [[],[4,30],[2,40],[5,50],[30],[5,40],[40]]
        test(
            &[
                Action::Change(4, 30),
                Action::Change(2, 40),
                Action::Change(5, 50),
                Action::Find(30),
                Action::Change(5, 40),
                Action::Find(40),
            ],
            &[0, 0, 0, 4, 0, 2],
        )
    }

    #[test]
    fn discussion_case4() {
        // ["NumberContainers","find","change","change","change","find","change"]
        // [[],[100],[10,100],[20,100],[30,100],[100],[10,200]]
        test(
            &[
                Action::Find(100),
                Action::Change(10, 100),
                Action::Change(20, 100),
                Action::Change(30, 100),
                Action::Find(100),
                Action::Change(10, 200),
            ],
            &[-1, 0, 0, 0, 10, 0],
        )
    }
    #[test]
    fn discussion_case4_1() {
        // ["NumberContainers","find","change","change","change","find","change","find", "find"]
        // [[],[100],[10,100],[20,100],[30,100],[100],[10,200],[100], [200]]
        test(
            &[
                Action::Find(100),
                Action::Change(10, 100),
                Action::Change(20, 100),
                Action::Change(30, 100),
                Action::Find(100),
                Action::Change(10, 200),
                Action::Find(100),
                Action::Find(200),
            ],
            &[-1, 0, 0, 0, 10, 0, 20, 10],
        )
    }

    #[test]
    fn discussion_case5() {
        // ["NumberContainers","change","change","find","change","change","find","change","find"]
        // [[],[6,40],[7,40],[40],[8,50],[9,40],[40],[6,60],[40]]
        test(
            &[
                Action::Change(6, 40),
                Action::Change(7, 40),
                Action::Find(40),
                Action::Change(8, 50),
                Action::Change(9, 40),
                Action::Find(40),
                Action::Change(6, 60),
                Action::Find(40),
            ],
            &[0, 0, 6, 0, 0, 6, 0, 7],
        )
    }

    #[test]
    fn discussion_case6() {
        // ["NumberContainers","change","change","find","change","change","find","change","find"]
        // [[],[2,15],[1,20],[15],[3,25],[4,15],[15],[2,30],[15]]
        test(
            &[
                Action::Change(2, 15),
                Action::Change(1, 20),
                Action::Find(15),
                Action::Change(3, 25),
                Action::Change(4, 15),
                Action::Find(15),
                Action::Change(2, 30),
                Action::Find(15),
            ],
            &[0, 0, 2, 0, 0, 2, 0, 4],
        )
    }

    #[test]
    fn discussion_case7() {
        // ["NumberContainers","change","find","change","change","change","find"]
        // [[],[1,100],[100],[2,100],[3,200],[4,100],[100]]
        test(
            &[
                Action::Change(1, 100),
                Action::Find(100),
                Action::Change(2, 100),
                Action::Change(3, 200),
                Action::Change(4, 100),
                Action::Find(100),
            ],
            &[0, 1, 0, 0, 0, 1],
        )
    }

    #[test]
    fn discussion_case8() {
        // ["NumberContainers","change","change","change","find","change","find","change","find"]
        // [[],[1,50],[2,60],[3,70],[50],[3,80],[50],[1,90],[50]]
        test(
            &[
                Action::Change(1, 50),
                Action::Change(2, 60),
                Action::Change(3, 70),
                Action::Find(50),
                Action::Change(3, 80),
                Action::Find(50),
                Action::Change(1, 90),
                Action::Find(50),
            ],
            &[0, 0, 0, 1, 0, 1, 0, -1],
        )
    }
}
