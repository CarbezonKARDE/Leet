impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;

        logs.sort_unstable_by_key(|log| log[1]);

        let mut ids: Vec<_> = (0..queries.len()).collect();
        ids.sort_unstable_by_key(|&i| queries[i]);

        let mut left = 0;
        let mut right = 0;

        let mut ret = vec![0; queries.len()];
        let mut used = vec![0; n as usize + 1];
        let mut count = 0;

        for i in 0..ids.len() {
            while left < logs.len() && logs[left][1] < queries[ids[i]] - x {
                used[logs[left][0] as usize] -= 1;
                if used[logs[left][0] as usize] == 0 {
                    count -= 1;
                }
                left += 1;
            }
            while right < logs.len() && logs[right][1] <= queries[ids[i]] {
                if used[logs[right][0] as usize] == 0 {
                    count += 1;
                }
                used[logs[right][0] as usize] += 1;
                right += 1;
            }
            ret[ids[i]] = n - count;
        }

        ret
    }
}
