impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut lo = 10_000;
        let mut hi = -10_000;
        let mut ans = 0;
        for (x, y) in arrays.iter().map(|v| (v[0], v[v.len() - 1])) {
            ans = ans.max(hi - x).max(y - lo);
            lo = lo.min(x);
            hi = hi.max(y);
        }
        ans
    }
}
