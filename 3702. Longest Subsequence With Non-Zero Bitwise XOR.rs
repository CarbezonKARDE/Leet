impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        if nums.iter().all(|&n| n == 0) { return 0; }
        nums.len() as i32 - (nums.iter().fold(0, |r, &n| r ^ n) == 0) as i32
    }
}
