use std::collections::HashMap;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();
        let mut idx = -1;
        let n = nums.len();
        for i in (0..n).rev() {
            let count = mp.entry(nums[i]).or_insert(0);
            *count += 1;
            if *count > 1 {
                idx = i as i32;
                break;
            }
        }
        if idx == -1 {
            return 0;
        }
        (idx + 3) / 3
    }
}
