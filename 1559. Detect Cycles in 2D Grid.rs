struct UnionFind {
    par: Vec<usize>,
    sizes: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            sizes: vec![1; n],
        }
    }
    fn find(&mut self, i: usize) -> usize {
        if i==self.par[i] {
            return i;
        }
        self.par[i] = self.find(self.par[i]);
        self.par[i]
    }
    fn union(&mut self, i:usize, j:usize) -> bool {
        let (pi, pj) = (self.find(i), self.find(j));
        if pi==pj {
            return false
        }
        if self.sizes[pi]<self.sizes[pj] {
            self.par[pi]=pj;
            self.sizes[pj]+=self.sizes[pi];
        } else {
            self.par[pj]=pi;
            self.sizes[pi]+=self.sizes[pj];
        }
        true
    }

}
impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let (nrow, ncol)=(grid.len(), grid[0].len());
        let mut uf = UnionFind::new(nrow*ncol);
        for i in 0..nrow {
            for j in 0..ncol {
                if i>0 && grid[i-1][j]==grid[i][j] {
                    if !uf.union((i-1)*ncol+j, i*ncol+j) {
                        return true;
                    }
                }
                if j>0 && grid[i][j-1]==grid[i][j] {
                    if !uf.union(i*ncol+(j-1), i*ncol+j) {
                        return true;
                    }
                }
            }
        }
        false 
    }
}
