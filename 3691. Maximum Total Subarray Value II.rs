use std::collections::BinaryHeap;
impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut logs = vec![0usize; n + 1];
        for len in 2..=n {
            logs[len] = logs[len / 2] + 1;
        }
        let levels = logs[n] + 1;
        let mut mins = vec![vec![0i32; n]; levels];
        let mut maxs = vec![vec![0i32; n]; levels];
        mins[0].clone_from_slice(&nums);
        maxs[0].clone_from_slice(&nums);
        for level in 1..levels {
            let span = 1usize << level;
            let half = span >> 1;
            for i in 0..=n - span {
                mins[level][i] =
                    mins[level - 1][i].min(mins[level - 1][i + half]);
                maxs[level][i] =
                    maxs[level - 1][i].max(maxs[level - 1][i + half]);
            }
        }
        let range_value = |left: usize, right: usize| -> i64 {
            let len = right - left + 1;
            let level = logs[len];
            let offset = right + 1 - (1usize << level);
            let min_value = mins[level][left].min(mins[level][offset]);
            let max_value = maxs[level][left].max(maxs[level][offset]);

            (max_value - min_value) as i64
        };
        let mut heap = BinaryHeap::new();
        for left in 0..n {
            heap.push((range_value(left, n - 1), left, n - 1));
        }
        let mut answer = 0i64;
        for _ in 0..k {
            let (value, left, right) = heap.pop().unwrap();
            answer += value;
            if right > left {
                heap.push((range_value(left, right - 1), left, right - 1));
            }
        }
        answer
    }
}
