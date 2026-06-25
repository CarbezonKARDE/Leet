impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        (0..nums.len()).fold(0, |mut count, i| {
            (i..nums.len()).fold(0, |mut c, j| {
                if nums[j] == target {
                    c += 1;
                }
                if c > (j - i + 1) / 2 {
                    count += 1;
                }
                c
            });
            count
        })
    }
}
