impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut ans = vec![vec![0; n - 2]; n - 2];
        for i in 0..n - 2 {
            for j in 0..n - 2 {
                for x in i..i + 3 {
                    for y in j..j + 3 {
                        ans[i][j] = ans[i][j].max(grid[x][y]);
                    }
                }
            }
        }
        ans
    }
}
