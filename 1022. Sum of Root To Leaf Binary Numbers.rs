use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {return 0};
        let mut leafs:Vec<Vec<i32>> = vec![];
        let mut stack = vec![(vec![root.clone().unwrap().borrow().val],root)];
        while !stack.is_empty() {
            let (path,node) = stack.pop().unwrap();
            let mut is_leaf = true;
            if let Some(left) = node.clone().unwrap().borrow().left.clone() {
                is_leaf = false;
                let mut path = path.clone();
                path.push(left.borrow().val);
                stack.push((path,Some(left)))
            }
            if let Some(right) = node.unwrap().borrow().right.clone() {
                is_leaf = false;
                let mut path = path.clone();
                path.push(right.borrow().val);
                stack.push((path,Some(right)))
            }
            if is_leaf {
                leafs.push(path);
            }
        }
        fn bin_to_num(num:&Vec<i32>) -> i32 {
            num.iter().fold(0,|acc,x| {
                acc * 2 + x
            })
        }
        leafs.iter().map(|v| bin_to_num(v)).sum::<i32>()
    }
}
