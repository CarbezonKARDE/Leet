impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let [top, right, bottom, left] = grid
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(j, cell)| (i, j, cell))
            })
            .filter(|&(_, _, cell)| cell == 1)
            .fold(
                [usize::MAX, usize::MIN, usize::MIN, usize::MAX],
                |[top, right, bottom, left], (i, j, _)| {
                    [top.min(i), right.max(j), bottom.max(i), left.min(j)]
                },
            );
        ((bottom - top + 1) * (right - left + 1)) as _
    }
}
