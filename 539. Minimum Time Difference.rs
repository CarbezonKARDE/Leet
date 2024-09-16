impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes = time_points
            .into_iter()
            .map(|time| Self::to_minutes(time.as_str()))
            .collect::<Vec<_>>();
        minutes.sort_unstable();
        let res = minutes.first().unwrap() + 24 * 60 - minutes.last().unwrap();
        minutes
            .windows(2)
            .fold(res, |res, pair| {
                res.min(pair[1] - pair[0])
            })
    }
    #[inline]
    fn to_minutes(time: &str) -> i32 {
        let mut it = time.split(":");
        it.next().unwrap().parse::<i32>().unwrap() * 60
            + it.next().unwrap().parse::<i32>().unwrap()
    }
}
