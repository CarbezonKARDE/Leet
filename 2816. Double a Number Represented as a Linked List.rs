impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut h) = head {
            if Solution::get_carry(&mut h) == 1 {
                Some(Box::new(ListNode { val: 1, next: Some(h) }))
            } else {
                Some(h)
            }
        } else {
            None
        }
    }
    fn get_carry(node: &mut ListNode) -> i32 {
        let mut val = node.val * 2;
        if let Some(ref mut next) = node.next {
            val += Solution::get_carry(next);
        }
        node.val = val % 10;
        val / 10
    }
}
