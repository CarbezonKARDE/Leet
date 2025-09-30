impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in (1..n).rev() {
            for j in 0..i {
                nums[j] = (nums[j] + nums[j + 1]) % 10;
            }
        }
        nums[0]
    }
}
