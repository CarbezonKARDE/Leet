impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let uvalue = value as usize;
        let mut multiset = vec![0; uvalue];
        for &n in &nums {
            multiset[n.rem_euclid(value) as usize] += 1;
        }
        for n in 0..=nums.len() {
            if multiset[n % uvalue] == 0 {
                return n as i32;
            }
            multiset[n % uvalue] -= 1;
        }
        0
    }
}
