impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len();
        let d = d as usize;
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by_key(|&i| arr[i]);
        let mut dp = vec![1i32; n];
        for i in order {
            for j in (i.saturating_sub(d)..i).rev() {
                if arr[j] >= arr[i] { break; }
                dp[i] = dp[i].max(dp[j] + 1);
            }
            for j in i+1..=(i+d).min(n-1) {
                if arr[j] >= arr[i] { break; }
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        *dp.iter().max().unwrap()
    }
}
