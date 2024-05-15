use std::collections::VecDeque;
impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let in_bounds = |r: usize, c: usize| { r < n && c < n };
        let mut queue = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    queue.push_back((i, j, 1));
                    grid[i][j] = 0;
                } else {
                    grid[i][j] = -1;
                }
            }
        }
        while let Some((i, j, safety)) = queue.pop_front() {
            let expand = [(i+1, j), (i-1, j), (i, j+1), (i, j-1)];
            for (r, c) in expand {
                if in_bounds(r, c) && grid[r][c] == -1 {
                    grid[r][c] = safety;
                    queue.push_back((r, c, safety + 1));
                }
            }
        }
        let mut min_safety = grid[0][0];
        queue.push_back((0, 0, grid[0][0]));
        while let Some((i, j, safety)) = queue.pop_front() {
            min_safety = i32::min(min_safety, safety);
            if (i, j) == (n-1, n-1) { break };
            let expand = [(i+1, j), (i-1, j), (i, j+1), (i, j-1)];
            for (r, c) in expand {
                if in_bounds(r, c) && grid[r][c] != -1 {
                    let safety = grid[r][c];
                    grid[r][c] = -1;
                    if safety < min_safety { queue.push_back((r, c, safety)); }
                    else { queue.push_front((r, c, safety)); }
                }
            }
        }
        min_safety
    }
}
