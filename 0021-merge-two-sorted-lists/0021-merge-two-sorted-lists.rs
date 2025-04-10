impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>, 
        list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    let next = Solution::merge_two_lists(l1.next.take(), Some(l2));
                    l1.next = next;
                    Some(l1)
                } else {
                    let next = Solution::merge_two_lists(Some(l1), l2.next.take());
                    l2.next = next;
                    Some(l2)
                }
            }
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (None, None) => None,
        }
    }
}
