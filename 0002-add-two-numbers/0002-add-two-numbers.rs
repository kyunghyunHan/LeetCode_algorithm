impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add(l1, l2, 0)
    }
    
    fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        let (a, n1) = Self::extract(l1);
        let (b, n2) = Self::extract(l2);
        
        let x = a + b + carry;
        let new_carry = x / 10;
        
        let mut node = ListNode::new(x % 10);
        node.next = Self::add(n1, n2, x / 10);
        Some(Box::new(node))
    }
    
    #[inline]
    fn extract(l: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>){
        if let Some(n) = l {
            (n.val, n.next)
        } else {
            (0, None)
        }
    }
}