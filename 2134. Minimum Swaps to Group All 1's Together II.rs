impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let total_ones = nums.iter().sum::<i32>();
        if total_ones == 0 || total_ones as usize == n {
            return 0;
        }
        let mut current_ones = nums.iter().take(total_ones as usize).sum::<i32>();
        let mut max_ones = current_ones;
        for i in 0..n {
            current_ones -= nums[i];
            current_ones += nums[(i + total_ones as usize) % n];
            max_ones = max_ones.max(current_ones);
        }
        total_ones - max_ones
    }
}
