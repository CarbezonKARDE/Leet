impl Solution {
    pub fn sort_diagonal(mut row: usize, mut col: usize, grid: &mut Vec<Vec<i32>>, rev: bool) {
        let (mut r, mut c, mut i, n) = (row, col, 0, grid.len());
        let mut diagonal = Vec::new();
        while r < n && c < n {
            diagonal.push(grid[r][c]);
            r += 1;
            c += 1;
        }
        if rev == true {
            diagonal.sort_by(|a, b| b.cmp(a));
        } else {
            diagonal.sort();
        }
        while row < n && col < n {
            grid[row][col] = diagonal[i];
            row += 1;
            col += 1;
            i += 1;
        }
    }
    pub fn sort_matrix(g: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = g.len();
        let mut grid = g;
        for col in 1..n {
            Self::sort_diagonal(0, col, &mut grid, false);
        }
        for row in 0..n {
            Self::sort_diagonal(row, 0, &mut grid, true);
        }
        grid
    }
}
