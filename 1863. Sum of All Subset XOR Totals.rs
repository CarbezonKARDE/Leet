impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        Self::dfs(&nums, 0, 0)
    }
    fn dfs(nums: &Vec<i32>, i: usize, xors: i32) -> i32 {
        if i == nums.len() {
            return xors;
        }
        let x = Self::dfs(nums, i + 1, xors);
        let y = Self::dfs(nums, i + 1, nums[i] ^ xors);
        x + y
    }
}
