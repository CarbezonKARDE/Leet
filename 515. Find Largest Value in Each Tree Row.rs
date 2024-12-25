use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        }
        std::iter::successors(Some(vec![root.clone().unwrap()]), |nodes| {
            let children = nodes
                .iter()
                .flat_map(|node| [node.borrow().left.clone(), node.borrow().right.clone()])
                .filter(|node| node.is_some())
                .flatten()
                .collect::<Vec<_>>();
            if children.len() > 0 {
                Some(children)
            } else {
                None
            }
        })
        .map(|nodes| nodes
              .into_iter()
              .map(|node| node.borrow().val)
              .max()
              .unwrap()
        )
        .collect()
    }
}
