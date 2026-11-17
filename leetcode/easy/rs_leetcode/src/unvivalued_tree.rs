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
       right: None
     }
   }
 }


use std::rc::Rc;
use std::cell::RefCell;


struct Solution;


impl Solution {

    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let val = match &root {
            Some(node) => node.borrow().val,
            None => return true,
        };

        Self::check(&root, val)
    }

    fn check(
        root: &Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> bool {
        let node = match root {
            Some(node) => node.borrow(),
            None => return true, 
        };

        if node.val != val {
            return false;
        }

        Self::check(&node.left, val)
            && Self::check(&node.right, val)
    }
}

