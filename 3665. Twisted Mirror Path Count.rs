impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = grid[0].len();
        let mut dp = vec![0; n];
        dp[0] = 1;

        for row in grid {
            let mut left = 0;
            for (cell, top) in row.into_iter().zip(dp.iter_mut()) {
                if cell == 1 {
                    std::mem::swap(&mut left, top);
                } else {
                    let mut total = left + *top;
                    if total >= MOD {
                        total -= MOD;
                    }
                    left = total;
                    *top = total;
                }
            }
        }

        dp[n - 1]
    }
}
