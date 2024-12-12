use std::collections::BinaryHeap;
impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, mut k: i32) -> i64 {
        let mut max_heap = BinaryHeap::new();
        for &val in &gifts {
            max_heap.push(val);
        }
        while k > 0 {
            let val = max_heap.pop().unwrap();
            let rem = (val as f64).sqrt().floor() as i32;
            max_heap.push(rem);
            k -= 1;
        }
        let mut ans = 0;
        while let Some(val) = max_heap.pop() {
            ans += val as i64;
        }
        ans
    }
}
