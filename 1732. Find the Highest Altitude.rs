impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.into_iter()
            .chain(std::iter::once(0))
            .scan(0, |c, g| Some(std::mem::replace(c, *c + g)))
            .fold(0, Ord::max)
    }
}
