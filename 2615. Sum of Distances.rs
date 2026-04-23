use std::collections::HashMap;
impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut idxs = HashMap::<i32, Vec<i64>>::new();
        for (i, &x) in nums.iter().enumerate() {
            idxs.entry(x).or_default().push(i as i64);
        }
        let n = nums.len(); 
        let mut res=vec![0; n];
        for g in idxs.values() {
            let sum:i64 = g.iter().sum();
            let l = g.iter().len() as i64;
            let mut psum = 0;
            for (i, &idx) in g.iter().enumerate() {
                let left = i as i64*idx - psum;
                let right = (sum-psum-idx)-(l-i as i64-1)*idx;
                res[idx as usize]=left+right;
                psum+=idx;
            }
        }
        res
    }
}
