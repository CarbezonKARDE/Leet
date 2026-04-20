impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let last = colors.last().unwrap();
        let first = colors.first().unwrap();
        let lpos = colors.iter().position(|x| x != last).unwrap();
        let rpos = colors.iter().rposition(|x| x != first).unwrap();
        rpos.max(colors.len() - 1 - lpos) as i32
    }
}
