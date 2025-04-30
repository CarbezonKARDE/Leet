impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter().filter(|num| num.ilog10() & 1 == 1).count() as _
    }
}
