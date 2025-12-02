impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        use std::collections::HashMap;
        let mut mp = HashMap::new();
        for p in &points {
            *mp.entry(p[1]).or_insert(0i64) += 1;
        }
        let seg: Vec<i64> = mp.values().filter(|&&v| v >= 2).map(|&v| v * (v-1)/2 % MOD).collect();
        let mut sum = 0i64;
        let mut ans = 0i64;
        for &v in &seg {
            ans = (ans + v * sum) % MOD;
            sum = (sum + v) % MOD;
        }
        ans as i32
    }
}
