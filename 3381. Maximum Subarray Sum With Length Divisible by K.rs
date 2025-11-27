impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n=nums.len();
        const INF: i64=i64::MAX/2;
        let K=k as usize;
        let mut minS: Vec<i64>=vec![INF; K];
        minS[K-1]=0;
        let mut prefix=0 as i64;
        let mut ans=-INF;
        for i in 0..n{
            prefix+=nums[i] as i64;
            let ik=i%K;
            ans=ans.max(prefix-minS[ik]);
            minS[ik]=minS[ik].min(prefix);
        }
        ans
    }
}
