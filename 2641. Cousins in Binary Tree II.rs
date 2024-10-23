use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = if let Some(root) = root { root } else { return None };
        let mut level_sum = 0;
        let mut queue = vec![(root.clone(), 0)];
        while !queue.is_empty() {
            let mut new_queue = vec![];
            let mut new_level_sum = 0;
            for (node, sibling_sum) in queue.into_iter() {
                node.borrow_mut().val = level_sum - sibling_sum;
                let mut sibling_sum = 0;
                let mut non_leaf_children = vec![];
                if let Some(child) = node.borrow().left.clone() {
                    sibling_sum += child.borrow().val;
                    non_leaf_children.push(child);
                }
                if let Some(child) = node.borrow().right.clone() {
                    sibling_sum += child.borrow().val;
                    non_leaf_children.push(child);
                }
                new_level_sum += sibling_sum;
                non_leaf_children.into_iter()
                    .for_each(|node| new_queue.push((node, sibling_sum)));
            }
            queue = new_queue;
            level_sum = new_level_sum;
        }
        Some(root)
    }
}
