impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let dim = [mat.len(), mat[0].len()];
        let mut ans = Vec::<i32>::with_capacity(dim[0] * dim[1]);
        let v = [usize::MAX, 1];
        let mut dir = 0usize;
        let mut coord = [0usize, 0];
        for k in 0..dim[0] + dim[1] - 1 {
            coord[dir] = k.min(dim[dir] - 1);
            coord[1 - dir] = k - coord[dir];
            while coord[0] < dim[0] && coord[1] < dim[1] {
                ans.push(mat[coord[0]][coord[1]]);
                coord[0] = coord[0].wrapping_add(v[dir]);
                coord[1] = coord[1].wrapping_add(v[1 - dir]);
            }
            dir = 1 - dir;
        }
        ans
    }
}
