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
use std::collections::HashSet;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut seen = HashSet::new();
        Self::dfs(&root, k, &mut seen)
    }

    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        k: i32,
        seen: &mut HashSet<i32>,
    ) -> bool {
        match node {
            None => false,
            Some(rc) => {
                let n = rc.borrow();
                if seen.contains(&(k - n.val)) {
                    return true;
                }
                seen.insert(n.val);
                Self::dfs(&n.left, k, seen) || Self::dfs(&n.right, k, seen)
            }
        }
    }
}
