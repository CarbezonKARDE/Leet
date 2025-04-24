impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        const MAX: usize = 2000;
        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let cnt_all = {
            let mut have = vec![0; MAX + 1];
            let mut cnt = 0;
            for &num in nums.iter() {
                if have[num] == 0 {
                    cnt += 1;
                }
                have[num] += 1;
            }
            cnt
        };
        let mut have = vec![0; MAX + 1];
        let mut cnt = 0;
        let mut ans = 0;
        let mut l = 0;
        for r in 0..nums.len() {
            let num = nums[r];
            if have[num] == 0 {
                cnt += 1;
            }
            have[num] += 1;
            while l < r && have[nums[l]] != 1 {
                have[nums[l]] -= 1;
                l += 1;
            }
            if cnt == cnt_all {
                ans += l + 1;
            }
        }
        ans as i32
    }
}
