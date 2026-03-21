impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize; let y = y as usize; let k = k as usize;
        for j in 0..k/2 {
            let (upper, lower) = grid.split_at_mut(x+k-j-1);
            upper[x+j][y..y+k].swap_with_slice(&mut lower[0][y..y+k]);
        }
        return grid;
    }
}
