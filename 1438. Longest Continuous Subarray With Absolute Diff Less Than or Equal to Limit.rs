use std::collections::VecDeque;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut increase = VecDeque::new();
        let mut decrease = VecDeque::new();
        let mut max_len = 0;
        let mut left = 0;
        for right in 0..nums.len() {
            while let Some(&back) = increase.back() {
                if nums[right] < back {
                    increase.pop_back();
                } else {
                    break;
                }
            }
            increase.push_back(nums[right]);
            while let Some(&back) = decrease.back() {
                if nums[right] > back {
                    decrease.pop_back();
                } else {
                    break;
                }
            }
            decrease.push_back(nums[right]);
            while decrease.front().unwrap() - increase.front().unwrap() > limit {
                if nums[left] == *decrease.front().unwrap() {
                    decrease.pop_front();
                }
                if nums[left] == *increase.front().unwrap() {
                    increase.pop_front();
                }
                left += 1;
            }
            max_len = max_len.max(right as i32 - left as i32 + 1);
        }
        max_len
    }
}
