impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                ans = ans.max(Solution::dfs(&grid, i as i32, j as i32));
            }
        }
        ans
    }
    fn dfs(grid: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        let n = grid.len() as i32;
        let m = grid[0].len() as i32;
        if i < 0 || j < 0 || i == n || j == m {
            return 0;
        }
        if grid[i as usize][j as usize] == 0 {
            return 0;
        }
        let gold = grid[i as usize][j as usize];
        let mut grid = grid.clone();
        grid[i as usize][j as usize] = 0;
        let max_path = vec![
            Solution::dfs(&grid, i + 1, j),
            Solution::dfs(&grid, i - 1, j),
            Solution::dfs(&grid, i, j + 1),
            Solution::dfs(&grid, i, j - 1),
        ]
        .into_iter()
        .max()
        .unwrap_or(0);
        grid[i as usize][j as usize] = gold;
        gold + max_path
    }
}
