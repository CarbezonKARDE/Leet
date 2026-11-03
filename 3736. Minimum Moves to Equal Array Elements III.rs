impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut maximum = nums[0];
        let mut sum = 0;
        for &num in &nums {
            maximum = maximum.max(num);
            sum += num;
        }
        maximum * nums.len() as i32 - sum
    }
}
