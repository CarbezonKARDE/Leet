use std::collections::VecDeque;
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
        let N = grid.len();
        let M = grid[0].len();
        let start_dir = grid[0][0] as usize - 1;
        let mut dist = vec![vec![[i32::MAX; 4]; M]; N];
        dist[0][0][start_dir] = 0;
        let mut q = VecDeque::new();
        q.push_back((0, 0, start_dir));
        while let Some((i, j, cur_dir)) = q.pop_front() {
            for (to_i, to_j, to_dir, dir_on_to) in
                DIRS.iter().enumerate().filter_map(|(to_dir, (di, dj))| {
                    let to_i = (i as i32 + di) as usize;
                    let to_j = (j as i32 + dj) as usize;
                    let dir_on_to = *grid.get(to_i)?.get(to_j)?;
                    Some((to_i, to_j, to_dir, dir_on_to as usize - 1))
                })
            {
                if cur_dir == to_dir {
                    if dist[to_i][to_j][dir_on_to] > dist[i][j][cur_dir] {
                        dist[to_i][to_j][dir_on_to] = dist[i][j][cur_dir];
                        q.push_front((to_i, to_j, dir_on_to));
                    }
                } else {
                    if dist[to_i][to_j][dir_on_to] > dist[i][j][cur_dir] + 1 {
                        dist[to_i][to_j][dir_on_to] = dist[i][j][cur_dir] + 1;
                        q.push_back((to_i, to_j, dir_on_to));
                    }
                }
            }
        }
        *dist[N - 1][M - 1].iter().min().unwrap()
    }
}
