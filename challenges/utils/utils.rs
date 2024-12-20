use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(PartialEq, Eq)]
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

    pub fn from_perfect_slice(slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        fn rec(i: usize, slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if i < slice.len() {
                let left = rec(2 * i + 1, slice);
                let right = rec(2 * i + 2, slice);
                Some(Rc::new(RefCell::new(TreeNode {
                    val: slice[i],
                    left,
                    right,
                })))
            } else {
                None
            }
        }
        rec(0, slice)
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

fn linearize(root: &TreeNode) -> Vec<Option<i32>> {
        let mut eles = vec![Some(root.val)];
        fn rec(i: usize, eles: &mut Vec<Option<i32>>, node: &Option<Rc<RefCell<TreeNode>>>) {
            let Some(node) = node else {
                return;
            };
            let contents = node.borrow();
            if eles.len() <= i + 1 {
                eles.resize(i + 1, None);
            }
            eles[i] = Some(contents.val);
            rec(2 * i + 1, eles, &contents.left);
            rec(2 * i + 2, eles, &contents.right);
        }
        rec(1, &mut eles, &root.left);
        rec(2, &mut eles, &root.right);
        eles
}

impl std::fmt::Debug for TreeNode {
    // Nodal debug fmt
    // fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    //     let mut d = f.debug_struct("Node");
    //     d.field("val", &self.val);
    //     if self.left.is_some() || self.right.is_some() {
    //         d.field("left", &self.left).field("right", &self.right)
    //     } else {
    //         &mut d
    //     }
    //     .finish()
    // }

    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let lin = linearize(self);
        f.write_str("Tree ")?;
        f.debug_list().entries(lin.into_iter()).finish()
    }
}

impl PartialEq<&[i32]> for TreeNode {
    fn eq(&self, other: &&[i32]) -> bool {
        let eles = linearize(self);
        if eles.len() != other.len() {
            return false;
        }
        for (ele, o) in std::iter::Iterator::zip(eles.into_iter(), other.into_iter().copied()) {
            if Some(o) != ele {
                return false;
            }
        }
        true
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
