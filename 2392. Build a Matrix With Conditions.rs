pub type TopoSortGraph = [Vec<usize>];
#[derive(Copy, Clone)]
enum TopoSortMark {
    Cleared,
    Temporary,
    Permanent,
}
use std::collections::VecDeque;
fn toposort_dfs(graph: &TopoSortGraph, marks: &mut [TopoSortMark], n: usize, mut rez: VecDeque<usize>) -> Option<VecDeque<usize>> {
    match marks[n] {
        TopoSortMark::Cleared => {
            marks[n] = TopoSortMark::Temporary;
            for m in &graph[n] {
                match toposort_dfs(graph, marks, *m, rez) {
                    Some(r) => rez = r,
                    None => return None,
                }
            }
            rez.push_front(n);
            marks[n] = TopoSortMark::Permanent;
            Some(rez)
        },
        TopoSortMark::Temporary => None,
        TopoSortMark::Permanent => Some(rez),
    }
}

pub fn toposort(graph: &TopoSortGraph) -> Option<Vec<usize>> {
    let mut marks = vec![TopoSortMark::Cleared; graph.len()];
    let mut rez = VecDeque::new();
    for n in 0..graph.len() {
        match toposort_dfs(graph, &mut marks, n, rez) {
            Some(r) => rez = r,
            None => return None,
        }
    }
    Some(rez.into_iter().collect())
}

fn build_graph(conditions: &[Vec<i32>], k: i32) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; k as usize];
    for condition in conditions {
        graph[(condition[0] - 1) as usize].push((condition[1] - 1) as usize);
    }
    graph
}

impl Solution {
    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        match (toposort(&build_graph(&row_conditions, k)), toposort(&build_graph(&col_conditions, k))) {
            (Some(r), Some(c)) => {
                let mut rez = vec![vec![0; k as usize]; k as usize];
                let mut rows = r.into_iter().enumerate().map(|(i, n)| (n, i)).collect::<Vec<_>>();
                let mut cols = c.into_iter().enumerate().map(|(i, n)| (n, i)).collect::<Vec<_>>();
                rows.sort_unstable();
                cols.sort_unstable();
                rows.into_iter().zip(cols.into_iter()).for_each(|((n, row), (_, col))| rez[row][col] = (n + 1) as i32);
                rez
            }
            _ => vec![],
        }
    }
}
