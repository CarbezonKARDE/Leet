impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut dp = vec![(0, -1000000000, 0); k + 1];
        for p in prices {
            let p = i64::from(p);
            for i in (1..=k).rev() {
                let (mut a, mut b, mut c) = dp[i];
                b = b.max(a - p);
                c = c.max(a + p);
                let (_, d, e) = dp[i - 1];
                a = a.max(d + p).max(e - p);
                dp[i] = (a, b, c);
            }
            let (_, mut b, mut c) = dp[0];
            b = b.max(-p);
            c = c.max(p);
            dp[0] = (0, b, c);
        }
        dp[k].0
    }
}
