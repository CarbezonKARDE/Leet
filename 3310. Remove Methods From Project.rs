use std::collections::VecDeque;
impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n_usize = n as usize;
        let k_usize = k as usize;
        let mut edges = vec![Vec::new(); n_usize];
        let mut in_degree = vec![0; n_usize];
        for inv in &invocations {
            let (u, v) = (inv[0] as usize, inv[1] as usize);
            edges[u].push(v);
            in_degree[v] += 1;
        }
        let mut queue = VecDeque::new();
        queue.push_back(k_usize);
        let mut suspicious = vec![false; n_usize];
        suspicious[k_usize] = true;
        while let Some(u) = queue.pop_front() {
            for &v in &edges[u] {
                in_degree[v] -= 1;
                if !suspicious[v] {
                    queue.push_back(v);
                    suspicious[v] = true;
                }
            }
        }
        let mut can_remove_all = true;
        let mut remaining = Vec::new();
        for i in 0..n_usize {
            if suspicious[i] && in_degree[i] > 0 {
                can_remove_all = false;
                break;
            } else if !suspicious[i] {
                remaining.push(i as i32);
            }
        }
        if !can_remove_all {
            return (0..n).collect();
        }
        remaining
    }
}
