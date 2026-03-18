impl Solution {
    pub fn count_submatrices(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut result = 0;
        for row in 0..grid.len() {
            let mut sum = 0;
            for col in 0..grid[0].len() {
                sum += grid[row][col];
                grid[row][col] = sum + if row>0 {grid[row-1][col]} else {0};
                if grid[row][col] <=k {result+=1;}
            }
        }
        result
    }
}
