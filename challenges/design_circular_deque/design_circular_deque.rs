// https://leetcode.com/problems/design-circular-deque/

// Initial sol'n
// pub struct MyCircularDeque {
//     data: Vec<i16>,
//     front: i16,
//     rear: i16,
//     len: i16,
// }
// impl MyCircularDeque {
//     pub fn new(k: i32) -> Self {
//         assert!(k >= 1);
//         assert!(k <= 1000);
//         MyCircularDeque {
//             data: vec![0; k as usize],
//             front: 0,
//             rear: 0,
//             len: 0,
//         }
//     }
//     pub fn insert_front(&mut self, value: i32) -> bool {
//         if self.is_full() {
//             return false;
//         }
//         self.len += 1;
//         self.front = (self.front - 1 + self.data.len() as i16) % self.data.len() as i16;
//         self.data[self.front as usize] = value as i16;
//         true
//     }
//     pub fn insert_last(&mut self, value: i32) -> bool {
//         if self.is_full() {
//             return false;
//         }
//         self.len += 1;
//         self.data[self.rear as usize] = value as i16;
//         self.rear = (self.rear + 1) % self.data.len() as i16;
//         true
//     }
//     pub fn delete_front(&mut self) -> bool {
//         if self.is_empty() {
//             return false;
//         }
//         self.len -= 1;
//         self.front = (self.front + 1) % self.data.len() as i16;
//         true
//     }
//     pub fn delete_last(&mut self) -> bool {
//         if self.is_empty() {
//             return false;
//         }
//         self.len -= 1;
//         self.rear = (self.rear - 1 + self.data.len() as i16) % self.data.len() as i16;
//         true
//     }
//     pub fn get_front(&self) -> i32 {
//         if self.is_empty() {
//             return -1;
//         }
//         self.data[self.front as usize] as i32
//     }
//     pub fn get_rear(&self) -> i32 {
//         if self.is_empty() {
//             return -1;
//         }
//         self.data[((self.rear - 1 + self.data.len() as i16) % self.data.len() as i16) as usize]
//             as i32
//     }
//     pub const fn is_empty(&self) -> bool {
//         self.len == 0
//     }
//     pub fn is_full(&self) -> bool {
//         self.len == self.data.len() as i16
//     }
// }

// Fixed-size allocation circular deque
const MAX_SIZE: i16 = 2000;
type ItemTyp = i32;
pub struct MyCircularDeque {
    k: u16,
    front: u16,
    rear: u16,
    len: u16,
    data: [ItemTyp; MAX_SIZE as usize],
}
impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        assert!(k >= 1);
        assert!(k <= 1000);
        MyCircularDeque {
            k: k as u16,
            front: 0,
            rear: 0,
            len: 0,
            data: [0; MAX_SIZE as usize],
        }
    }
    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.len += 1;
        self.front = (self.front + self.k - 1) % self.k;
        self.data[self.front as usize] = value as ItemTyp;
        true
    }
    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.len += 1;
        self.data[self.rear as usize] = value as ItemTyp;
        self.rear = (self.rear + 1) % self.k;
        true
    }
    pub fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.len -= 1;
        self.front = (self.front + 1) % self.k;
        true
    }
    pub fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.len -= 1;
        self.rear = (self.rear + self.k - 1) % self.k;
        true
    }
    pub fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.front as usize] as i32
    }
    pub fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[((self.rear + self.k - 1) % self.k) as usize] as i32
    }
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn is_full(&self) -> bool {
        self.len == self.k
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Action {
        New(i32),
        InsertFront(i32),
        InsertLast(i32),
        DeleteFront,
        DeleteLast,
        GetFront,
        GetRear,
        IsEmpty,
        IsFull,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum ExpectedResult {
        NULL,
        TRUE,
        FALSE,
        I(i32),
    }

    use Action::*;
    use ExpectedResult::*;

    fn exec_test(actions: &[Action], expected: &[ExpectedResult]) {
        let mut obj = None;
        for (i, (action, expected)) in
            std::iter::Iterator::zip(actions.into_iter().copied(), expected.into_iter().copied())
                .enumerate()
        {
            match (action, expected, obj.as_mut()) {
                (New(k), NULL, _) => {
                    if obj.is_some() {
                        println!("Warning: object already initialized");
                    }
                    obj = Some(MyCircularDeque::new(k));
                }
                (InsertFront(value), expected @ (TRUE | FALSE), Some(obj)) => {
                    assert_eq!(
                        obj.insert_front(value),
                        expected == TRUE,
                        "Step {} InsertFront({}) incorrect",
                        i,
                        value
                    );
                }
                (InsertLast(value), expected @ (TRUE | FALSE), Some(obj)) => {
                    assert_eq!(
                        obj.insert_last(value),
                        expected == TRUE,
                        "Step {} InsertLast({}) incorrect",
                        i,
                        value
                    );
                }
                (DeleteFront, expected @ (TRUE | FALSE), Some(obj)) => {
                    assert_eq!(
                        obj.delete_front(),
                        expected == TRUE,
                        "Step {} DeleteFront",
                        i
                    );
                }
                (DeleteLast, expected @ (TRUE | FALSE), Some(obj)) => {
                    assert_eq!(obj.delete_last(), expected == TRUE, "Step {} DeleteLast", i);
                }
                (GetFront, I(expected), Some(obj)) => {
                    assert_eq!(obj.get_front(), expected, "Step {} GetFront", i);
                }
                (GetRear, I(expected), Some(obj)) => {
                    assert_eq!(obj.get_rear(), expected, "Step {} GetRear", i);
                }
                (IsEmpty, expected @ (TRUE | FALSE), Some(obj)) => {
                    assert_eq!(obj.is_empty(), expected == TRUE, "Step {} IsEmpty", i);
                }
                (IsFull, expected @ (TRUE | FALSE), Some(obj)) => {
                    assert_eq!(obj.is_full(), expected == TRUE, "Step {} IsFull", i);
                }
                (_, _, None) => panic!("Object not initialized"),
                (_, _, Some(_)) => panic!(
                    "Action and expected result do not match: {:?} {:?}",
                    action, expected
                ),
            }
        }
    }

    fn test(actions: &[Action], expected: &[ExpectedResult]) {
        assert_eq!(actions.len(), expected.len());
        exec_test(actions, expected);
        // The same test should work if we swap all Front and Last operations
        // because the deque is circular and should behave symmetrically
        println!("Swapping Front and Last operations");
        exec_test(
            &actions
                .iter()
                .map(|&action| match action {
                    InsertFront(value) => InsertLast(value),
                    InsertLast(value) => InsertFront(value),
                    DeleteFront => DeleteLast,
                    DeleteLast => DeleteFront,
                    GetFront => GetRear,
                    GetRear => GetFront,
                    action => action,
                })
                .collect::<Vec<_>>(),
            expected,
        );
    }

    #[test]
    fn ex1() {
        test(
            &[
                New(3),
                InsertLast(1),
                InsertLast(2),
                InsertFront(3),
                InsertFront(4),
                GetRear,
                IsFull,
                DeleteLast,
                InsertFront(4),
                GetFront,
            ],
            &[NULL, TRUE, TRUE, TRUE, FALSE, I(2), TRUE, TRUE, TRUE, I(4)],
        )
    }

    #[test]
    fn myex1() {
        test(
            &[
                New(3),
                IsEmpty,
                IsFull,
                InsertFront(1),
                IsEmpty,
                IsFull,
                InsertFront(2),
                IsEmpty,
                IsFull,
                InsertFront(3),
                IsEmpty,
                IsFull,
                InsertFront(4),
                IsEmpty,
                IsFull,
                GetRear,
                GetFront,
                DeleteLast,
                IsEmpty,
                IsFull,
                GetRear,
                GetFront,
                DeleteFront,
                IsEmpty,
                IsFull,
                GetRear,
                GetFront,
                DeleteFront,
                IsEmpty,
                IsFull,
                GetRear,
                GetFront,
            ],
            &[
                NULL,
                TRUE,  // New circular deque is empty
                FALSE, // New circular deque is not full
                TRUE,  // InsertFront(1) is successful
                FALSE, // No longer empty
                FALSE, // Not full yet
                TRUE,  // InsertFront(2) is successful
                FALSE, // Not empty
                FALSE, // Not full yet
                TRUE,  // InsertFront(3) is successful
                FALSE, // Not empty
                TRUE,  // Now full
                FALSE, // InsertFront(4) is unsuccessful
                FALSE, // Still not empty
                TRUE,  // Still full
                I(1),  // GetRear
                I(3),  // GetFront
                TRUE,  // DeleteLast is successful
                FALSE, // Not empty
                FALSE, // Not full
                I(2),  // GetRear
                I(3),  // GetFront has not changed
                TRUE,  // DeleteFront is successful
                FALSE, // Not empty
                FALSE, // Not full
                I(2),  // GetRear
                I(2),  // GetFront
                TRUE,  // DeleteFront is successful
                TRUE,  // Now empty
                FALSE, // Not full
                I(-1), // GetRear fails
                I(-1), // GetFront fails
            ],
        )
    }

    #[test]
    fn myex2() {
        test(
            &[
                New(2),
                IsEmpty,
                IsFull,
                InsertFront(1),
                IsEmpty,
                IsFull,
                InsertFront(2),
                IsEmpty,
                IsFull,
                InsertFront(3),
                IsEmpty,
                IsFull,
                GetRear,
                GetFront,
                DeleteLast,
                IsEmpty,
                IsFull,
                GetRear,
                GetFront,
                DeleteFront,
                IsEmpty,
                IsFull,
                GetRear,
                GetFront,
            ],
            &[
                NULL,  // New circular deque
                TRUE,  // New circular deque is empty
                FALSE, // New circular deque is not full
                TRUE,  // InsertFront(1) is successful
                FALSE, // No longer empty
                FALSE, // Not full yet
                TRUE,  // InsertFront(2) is successful
                FALSE, // Not empty
                TRUE,  // Full
                FALSE, // InsertFront(3) is unsuccessful
                FALSE, // Still not empty
                TRUE,  // Still full
                I(1),  // GetRear
                I(2),  // GetFront
                TRUE,  // DeleteLast is successful
                FALSE, // Not empty
                FALSE, // Not full
                I(2),  // GetRear
                I(2),  // GetFront
                TRUE,  // DeleteFront is successful
                TRUE,  // Now empty
                FALSE, // Not full
                I(-1), // GetRear fails
                I(-1), // GetFront fails
            ],
        )
    }

    #[test]
    fn discussion_case1() {
        // ["MyCircularDeque","insertFront","insertLast","deleteFront","getFront","deleteLast","insertLast","isEmpty","deleteLast","insertFront","getRear","deleteFront","insertFront","insertLast","deleteLast","getFront","getRear","insertFront","getRear","getFront"]
        // [[999],[93],[578],[],[],[],[533],[],[],[913],[],[],[100],[57],[],[],[],[900],[],[]]
        test(
            &[
                New(999),
                InsertFront(93),
                InsertLast(578),
                DeleteFront,
                GetFront,
                DeleteLast,
                InsertLast(533),
                IsEmpty,
                DeleteLast,
                InsertFront(913),
                GetRear,
                DeleteFront,
                InsertFront(100),
                InsertLast(57),
                DeleteLast,
                GetFront,
                GetRear,
                InsertFront(900),
                GetRear,
                GetFront,
            ],
            &[
                NULL,   // New circular deque
                TRUE,   // InsertFront(93) is successful
                TRUE,   // InsertLast(578) is successful
                TRUE,   // DeleteFront is successful
                I(578), // GetFront
                TRUE,   // DeleteLast is successful
                TRUE,   // InsertLast(533) is successful
                FALSE,  // Not empty
                TRUE,   // DeleteLast is successful
                TRUE,   // InsertFront(913) is successful
                I(913), // GetRear
                TRUE,   // DeleteFront is successful
                TRUE,   // InsertFront(100) is successful
                TRUE,   // InsertLast(57) is successful
                TRUE,   // DeleteLast is successful
                I(100), // GetFront
                I(100), // GetRear
                TRUE,   // InsertFront(900) is successful
                I(100), // GetRear
                I(900), // GetFront
            ],
        )
    }

    #[test]
    fn discussion_case2() {
        // ["MyCircularDeque","insertFront","insertFront","getRear","getFront","getFront","deleteLast","deleteFront","getRear"]
        // [[41],[70],[11],[],[],[],[],[],[]]
        test(
            &[
                New(41),
                InsertFront(70),
                InsertFront(11),
                GetRear,
                GetFront,
                GetFront,
                DeleteLast,
                DeleteFront,
                GetRear,
            ],
            &[
                NULL,  // New circular deque
                TRUE,  // InsertFront(70) is successful
                TRUE,  // InsertFront(11) is successful
                I(70), // GetRear
                I(11), // GetFront
                I(11), // GetFront
                TRUE,  // DeleteLast is successful
                TRUE,  // DeleteFront is successful
                I(-1), // GetRear fails
            ],
        )
    }
}
