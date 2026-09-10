use std::{cell::RefCell, rc::Rc};
impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root.as_ref()).2
    }
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        let Some(node) = node.map(|n| n.borrow()) else {
            return (0, 0, 0);
        };
        let (lc, ls, lr) = Self::dfs(node.left.as_ref());
        let (rc, rs, rr) = Self::dfs(node.right.as_ref());
        let cs = ls + rs + node.val;
        let cc = lc + rc + 1;
        (cc, cs, i32::from(cs / cc == node.val) + lr + rr)
    }
}
