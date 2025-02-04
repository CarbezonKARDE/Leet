use::std::cmp::max;
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut ans=nums[0];
        let mut curr=0; let mut dec=true;
        for i in 1..nums.len(){
            if nums[i]>nums[i-1] {
                if dec==true {
                    dec=false;
                    curr+=nums[i]+nums[i-1];
                }else {
                    curr+=nums[i];
                }
                ans=max(ans,curr);
            }else {
                dec=true; curr=0;
            }
        }
        ans 
    }
}
