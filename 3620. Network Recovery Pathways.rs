use std::collections::VecDeque;
impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let n = online.len();
        let mut g: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];
        let mut deg = vec![0; n];
        let mut l = i32::MAX;
        let mut r = 0;
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            if !online[u] || !online[v] {
                continue;
            }
            g[u].push((v, w));
            deg[v] += 1;
            l = l.min(w);
            r = r.max(w);
        }
        let mut q = VecDeque::new();
        for i in 1..n {
            if deg[i] == 0 {
                q.push_back(i);
            }
        }
        while let Some(u) = q.pop_front() {
            for &(v, _) in &g[u] {
                deg[v] -= 1;
                if deg[v] == 0 && v != 0 {
                    q.push_back(v);
                }
            }
        }
        if !Self::check(l, k, &g, &deg, n) {
            return -1;
        }
        while l <= r {
            let mid = (l + r) >> 1;
            if Self::check(mid, k, &g, &deg, n) {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        r
    }
    fn check(mid: i32, k: i64, g: &[Vec<(usize, i32)>], deg: &[i32], n: usize) -> bool {
        let mut dp = vec![i64::MAX / 2; n];
        let mut cdeg = deg.to_vec();
        let mut q = VecDeque::new();
        dp[0] = 0;
        q.push_back(0);
        while let Some(u) = q.pop_front() {
            if u == n - 1 {
                return dp[u] <= k;
            }
            for &(v, w) in &g[u] {
                if w >= mid {
                    dp[v] = dp[v].min(dp[u].saturating_add(w as i64));
                }
                cdeg[v] -= 1;
                if cdeg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        false
    }
}
