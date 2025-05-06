impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|&num| nums[num as usize]).collect()
    }
}
