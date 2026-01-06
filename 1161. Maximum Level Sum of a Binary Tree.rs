use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(root.unwrap());
        let mut level = 1;
        let mut ans = 1;
        let mut max_sum: i64 = i64::MIN;
        while !q.is_empty() {
            let size = q.len();
            let mut sum: i64 = 0;
            for _ in 0..size {
                let node = q.pop_front().unwrap();
                let n = node.borrow();
                sum += n.val as i64;
                if let Some(left) = n.left.clone() {
                    q.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    q.push_back(right);
                }
            }
            if sum > max_sum {
                max_sum = sum;
                ans = level;
            }
            level += 1;
        }
        ans
    }
}
