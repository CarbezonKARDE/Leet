use std::collections::HashSet;
impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ans = vec![vec![]; n];
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
        }
        for i in 0..n {
            let mut seen = vec![false; n];
            Self::dfs(&graph, i, i, &mut seen, &mut ans);
        }
        for ancestors in &mut ans {
            ancestors.sort_unstable();
        }
        ans
    }
    fn dfs(graph: &[Vec<usize>], u: usize, ancestor: usize, seen: &mut [bool], ans: &mut [Vec<i32>]) {
        seen[u] = true;
        for &v in &graph[u] {
            if seen[v] {
                continue;
            }
            ans[v].push(ancestor as i32);
            Self::dfs(graph, v, ancestor, seen, ans);
        }
    }
}
