use std::mem::swap;
impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = events.len();
        let k = k as usize;
        events.sort_unstable_by_key(|x| x[1]);
        let mut dp0 = vec![i32::MIN; n + 1];
        let mut dp1 = vec![i32::MIN; n + 1];
        let mut prevs = vec![0usize; n];
        for i in 0..n {
            let v = dp1[i].max(events[i][2]);
            dp1[i + 1] = v;
            prevs[i] = events[..i].partition_point(|x| x[1] < events[i][0]);
        }
        for i in 1..k {
            for j in i..n {
                let v = dp1[j + 1]                      
                    .max(dp1[prevs[j]] + events[j][2])  
                    .max(dp0[j]);                       
                dp0[j + 1] = v;
            }
            swap(&mut dp0, &mut dp1);
        }
        dp1[n]
    }
}
