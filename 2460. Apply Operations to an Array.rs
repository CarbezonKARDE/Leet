impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let mut write = 0;
        let n = nums.len();
        let mut read = 0;
        while read < n {
            if nums[read] != 0 {
                if read + 1 < n && nums[read] == nums[read + 1] {
                    nums[write] = 2 * nums[read];
                    read += 1;
                } else {
                    nums[write] = nums[read];
                }
                write += 1;
            }
            read += 1;
        }
        nums[write..].fill(0);
        nums
    }
}
