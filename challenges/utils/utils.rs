use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_slice(slice: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = slice
            .iter()
            .map(|val| val.map(|val| TreeNode::new(val)))
            .collect::<Vec<_>>();

        for i in (0..nodes.len()).rev() {
            if nodes[i].is_some() {
                let left = nodes.get_mut(2 * i + 1).map(Option::take).flatten();
                let right = nodes.get_mut(2 * i + 2).map(Option::take).flatten();

                nodes[i].as_mut().map(|node| {
                    let left_ptr = left.map(|left| Rc::new(RefCell::new(left)));
                    let right_ptr = right.map(|right| Rc::new(RefCell::new(right)));
                    node.left = left_ptr;
                    node.right = right_ptr;
                });
            }
        }

        nodes[0].take().map(|node| Rc::new(RefCell::new(node)))
    }

    pub fn from_leetcode_slice(slice: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = slice.iter().cloned();

        let root = Rc::new(RefCell::new(TreeNode::new(nodes.next()??)));
        let mut queue = vec![root.clone()];
        let mut next_queue = Vec::new();

        while !queue.is_empty() {
            for node in queue {
                let mut node = node.borrow_mut();
                if let Some(Some(val)) = nodes.next() {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.left = Some(left.clone());
                    next_queue.push(left);
                }
                if let Some(Some(val)) = nodes.next() {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(right.clone());
                    next_queue.push(right);
                }
            }
            queue = next_queue;
            next_queue = Vec::new();
        }

        Some(root)
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    pub fn from_slice(arr: &[i32]) -> Option<Box<Self>> {
        match arr {
            [] => None,
            [v] => Some(Self::new(*v).into()),
            [v, rest @ ..] => {
                let mut node = Self::new(*v);
                node.next = Self::from_slice(rest);
                Some(node.into())
            }
        }
    }
}

impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node = self;
        write!(f, "{}", node.val)?;
        while let Some(next) = &node.next {
            node = next;
            write!(f, " -> {}", node.val)?;
        }
        Ok(())
    }
}
