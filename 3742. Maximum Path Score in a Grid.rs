impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let width = grid[0].len();
        let height = grid.len();
        let k = k as usize;
        let max_cost = (k + 1).min(width + height + 1); //?
        let mut dp = vec![vec![vec![-1; max_cost]; width]; height];
        dp[0][0][0] = 0;
        for row in 0..height {
            for col in 0..width {
                for cost in 0..max_cost {
                    if dp[row][col][cost] == -1 {
                        continue;
                    }
                    if row < height - 1 {
                        let new_cost = cost + if grid[row + 1][col] > 0 { 1 } else { 0 };
                        if new_cost < max_cost {
                            dp[row + 1][col][new_cost] = dp[row][col][cost] + grid[row + 1][col];
                        }
                    }
                    if col < width - 1 {
                        let new_cost = cost + if grid[row][col + 1] > 0 { 1 } else { 0 };
                        if new_cost < max_cost {
                            dp[row][col + 1][new_cost] =
                                dp[row][col + 1][new_cost].max(dp[row][col][cost] + grid[row][col + 1]);
                        }
                    }
                }
            }
        }
        *dp[height - 1][width - 1].iter().max().unwrap()
    }
}
