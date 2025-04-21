impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        (upper - lower + 1 - match differences.into_iter()
            .try_fold((0, 0, 0), |(mmin, mmax, mut sum), e| {
                if mmax - mmin > upper - lower {
                    Err("IMPOSSIBLE")
                } else {
                    sum += e;
                    Ok((mmin.min(sum), mmax.max(sum), sum))
                }
            }) {
                Err(e) => i32::MIN,
                Ok((mmin, mmax, _)) => mmax - mmin,
            }
        ).max(0)
    }
}
