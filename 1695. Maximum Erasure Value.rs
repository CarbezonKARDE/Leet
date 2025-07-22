use std::collections::HashSet;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let (mut left, mut max, mut cur_sum) = (0, 0, 0);
        let mut seen = HashSet::new();
        for &num_right in &nums {
            if seen.insert(num_right) {
                cur_sum += num_right;
            } else {
                max = max.max(cur_sum);
                for &num_left in &nums[left..] {
                    left += 1;
                    if num_left == num_right {
                        break;
                    }
                    cur_sum -= num_left;
                    seen.remove(&num_left);
                }
            }
        }
        max.max(cur_sum)
    }
}
