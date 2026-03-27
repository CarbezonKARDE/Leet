impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let rows = mat.len();
        let cols = mat[0].len();
        let k_effective = (k as usize) % cols;
        for i in 0..rows {
            for j in 0..cols {
                if mat[i][j] != mat[i][(j + k_effective) % cols] {
                    return false;
                }
            }
        }
        true
    }
}
