impl Solution {
    pub fn shift_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let copy: Vec<_> = grid.iter().flat_map(|r| r.iter()).copied().collect();
        for (l, &r) in grid
            .iter_mut()
            .flat_map(|r| r.iter_mut())
            .zip(copy.iter().cycle().skip(copy.len() - k as usize % copy.len()))
        {
            *l = r;
        }
        grid
    }
}
