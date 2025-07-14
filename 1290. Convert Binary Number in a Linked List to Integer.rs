impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut result = head.as_ref().unwrap().val;
        while head.as_ref().unwrap().next.is_some() {
            let next_node = head.unwrap().next.unwrap();
            result = (result << 1) | next_node.val;
            head = Some(next_node);
        }
        result
    }
}
