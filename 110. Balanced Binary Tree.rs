use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::depth_finder(root.as_ref()).is_some()
    }
    fn depth_finder(root: Option<&Rc<RefCell<TreeNode>>>) -> Option<i32> {
        let Some(node) = root.map(|node| node.borrow()) else {
            return Some(0);
        };
        let left_depth = Self::depth_finder(node.left.as_ref())?;
        let right_depth = Self::depth_finder(node.right.as_ref())?;
        (left_depth.abs_diff(right_depth) < 2).then_some(left_depth.max(right_depth) + 1)
    }
}
