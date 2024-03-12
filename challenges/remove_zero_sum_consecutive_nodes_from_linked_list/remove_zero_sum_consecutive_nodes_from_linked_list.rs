// https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/

use utils::ListNode;

pub struct Solution;

// Initial sol'n
// impl Solution {
//     pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         // This is dumb. Let's get `unsafe {}`.
//         type ListNodeNextPtr = Option<Box<ListNode>>;
//         // The sum of all nodes thus far
//         let mut sum = 0;
//         // A map of the sum of all nodes thus far to the node that the sum was at
//         // This is so we can remove the nodes between the current node and the node
//         let mut map: std::collections::HashMap<i32, *mut ListNodeNextPtr> =
//             std::collections::HashMap::new();
//         // The `head` variable will be our dummy, since our pointers are to
//         // the `next` field of the previous `ListNode` and `head` has the
//         // correct type for that.
//         map.insert(0, &mut head as *mut ListNodeNextPtr);
//         // We'll use a `while let` loop to iterate through the list
//         let mut current = &mut head;
//         while let Some(node) = current {
//             // Add the current node's value to the sum
//             sum += node.val;
//             // If we've seen this sum before, remove the nodes between the
//             // current node and the node that the sum was at
//             let entry = map.entry(sum);
//             match entry {
//                 std::collections::hash_map::Entry::Occupied(occupied) => {
//                     let start =
//                         unsafe { &mut *(occupied.remove_entry().1 as *mut ListNodeNextPtr) };
//                     *start = node.next.take();
//                     return Self::remove_zero_sum_sublists(head);
//                 }
//                 std::collections::hash_map::Entry::Vacant(vacant) => {
//                     vacant.insert(&mut node.next as *mut ListNodeNextPtr);
//                 }
//             }
//             // Move to the next node
//             current = &mut node.next;
//         }
//         head
//     }
// }

// Cheesy sol'n
impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec = head
            .map(|mut head| {
                let mut vec = vec![head.val];
                while let Some(node) = head.next.take() {
                    vec.push(node.val);
                    head = node;
                }
                vec
            })
            .unwrap_or(vec![]);
        let mut sum = 0;
        let mut map = std::collections::HashMap::new();
        map.insert(0, -1i16);
        let mut i = 0;
        while i < vec.len() as i16 {
            sum += vec[i as usize];
            match map.entry(sum) {
                std::collections::hash_map::Entry::Occupied(_) => {
                    let start = (map[&sum] + 1) as usize;
                    vec.drain(start..=i as usize);
                    // Internal loop reset
                    sum = 0;
                    map.clear();
                    map.insert(0, -1i16);
                    i = 0;
                    continue;
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(i);
                }
            }
            i += 1;
        }
        let mut head = None;
        let mut current = &mut head;
        for val in vec {
            *current = Some(Box::new(ListNode::new(val)));
            current = &mut current.as_mut().unwrap().next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let head = ListNode::from_slice(&[1, 2, -3, 3, 1]);
        let expected = ListNode::from_slice(&[3, 1]);
        assert_eq!(Solution::remove_zero_sum_sublists(head), expected);
    }

    #[test]
    fn ex2() {
        let head = ListNode::from_slice(&[1, 2, 3, -3, 4]);
        let expected = ListNode::from_slice(&[1, 2, 4]);
        assert_eq!(Solution::remove_zero_sum_sublists(head), expected);
    }

    #[test]
    fn ex3() {
        let head = ListNode::from_slice(&[1, 2, 3, -3, -2]);
        let expected = ListNode::from_slice(&[1]);
        assert_eq!(Solution::remove_zero_sum_sublists(head), expected);
    }

    #[test]
    fn myex0() {
        let head = ListNode::from_slice(&[0, 0]);
        let expected = None;
        assert_eq!(Solution::remove_zero_sum_sublists(head), expected);
    }

    #[test]
    fn discussion_case1() {
        let head = ListNode::from_slice(&[1, 2, 3, 4, -10, 1, 3, -3]);
        let expected = ListNode::from_slice(&[1]);
        assert_eq!(Solution::remove_zero_sum_sublists(head), expected);
    }

    #[test]
    fn my_extreme_example0() {
        let head = ListNode::from_slice(
            &(1..=1000)
                .into_iter()
                .map(|x| x - 500)
                .collect::<Vec<i32>>(),
        );
        let expected = ListNode::from_slice(&[500]);
        assert_eq!(Solution::remove_zero_sum_sublists(head), expected);
    }
}

#[cfg(test)]
mod array_practice_tests {
    pub fn remove_zero_sum_subarrays(mut arr: Vec<i32>) -> Vec<i32> {
        if arr.is_empty() {
            return arr;
        }
        let mut map = std::collections::HashMap::new();
        let mut sum = 0;
        map.insert(0, -1i16);
        for i in 0..arr.len() as i16 {
            sum += arr[i as usize];
            match map.entry(sum) {
                std::collections::hash_map::Entry::Occupied(_) => {
                    let start = (map[&sum] + 1) as usize;
                    arr.drain(start..=i as usize);
                    return remove_zero_sum_subarrays(arr);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(i);
                }
            }
        }
        arr
    }

    #[test]
    fn ex1() {
        let arr = vec![1, 2, -3, 3, 1];
        let result = remove_zero_sum_subarrays(arr);
        assert!(result == [3, 1] || result == [1, 2, 1])
    }

    #[test]
    fn ex2() {
        let arr = vec![1, 2, 3, -3, 4];
        let expected = vec![1, 2, 4];
        assert_eq!(remove_zero_sum_subarrays(arr), expected);
    }

    #[test]
    fn ex3() {
        let arr = vec![1, 2, 3, -3, -2];
        let expected = vec![1];
        assert_eq!(remove_zero_sum_subarrays(arr), expected);
    }

    #[test]
    fn myex0() {
        let arr = vec![0, 0];
        let expected = vec![];
        assert_eq!(remove_zero_sum_subarrays(arr), expected);
    }

    #[test]
    fn discussion_case1() {
        let arr = vec![1, 2, 3, 4, -10, 1, 3, -3];
        let expected = vec![1];
        assert_eq!(remove_zero_sum_subarrays(arr), expected);
    }

    #[test]
    fn my_extreme_example0() {
        let head: Vec<i32> = (1..=1000)
            .into_iter()
            .map(|x| x - 500)
            .collect::<Vec<i32>>();
        let expected = &[500];
        assert_eq!(remove_zero_sum_subarrays(head), expected);
    }
}
