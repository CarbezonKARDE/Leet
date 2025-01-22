use std::collections::VecDeque;
impl Solution {
    pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();
        let mut heights = vec![vec![-1; n]; m];
        let mut dq = VecDeque::new();        
        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    heights[i][j] = 0; 
                    dq.push_back((i, j));
                }
            }
        }
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        while let Some((x, y)) = dq.pop_front() {
            for (dx, dy) in directions.iter() {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < m && ny < n && heights[nx][ny] == -1 {
                    heights[nx][ny] = heights[x][y] + 1;
                    dq.push_back((nx, ny));
                }
            }
        }
        heights
    }
}
