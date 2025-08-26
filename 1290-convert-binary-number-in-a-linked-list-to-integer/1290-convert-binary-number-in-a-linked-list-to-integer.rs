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
   pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut res = 0;
    let mut node = head;

    while let Some(n) = node {
        res = res * 2 + n.val;
        node = n.next;
    }
    res
}
}