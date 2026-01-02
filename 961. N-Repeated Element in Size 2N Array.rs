impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        if nums[0] == nums[nums.len() - 2] || nums[0] == nums[nums.len() - 1] {
            return nums[0];
        }
        if  nums[1] == nums[nums.len() - 1] || nums[1] == nums[nums.len() - 2] {
            return nums[1];
        }
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                return nums[i];
            }
        }
        return 0;
    }
}
