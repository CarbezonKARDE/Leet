impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut prefix = vec![0; n];
        for i in 1..n {
            prefix[i] = prefix[i - 1];
            if (nums[i - 1] % 2 == 0 && nums[i] % 2 == 0) || (nums[i - 1] % 2 != 0 && nums[i] % 2 != 0) {
                prefix[i] += 1;
            }
        }
        let mut result = Vec::new();
        for q in queries {
            let left = q[0] as usize;
            let right = q[1] as usize;
            let special_count = prefix[right] - (if left > 0 { prefix[left] } else { 0 });
            result.push(special_count == 0);
        }
        result
    }
}
