use std::collections::VecDeque;
impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let directions = vec![
            (-1, 0), (1, 0), (0, -1), (0, 1),
        ];
        let mut dist = vec![vec![i32::MAX; cols]; rows];
        let mut front = VecDeque::new();
        let mut back = VecDeque::new();
        dist[0][0] = 0;
        front.push_back((0, 0));
        while !front.is_empty() || !back.is_empty() {
            if front.is_empty() {
                std::mem::swap(&mut front, &mut back);
            }
            let (row, col) = front.pop_front().unwrap();
            for (dr, dc) in &directions {
                let new_row = row + dr;
                let new_col = col + dc;
                if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    let new_dist = dist[row as usize][col as usize] + grid[new_row][new_col];
                    if new_dist < dist[new_row][new_col] {
                        dist[new_row][new_col] = new_dist;
                        if grid[new_row][new_col] == 0 {
                            front.push_back((new_row as i32, new_col as i32));
                        } else {
                            back.push_back((new_row as i32, new_col as i32));
                        }
                    }
                }
            }
        }
        dist[rows - 1][cols - 1]
    }
}
