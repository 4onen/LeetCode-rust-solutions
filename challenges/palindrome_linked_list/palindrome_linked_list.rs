use utils::ListNode;

pub struct Solution;

// As fast as we can go with a side stack,
// while valid, this isn't the O(1) memory
// alg for reversing a linked list we could use.
// impl Solution {
//     pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
//         let mut stack: Vec<u8> = Vec::new();
//
//         let mut to_check: usize = 0;
//         let mut next = head.as_ref();
//         while let Some(b) = next {
//             to_check += 1;
//             next = b.next.as_ref();
//             stack.push(b.val as u8);
//         }
//
//         to_check /= 2;
//
//         let mut next = head;
//         while let Some(b) = next {
//             next = b.next;
//             if !(stack.pop() == Some(b.val as u8)) {
//                 return false
//             }
//             to_check -= 1;
//             if to_check < 1 {
//                 break
//             }
//         }
//
//         true
//     }
// }

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        head.is_none() || {
            let length = {
                let mut length = 0;
                let mut to_end = head.as_ref();
                while let Some(boxed) = to_end {
                    length += 1;
                    to_end = boxed.next.as_ref();
                }
                length
            };

            match length {
                0 | 1 => true,
                2 => {
                    let ListNode { val, next } = *head.unwrap();
                    val == next.unwrap().val
                }
                3 => {
                    let ListNode { val, next } = *head.unwrap();
                    let val_end = next.unwrap().next.unwrap().val;
                    val == val_end // Doesn't matter what the middle is.
                }
                _ => {
                    let mut head_node = head.unwrap();
                    let middle = {
                        let mut to_middle = length / 2 - 1;
                        let mut next = &mut head_node;
                        while to_middle > 0 {
                            next = next.next.as_mut().unwrap();
                            to_middle -= 1;
                        }
                        next
                    };

                    // Reverse the second half of the list
                    let mut prev = middle.next.take().unwrap();
                    let mut next = prev.next.take();
                    while let Some(mut boxed) = next {
                        next = boxed.next.take();
                        boxed.next = Some(prev);
                        prev = boxed;
                    }
                    // Now prev is the head of the reversed second half
                    // Compare the two halves
                    let mut next_front = Some(&head_node);
                    let mut next_back = Some(&prev);
                    while let (Some(front), Some(back)) = (next_front, next_back) {
                        if front.val != back.val {
                            return false;
                        }
                        next_front = front.next.as_ref();
                        next_back = back.next.as_ref();
                    }

                    true
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ListNode, Solution};

    #[test]
    fn ex1() {
        assert!(Solution::is_palindrome(ListNode::from_slice(&[1, 2, 2, 1])));
    }

    #[test]
    fn ex1_var1() {
        assert!(Solution::is_palindrome(ListNode::from_slice(&[
            1, 2, 3, 2, 1
        ])));
    }

    #[test]
    fn ex1_var2() {
        assert!(!Solution::is_palindrome(ListNode::from_slice(&[
            1, 2, 3, 3, 1
        ])));
    }

    #[test]
    fn ex2() {
        assert!(!Solution::is_palindrome(ListNode::from_slice(&[1, 2])));
    }

    #[test]
    fn ex2_var1() {
        assert!(Solution::is_palindrome(ListNode::from_slice(&[1, 2, 1])));
    }

    #[test]
    fn ex2_var2() {
        assert!(Solution::is_palindrome(ListNode::from_slice(&[1, 1])));
    }

    #[test]
    fn ex2_var3() {
        assert!(Solution::is_palindrome(ListNode::from_slice(&[2, 1, 2])));
    }

    #[test]
    fn empty() {
        assert!(Solution::is_palindrome(None));
    }

    #[test]
    fn long1() {
        assert!(Solution::is_palindrome(ListNode::from_slice(&[
            1, 2, 3, 4, 5, 4, 3, 2, 1
        ])));
    }

    #[test]
    fn long2() {
        assert!(Solution::is_palindrome(ListNode::from_slice(&[
            1, 2, 3, 4, 5, 5, 4, 3, 2, 1
        ])));
    }

    #[test]
    fn wrong_answer1() {
        assert!(!Solution::is_palindrome(ListNode::from_slice(&[
            1, 1, 2, 1
        ])));
    }

    #[test]
    fn wrong_answer1_var1() {
        assert!(Solution::is_palindrome(ListNode::from_slice(&[
            1, 1, 2, 1, 1
        ])));
    }

    #[test]
    fn wrong_answer1_var2() {
        assert!(!Solution::is_palindrome(ListNode::from_slice(&[
            1, 2, 1, 1
        ])));
    }

    #[test]
    fn wrong_answer1_var3() {
        assert!(!Solution::is_palindrome(ListNode::from_slice(&[
            1, 2, 1, 2
        ])));
    }
}
