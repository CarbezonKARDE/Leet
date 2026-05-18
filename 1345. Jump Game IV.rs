use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &val) in arr.iter().enumerate() {
            map.entry(val).or_default().push(i);
        }
        let mut v = vec![false; n];
        let mut queue = VecDeque::new();
        queue.push_back(0);
        v[0] = true;
        let mut steps = 0;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let idx = queue.pop_front().unwrap();
                if idx == n - 1 {
                    return steps;
                }
                if idx >= 1 && !v[idx - 1] {
                    v[idx - 1] = true;
                    queue.push_back(idx - 1);
                }
                if idx + 1 < n && !v[idx + 1] {
                    v[idx + 1] = true;
                    queue.push_back(idx + 1);
                }
                if let Some(indices) = map.remove(&arr[idx]) {
                    for j in indices {
                        if !v[j] {
                            v[j] = true;
                            queue.push_back(j);
                        }
                    }
                }
            }
            steps += 1;
        }
        steps
    }
}
