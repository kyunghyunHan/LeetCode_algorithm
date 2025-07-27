// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, l2) => l2,
        (l1, None) => l1,
        (Some(mut l1_node), Some(mut l2_node)) => {
            if l1_node.val < l2_node.val {
                l1_node.next = Self::merge_two_lists(l1_node.next, Some(l2_node));
                Some(l1_node)
            } else {
                l2_node.next =Self:: merge_two_lists(Some(l1_node), l2_node.next);
                Some(l2_node)
            }
        }
    }
}

}