impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .scan([false; 101], |f, num| {
                Some(match std::mem::replace(&mut f[num as usize], true) {
                    true => Some(num),
                    false => None,
                })
            })
            .flatten()
            .collect()
    }
}
