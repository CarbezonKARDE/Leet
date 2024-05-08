impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let n = nums.len();
        let mut ans = vec!["".to_string(); n];
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_unstable_by(|&a, &b| nums[b].cmp(&nums[a]));
        for (i, &index) in indices.iter().enumerate() {
            ans[index] = match i {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (i + 1).to_string(),
            };
        }
        ans
    }
}
