impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut points: Vec<(u8, u8)> = (0..m).flat_map(|i| 
            (0..n).map(move |j| (i as u8, j as u8))
        ).collect();
        points.sort_unstable_by_key(|&(i, j)| grid[i as usize][j as usize]);
        let mut costs = vec![i32::MAX; m * n];
        for _ in 0..=k {
            let mut min_cost = i32::MAX;
            let mut start = 0;
            for end in 0..points.len() {
                let (i, j) = (points[end].0 as usize, points[end].1 as usize);
                min_cost = min_cost.min(costs[i * n + j]);
                if end + 1 >= points.len() || 
                   grid[points[end].0 as usize][points[end].1 as usize] != 
                   grid[points[end + 1].0 as usize][points[end + 1].1 as usize] {
                    for idx in start..=end {
                        let (pi, pj) = (points[idx].0 as usize, points[idx].1 as usize);
                        costs[pi * n + pj] = min_cost;
                    }
                    start = end + 1;
                }
            }
            costs[(m - 1) * n + (n - 1)] = 0;
            for i in (0..m).rev() {
                for j in (0..n).rev() {
                    if i == m - 1 && j == n - 1 {
                        continue;
                    }
                    let idx = i * n + j;
                    let mut best = costs[idx];
                    if i + 1 < m {
                        best = best.min(costs[(i + 1) * n + j] + grid[i + 1][j]);
                    }
                    if j + 1 < n {
                        best = best.min(costs[i * n + (j + 1)] + grid[i][j + 1]);
                    }
                    costs[idx] = best;
                }
            }
        }
        costs[0]
    }
}
