use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
type OptNode = Option<Rc<RefCell<TreeNode>>>;
struct FindElements {
    included: HashSet<i32>,
}
impl FindElements {
    fn new(root: OptNode) -> Self {
        fn walk(root: &OptNode, index: i32, included: &mut HashSet<i32>) {
            if let Some(rc) = root.as_ref() {
                let node = rc.borrow();
                included.insert(index);
                walk(&node.left, 2*index + 1, included);
                walk(&node.right, 2*index + 2, included);
            }
        }
        let mut included = HashSet::new();
        walk(&root, 0, &mut included);
        Self { included }
    }
    fn find(&self, target: i32) -> bool {
        self.included.contains(&target)
    }
}
