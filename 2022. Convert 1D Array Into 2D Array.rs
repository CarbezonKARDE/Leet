impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        match original.len() as i32 == m * n {
            false => vec![],
            true => original
                .chunks(n as usize)
                .map(|v| v.to_vec())
                .collect::<Vec<_>>(),
        }
    }
}
