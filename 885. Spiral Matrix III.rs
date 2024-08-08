impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut result = vec![vec![r_start, c_start]];
        let (mut steps, mut d) = (0, 0);
        let (mut r, mut c) = (r_start, c_start);
        while result.len() < (rows * cols) as usize {
            if d % 2 == 0 {
                steps += 1;
            }
            for _ in 0..steps {
                r += directions[d].0;
                c += directions[d].1;
                if r >= 0 && r < rows && c >= 0 && c < cols {
                    result.push(vec![r, c]);
                }
                if result.len() == (rows * cols) as usize {
                    return result;
                }
            }
            d = (d + 1) % 4;
        }
        result
    }
}
