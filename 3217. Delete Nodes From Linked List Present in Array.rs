use std::collections::HashSet;
impl Solution {
        pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = head;
        let set: HashSet<i32> = nums.into_iter().collect();
        while let Some(ref mut node) = new_head {
            if set.contains(&node.val) {
                new_head = node.next.take();
            } else {
                break;
            }
        }
        let mut curr = new_head.as_mut().unwrap();
        while let Some(node) = curr.next.as_mut() {
                if set.contains(&node.val) {
                    curr.next = node.next.take();
                } else {
                    curr = curr.next.as_mut().unwrap();
                }
        }
        new_head
    }
}
