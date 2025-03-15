impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = i32::MAX;
        let mut right = i32::MIN;
        for num in nums.iter() {
            if *num < left {
                left = *num;
            }

            if *num > right {
                right = *num;
            }
        }
        let size = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            let mut count = 0;
            let mut index = 0;
            while index < size {
                if nums[index] <= mid {
                    count += 1;
                    index += 2;
                } else {
                    index += 1;
                }
            }
            if count >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
