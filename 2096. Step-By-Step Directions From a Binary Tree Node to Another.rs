use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        fn dfs(
            root: &Rc<RefCell<TreeNode>>, 
            start_val: i32,
            dest_val: i32,
            dir: &mut Vec<u8>,
            start_dir: &mut Vec<u8>,
            dest_dir: &mut Vec<u8>) 
        {
            if !start_dir.is_empty() && !dest_dir.is_empty() {
                return;
            }
            if root.borrow().val == start_val {
                *start_dir = dir.clone();
            }
            if root.borrow().val == dest_val {
                *dest_dir = dir.clone();
            }
            if let Some(left) = &root.borrow().left {
                dir.push(b'L');
                dfs(left, start_val, dest_val, dir, start_dir, dest_dir);
                dir.pop();
            }
            if let Some(right) = &root.borrow().right {
                dir.push(b'R');
                dfs(right, start_val, dest_val, dir, start_dir, dest_dir);
                dir.pop();
            }
        }
        if let Some(new_root) = &root {
            let (mut start_dir, mut dest_dir) = (Vec::new(), Vec::new());
            let mut dir = Vec::new();
            dfs(
                new_root,
                start_value,
                dest_value,
                &mut dir,
                &mut start_dir,
                &mut dest_dir
            );
            let mut res = String::new();
            let mut idx = 0;
            while idx < start_dir.len().min(dest_dir.len()) 
                && start_dir[idx] == dest_dir[idx] {
                    idx += 1;
                }
            for _ in idx..start_dir.len() {
                res.push('U');
            }
            for new_idx in idx..dest_dir.len() {
                res.push(dest_dir[new_idx] as char);
            }
            return res;
        }
        String::new()
    }
}
