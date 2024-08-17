impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>) -> i64 {
        let width = grid[0].len();
        let mut current = vec![0i64; width];
        let mut previous = vec![0i64; width];
        for level in grid.iter() {
            let mut peak = 0i64;
            for i in 0..width {
                peak = peak.saturating_sub(1).max(previous[i]);
                current[i] = peak;
            }
            peak = 0;
            for i in (0..width).rev() {
                peak = peak.saturating_sub(1).max(previous[i]);
                current[i] = current[i].max(peak) + level[i] as i64;
            }
            std::mem::swap(&mut previous, &mut current);
        }
        *previous.iter().max().unwrap()
    }
}
