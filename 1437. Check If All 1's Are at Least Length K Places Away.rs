impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let n=nums.len();
        let mut last=-200 as i32;
        for i in 0..n{
            if nums[i]==1{
                if (i as i32)-last-1<k{ return false;}
                last=(i as i32);
            }
        }
        true
    }
}
