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
        root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        self.middle_trav(&root);
        self.head.clone()
    }

    fn middle_trav(&mut self, current: &Option<Rc<RefCell<TreeNode>>>) -> () {
        if *current == None {
            return;
        }

        // Left
        self.middle_trav(&(current.as_ref().unwrap().borrow().left));
        println!("c:{}", current.as_ref().unwrap().borrow().val);
        //current.as_ref().unwrap().borrow_mut().left = None;

        if self.last != None {
            println!("l:{}", self.last.as_ref().unwrap().borrow().val);
            self.last.as_ref().unwrap().borrow_mut().right = current.clone();
            self.last.as_ref().unwrap().borrow_mut().left = None;
        } else {
            // first
            self.head = current.clone();
        }

        self.last = current.clone();
        //current.as_ref().unwrap().borrow_mut().left = None;
        println!("----");

        // Right
        self.middle_trav(&(current.as_ref().unwrap().borrow().right));
    }
}

pub struct Solution {}

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut h = Handler::new();
        h.increasing_bst(&root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_897() {
        assert_eq!(
            Solution::increasing_bst(tree![5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9]),
            tree![1, null, 2, null, 3, null, 4, null, 5, null, 6, null, 7, null, 8, null, 9]
        )
    }
}
