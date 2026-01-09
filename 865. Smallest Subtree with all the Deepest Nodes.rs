use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn subtree_with_all_deepest(root: Node) -> Node {
        Self::post_order_traversal(root).0
    }
    fn post_order_traversal(root: Node) -> (Node, usize) {
        match root {
            Some(node) => {
                let (left_smallest, left_height) =
                    Self::post_order_traversal(
                        node.borrow().left.clone(),
                    );
                let (right_smallest, right_height) =
                    Self::post_order_traversal(
                        node.borrow().right.clone(),
                    );
                match left_height.cmp(&right_height) {
                    Ordering::Less => {
                        (right_smallest, right_height + 1)
                    }
                    Ordering::Greater => {
                        (left_smallest, left_height + 1)
                    }
                    Ordering::Equal => (Some(node), left_height + 1),
                }
            }
            None => (None, 0),
        }
    }
}
