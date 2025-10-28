impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut count = 0;
        let mut left = 0;
        let mut right: i32 = nums.iter().sum();
        for i in 0..length {
            left += nums[i];
            right -= nums[i];            
            if nums[i] != 0 { continue; }            
            if left == right { count += 2; }            
            if (left - right).abs() == 1 { count += 1; }
        }
        count
    }
}
