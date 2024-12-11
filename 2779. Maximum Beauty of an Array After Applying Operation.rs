impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut len = 0;
        let mut l = 0;
        for r in 0..nums.len() {
            if nums[r] - nums[l] <= 2 * k {
                len = len.max(r - l + 1);
            } else {
                l += 1;
            }
        }
        len as i32
    }
}
