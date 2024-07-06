impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let cycle_time = (n - 1) * 2;
        let time = time % cycle_time;
        if time < n {
            return 1 + time;
        }
        return n - (time - (n - 1));
    }
}
