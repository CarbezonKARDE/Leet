impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        if n == 2 {
            return vec![1,1];
        }
        let nu = n as usize;
        let mut edge_list = vec![vec![]; nu];
        for edge in edges{
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            edge_list[a].push(b);
            edge_list[b].push(a);
        }
        let ACTION_DOWN = 1;
        let ACTION_UP = 2;
        let mut ans: Vec<i32> = vec![0; nu];
        let mut count: Vec<i32> = vec![0; nu];
        let mut stack: Vec<(usize, usize, usize)> = vec![(ACTION_DOWN, 0, 0)];
        let mut i = 0;
        while let Some((action, p, v)) = stack.pop() {
            match action {
                _ if action == ACTION_DOWN => {
                    stack.push((ACTION_UP, p, v));
                    for xi in 0..edge_list[v].len() {
                        let x = edge_list[v][xi];
                        if p != x {
                            stack.push((ACTION_DOWN, v, x));
                        }
                    }
                },
                _ if action == ACTION_UP => {
                    for xi in 0..edge_list[v].len() {
                        let x = edge_list[v][xi];
                        if p != x {
                            count[v] += count[x];
                            ans[v] += ans[x] + count[x];
                        }
                    }
                    count[v] += 1;
                },
                _=> {}
            }
        }
        let mut stack2 = vec![(0,0)];
        while let Some((p, v)) = stack2.pop() {
            for xi in 0..edge_list[v].len() {
                let x = edge_list[v][xi];
                if p != x {
                    ans[x] = ans[v] - count[x] + n - count[x];
                    stack2.push((v, x))
                }
            }
        }
        ans
    }
}
