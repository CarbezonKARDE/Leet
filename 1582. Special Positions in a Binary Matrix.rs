impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let [rones, cones] = (0..mat.len())
            .flat_map(|y| (0..mat[0].len()).map(move |x| (y, x)))
            .filter(|&(y, x)| mat[y][x] == 1)
            .fold(
                [vec![0; mat.len()], vec![0; mat[0].len()]],
                |mut ones, (y, x)| {
                    ones[0][y] += 1;
                    ones[1][x] += 1;
                    ones
                },
            );
        mat.into_iter()
            .zip(rones)
            .filter_map(|(r, rone)| (rone == 1).then_some(r))
            .flat_map(|r| r.into_iter().zip(&cones))
            .fold(0, |acc, (v, c)| acc + i32::from(v == 1 && *c == 1))
    }
}
