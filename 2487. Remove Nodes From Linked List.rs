use std::ptr;
impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut node) => {
                node.next = Solution::remove_nodes(node.next.take());
                if let Some(next_node) = node.next.as_ref() {
                    if node.val < next_node.val {
                        node.next
                    } else {
                        Some(node)
                    }
                } else {
                    Some(node)
                }
            }
        }
    }
}
