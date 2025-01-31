use std::collections::HashSet;
impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut sizes = vec![0; n * n + 2];
        let mut id = 2;
        let mut max_size = 1;
        const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let size = Self::mark_island(i, j, id, &mut grid);
                    sizes[id] = size;
                    max_size = max_size.max(size);
                    id += 1;
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    let mut seen = HashSet::new();
                    let mut size = 1;
                    for &(di, dj) in &DIRS {
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;
                        if ni >= 0 && ni < n as i32 && nj >= 0 && nj < n as i32 {
                            let nid = grid[ni as usize][nj as usize];
                            if nid > 1 && seen.insert(nid) {
                                size += sizes[nid as usize];
                            }
                        }
                    }
                    max_size = max_size.max(size);
                }
            }
        }
        max_size
    }
    fn mark_island(i: usize, j: usize, id: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] != 1 {
            return 0;
        }
        grid[i][j] = id as i32;
        1 + Self::mark_island(i + 1, j, id, grid) +
            Self::mark_island(i.wrapping_sub(1), j, id, grid) +
            Self::mark_island(i, j + 1, id, grid) +
            Self::mark_island(i, j.wrapping_sub(1), id, grid)
    }
}
