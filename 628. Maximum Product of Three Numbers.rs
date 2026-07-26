impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut max_mul = nums[n - 1] * nums[n - 2] * nums[n - 3];
        if nums[0] < 0 && nums[1] < 0 {
            max_mul = max_mul.max(nums[0] * nums[1] * nums[n - 1]);
        }
        max_mul
    }
}
