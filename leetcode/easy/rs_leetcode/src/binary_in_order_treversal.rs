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

struct  Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            result: &mut Vec<i32>,
        ) {
            if let Some(node) = node {
                let node_ref = node.borrow();

                // left
                dfs(node_ref.left.clone(), result);

                // root
                result.push(node_ref.val);

                // right
                dfs(node_ref.right.clone(), result);
            }
        }

        dfs(root, &mut result);
        result
    }
}

