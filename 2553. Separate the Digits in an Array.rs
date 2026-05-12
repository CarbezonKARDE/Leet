use std::collections::VecDeque;
impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let length : usize = nums.len();
        let mut answer : VecDeque<i32> = VecDeque::with_capacity(nums.len());
        for i in (0..length).rev() {
            let mut num : i32 = nums[i];
            while num > 0 {
                answer.push_front(num % 10);
                num /= 10;
            }

        }
        return Vec::from(answer);

    }
}
