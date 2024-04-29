use std::cmp::{max, min};
impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp = vec![0; 26];
        for c in s.chars() {
            let i = (c as u8 - b'a') as usize;
            dp[i] = 1 + Self::get_max_reachable(&dp, i, k);
        }
        *dp.iter().max().unwrap_or(&0)
    }
    fn get_max_reachable(dp: &Vec<i32>, i: usize, k: i32) -> i32 {
        let first = max(0, i as i32 - k) as usize;
        let last = min(25, (i as i32 + k) as usize);
        (first..=last).map(|j| dp[j]).max().unwrap_or(0)
    }
}
