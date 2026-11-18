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

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, other) | (other, None) => other,

            (Some(first), Some(second)) => {
                let (value, left, right) = {
                    let second = second.borrow();
                    (second.val, second.left.clone(), second.right.clone())
                };

                let mut node = first.borrow_mut();
                node.val += value;
                node.left = Self::merge_trees(node.left.take(), left);
                node.right = Self::merge_trees(node.right.take(), right);
                drop(node);

                Some(first)
            }
        }
    }
}
