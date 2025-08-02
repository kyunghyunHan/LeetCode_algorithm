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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn helper(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node_rc) = node {
            let node_ref = node_rc.borrow();
            helper(node_ref.left.clone(), result);  // 왼쪽 자식
            result.push(node_ref.val);              // 현재 노드
            helper(node_ref.right.clone(), result); // 오른쪽 자식
        }
    }

    let mut result = Vec::new();
    helper(root, &mut result);
    result
}

}