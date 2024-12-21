impl Solution {
    pub fn max_k_divisible_components(n      : i32, 
                                      edges  : Vec<Vec<i32>>, 
                                      values : Vec<i32>, 
                                      k      : i32) 
        -> i32 
    {
        let mut graph = vec![Vec::with_capacity(3); n as usize];

        for e in edges {
            graph[e[0] as usize].push(e[1] as usize);
            graph[e[1] as usize].push(e[0] as usize);
        }
        Self::dfs(0, None, &graph, &values, k).0
    }
    fn dfs(node   : usize, 
           parent : Option<usize>, 
           graph  : &[Vec<usize>],
           values : &[i32],
           k      : i32) 
        -> (i32, i64)
    {
        let mut total   = values[node] as i64;
        let mut n_trees = 0;
        for &adj in &graph[node] {
            if Some(adj) != parent {
                let (n, t) = Self::dfs(adj, Some(node), graph, values, k);
                n_trees += n;
                total   += t;
            }
        }
        if total % k as i64 == 0 {
            n_trees += 1;
        }
        (n_trees, total)
    }
}
