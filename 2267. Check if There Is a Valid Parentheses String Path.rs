impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let length = m + n - 1;
        if length % 2 == 1
            || grid[0][0] != '('
            || grid[m - 1][n - 1] != ')'
        {
            return false;
        }
        let mut memo = vec![vec![vec![0u8; length + 1]; n]; m];
        Self::dfs(&grid, &mut memo, 0, 0, 0, m, n)
    }
    fn dfs(
        grid: &Vec<Vec<char>>,
        memo: &mut Vec<Vec<Vec<u8>>>,
        row: usize,
        col: usize,
        balance: usize,
        m: usize,
        n: usize,
    ) -> bool {
        let balance = if grid[row][col] == '(' {
            balance + 1
        } else {
            if balance == 0 {
                return false;
            }
            balance - 1
        };
        let remaining = (m - 1 - row) + (n - 1 - col);
        if balance > remaining {
            return false;
        }
        if row == m - 1 && col == n - 1 {
            return balance == 0;
        }
        if memo[row][col][balance] != 0 {
            return memo[row][col][balance] == 2;
        }
        let possible =
            (row + 1 < m
                && Self::dfs(grid, memo, row + 1, col, balance, m, n))
            || (col + 1 < n
                && Self::dfs(grid, memo, row, col + 1, balance, m, n));
        memo[row][col][balance] = if possible { 2 } else { 1 };
        possible
    }
}
