// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
       pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(pn), Some(qn)) => {
                if pn.borrow().val != qn.borrow().val { return false; }
                let p_left = pn.borrow().left.clone();
                let p_right = pn.borrow().right.clone();
                let q_left = qn.borrow().left.clone();
                let q_right = qn.borrow().right.clone();

                Self::is_same_tree(p_left, q_left) && Self::is_same_tree(p_right, q_right)
            }
        }
    }

}