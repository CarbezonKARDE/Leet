impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        remove_nth_from_end_recr(head, n).0
    }
}
fn remove_nth_from_end_recr(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, usize) {
    match head {
        None => (None, 1),
        Some(mut node) => {
            let (prev, num) = remove_nth_from_end_recr(node.next.take(), n);
            if n == num as i32 {
                (prev, num+1)
            } else {
                node.next = prev;
                (Some(node), num+1)
            }
        }
    }
}
