impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max_value = 0;
        let n = nums.len();
        let mut prefix_max = vec![0; n];
        prefix_max[0] = nums[0] as i64;
        for i in 1..n {
            prefix_max[i] = prefix_max[i - 1].max(nums[i] as i64);
        }
        let mut max_result = 0;
        for j in 1..n - 1 {
            let max_i = prefix_max[j - 1] - nums[j] as i64;
            let max_k = *nums[j + 1..].iter().max().unwrap() as i64;
            max_result = max_result.max(max_i * max_k);
        }
        max_result
    }
}
