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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
         let mut sum = 0;
        if let Some(root) = root {
          let mut stack = vec![root];
           while let Some(node) = stack.pop() {
        let node = node.borrow();

        if let Some(left) = &node.left {
            let left_node = left.borrow();

            if left_node.left.is_none() && left_node.right.is_none() {
                sum += left_node.val;
            } else {
                stack.push(Rc::clone(left));
            }
        }

        if let Some(right) = &node.right {
            stack.push(Rc::clone(right));
        }
    }

        
      }
       
        sum
    }
}