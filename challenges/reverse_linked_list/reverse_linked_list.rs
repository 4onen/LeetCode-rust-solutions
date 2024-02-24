use utils::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut node| {
            let mut next = node.next.take();
            while let Some(mut next_node) = next {
                next = next_node.next.take();
                next_node.next = Some(node);
                node = next_node;
            }
            node
        })
    }
}

#[cfg(test)]
mod test {
    use super::{ListNode, Solution};

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::reverse_list(ListNode::from_slice(&[1, 2, 3, 4, 5])),
            ListNode::from_slice(&[5, 4, 3, 2, 1])
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::reverse_list(ListNode::from_slice(&[1, 2])),
            ListNode::from_slice(&[2, 1])
        );
    }
}
