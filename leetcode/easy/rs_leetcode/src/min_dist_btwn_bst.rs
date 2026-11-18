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

    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        fn inorder (
            node : Option<Rc<RefCell<TreeNode>>>, 
            prev : &mut Option<i32>, 
            best : &mut i32,
        ) { 
            let Some(node) = node else { 
                return;
            }; 

            let node = node.borrow(); 
            inorder(node.left.clone(), prev, best);

            if let Some(previous) = *prev { 
                *best = (*best).min(node.val - previous); 
            }
            *prev = Some(node.val); 

            inorder(node.right.clone(), prev, best);
        }

        let mut prev  = None; 
        let mut best = i32::MAX ; 
        inorder(root, &mut prev, &mut best);
        best
    }
}
