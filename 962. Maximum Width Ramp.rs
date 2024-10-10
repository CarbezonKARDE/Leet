impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let n = nums.len();
        for i in 0..n {
            if stack.is_empty() || nums[i] < nums[*stack.last().unwrap()] {
                stack.push(i);
            }
        }
        let mut max_width = 0;
        for j in (0..n).rev() {
            while let Some(&i) = stack.last() {
                if nums[j] >= nums[i] {
                    max_width = max_width.max(j as i32 - i as i32);
                    stack.pop();
                } else {
                    break;
                }
            }
        }
        max_width
    }
}
