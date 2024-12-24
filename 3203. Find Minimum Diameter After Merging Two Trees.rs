use std::collections::VecDeque;
impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, 
                                        edges2: Vec<Vec<i32>>) 
        -> i32 
    {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let d1 = to_diameter(to_graph(edges1, n), n);
        let d2 = to_diameter(to_graph(edges2, m), m);
        ((d1 + 1) / 2 + (d2 + 1) / 2 + 1).max(d1).max(d2) as i32
    }
}
fn to_graph(edges: Vec<Vec<i32>>, n: usize) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n];
    for e in edges {
        let (u, v) = (e[0] as usize, e[1] as usize);
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}
fn to_diameter(graph: Vec<Vec<usize>>, n: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut seen  = vec![false; n];
    let mut start = 0;
    let mut diam  = 0;
    for i in 0..2 {
        queue.push_back(start);
        seen[start] = true;
        let mut height = 0;
        while !queue.is_empty() {
            height += 1;
            for _ in 0..queue.len() {
                let v = queue.pop_front().unwrap();
                start = v;
                for &u in &graph[v] {
                    if !seen[u] {
                        seen[u] = true;
                        queue.push_back(u);
                    }
                }
            }
        }
        if i == 0 {
            seen.fill(false);
        } else {
            diam = height - 1;
        }
    }
    diam
}
