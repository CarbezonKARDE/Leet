impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut path = Vec::new();
        Self::dfs(&nums, 0, &mut path, &mut ans);
        ans
    }
    fn dfs(nums: &Vec<i32>, s: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        ans.push(path.clone());

        for i in s..nums.len() {
            path.push(nums[i]);
            Self::dfs(nums, i + 1, path, ans);
            path.pop();
        }
    }
}
