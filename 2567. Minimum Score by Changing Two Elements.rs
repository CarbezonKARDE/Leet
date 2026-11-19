impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        let mut biggest0 = 0;
        let mut biggest1 = 0;
        let mut biggest2 = 0;
        let mut smallest0 = i32::MAX;
        let mut smallest1 = i32::MAX;
        let mut smallest2 = i32::MAX;
        nums.iter().for_each(|&x| {
            if x < smallest0 {
                smallest2 = smallest1;
                smallest1 = smallest0;
                smallest0 = x;
            } else if x < smallest1 {
                smallest2 = smallest1;
                smallest1 = x;
            } else if x < smallest2 {
                smallest2 = x;
            }
            if x > biggest0 {
                biggest2 = biggest1;
                biggest1 = biggest0;
                biggest0 = x;
            } else if x > biggest1 {
                biggest2 = biggest1;
                biggest1 = x;
            } else if x > biggest2 {
                biggest2 = x;
            }
        });
        (biggest0 - smallest2)
            .min(biggest2 - smallest0)
            .min(biggest1 - smallest1)
    }
}
