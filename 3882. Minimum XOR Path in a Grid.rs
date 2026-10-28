impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 1024];n];m];
        dp[0][0][grid[0][0] as usize] = true;
        for i in 1..m
        {
            let mut number = 0 as usize; 
            for j in 0..1024 
            {
                if dp[i-1][0][j]
                {
                    number = j; 
                }
            }
            dp[i][0][grid[i][0] as usize ^ number] = true;
        }
        for i in 1..n
        {
            let mut number = 0 as usize; 
            for j in 0..1024 
            {
                if dp[0][i-1][j]
                {
                    number = j; 
                }
            }
            dp[0][i][grid[0][i] as usize ^ number] = true;
        }
        for i in 1..m
        {
            for j in 1..n
            {
                for no in 0..1024
                {
                    if dp[i-1][j][no] | dp[i][j-1][no]
                    {
                        dp[i][j][no ^ grid[i][j] as usize] = true;
                    }
                }
            }
        }
        for i in 0..1024
        {
            if dp[m-1][n-1][i]
            {
                return i as i32;
            }
        }
        return 67;
    }
}
