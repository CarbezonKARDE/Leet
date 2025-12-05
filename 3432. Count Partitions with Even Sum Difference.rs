impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 0 {
            (nums.len() as i32) - 1
        } else {
            0
        }
    }
}
