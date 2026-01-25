impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans: i32 = i32::MAX;
        let bound: usize = nums.len() - (k as usize) + 1;
        for i in 0..bound {
            ans = ans.min(nums[i+(k as usize)-1] - nums[i]);
        }
        ans
    }
}
