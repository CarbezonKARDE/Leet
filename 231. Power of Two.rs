impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let base: i32 = 2;
        for i in 0..31 {
                if n == base.pow(i) {
                        return true;
                }
        }
        return false;
    }
}
