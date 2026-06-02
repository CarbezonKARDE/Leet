impl Solution {
    pub fn earliest_finish_time(lst: Vec<i32>, ld: Vec<i32>, wst: Vec<i32>, wd: Vec<i32>) -> i32 {
        let le = lst
            .iter()
            .zip(&ld)
            .map(|(s, d)| s + d)
            .fold(i32::MAX, Ord::min);
        let (lf, we) = wst
            .into_iter()
            .zip(wd)
            .fold((i32::MAX, i32::MAX), |(lf, we), (s, d)| {
                (lf.min(le.max(s) + d), we.min(s + d))
            });
        lst.into_iter()
            .zip(ld)
            .map(|(s, d)| s.max(we) + d)
            .fold(i32::MAX, Ord::min)
            .min(lf)
    }
}
