use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut ans = f64::INFINITY;
        let mut quality_sum = 0;
        let mut workers = Vec::new();
        let mut max_heap = BinaryHeap::new();
        for i in 0..quality.len() {
            workers.push((wage[i] as f64 / quality[i] as f64, quality[i]));
        }
        workers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for (wage_per_quality, q) in workers {
            max_heap.push(q);
            quality_sum += q;
            if max_heap.len() > k as usize {
                if let Some(max_q) = max_heap.pop() {
                    quality_sum -= max_q;
                }
            }
            if max_heap.len() == k as usize {
                ans = ans.min(quality_sum as f64 * wage_per_quality);
            }
        }
        ans
    }
}
