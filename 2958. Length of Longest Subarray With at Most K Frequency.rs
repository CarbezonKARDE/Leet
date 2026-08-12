use std::collections::HashMap;
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        let mut left = 0usize;
        let mut ans = 0;
        for right in 0..nums.len() {
            *freq.entry(nums[right]).or_insert(0) += 1;
            while *freq.get(&nums[right]).unwrap() > k {
                *freq.get_mut(&nums[left]).unwrap() -= 1;
                left += 1;
            }
            ans = ans.max((right - left + 1) as i32);
        }
        ans
    }
}
