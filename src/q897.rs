use super::util::tree::{to_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Handler {
    head: Option<Rc<RefCell<TreeNode>>>,
    last: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl Handler {
    pub fn new() -> Self {
        Handler {
            head: None,
            last: None,
        }
    }

    pub fn increasing_bst(
        &mut self,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        self.middle_trav(root);
        self.head.clone()
    }

    fn middle_trav(&mut self, current: Option<Rc<RefCell<TreeNode>>>) -> () {
        if current == None {
            return;
        }

        // Left
        self.middle_trav(current.as_ref().unwrap().borrow().left.clone());

        println!("cu:{}", current.as_ref().unwrap().borrow().val);

        if self.last != None {
            unsafe {
                let t = self.last.as_ref().unwrap().as_ptr();
                let mut x = &mut *t;
                //x.right = current.clone();
                //x.right = None;
                x.left = None;
            }
        } else {
            // first
            self.head = current.clone();
        }

        self.last = current.clone();
        // right
        self.middle_trav(current.as_ref().unwrap().borrow().right.clone());
    }
}

pub struct Solution {}
/*
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut h = Handler::new();
        h.increasing_bst(root)
    }
}*/

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        Self::do_increasing_bst(root, node.clone());
        node.unwrap().borrow().right.clone()
    }

    #[inline]
    fn ptr_node<'a>(n: &'a Option<Rc<RefCell<TreeNode>>>) -> &'a mut TreeNode {
        let t = n.as_ref().unwrap().as_ptr();
        unsafe { &mut *t }
    }

    pub fn do_increasing_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        tail: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root != None {
            let left_tail = Self::do_increasing_bst(
                root.as_ref().unwrap().borrow_mut().left.clone(),
                tail.clone(),
            );

            //Self::ptr_node(&root).left = None;
            //Self::ptr_node(&left_tail).right = root.clone();

            root.as_ref().unwrap().borrow_mut().left = None;
            left_tail.as_ref().unwrap().borrow_mut().right = root.clone();

            let right = root.as_ref().unwrap().borrow().right.clone();

            Self::do_increasing_bst(right, root.clone())
        } else {
            // println!("t:{}", tail.as_ref().unwrap().borrow().val);
            tail
        }
    }
}
/*
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        Self::do_increasing_bst(root, node.clone());
        node.unwrap().borrow().right.clone()
    }
    pub fn do_increasing_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        tail: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut left_tail =
                Self::do_increasing_bst(node.borrow_mut().left.take(), tail.clone());
            left_tail.unwrap().borrow_mut().right = Some(node.clone());
            let right = node.borrow_mut().right.clone();
            Self::do_increasing_bst(right, Some(node))
        } else {
            println!("t:{}", tail.as_ref().unwrap().borrow().val);
            tail
        }
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_897() {
        assert_eq!(
            Solution::increasing_bst(tree![5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9]),
            tree![1, null, 2, null, 3, null, 4, null, 5, null, 6, null, 7, null, 8, null, 9]
        );

        assert_eq!(Solution::increasing_bst(tree![4, 3]), tree![3, null, 4])
    }
}
