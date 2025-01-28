impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(v: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            if i>=v.len() || j>=v[0].len() || v[i][j] == 0 {return 0;}
            let x = v[i][j];
            v[i][j] = 0;
            x + dfs(v, i+1, j) + dfs(v, i, j+1)
            + dfs(v, i.saturating_sub(1), j)
            + dfs(v, i, j.saturating_sub(1))
        }
        (0..grid.len())
        .map(|i| {
            (0..grid[0].len())
            .map(|j| {
                dfs(&mut grid, i, j)
            })
            .max()
            .unwrap()    
        })
        .max()
        .unwrap()
    }
}
