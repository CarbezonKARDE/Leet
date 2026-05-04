pub struct RotationContext<'a, F>
where F: Fn(usize, usize) -> (usize, usize),
{
    matrix: &'a mut [Vec<i32>],
    transform: F,
    start_row: usize,
    start_col: usize,
}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let transform = |r, c| (c, n - r - 1);
        for start_row in 0..n / 2 {
            for start_col in 0..(n + 1) / 2 {
                RotationContext {matrix, transform, start_row, start_col}
                    .do_rotations()
            }
        }
    }
}

impl<'a, F> RotationContext<'a, F>
where
    F: Fn(usize, usize) -> (usize, usize),
{
    fn do_rotations(&mut self) {
        let val = self.matrix[self.start_row][self.start_col];
        self.rotate(self.start_row, self.start_col, val)
    }
    pub fn rotate(&mut self, r: usize, c: usize, mut val: i32) {
        let (next_r, next_c) = (self.transform)(r, c);
        std::mem::swap(&mut self.matrix[next_r][next_c], &mut val);
        if (next_r, next_c) != (self.start_row, self.start_col) {
            self.rotate(next_r, next_c, val)
        }
    }
}
